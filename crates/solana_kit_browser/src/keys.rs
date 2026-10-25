//! Explicit Web Crypto entropy for application key generation.

use core::{
    fmt,
    future::{Ready, ready},
};

use js_sys::{Function, Reflect, Uint8Array};
use solana_kit_core::crypto::Entropy;
use wasm_bindgen::{JsCast, JsValue};

use crate::BrowserCause;

const WEB_CRYPTO_MAXIMUM: usize = 65_536;

/// A Web Crypto failure retaining its exact JavaScript cause.
pub struct BrowserEntropyError(BrowserCause);

impl BrowserEntropyError {
    /// The original host exception. Do not log untrusted exception contents.
    #[must_use]
    pub fn host_cause(&self) -> &JsValue {
        self.0.value()
    }

    fn from_js(cause: JsValue) -> Self {
        Self(BrowserCause::new(cause))
    }
}

impl fmt::Debug for BrowserEntropyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("BrowserEntropyError(REDACTED)")
    }
}

impl fmt::Display for BrowserEntropyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("browser secure randomness is unavailable")
    }
}

impl std::error::Error for BrowserEntropyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

/// Caller-owned browser entropy. Construction creates no tasks or random output.
///
/// Uses only the host's Crypto.getRandomValues. There is no fallback to time,
/// Math.random or native randomness. Host errors fail the operation explicitly.
pub struct BrowserEntropy {
    crypto: JsValue,
    random: Function,
}

impl BrowserEntropy {
    /// Selects the current JavaScript global's Web Crypto implementation.
    /// This also works in workers exposing the same standard global API.
    ///
    /// # Errors
    /// Returns the original getter/type failure when Web Crypto is unavailable.
    pub fn new() -> Result<Self, BrowserEntropyError> {
        let crypto = Reflect::get(&js_sys::global(), &JsValue::from_str("crypto"))
            .map_err(BrowserEntropyError::from_js)?;
        let random = Reflect::get(&crypto, &JsValue::from_str("getRandomValues"))
            .map_err(BrowserEntropyError::from_js)?
            .dyn_into::<Function>()
            .map_err(BrowserEntropyError::from_js)?;

        Ok(Self { crypto, random })
    }

    fn fill_sync(&self, output: &mut [u8]) -> Result<(), BrowserEntropyError> {
        for chunk in output.chunks_mut(WEB_CRYPTO_MAXIMUM) {
            let bytes = Uint8Array::new_with_length(chunk.len() as u32);
            let result = self.random.call1(&self.crypto, bytes.as_ref());

            if let Err(cause) = result {
                bytes.fill(0, 0, bytes.length());

                return Err(BrowserEntropyError::from_js(cause));
            }

            bytes.copy_to(chunk);
            bytes.fill(0, 0, bytes.length());
        }

        Ok(())
    }
}

impl Entropy for BrowserEntropy {
    type Error = BrowserEntropyError;
    type Fill<'a> = Ready<Result<(), Self::Error>>;

    fn fill<'a>(&'a mut self, output: &'a mut [u8]) -> Self::Fill<'a> {
        ready(self.fill_sync(output))
    }
}
