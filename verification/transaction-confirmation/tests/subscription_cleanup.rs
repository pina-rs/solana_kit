//! Explicit cleanup drives the actual bounded protocol owner and acknowledgements.
use core::{
    future::{Future, Ready, ready},
    pin::Pin,
    task::{Context, Poll},
};
use solana_kit_client::{
    confirmation::{Confirmation, Event, Failure, Observer, Policy, WaitError, wait},
    subscriptions::{
        Budget, Connection, Drain, Failure as SubscriptionFailure, Slot, Subscription,
    },
};
use solana_kit_core::{Hash, Signature, transaction::Lifetime};
const GENERATION: u64 = 1;
const SERVER: u64 = 41;
struct Owner {
    connection: Connection,
    handle: Option<Subscription<Slot>>,
    accepted: bool,
}
fn request_id(frame: &[u8]) -> u64 {
    serde_json::from_slice::<serde_json::Value>(frame).unwrap()["id"]
        .as_u64()
        .unwrap()
}
fn ack(id: u64, result: serde_json::Value) -> Vec<u8> {
    serde_json::to_vec(&serde_json::json!({"jsonrpc":"2.0", "id":id, "result":result})).unwrap()
}
fn owner(accepted: bool) -> Owner {
    let budget = Budget {
        subscriptions: 1,
        ..Budget::default()
    };
    let mut connection = Connection::new(GENERATION, budget);
    let handle = connection.subscribe(Slot, 10).unwrap();
    let frame = connection.outbound().unwrap();
    connection
        .ingest(GENERATION, &ack(request_id(&frame), SERVER.into()))
        .unwrap();
    assert!(matches!(
        connection.subscribe(Slot, 10),
        Err(SubscriptionFailure::Capacity)
    ));
    Owner {
        connection,
        handle: Some(handle),
        accepted,
    }
}
struct Close<'a> {
    owner: &'a mut Owner,
    request: Option<u64>,
}
impl Future for Close<'_> {
    type Output = Result<(), Failure>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let Some(request) = self.request else {
            let this = &mut *self;
            let handle = this.owner.handle.as_ref().unwrap();
            let cancel = this.owner.connection.cancel(handle);

            if let Err(error) = cancel {
                return Poll::Ready(Err(Failure::Subscription(error)));
            }

            this.owner.connection.tick(1, 10).unwrap();
            let frame = this.owner.connection.outbound().unwrap();
            let json: serde_json::Value = serde_json::from_slice(&frame).unwrap();
            assert_eq!(json["method"], "slotUnsubscribe");
            this.request = Some(request_id(&frame));
            cx.waker().wake_by_ref();
            return Poll::Pending;
        };
        let this = &mut *self;
        let frame = ack(request, this.owner.accepted.into());
        this.owner.connection.ingest(GENERATION, &frame).unwrap();
        let handle = this.owner.handle.take().unwrap();
        let result = this.owner.connection.next(&handle);
        this.owner.connection.forget(handle).unwrap();
        assert!(this.owner.connection.subscribe(Slot, 10).is_ok());

        match result {
            Ok(Drain::Closed) => Poll::Ready(Ok(())),
            Err(error) => Poll::Ready(Err(Failure::Subscription(error))),
            _ => panic!("cleanup must settle only after a terminal acknowledgement"),
        }
    }
}
impl Observer for Owner {
    type Next<'a> = Ready<Result<Event, Failure>>;
    type Close<'a> = Close<'a>;
    fn next(&mut self) -> Self::Next<'_> {
        ready(Ok(Event::Timeout))
    }
    fn close(&mut self) -> Self::Close<'_> {
        Close {
            owner: self,
            request: None,
        }
    }
}
fn confirmation() -> Confirmation {
    Confirmation::new(
        Signature::from_bytes([7; 64]),
        Lifetime::Blockhash {
            blockhash: Hash::from_bytes([1; 32]),
            last_valid_block_height: 100,
        },
        Policy::default(),
    )
    .unwrap()
}
#[test]
fn quiet_cancel_awaits_actual_true_unsubscribe_before_returning() {
    let result = futures::executor::block_on(wait(confirmation(), owner(true), ready(())));
    assert!(matches!(
        result,
        Err(WaitError::Outcome(Failure::Cancelled))
    ));
}
#[test]
fn actual_false_unsubscribe_is_retained_with_original_timeout() {
    let result =
        futures::executor::block_on(wait(confirmation(), owner(false), core::future::pending()));
    assert!(
        matches!(result, Err(WaitError::Cleanup { outcome: Err(Failure::Timeout), source })
        if matches!(*source, Failure::Subscription(SubscriptionFailure::UnsubscribeFalse)))
    );
}
#[test]
fn old_handle_cannot_close_a_replacement_connection() {
    let mut observer = owner(true);
    observer.connection.replace(2).unwrap();
    let result =
        futures::executor::block_on(wait(confirmation(), observer, core::future::pending()));
    assert!(
        matches!(result, Err(WaitError::Cleanup { outcome: Err(Failure::Timeout), source })
        if matches!(*source, Failure::Subscription(SubscriptionFailure::Closed)))
    );
}
