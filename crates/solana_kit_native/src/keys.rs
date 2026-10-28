//! Explicit native entropy and atomic key backups for off-chain applications.
//!
//! File operations are blocking. The caller selects its blocking executor and
//! retains responsibility for joining that work before application teardown.

mod persistence;

use core::{
    fmt,
    future::{Ready, ready},
};
use std::sync::Arc;

use solana_kit_core::crypto::Entropy;
use tokio_rustls::rustls::crypto::{CryptoProvider, GetRandomFailed, ring};

pub use persistence::{KeyFileError, KeyFileStage, write_keypair_new};

/// A native secure provider's original randomness failure.
#[derive(Debug)]
pub struct NativeEntropyError(GetRandomFailed);

impl NativeEntropyError {
    /// Borrows the exact provider cause, which does not implement Error itself.
    #[must_use]
    pub fn provider_cause(&self) -> &GetRandomFailed {
        &self.0
    }
}

impl fmt::Display for NativeEntropyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("native secure randomness is unavailable")
    }
}

impl std::error::Error for NativeEntropyError {}

/// Operating-system randomness through the same audited provider as native TLS.
/// No runtime, global provider installation or secret cache is created.
#[derive(Clone, Debug)]
pub struct NativeEntropy {
    provider: Arc<CryptoProvider>,
}

impl NativeEntropy {
    /// Uses the caller's selected secure provider.
    #[must_use]
    pub fn new(provider: Arc<CryptoProvider>) -> Self {
        Self { provider }
    }

    fn fill_sync(&self, output: &mut [u8]) -> Result<(), NativeEntropyError> {
        self.provider
            .secure_random
            .fill(output)
            .map_err(NativeEntropyError)
    }
}

impl Default for NativeEntropy {
    fn default() -> Self {
        Self::new(Arc::new(ring::default_provider()))
    }
}

impl Entropy for NativeEntropy {
    type Error = NativeEntropyError;
    type Fill<'a> = Ready<Result<(), Self::Error>>;

    fn fill<'a>(&'a mut self, output: &'a mut [u8]) -> Self::Fill<'a> {
        ready(self.fill_sync(output))
    }
}
