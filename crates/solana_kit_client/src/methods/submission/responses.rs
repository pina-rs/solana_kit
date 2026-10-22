//! Signature execution and confirmation metadata are distinct.
use super::TransactionFailure;
use crate::{Error, methods::history::ConfirmationStatus};
use serde::Deserialize;

/// Positional status of a known signature; a missing signature is outer null.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignatureStatus {
    /// Exact node-observed slot.
    pub slot: u64,
    /// Null means rooted; zero is a valid present confirmation count.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub confirmations: Option<u64>,
    /// Null indicates successful execution, not finality.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub err: Option<TransactionFailure>,
    /// Deprecated node status, still a required field of this response schema.
    pub status: ExecutionStatus,
    /// Reported confirmation state; null means unavailable, absence is malformed.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub confirmation_status: Option<ConfirmationStatus>,
}

/// Deprecated transaction execution outcome, separate from RPC errors.
#[derive(Debug, Deserialize)]
pub enum ExecutionStatus {
    /// Execution succeeded; the JSON payload must be null.
    Ok(()),
    /// Execution failed with the node's typed failure.
    Err(TransactionFailure),
}
impl SignatureStatus {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        let consistent = match &self.status {
            ExecutionStatus::Ok(()) => self.err.is_none(),
            ExecutionStatus::Err(_) => self.err.is_some(),
        };

        if !consistent {
            return Err(schema("deprecated status contradicts execution error"));
        }

        Ok(())
    }
}
pub(crate) fn schema(message: &str) -> Error {
    Error::Schema(<serde_json::Error as serde::de::Error>::custom(message))
}
