//! Secret-bearing provider fields never reveal their contents through formatting.

use serde::{Deserialize, Serialize};
use std::fmt;

/// An explicitly exposed secret string. Serialization deliberately retains its bytes.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SensitiveString(String);

impl SensitiveString {
    /// Wraps a caller-supplied value without resolving ambient credentials.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Reveals the value for an explicit authenticated operation.
    pub fn expose(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for SensitiveString {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SensitiveString([REDACTED])")
    }
}

impl fmt::Display for SensitiveString {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("[REDACTED]")
    }
}
