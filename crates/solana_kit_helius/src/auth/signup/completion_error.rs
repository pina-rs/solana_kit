//! Completion failures never consume the caller's submitted-payment receipt.

use super::SignupError;
use crate::auth::checkout::PollError;

/// An observation or provisioning failure while resuming an already submitted payment.
#[derive(Debug)]
pub enum SignupCompletionError<E> {
    /// Checkout policy, caller timer, or service observation failed.
    Checkout(PollError<E>),
    /// Project-list policy, caller timer, or service observation failed.
    Provisioning(PollError<E>),
    /// No project appeared before the explicit time or observation budget ended.
    ProvisioningTimeout,
    /// Reading the discovered project or reusing/creating its key failed.
    Project(SignupError),
}

impl<E: core::fmt::Display> core::fmt::Display for SignupCompletionError<E> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Checkout(error) => {
                write!(formatter, "signup checkout observation failed: {error}")
            }
            Self::Provisioning(error) => {
                write!(formatter, "signup project observation failed: {error}")
            }
            Self::ProvisioningTimeout => formatter
                .write_str("payment completed but no project appeared within the caller's budget"),
            Self::Project(error) => {
                write!(formatter, "signup project provisioning failed: {error}")
            }
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for SignupCompletionError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Checkout(error) | Self::Provisioning(error) => Some(error),
            Self::Project(error) => Some(error),
            Self::ProvisioningTimeout => None,
        }
    }
}
