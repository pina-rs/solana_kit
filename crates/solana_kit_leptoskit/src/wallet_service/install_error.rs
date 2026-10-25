//! Native constructors retain causes before crossing the fixed framework factory.
use leptoskit_bridge::Error;
use std::{fmt, io};

/// Original native installation failure, with redacted automatic diagnostics.
pub enum WalletInstallError {
    /// The host-selected private directory could not be prepared.
    Directory(io::Error),
    /// The actual bridge refused registration or permission.
    Bridge(Error),
    /// The owned native worker thread could not be created.
    Worker(io::Error),
}

impl fmt::Debug for WalletInstallError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("WalletInstallError(REDACTED)")
    }
}

impl fmt::Display for WalletInstallError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("native wallet service installation failed")
    }
}

impl std::error::Error for WalletInstallError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Directory(error) | Self::Worker(error) => Some(error),
            Self::Bridge(error) => Some(error),
        }
    }
}

impl From<Error> for WalletInstallError {
    fn from(error: Error) -> Self {
        Self::Bridge(error)
    }
}
