extern crate std;
use super::polling::block_on;
use solana_kit_client::plans::{
    Boundary, Cancellation, ExecutionError, ExecutionMode, Executor, LeafOutcome, Sender, Signer,
    StageError, execute,
};
use solana_kit_core::{
    Address, Hash, Instruction, Signature,
    plans::{Plan, Status},
    transaction::{Lifetime, Message, Transaction, Version},
};
use std::format;
use std::prelude::v1::*;
use std::vec;
use std::{
    cell::{Cell, RefCell},
    future::{Future, Ready, ready},
    pin::Pin,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll, Wake},
};

fn message(index: u8) -> Message {
    Message::compile(
        Version::V0,
        Address::from_bytes([1; 32]),
        Lifetime::Blockhash {
            blockhash: Hash::from_bytes([2; 32]),
            last_valid_block_height: 7,
        },
        &[Instruction {
            program_address: Address::from_bytes([0; 32]),
            accounts: vec![],
            data: vec![index],
        }],
        &[],
    )
    .unwrap()
}
fn signature() -> Signature {
    Signature::from_bytes([7; 64])
}
fn plan() -> Plan<Message> {
    Plan::sequential((0..3).map(|index| Plan::Single(message(index))).collect())
}

struct Immediate {
    fail: u8,
    calls: Cell<usize>,
}
impl Executor for Immediate {
    type Context = usize;
    type Error = &'static str;
    type Execute<'a> = Ready<LeafOutcome<usize, &'static str>>;
    fn execute(&self, message: Message) -> Self::Execute<'_> {
        self.calls.set(self.calls.get() + 1);
        let result = if message.instructions()[0].data()[0] == self.fail {
            Err("declined")
        } else {
            Ok(signature())
        };

        ready(LeafOutcome {
            context: self.calls.get(),
            result,
        })
    }
}
pub(super) fn failed_sequence_retains_success_original_error_context_and_canceled_tail()
-> Result<(), &'static str> {
    let owner = Immediate {
        fail: 1,
        calls: Cell::new(0),
    };
    let error = block_on(execute(
        &owner,
        plan(),
        ExecutionMode::Ordered,
        &Cancellation::default(),
    ))
    .unwrap_err();
    let result = error.result().unwrap();
    let leaves = result.flatten();
    super::assert_eq!(owner.calls.get(), 2);
    super::assert_eq!(leaves[0].status, Status::Successful(signature()));
    super::assert_eq!(leaves[1].status, Status::Failed("declined"));
    super::assert_eq!(leaves[1].context, 2);

    super::assert_eq!(leaves[2].status, Status::Canceled);
    super::assert_eq!(result.first_failure().unwrap().planned_message, message(1));
    super::assert_eq!(result.summarize().successful.len(), 1);
    super::assert!(error.to_string().contains("[Tx #2] declined"));
    super::assert!(
        include_str!("../../../instruction-plans/oracle.tsv")
            .contains("failure\tfailed,canceled,canceled")
    );
    Ok(())
}

