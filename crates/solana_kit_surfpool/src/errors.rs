//! Surfpool diagnostics redact untrusted text while retaining explicit cause access.

use solana_kit_client::{Error, RemoteError};

/// A failed method, retaining HTTP, remote payload, schema and host causes.
///
/// Formatting includes only approved method names. The implicit error chain ends
/// here because provider values and adapter sources may contain secrets. Inspect
/// [`Self::cause`] explicitly when those original details are needed.
pub struct SurfpoolRpcError {
    /// Exact method being executed; custom names may contain sensitive text.
    pub method: &'static str,
    /// Original shared-client failure, without textual redaction or conversion.
    pub cause: Error,
}

impl SurfpoolRpcError {
    /// HTTP status when the transport returned a non-success response.
    pub const fn status_code(&self) -> Option<u16> {
        match self.cause {
            Error::Status(status) => Some(status),
            _ => None,
        }
    }

    /// Exact JSON-RPC code, including unknown provider codes.
    pub fn rpc_code(&self) -> Option<i64> {
        self.remote().map(|remote| remote.code)
    }

    /// Original provider message and exact missing/null/data distinction.
    pub const fn remote(&self) -> Option<&RemoteError> {
        match &self.cause {
            Error::Remote(remote) => Some(remote),
            _ => None,
        }
    }

    /// Original provider message; source data remains available separately.
    pub fn message(&self) -> Option<&str> {
        self.remote().map(|remote| remote.message.as_str())
    }

    /// Returns only library-approved literals for implicit diagnostic context.
    fn diagnostic_method(&self) -> &'static str {
        const RENT_METHOD: &str = "getMinimumBalanceForRentExemption";

        if self.method == RENT_METHOD {
            return RENT_METHOD;
        }

        crate::cheatcodes::SURFNET_CHEATCODE_METHODS
            .iter()
            .copied()
            .find(|method| *method == self.method)
            .unwrap_or("<custom>")
    }
}

impl core::fmt::Debug for SurfpoolRpcError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("SurfpoolRpcError")
            .field("method", &self.diagnostic_method())
            .finish_non_exhaustive()
    }
}

impl core::fmt::Display for SurfpoolRpcError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "Surfpool method {} failed",
            self.diagnostic_method()
        )
    }
}

impl std::error::Error for SurfpoolRpcError {}
