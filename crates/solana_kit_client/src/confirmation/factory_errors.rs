//! Error chains preserve original confirmation and separate cleanup failures.
use super::{FactoryError, FactoryFailure, NonceError};
use core::fmt;

impl fmt::Display for NonceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Account(error) => write!(formatter, "nonce account validation failed: {error}"),
            Self::Unusable(address) => write!(
                formatter,
                "nonce account {address} is not current and initialized"
            ),
            Self::Executable(address) => write!(formatter, "nonce account {address} is executable"),
        }
    }
}
impl core::error::Error for NonceError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Account(error) => Some(error),
            _ => None,
        }
    }
}
impl<E: fmt::Display> fmt::Display for FactoryFailure<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Confirmation(error) => error.fmt(formatter),
            Self::Session(error) => error.fmt(formatter),
            Self::Nonce(error) => error.fmt(formatter),
        }
    }
}
impl<E: core::error::Error + 'static> core::error::Error for FactoryFailure<E> {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Confirmation(error) => Some(error),
            Self::Session(error) => Some(error),
            Self::Nonce(error) => Some(error),
        }
    }
}
impl<E: fmt::Display> fmt::Display for FactoryError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.outcome {
            Ok(()) => write!(formatter, "confirmation succeeded"),
            Err(error) => error.fmt(formatter),
        }?;

        for error in &self.cleanup {
            write!(formatter, "; cleanup failed: {error}")?;
        }

        Ok(())
    }
}
impl<E: core::error::Error + 'static> core::error::Error for FactoryError<E> {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match &self.outcome {
            Err(error) => Some(error.as_ref()),
            Ok(()) => self.cleanup.first().map(|error| error as _),
        }
    }
}
