extern crate std;
use solana_kit_core::{
    Address, Hash, Instruction,
    plans::*,
    transaction::{Lifetime, Version},
};
use std::format;
use std::prelude::v1::*;
use std::vec;
use std::{
    cell::{Cell, RefCell},
    future::Future,
    pin::Pin,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll, Wake, Waker},
};

#[derive(Clone, Copy, Default)]
enum Failure {
    #[default]
    None,
    Rejected,
    Capacity,
    Prefix,
    Builder,
}
#[derive(Default)]
struct Control {
    permits: Cell<usize>,
    context: Cell<u8>,
    calls: RefCell<Vec<String>>,
    registered: RefCell<Option<Waker>>,
    polls: Cell<usize>,
    drops: Cell<usize>,
    fail_at: usize,
    failure: Failure,
}
struct Builder(Rc<Control>);
struct Callback {
    state: Rc<Control>,
    id: usize,
    proposal: Option<MessageDraft>,
}
fn instruction(tag: u8) -> Instruction {
    Instruction {
        program_address: Address::from_bytes([9; 32]),
        accounts: vec![],
        data: vec![tag],
    }
}
fn draft() -> MessageDraft {
    MessageDraft {
        version: Version::V0,
        payer: Address::from_bytes([1; 32]),
        lifetime: Lifetime::Blockhash {
            blockhash: Hash::from_bytes([0; 32]),
            last_valid_block_height: 1,
        },
        instructions: vec![],
        lookup_tables: vec![],
        config: None,
    }
}
impl Builder {
    fn callback(&self, proposal: Option<MessageDraft>) -> Callback {
        let event = match &proposal {
            Some(value) => format!("update:{}", value.instructions.len()),
            None => "create:0".into(),
        };

        self.0.calls.borrow_mut().push(event);
        Callback {
            state: self.0.clone(),
            id: self.0.calls.borrow().len(),
            proposal,
        }
    }
}
impl AsyncMessageBuilder for Builder {
    type Create<'a> = Callback;
    type Update<'a> = Callback;
    fn create_async(&mut self) -> Callback {
        self.callback(None)
    }
    fn update_async(&mut self, proposed: MessageDraft) -> Callback {
        self.callback(Some(proposed))
    }
}
impl Future for Callback {
    type Output = Result<MessageDraft, Error>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.state.polls.set(self.state.polls.get() + 1);

        if self.state.permits.get() < self.id {
            *self.state.registered.borrow_mut() = Some(cx.waker().clone());

            return Poll::Pending;
        }

        let mut output = self.proposal.take().unwrap_or_else(draft);

        if self.id == self.state.fail_at {
            match self.state.failure {
                Failure::Rejected => {
                    return Poll::Ready(Err(Error::Rejected("consumed proposal".into())));
                }
                Failure::Builder => {
                    return Poll::Ready(Err(Error::Builder("nested update denied".into())));
                }
                Failure::Capacity => output.instructions.push(instruction(99)),
                Failure::Prefix => output.instructions.clear(),
                Failure::None => {}
            }
        }

        let value = self.state.context.get();
        output.lifetime = Lifetime::Blockhash {
            blockhash: Hash::from_bytes([value; 32]),
            last_valid_block_height: u64::from(value),
        };
        Poll::Ready(Ok(output))
    }
}
impl Drop for Callback {
    fn drop(&mut self) {
        self.state.drops.set(self.state.drops.get() + 1);
    }
}
struct Wakes(AtomicUsize);
impl Wake for Wakes {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}
fn drive<T>(future: impl Future<Output = T>, state: &Control) -> T {
    let mut future = Box::pin(future);
    let wakes = Arc::new(Wakes(AtomicUsize::new(0)));
    let waker = Waker::from(wakes.clone());
    let mut cx = Context::from_waker(&waker);

    loop {
        let Poll::Pending = future.as_mut().poll(&mut cx) else {
            std::panic!("each callback must first register readiness");
        };
        let calls = state.calls.borrow().len();

        for _ in 0..3 {
            std::assert!(future.as_mut().poll(&mut cx).is_pending());
            std::assert_eq!(state.calls.borrow().len(), calls);
        }

        state.context.set(u8::try_from(calls * 10).unwrap());
        state.permits.set(calls);
        state.registered.borrow_mut().take().unwrap().wake();
        std::assert_eq!(wakes.0.load(Ordering::SeqCst), calls);

        if let Poll::Ready(value) = future.as_mut().poll(&mut cx) {
            return value;
        }
    }
}

pub(super) fn external_registered_wakes_gate_each_actual_proposal_without_reacquisition()
-> Result<(), &'static str> {
    let state = Rc::new(Control::default());
    let input: InstructionPlan = Plan::sequential((1..=3).map(|i| instruction(i).into()).collect());
    let mut planner = Planner::new(Builder(state.clone()), MaxInstructions::new(2).unwrap());
    let result = drive(planner.plan_async(&input), &state).unwrap();
    let messages = result.flatten();
    super::assert_eq!(
        *state.calls.borrow(),
        [
            "create:0", "update:1", "update:2", "update:3", "create:0", "update:1"
        ]
    );
    super::assert_eq!(messages.len(), 2);

    super::assert_eq!(
        messages[0]
            .instructions()
            .iter()
            .map(|i| i.data()[0])
            .collect::<Vec<_>>(),
        [1, 2]
    );
    super::assert_eq!(messages[1].instructions()[0].data(), [3]);
    super::assert_eq!(
        messages[0].lifetime(),
        Lifetime::Blockhash {
            blockhash: Hash::from_bytes([30; 32]),
            last_valid_block_height: 30
        }
    );
    super::assert_eq!(
        messages[1].lifetime(),
        Lifetime::Blockhash {
            blockhash: Hash::from_bytes([60; 32]),
            last_valid_block_height: 60
        }
    );
    super::assert_eq!(state.drops.get(), 6);
    Ok(())
}

