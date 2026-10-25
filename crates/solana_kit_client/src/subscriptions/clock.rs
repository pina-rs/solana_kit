//! Fallible application time without a runtime or universal Send bound.
use crate::TransportError;
use core::future::Future;

/// A clock failure distinct from a successfully elapsed session deadline.
#[derive(Debug)]
pub enum ClockError {
    /// The clock's bounded readiness owner cannot admit another sleep.
    Capacity,
    /// The requested tick cannot be represented by the actual clock.
    DeadlineRange,
    /// The host clock is unavailable, retaining its actual cause when supplied.
    Unavailable(TransportError),
}

impl core::fmt::Display for ClockError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Capacity => formatter.write_str("clock readiness capacity exhausted"),
            Self::DeadlineRange => formatter.write_str("clock deadline is out of range"),
            Self::Unavailable(error) => write!(formatter, "clock unavailable: {error}"),
        }
    }
}

impl core::error::Error for ClockError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Unavailable(error) => Some(error),
            _ => None,
        }
    }
}

/// Application clock in one monotonic millisecond namespace.
pub trait Clock {
    /// Concrete native Send or local sleep. Construction and poll failures propagate.
    type Sleep<'a>: Future<Output = Result<(), ClockError>> + 'a
    where
        Self: 'a;
    /// Current tick. Values must never decrease.
    fn now(&self) -> u64;
    /// Waits until a tick and registers the current waker when pending.
    fn sleep_until(&self, deadline: u64) -> Self::Sleep<'_>;
}
