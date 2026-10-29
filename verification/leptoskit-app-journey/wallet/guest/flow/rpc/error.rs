//! Original failures remain available to the generated application's observer.
use super::AppError;
use solana_kit_client::confirmation::FactoryFailure;
use solana_kit_core::Signature;
use std::{convert::Infallible, fmt};

#[derive(Debug)]
pub(super) enum Error {
    Queue,
    Generation,
    Deadline,
    Cancelled,
    Subscription(&'static str),
    SubscriptionCleanup {
        source: AppError,
        cleanup: AppError,
    },
    MissingSignature,
    ReportedSignature {
        expected: Signature,
        actual: Signature,
    },
    Effect {
        expected: u64,
        actual: u64,
    },
    AmountOverflow,
    Reconciliation(FactoryFailure<Infallible>),
    Submitted {
        signature: Signature,
        source: AppError,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Subscription(message) => formatter.write_str(message),
            Self::SubscriptionCleanup { source, cleanup } => {
                write!(
                    formatter,
                    "Subscription failed: {source}; cleanup failed: {cleanup}"
                )
            }
            Self::Submitted { signature, .. } => write!(
                formatter,
                "Transfer outcome uncertain for {signature}. Check its status before another transfer."
            ),
            Self::Cancelled => formatter.write_str("Request cancelled; wallet disconnected"),
            Self::Deadline => formatter.write_str("Request deadline elapsed"),
            Self::Queue => formatter.write_str("Application action queue unavailable"),
            Self::Generation => formatter.write_str("Signing review sequence exhausted"),
            Self::MissingSignature => formatter.write_str("Approved signature missing"),
            Self::ReportedSignature { expected, actual } => write!(
                formatter,
                "Node returned {actual} for a transfer signed as {expected}"
            ),
            Self::Effect { expected, actual } => write!(
                formatter,
                "Recipient balance was {actual} lamports; expected {expected}"
            ),
            Self::AmountOverflow => formatter.write_str("Recipient balance would overflow"),
            Self::Reconciliation(error) => {
                write!(formatter, "Transaction observation failed: {error:?}")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Submitted { source, .. } | Self::SubscriptionCleanup { source, .. } => {
                Some(source.as_ref())
            }
            _ => None,
        }
    }
}