#[derive(Default)]
struct CursorState {
    calls: Cell<usize>,
    consumed: Cell<usize>,
    drops: Cell<usize>,
}
struct Factory(Rc<CursorState>);
struct Cursor {
    state: Rc<CursorState>,
    done: bool,
}
impl PackerFactory for Factory {
    type Packer = Cursor;
    fn create(&self) -> Cursor {
        Cursor {
            state: self.0.clone(),
            done: false,
        }
    }
}
impl MessagePacker for Cursor {
    fn done(&self) -> bool {
        self.done
    }
    fn pack(
        &mut self,
        candidate: &MessageDraft,
        _: MaxInstructions,
    ) -> Result<MessageDraft, Error> {
        self.state.calls.set(self.state.calls.get() + 1);
        std::assert!(!self.done, "consumed proposal was retried");
        let mut output = candidate.clone();
        output.instructions.push(instruction(2));
        self.done = true;
        self.state.consumed.set(self.state.consumed.get() + 1);
        Ok(output)
    }
}
impl Drop for Cursor {
    fn drop(&mut self) {
        self.state.drops.set(self.state.drops.get() + 1);
    }
}

pub(super) fn consumed_cursor_update_rejection_capacity_and_invalid_prefix_abort_without_retry()
-> Result<(), &'static str> {
    for failure in [Failure::Rejected, Failure::Capacity, Failure::Prefix] {
        let state = Rc::new(Control {
            fail_at: 3,
            failure,
            ..Control::default()
        });
        let cursor = Rc::new(CursorState::default());
        let input = Plan::sequential(vec![
            instruction(1).into(),
            Plan::Single(InstructionLeaf::Packer(Factory(cursor.clone()))),
        ]);
        let mut planner = Planner::new(Builder(state.clone()), MaxInstructions::new(2).unwrap());
        let result = drive(planner.plan_async(&input), &state);

        let expected = match failure {
            Failure::Rejected => Error::Rejected("consumed proposal".into()),
            Failure::Capacity => Error::InstructionLimit {
                actual: 3,
                maximum: 2,
            },
            _ => Error::InvalidPackerOutput,
        };

        super::assert_eq!(result, Err(expected));
        super::assert_eq!(*state.calls.borrow(), ["create:0", "update:1", "update:2"]);
        super::assert_eq!(
            (
                cursor.calls.get(),
                cursor.consumed.get(),
                cursor.drops.get()
            ),
            (1, 1, 1)
        );
    }

    Ok(())
}

pub(super) fn dropping_pending_consumed_update_releases_cursor_and_callback_owners()
-> Result<(), &'static str> {
    let state = Rc::new(Control::default());
    let cursor = Rc::new(CursorState::default());
    let state_weak = Rc::downgrade(&state);
    let cursor_weak = Rc::downgrade(&cursor);
    let input = Plan::sequential(vec![
        instruction(1).into(),
        Plan::Single(InstructionLeaf::Packer(Factory(cursor.clone()))),
    ]);
    let mut planner = Planner::new(Builder(state.clone()), MaxInstructions::new(2).unwrap());

    state.permits.set(2);
    let mut future = Box::pin(planner.plan_async(&input));
    let mut cx = Context::from_waker(Waker::noop());

    for _ in 0..5 {
        super::assert!(future.as_mut().poll(&mut cx).is_pending());
    }

    super::assert_eq!(*state.calls.borrow(), ["create:0", "update:1", "update:2"]);
    super::assert_eq!(cursor.consumed.get(), 1);
    super::assert_eq!(cursor.drops.get(), 0);

    drop(future);
    super::assert_eq!(cursor.drops.get(), 1);
    super::assert_eq!(state.drops.get(), 3);
    drop(planner);
    drop(input);
    drop(state);
    drop(cursor);

    super::assert!(state_weak.upgrade().is_none());
    super::assert!(cursor_weak.upgrade().is_none());
    Ok(())
}

pub(super) fn later_nested_update_error_keeps_original_cause_and_stops_unstarted_tail()
-> Result<(), &'static str> {
    let state = Rc::new(Control {
        fail_at: 6,
        failure: Failure::Builder,
        ..Control::default()
    });
    let input: InstructionPlan = Plan::sequential(vec![
        instruction(1).into(),
        Plan::Parallel(vec![
            Plan::indivisible(vec![instruction(2).into(), instruction(3).into()]),
            instruction(4).into(),
        ]),
        instruction(5).into(),
    ]);
    let mut planner = Planner::new(Builder(state.clone()), MaxInstructions::new(1).unwrap());
    super::assert_eq!(
        drive(planner.plan_async(&input), &state),
        Err(Error::Builder("nested update denied".into()))
    );
    super::assert_eq!(state.calls.borrow().len(), 6);
    super::assert_eq!(state.drops.get(), 6);
    Ok(())
}
