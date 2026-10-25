//! Private app-sandbox persistence, selected only by explicit user actions.
use serde::Deserializer;
use serde::de::{SeqAccess, Visitor};
use solana_kit_core::crypto::{ExportableKeypair, KeypairImportError};
use solana_kit_native::keys::{KeyFileError, write_keypair_new};
use std::{
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
};
use zeroize::{Zeroize, Zeroizing};

const MAX_KEY_FILE_BYTES: usize = 1024;

pub(super) enum StorageError {
    Io(io::Error),
    Backup(KeyFileError),
    Key(solana_kit_core::crypto::KeypairImportError),
    Json(serde_json::Error),
}

impl std::fmt::Debug for StorageError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("WalletStorageError(REDACTED)")
    }
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("private wallet storage failed")
    }
}

impl std::error::Error for StorageError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Backup(error) => Some(error),
            Self::Key(error) => Some(error),
            Self::Json(error) => Some(error),
        }
    }
}

/// The host supplies a trusted sandbox root; guest bytes never select a path.
pub(super) struct Directory(PathBuf);

impl Directory {
    pub(super) fn new(data: &Path) -> io::Result<Self> {
        let canonical = data.canonicalize()?;

        if !canonical.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "wallet directory alias",
            ));
        }

        let directory = Self(canonical.join("solana-wallet"));
        directory.prepare()?;
        Ok(directory)
    }

    #[cfg(unix)]
    fn prepare(&self) -> io::Result<()> {
        use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
        let result = fs::DirBuilder::new().mode(0o700).create(&self.0);

        if let Err(error) = result {
            if error.kind() != io::ErrorKind::AlreadyExists {
                return Err(error);
            }
        }

        let metadata = fs::symlink_metadata(&self.0)?;

        if !metadata.is_dir()
            || metadata.permissions().mode() & 0o077 != 0
            || self.0.canonicalize()? != self.0
        {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "wallet directory is not private",
            ));
        }

        Ok(())
    }

    #[cfg(not(unix))]
    fn prepare(&self) -> io::Result<()> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "private wallet files require Unix permissions",
        ))
    }

    pub(super) fn save(&self, key: &ExportableKeypair, backup: bool) -> Result<(), StorageError> {
        self.prepare().map_err(StorageError::Io)?;
        let name = if backup {
            "wallet-backup.json"
        } else {
            "wallet.json"
        };

        write_keypair_new(&self.0.join(name), key).map_err(StorageError::Backup)
    }

    #[cfg(unix)]
    /// Reject leaf aliases and permissive or excessive files before parsing
    /// secret bytes; temporary key storage is erased on success and failure.
    pub(super) fn load(&self, backup: bool) -> Result<ExportableKeypair, StorageError> {
        use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
        self.prepare().map_err(StorageError::Io)?;
        let file = fs::OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NOFOLLOW)
            .open(self.0.join(if backup {
                "wallet-backup.json"
            } else {
                "wallet.json"
            }))
            .map_err(StorageError::Io)?;
        let metadata = file.metadata().map_err(StorageError::Io)?;

        if !metadata.is_file()
            || metadata.len() > MAX_KEY_FILE_BYTES as u64
            || metadata.permissions().mode() & 0o077 != 0
        {
            return Err(StorageError::Io(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "wallet file is not private",
            )));
        }

        let mut text = Zeroizing::new([0; MAX_KEY_FILE_BYTES + 1]);
        let length = read_bounded(file, &mut text[..]).map_err(StorageError::Io)?;

        if length > MAX_KEY_FILE_BYTES {
            return Err(StorageError::Io(io::Error::new(
                io::ErrorKind::InvalidData,
                "wallet file too large",
            )));
        }

        let mut bytes = Zeroizing::new([0; 64]);
        decode_key(&text[..length], &mut bytes)
    }

    #[cfg(not(unix))]
    pub(super) fn load(&self, _backup: bool) -> Result<ExportableKeypair, StorageError> {
        Err(StorageError::Io(io::Error::new(
            io::ErrorKind::Unsupported,
            "private wallet files require Unix permissions",
        )))
    }
}

fn read_bounded(mut reader: impl Read, bytes: &mut [u8]) -> io::Result<usize> {
    let mut length = 0;

    while length < bytes.len() {
        match reader.read(&mut bytes[length..]) {
            Ok(0) => break,
            Ok(read) => length += read,
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(error),
        }
    }

    Ok(length)
}

/// Decode into the existing secret owner; no parser-owned growable byte copy exists.
pub(super) fn decode_key(
    text: &[u8],
    bytes: &mut Zeroizing<[u8; 64]>,
) -> Result<ExportableKeypair, StorageError> {
    let result = (|| {
        let mut decoder = serde_json::Deserializer::from_slice(text);
        let length = decoder
            .deserialize_seq(KeyBytes(bytes))
            .map_err(StorageError::Json)?;
        decoder.end().map_err(StorageError::Json)?;

        if length != 64 {
            return Err(StorageError::Key(KeypairImportError::InvalidLength {
                actual: length,
            }));
        }

        ExportableKeypair::from_bytes(&bytes[..]).map_err(StorageError::Key)
    })();
    bytes.zeroize();
    result
}

struct KeyBytes<'a>(&'a mut [u8; 64]);

impl<'de> Visitor<'de> for KeyBytes<'_> {
    type Value = usize;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("an array of keypair bytes")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<usize, A::Error> {
        let mut length = 0;

        while let Some(byte) = sequence.next_element::<u8>()? {
            if let Some(slot) = self.0.get_mut(length) {
                *slot = byte;
            }

            length += 1;
        }

        Ok(length)
    }
}
