//! Complete backups are published with a no-overwrite hard link.

use std::{error::Error, fmt, io, path::Path};

#[cfg(unix)]
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::PathBuf,
};

use solana_kit_core::crypto::ExportableKeypair;

#[cfg(unix)]
use super::NativeEntropy;

#[cfg(unix)]
const DIRECTORY_NONCE_BYTES: usize = 16;
#[cfg(unix)]
const PRIVATE_DIRECTORY_MODE: u32 = 0o700;
#[cfg(unix)]
const PRIVATE_FILE_MODE: u32 = 0o600;

/// The fixed operation at which a private key backup failed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum KeyFileStage {
    /// Checking whether private file permissions are supported.
    CheckPlatform,
    /// Checking that the destination has a file name.
    ValidateDestination,
    /// Generating a random staging directory name.
    GenerateStagingName,
    /// Creating the destination's parent directories.
    CreateParentDirectory,
    /// Creating the private staging directory.
    CreateStagingDirectory,
    /// Opening a new private staging file.
    OpenStagingFile,
    /// Writing the key's JSON into the staging file.
    WriteStagingFile,
    /// Synchronizing the complete staging file.
    SyncStagingFile,
    /// Linking the complete file into the destination without overwriting.
    PublishDestination,
    /// Removing the staging file or directory after publication.
    Cleanup,
}

/// A backup failure with its known stage, exact I/O cause and publication outcome.
#[derive(Debug)]
pub struct KeyFileError {
    stage: Option<KeyFileStage>,
    source: io::Error,
    cleanup: Vec<io::Error>,
    published: bool,
}

impl KeyFileError {
    /// The operation that failed, without paths or key material.
    ///
    /// Library failures return `Some(stage)`; caller-supplied I/O conversions return `None`.
    #[must_use]
    pub fn stage(&self) -> Option<KeyFileStage> {
        self.stage
    }

    /// True means the complete destination was published before cleanup failed.
    #[must_use]
    pub fn published(&self) -> bool {
        self.published
    }

    /// Additional staging cleanup failures, without key material.
    #[must_use]
    pub fn cleanup_errors(&self) -> &[io::Error] {
        &self.cleanup
    }

    fn new(stage: KeyFileStage, source: io::Error) -> Self {
        Self {
            stage: Some(stage),
            source,
            cleanup: Vec::new(),
            published: false,
        }
    }
}

/// Writes Solana-keygen JSON without overwriting or following a destination link.
///
/// The caller must supply a trusted parent directory. On Unix, a new private
/// staging directory and file are created with permissions 0700 and 0600. Only
/// a fully flushed file is linked into the destination. Competing writers and
/// existing files, including symbolic links, fail atomically. Filesystems without
/// hard-link support fail rather than falling back to an overwriting rename.
///
/// # Errors
/// Returns the failed stage, original I/O error and all cleanup failures. On
/// platforms without the Unix permission primitive, returns Unsupported before
/// creating anything.
pub fn write_keypair_new(path: &Path, keypair: &ExportableKeypair) -> Result<(), KeyFileError> {
    write_supported(path, keypair)
}

#[cfg(not(unix))]
fn write_supported(_: &Path, _: &ExportableKeypair) -> Result<(), KeyFileError> {
    Err(KeyFileError::new(
        KeyFileStage::CheckPlatform,
        io::Error::new(
            io::ErrorKind::Unsupported,
            "private key-file storage requires Unix permissions",
        ),
    ))
}

#[cfg(unix)]
fn write_supported(path: &Path, keypair: &ExportableKeypair) -> Result<(), KeyFileError> {
    use std::os::unix::fs::OpenOptionsExt;

    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let name = path.file_name().ok_or_else(|| {
        KeyFileError::new(
            KeyFileStage::ValidateDestination,
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "backup destination requires a file name",
            ),
        )
    })?;
    let directory = staging_directory(parent)?;
    let staged = directory.join("keypair.json");

    let mut published = false;
    let operation = (|| -> Result<(), KeyFileError> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(PRIVATE_FILE_MODE)
            .open(&staged)
            .map_err(|source| KeyFileError::new(KeyFileStage::OpenStagingFile, source))?;
        write_staged(&mut file, keypair)?;
        fs::hard_link(&staged, parent.join(name))
            .map_err(|source| KeyFileError::new(KeyFileStage::PublishDestination, source))?;
        published = true;

        Ok(())
    })();
    let cleanup = cleanup(&staged, &directory);

    match (operation, cleanup.is_empty()) {
        (Ok(()), true) => Ok(()),
        (Ok(()), false) => Err(KeyFileError {
            stage: Some(KeyFileStage::Cleanup),
            source: io::Error::other("backup published but staging cleanup failed"),
            cleanup,
            published,
        }),
        (Err(mut error), _) => {
            error.cleanup = cleanup;
            error.published = published;
            Err(error)
        }
    }
}

#[cfg(unix)]
fn staging_directory(parent: &Path) -> Result<PathBuf, KeyFileError> {
    use std::os::unix::fs::DirBuilderExt;

    let mut nonce = [0; DIRECTORY_NONCE_BYTES];
    NativeEntropy::default()
        .fill_sync(&mut nonce)
        .map_err(|source| {
            KeyFileError::new(KeyFileStage::GenerateStagingName, io::Error::other(source))
        })?;
    let suffix = nonce
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    let directory = parent.join(format!(".solana-kit-keypair-{suffix}"));
    fs::DirBuilder::new()
        .recursive(true)
        .mode(PRIVATE_DIRECTORY_MODE)
        .create(parent)
        .map_err(|source| KeyFileError::new(KeyFileStage::CreateParentDirectory, source))?;
    fs::DirBuilder::new()
        .mode(PRIVATE_DIRECTORY_MODE)
        .create(&directory)
        .map_err(|source| KeyFileError::new(KeyFileStage::CreateStagingDirectory, source))?;

    Ok(directory)
}

#[cfg(unix)]
fn write_staged(file: &mut File, keypair: &ExportableKeypair) -> Result<(), KeyFileError> {
    write_json(file, keypair)
        .map_err(|source| KeyFileError::new(KeyFileStage::WriteStagingFile, source))?;
    file.sync_all()
        .map_err(|source| KeyFileError::new(KeyFileStage::SyncStagingFile, source))
}

#[cfg(unix)]
fn write_json(file: &mut File, keypair: &ExportableKeypair) -> io::Result<()> {
    let backup = keypair.backup();
    file.write_all(b"[")?;

    for (index, byte) in backup.expose_secret().iter().enumerate() {
        if index != 0 {
            file.write_all(b",")?;
        }

        write!(file, "{byte}")?;
    }

    file.write_all(b"]")
}

#[cfg(unix)]
fn cleanup(staged: &Path, directory: &Path) -> Vec<io::Error> {
    let mut errors = Vec::new();

    if let Err(error) = fs::remove_file(staged)
        && error.kind() != io::ErrorKind::NotFound
    {
        errors.push(error);
    }

    if let Err(error) = fs::remove_dir(directory) {
        errors.push(error);
    }

    errors
}

impl fmt::Display for KeyFileError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "key backup failed (published={}): {}",
            self.published, self.source
        )
    }
}

impl Error for KeyFileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

impl From<io::Error> for KeyFileError {
    fn from(source: io::Error) -> Self {
        Self {
            stage: None,
            source,
            cleanup: Vec::new(),
            published: false,
        }
    }
}

#[cfg(all(test, unix))]
mod tests;
