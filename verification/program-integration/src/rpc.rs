//! Confirmation observation and small RPC/domain helpers.

use core::future::{Future, ready};
use solana_kit_client::{
    Client, Transport,
    confirmation::{Event, Failure, Observer},
    methods::{
        Commitment, Contextual, ReadConfig,
        history::ConfirmationStatus,
        submission::{SignatureStatus, SignatureStatuses, StatusesConfig},
    },
};
use solana_kit_core::{Address, Signature};
use std::{
    cell::Cell,
    pin::Pin,
    time::{Duration, Instant},
};

const STATUS_POLL: Duration = Duration::from_millis(100);
const PROGRAM_STATE_BYTES: usize = 36;
const PROGRAM_STATE_TAG: u32 = 2;
const ADDRESS_BYTES: usize = 32;
const ADDRESS_OFFSET: usize = 4;

/// Confirmed read constraints used by every harness query.
#[must_use]
pub(crate) fn confirmed() -> ReadConfig {
    ReadConfig {
        commitment: Some(Commitment::Confirmed),
        min_context_slot: None,
    }
}

/// Classifies an RPC failure without exposing a remote body.
pub(crate) fn rpc_category(error: &solana_kit_client::Error) -> String {
    match error {
        solana_kit_client::Error::Transport(error) => format!("transport {:?}", error.kind),
        solana_kit_client::Error::Remote(error) => format!("remote code {}", error.code),
        solana_kit_client::Error::Status(status) => format!("HTTP status {status}"),
        other => format!("{other:?}"),
    }
}

/// Extracts the ProgramData address from a loader v3 Program account.
pub(crate) fn program_data_address(data: &[u8]) -> Option<Address> {
    if data.len() != PROGRAM_STATE_BYTES {
        return None;
    }

    let tag = u32::from_le_bytes(data.get(..4)?.try_into().ok()?);

    if tag != PROGRAM_STATE_TAG {
        return None;
    }

    let mut bytes = [0_u8; ADDRESS_BYTES];
    bytes.copy_from_slice(data.get(ADDRESS_OFFSET..PROGRAM_STATE_BYTES)?);

    Some(Address::from_bytes(bytes))
}

pub(crate) struct StatusObserver<'a, T> {
    client: &'a Client<T>,
    signature: Signature,
    deadline: Instant,
    observed_slot: &'a Cell<u64>,
}

impl<'a, T> StatusObserver<'a, T> {
    pub(crate) const fn new(
        client: &'a Client<T>,
        signature: Signature,
        deadline: Instant,
        observed_slot: &'a Cell<u64>,
    ) -> Self {
        Self {
            client,
            signature,
            deadline,
            observed_slot,
        }
    }
}

impl<T: Transport> Observer for StatusObserver<'_, T> {
    type Next<'a>
        = Pin<Box<dyn Future<Output = Result<Event, Failure>> + 'a>>
    where
        Self: 'a;
    type Close<'a>
        = core::future::Ready<Result<(), Failure>>
    where
        Self: 'a;

    fn next(&mut self) -> Self::Next<'_> {
        Box::pin(async move {
            if Instant::now() >= self.deadline {
                return Err(Failure::Timeout);
            }

            tokio::time::sleep(STATUS_POLL).await;
            let signatures = [self.signature];
            let response = self
                .client
                .call(SignatureStatuses(
                    &signatures,
                    Some(StatusesConfig {
                        search_transaction_history: Some(true),
                    }),
                ))
                .await
                .map_err(Failure::Rpc)?;
            let value = confirmed_status(
                response.value.into_iter().next().flatten(),
                self.observed_slot,
            );

            Ok(Event::Status {
                signature: self.signature,
                snapshot: Contextual {
                    context: response.context,
                    value,
                },
            })
        })
    }

    fn close(&mut self) -> Self::Close<'_> {
        ready(Ok(()))
    }
}

// The reducer can report an execution failure at processed commitment. These
// journeys compare confirmed post-state, so even failures must reach confirmed
// before returning; subsequent reads cannot precede the transaction's slot.
fn confirmed_status(
    status: Option<SignatureStatus>,
    observed: &Cell<u64>,
) -> Option<SignatureStatus> {
    let status = status?;

    if !matches!(
        status.confirmation_status,
        Some(ConfirmationStatus::Confirmed | ConfirmationStatus::Finalized)
    ) {
        return None;
    }

    observed.set(observed.get().max(status.slot));
    Some(status)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failed_execution_waits_for_confirmation_and_advances_the_read_floor() {
        let observed = Cell::new(7);
        let processed = r#"{"slot":10,"confirmations":0,"err":{"InstructionError":[0,{"Custom":1}]},"status":{"Err":{"InstructionError":[0,{"Custom":1}]}},"confirmationStatus":"processed"}"#;
        assert!(
            confirmed_status(Some(serde_json::from_str(processed).unwrap()), &observed).is_none()
        );
        assert_eq!(observed.get(), 7);

        let confirmed = processed.replace("processed", "confirmed");
        assert!(
            confirmed_status(Some(serde_json::from_str(&confirmed).unwrap()), &observed).is_some()
        );
        assert_eq!(observed.get(), 10);
        observed.set(12);
        assert!(
            confirmed_status(Some(serde_json::from_str(&confirmed).unwrap()), &observed).is_some()
        );
        assert_eq!(observed.get(), 12);
    }
}