struct LocalOwner {
    started: Rc<RefCell<Vec<u8>>>,
    dropped: Rc<Cell<usize>>,
    ready: Rc<Cell<bool>>,
}
struct LocalFuture {
    value: u8,
    dropped: Rc<Cell<usize>>,
    ready: Rc<Cell<bool>>,
}
impl Drop for LocalFuture {
    fn drop(&mut self) {
        self.dropped.set(self.dropped.get() + 1);
    }
}
impl Future for LocalFuture {
    type Output = LeafOutcome<usize, &'static str>;
    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        if self.ready.get() {
            Poll::Ready(LeafOutcome {
                context: usize::from(self.value),
                result: Ok(signature()),
            })
        } else {
            context.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
impl Executor for LocalOwner {
    type Context = usize;
    type Error = &'static str;
    type Execute<'a> = LocalFuture;
    fn execute(&self, message: Message) -> Self::Execute<'_> {
        let value = message.instructions()[0].data()[0];
        self.started.borrow_mut().push(value);
        LocalFuture {
            value,
            dropped: self.dropped.clone(),
            ready: self.ready.clone(),
        }
    }
}
fn local_owner() -> LocalOwner {
    LocalOwner {
        started: Rc::default(),
        dropped: Rc::new(Cell::new(0)),
        ready: Rc::new(Cell::new(false)),
    }
}
struct Counter(AtomicUsize);
impl Wake for Counter {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}
pub(super) fn parallel_futures_overlap_preserve_order_and_wake_with_local_rc()
-> Result<(), &'static str> {
    let owner = local_owner();
    let cancellation = Cancellation::default();
    let input = Plan::Parallel(vec![Plan::Single(message(2)), Plan::Single(message(0))]);
    let mut future = Box::pin(execute(
        &owner,
        input,
        ExecutionMode::Ordered,
        &cancellation,
    ));
    let counter = Arc::new(Counter(AtomicUsize::new(0)));
    let waker = counter.clone().into();
    super::assert!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(&waker))
            .is_pending()
    );
    super::assert_eq!(*owner.started.borrow(), [2, 0]);

    super::assert_eq!(counter.0.load(Ordering::Relaxed), 2);
    owner.ready.set(true);
    let result = block_on(future).unwrap();
    super::assert_eq!(
        result
            .flatten()
            .iter()
            .map(|leaf| leaf.context)
            .collect::<Vec<_>>(),
        [2, 0]
    );
    super::assert_eq!(owner.dropped.get(), 2);
    Ok(())
}
pub(super) fn sequential_pending_cannot_overtake_and_drop_cancels_actual_owned_futures()
-> Result<(), &'static str> {
    let owner = local_owner();
    let cancellation = Cancellation::default();
    let mut future = Box::pin(execute(
        &owner,
        plan(),
        ExecutionMode::Ordered,
        &cancellation,
    ));
    let waker = Arc::new(Counter(AtomicUsize::new(0))).into();
    super::assert!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(&waker))
            .is_pending()
    );
    super::assert_eq!(*owner.started.borrow(), [0]);
    drop(future);
    super::assert_eq!(owner.dropped.get(), 1);
    Ok(())
}

pub(super) fn abort_settles_started_work_and_cancels_unstarted_tail() -> Result<(), &'static str> {
    let owner = local_owner();
    let cancellation = Cancellation::default();
    let mut future = Box::pin(execute(
        &owner,
        plan(),
        ExecutionMode::Ordered,
        &cancellation,
    ));
    let waker = Arc::new(Counter(AtomicUsize::new(0))).into();
    super::assert!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(&waker))
            .is_pending()
    );
    cancellation.cancel();
    owner.ready.set(true);
    let error = block_on(future).unwrap_err();

    let result = error.result().unwrap();
    super::assert_eq!(result.summarize().successful.len(), 1);
    super::assert_eq!(result.summarize().canceled.len(), 2);
    Ok(())
}
pub(super) fn concurrent_leaves_allow_atomic_groups_and_do_not_halt_on_failure()
-> Result<(), &'static str> {
    let owner = Immediate {
        fail: 1,
        calls: Cell::new(0),
    };
    let input = Plan::indivisible(vec![
        Plan::Single(message(0)),
        Plan::Single(message(1)),
        Plan::Single(message(2)),
    ]);
    super::assert!(matches!(
        block_on(execute(
            &owner,
            input.clone(),
            ExecutionMode::Ordered,
            &Cancellation::default()
        )),
        Err(ExecutionError::NonDivisible)
    ));
    super::assert_eq!(owner.calls.get(), 0);
    let error = block_on(execute(
        &owner,
        input,
        ExecutionMode::ConcurrentLeaves,
        &Cancellation::default(),
    ))
    .unwrap_err();
    super::assert_eq!(owner.calls.get(), 3);
    super::assert_eq!(error.result().unwrap().summarize().successful.len(), 2);
    super::assert!(matches!(
        error.result().unwrap(),
        Plan::Sequential {
            divisible: false,
            ..
        }
    ));
    Ok(())
}

struct Native;
impl Executor for Native {
    type Context = usize;
    type Error = &'static str;
    type Execute<'a> = Ready<LeafOutcome<usize, &'static str>>;
    fn execute(&self, _: Message) -> Self::Execute<'_> {
        ready(LeafOutcome {
            context: 1,
            result: Ok(signature()),
        })
    }
}

const CHANGED_MESSAGE: u8 = 99;
struct Signing {
    fail: bool,
    change: bool,
}
impl Signer for Signing {
    type Error = &'static str;
    type Sign<'a> = Ready<Result<Transaction, &'static str>>;
    fn sign(&self, planned: Message) -> Self::Sign<'_> {
        if self.fail {
            return ready(Err("sign rejected"));
        }

