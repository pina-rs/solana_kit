//! Null branch presence and completion-time deadlines remain admission boundaries.

use super::checks::{Outcome, require};
use super::{routes::ingestion, support::*};
use solana_kit_client::{Presence, Reply, TransportError, rest};
use solana_kit_dapp_publisher::portal::*;
use std::{
    cell::Cell,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    vec,
};

#[cfg_attr(test, test)]
pub fn null_envelope_branches_cannot_masquerade_as_missing_fields() -> Outcome {
    for envelope in [
        r#"{"0":{"result":{"action":"deleted"}},"result":null}"#,
        r#"{"result":{"data":{"action":"deleted"}},"error":null}"#,
        r#"{"result":{"data":{"_tag":"Right","right":{"action":"deleted"},"left":null}}}"#,
        r#"{"result":{"data":null,"action":"deleted"}}"#,
        r#"{"result":null}"#,
        r#"{"error":null}"#,
        r#"{"result":{"data":{"_tag":"Right","right":null}}}"#,
        r#"{"result":{"data":{"_tag":"Left","left":null}}}"#,
        r#"{"result":{"data":{"_tag":"Left","left":{},"right":null}}}"#,
    ] {
        let (client, state) = client(vec![reply(200, envelope)]);
        require(
            complete(client.call(&CleanupRelease {
                release_id: "fixture".into(),
            }))?
            .is_err(),
            "null portal branch masqueraded as absence",
        )?;
        require(
            state.requests.borrow().len() == 1,
            "null envelope exchange count changed",
        )?;
    }
    Ok(())
}

#[cfg_attr(test, test)]
pub fn legitimate_nullable_backend_fields_still_survive_every_valid_envelope_form() -> Outcome {
    for envelope in [
        r#"{"result":{"hubspotTicketId":null}}"#,
        r#"{"result":{"data":{"hubspotTicketId":null}}}"#,
        r#"{"0":{"result":{"data":{"hubspotTicketId":null}}}}"#,
        r#"{"result":{"data":{"_tag":"Right","right":{"hubspotTicketId":null}}}}"#,
    ] {
        let (client, state) = client(vec![reply(200, envelope)]);
        let result = complete(client.call(&SubmitToStore {
            release_id: "fixture".into(),
            whats_new: "fixture".into(),
            critical_update: false,
            testing_instructions: None,
            is_resubmission: None,
            attestation: StoreAttestation {
                payload: "synthetic".into(),
                request_unique_id: "fixture".into(),
            },
        }))?
        .map_err(|_| "legitimate nullable receipt rejected")?;

        require(
            matches!(result.hubspot_ticket_id, Presence::Present(None)),
            "nullable receipt field changed",
        )?;
        require(
            state.requests.borrow().len() == 1,
            "nullable receipt exchange count changed",
        )?;
    }
    Ok(())
}

struct AdvancingTransport<'a> {
    clock: &'a TestClock,
    completion: u64,
    calls: &'a Cell<usize>,
}

impl rest::Transport<PortalService> for AdvancingTransport<'_> {
    type Exchange<'a>
        = AdvancingExchange<'a>
    where
        Self: 'a;

    fn exchange(&self, _: rest::Request<PortalService>) -> Self::Exchange<'_> {
        self.calls.set(self.calls.get() + 1);
        AdvancingExchange {
            clock: self.clock,
            completion: self.completion,
        }
    }
}

struct AdvancingExchange<'a> {
    clock: &'a TestClock,
    completion: u64,
}

impl Future for AdvancingExchange<'_> {
    type Output = Result<Reply, TransportError>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        self.clock.now.set(self.completion);
        Poll::Ready(Ok(Reply {
            status: 200,
            body: br#"{"result":{"data":{"id":"completed"}}}"#.to_vec(),
        }))
    }
}

/// Advancing the fake clock inside Ready distinguishes deadline ties without wall-clock races.
#[cfg_attr(test, test)]
pub fn deadline_wins_when_ready_exchange_advances_clock_to_or_past_completion_limit() -> Outcome {
    const DEADLINE: u64 = 100;

    for completion in [DEADLINE - 1, DEADLINE, DEADLINE + 1] {
        let clock = TestClock::new(DEADLINE);
        let calls = Cell::new(0);
        let transport = AdvancingTransport {
            clock: &clock,
            completion,
            calls: &calls,
        };
        let client = Client::new(transport, BUDGETS);
        let policy = IngestionRetry {
            delay_ms: 1,
            deadline: DEADLINE,
        };
        let result = complete(client.create_ingestion_with_retry(&ingestion(), &clock, policy))?;

        if completion < DEADLINE {
            let session = result.map_err(|_| "pre-deadline Ready response rejected")?;
            require(
                session.id == Presence::Present(Some("completed".into())),
                "pre-deadline Ready identity changed",
            )?;
        } else {
            require(
                matches!(result, Err(Error::Deadline)),
                "Ready completion won a deadline tie",
            )?;
        }

        require(calls.get() == 1, "Ready arbitration exchange count changed")?;
        require(
            clock.dropped.get() == 1,
            "Ready arbitration timer did not drop",
        )?;
    }
    Ok(())
}
