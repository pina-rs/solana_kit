//! Original local JavaScript failures behind redacted HTTP diagnostics.
use std::{error::Error, fmt};
use wasm_bindgen::JsValue;

/// A browser HTTP failure's exact original JavaScript value.
///
/// Obtain this cause by traversing `std::error::Error::source` and downcasting.
/// Normal diagnostics omit the value. Explicit access can expose private request
/// details held by JavaScript. This local cause does not promise `Send` or `Sync`.
pub struct BrowserCause(JsValue);

impl BrowserCause {
    pub(crate) fn new(value: JsValue) -> Self {
        Self(value)
    }

    /// Borrows the original value, preserving JavaScript object identity.
    pub fn value(&self) -> &JsValue {
        &self.0
    }
}

impl fmt::Debug for BrowserCause {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("BrowserCause")
    }
}

impl fmt::Display for BrowserCause {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("browser HTTP failure")
    }
}

impl Error for BrowserCause {}