        let selected = if self.change {
            message(CHANGED_MESSAGE)
        } else {
            planned
        };

        ready(Ok(Transaction::new(selected)))
    }
}
struct Sending(bool);
impl Sender for Sending {
    type Error = &'static str;
    type Send<'a> = Ready<Result<Signature, &'static str>>;
    fn send(&self, _: Transaction) -> Self::Send<'_> {
        ready(if self.0 {
            Err("send rejected")
        } else {
            Ok(signature())
        })
    }
}
pub(super) fn boundary_preserves_unsigned_transaction_and_exact_signing_or_sending_error()
-> Result<(), &'static str> {
    for (sign_fail, send_fail, changed, expected) in [
        (true, false, false, StageError::Signing("sign rejected")),
        (false, true, false, StageError::Sending("send rejected")),
        (false, false, true, StageError::ChangedMessage),
    ] {
        let boundary = Boundary {
            signer: Signing {
                fail: sign_fail,
                change: changed,
            },
            sender: Sending(send_fail),
        };
        let error = block_on(execute(
            &boundary,
            Plan::Single(message(0)),
            ExecutionMode::Ordered,
            &Cancellation::default(),
        ))
        .unwrap_err();
        let result = error.result().unwrap();
        let failure = result.first_failure().unwrap();
        super::assert_eq!(failure.status, Status::Failed(expected));
        super::assert_eq!(failure.context.transaction.is_some(), !sign_fail);
    }

    Ok(())
}

pub(super) fn preflight_diagnostics_preserve_payload_indices_logs_and_signing_indicators()
-> Result<(), &'static str> {
    use solana_kit_client::plans::{FailureAction, FailureDetails, FailureReport};
    use solana_kit_core::plans::LeafResult;
    let logs: Vec<_> = (1..=12).map(|index| format!("Log line {index}")).collect();
    let details = FailureDetails {
        cause: "insufficient funds",
        logs: Some(logs),
        preflight: Some("original payload"),
        signature: Some(signature()),
    };
    let result = Plan::Single(LeafResult {
        planned_message: message(0),
        context: (),
        status: Status::Failed(details),
    });
    let report = FailureReport {
        result: &result,
        action: FailureAction::Send,
        abort_reason: None,
    };
    let text = report.to_string();
    super::assert!(text.starts_with(
        "Failed to send transaction (preflight): insufficient funds\n\nLogs (last 8 of 12):"
    ));
    super::assert!(!text.contains("  > Log line 4\n"));
    super::assert!(text.ends_with("  > Log line 12\n"));

    super::assert_eq!(report.failures()[0].1.preflight, Some("original payload"));
    let signing = FailureReport {
        action: FailureAction::Sign,
        ..report
    };
    super::assert!(
        signing
            .to_string()
            .starts_with("Failed to sign transaction: insufficient funds")
    );
    super::assert!(!signing.to_string().contains("(preflight)"));
    Ok(())
}

pub(super) fn actual_dart_failure_and_concurrent_result_oracles_match() -> Result<(), &'static str>
{
    let owner = Immediate {
        fail: 0,
        calls: Cell::new(0),
    };
    let error = block_on(execute(
        &owner,
        plan(),
        ExecutionMode::Ordered,
        &Cancellation::default(),
    ))
    .unwrap_err();
    let statuses = error
        .result()
        .unwrap()
        .flatten()
        .iter()
        .map(|leaf| match leaf.status {
            Status::Failed(_) => "failed",
            Status::Canceled => "canceled",
            Status::Successful(_) => "successful",
        })
        .collect::<Vec<_>>()
        .join(",");
    let oracle = include_str!("../../../instruction-plans/oracle.tsv");
    super::assert!(
        oracle
            .lines()
            .any(|line| line == format!("failure\t{statuses}"))
    );
    let result = block_on(execute(
        &Native,
        Plan::indivisible(vec![Plan::Single(message(0)), Plan::Single(message(1))]),
        ExecutionMode::ConcurrentLeaves,
        &Cancellation::default(),
    ))
    .unwrap();
    let statuses = result
        .flatten()
        .iter()
        .map(|leaf| match leaf.status {
            Status::Successful(_) => "successful",
            _ => "unexpected",
        })
        .collect::<Vec<_>>()
        .join(",");
    super::assert!(
        oracle
            .lines()
            .any(|line| line == format!("concurrent\t{statuses}"))
    );
    Ok(())
}
