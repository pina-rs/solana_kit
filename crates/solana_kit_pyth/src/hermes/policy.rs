//! Finite retry and per-attempt deadlines, independent of the selected runtime.

/// Official default root; custom roots and credentials belong to the HTTP adapter.
pub const DEFAULT_BASE_URL: &str = "https://hermes.pyth.network";
/// Maximum explicitly configured retries, excluding the initial request.
pub const MAX_HTTP_RETRIES: u8 = 32;

/// Invalid timing policy. Diagnostics contain no supplied endpoint or credentials.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigError {
    /// Each exchange needs a positive timeout in the clock's millisecond namespace.
    ZeroTimeout,
    /// The requested retry count exceeds the finite client limit.
    RetryLimit,
    /// An exponential delay cannot be represented in monotonic milliseconds.
    BackoffRange,
}

impl core::fmt::Display for ConfigError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "invalid Hermes timing policy: {self:?}")
    }
}

impl std::error::Error for ConfigError {}

/// Validated per-attempt timeout and finite exponential retry schedule.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Policy {
    pub(super) timeout_ms: u64,
    pub(super) http_retries: u8,
    pub(super) backoff_ms: u64,
}

impl Policy {
    /// Validates every possible delay before any request can start.
    ///
    /// # Errors
    /// Rejects zero timeout, more than `MAX_HTTP_RETRIES` retries or delay overflow.
    pub fn new(timeout_ms: u64, http_retries: u8, backoff_ms: u64) -> Result<Self, ConfigError> {
        if timeout_ms == 0 {
            return Err(ConfigError::ZeroTimeout);
        }

        if http_retries > MAX_HTTP_RETRIES {
            return Err(ConfigError::RetryLimit);
        }

        let multiplier = 1_u64 << http_retries.saturating_sub(1);

        if backoff_ms.checked_mul(multiplier).is_none() {
            return Err(ConfigError::BackoffRange);
        }

        Ok(Self {
            timeout_ms,
            http_retries,
            backoff_ms,
        })
    }

    /// Maximum monotonic milliseconds for each individual exchange.
    pub const fn timeout_ms(self) -> u64 {
        self.timeout_ms
    }
    /// Retries after the initial request, up to the explicit finite limit.
    pub const fn http_retries(self) -> u8 {
        self.http_retries
    }
    /// First retry delay in monotonic milliseconds; later delays double.
    pub const fn backoff_ms(self) -> u64 {
        self.backoff_ms
    }
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            timeout_ms: 5_000,
            http_retries: 3,
            backoff_ms: 100,
        }
    }
}
