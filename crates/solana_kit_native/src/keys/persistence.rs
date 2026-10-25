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

/// A backup failure with its exact I/O cause and publication outcome.
#[derive(Debug)]
pub struct KeyFileError {
    source: io::Error,
    cleanup: Vec<io::Error>,
    published: bool,
}

impl KeyFileError {
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
/// Returns the original I/O error and all cleanup failures. On platforms without
/// the Unix permission primitive, returns Unsupported before creating anything.
pub fn write_keypair_new(path: &Path, keypair: &ExportableKeypair) -> Result<(), KeyFileError> {
    write_supported(path, keypair)
}

#[cfg(not(unix))]
fn write_supported(_: &Path, _: &ExportableKeypair) -> Result<(), KeyFileError> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "private key-file storage requires Unix permissions",
    )
    .into())
}

#[cfg(unix)]
fn write_supported(path: &Path, keypair: &ExportableKeypair) -> Result<(), KeyFileError> {
    use std::os::unix::fs::OpenOptionsExt;

    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let name = path.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "backup destination requires a file name",
        )
    })?;
    let directory = staging_directory(parent)?;
    let staged = directory.join("keypair.json");

    let mut published = false;
    let operation = (|| -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(PRIVATE_FILE_MODE)
            .open(&staged)?;
        write_json(&mut file, keypair)?;
        file.sync_all()?;
        fs::hard_link(&staged, parent.join(name))?;
        published = true;

        Ok(())
    })();
    let cleanup = cleanup(&staged, &directory);

    match (operation, cleanup.is_empty()) {
        (Ok(()), true) => Ok(()),
        (Ok(()), false) => Err(KeyFileError {
            source: io::Error::other("backup published but staging cleanup failed"),
            cleanup,
            published,
        }),
        (Err(source), _) => Err(KeyFileError {
            source,
            cleanup,
            published,
        }),
    }
}

#[cfg(unix)]
fn staging_directory(parent: &Path) -> io::Result<PathBuf> {
    use std::os::unix::fs::DirBuilderExt;

    let mut nonce = [0; DIRECTORY_NONCE_BYTES];
    NativeEntropy::default()
        .fill_sync(&mut nonce)
        .map_err(io::Error::other)?;
    let suffix = nonce
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    let directory = parent.join(format!(".solana-kit-keypair-{suffix}"));
    fs::DirBuilder::new()
        .recursive(true)
        .mode(PRIVATE_DIRECTORY_MODE)
        .create(parent)?;
    fs::DirBuilder::new()
        .mode(PRIVATE_DIRECTORY_MODE)
        .create(&directory)?;

    Ok(directory)
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

    if let Err(error) = fs::remove_file(staged) {
        if error.kind() != io::ErrorKind::NotFound {
            errors.push(error);
        }
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
            source,
            cleanup: Vec::new(),
            published: false,
        }
    }
}
