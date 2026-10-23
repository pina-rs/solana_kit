extern crate std;
use super::polling::block_on;
use solana_kit_core::{
    Address, Hash, Instruction,
    plans::{
        AsyncMessageBuilder, Error, InstructionBatch, InstructionLeaf, InstructionPlan,
        MaxInstructions, MessageDraft, MessagePacker, PackerFactory, Plan, Planner,
    },
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

const CALLBACK_PENDING_COUNT: u8 = 5;
const FINAL_CONTEXT: u8 = 9;

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
fn instruction(data: u8) -> Instruction {
    Instruction {
        program_address: Address::from_bytes([0; 32]),
        accounts: vec![],
        data: vec![data],
    }
}
fn input() -> InstructionPlan {
    Plan::sequential(vec![instruction(1).into(), instruction(2).into()])
}
#[derive(Default)]
struct State {
    context: Cell<u8>,
    events: RefCell<Vec<String>>,
    wakers: RefCell<Vec<Waker>>,
    dropped: Cell<usize>,
}
struct Local {
    state: Rc<State>,
    creates: usize,
    updates: usize,
    fail_create: usize,
    fail_update: usize,
    failure: Option<Error>,
}
impl Local {
    fn new(state: Rc<State>) -> Self {
        Self {
            state,
            creates: 0,
            updates: 0,
            fail_create: 0,
            fail_update: 0,
            failure: None,
        }
    }
}
struct Callback<'a> {
    owner: &'a mut Local,
    proposed: Option<MessageDraft>,
    pending: bool,
    update: bool,
}
impl Future for Callback<'_> {
    type Output = Result<MessageDraft, Error>;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.pending {
            self.pending = true;
            self.owner
                .state
                .wakers
                .borrow_mut()
                .push(cx.waker().clone());
            self.owner
                .state
                .events
                .borrow_mut()
                .push(format!("pending:{}", self.update));
            cx.waker().wake_by_ref();

            return Poll::Pending;
        }

        let update = self.update;
        let owner = &mut self.owner;
        let count = if update {
            owner.updates += 1;
            owner.updates
        } else {
            owner.creates += 1;
            owner.creates
        };

        owner
            .state
            .events
            .borrow_mut()
            .push(format!("ready:{update}:{count}"));

        if (update && count == owner.fail_update) || (!update && count == owner.fail_create) {
            return Poll::Ready(Err(owner
                .failure
                .clone()
                .unwrap_or_else(|| Error::Builder(format!("original:{update}:{count}")))));
        }

        let context = owner.state.context.get();
        let mut output = self.proposed.take().unwrap_or_else(draft);

        if update {
            output.lifetime = Lifetime::Blockhash {
                blockhash: Hash::from_bytes([context; 32]),
                last_valid_block_height: u64::from(context),
            };
        }

        Poll::Ready(Ok(output))
    }
}
impl Drop for Callback<'_> {
    fn drop(&mut self) {
        self.owner
            .state
            .dropped
            .set(self.owner.state.dropped.get() + 1);
    }
}
impl AsyncMessageBuilder for Local {
    type Create<'a> = Callback<'a>;
    type Update<'a> = Callback<'a>;
    fn create_async(&mut self) -> Self::Create<'_> {
        Callback {
            owner: self,
            proposed: None,
            pending: false,
            update: false,
        }
    }
    fn update_async(&mut self, proposed: MessageDraft) -> Self::Update<'_> {
        Callback {
            owner: self,
            proposed: Some(proposed),
            pending: false,
            update: true,
        }
    }
}
struct Wakes(AtomicUsize);
impl Wake for Wakes {
    fn wake(self: Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}
pub(super) fn local_pending_callbacks_read_each_actual_proposal_context() -> Result<(), &'static str>
{
    let state = Rc::new(State::default());
    let mut planner = Planner::new(Local::new(state.clone()), MaxInstructions::new(1).unwrap());
    let plan = input();
    let mut future = Box::pin(planner.plan_async(&plan));
    let wakes = Arc::new(Wakes(AtomicUsize::new(0)));
    let waker = Waker::from(wakes.clone());
    let mut cx = Context::from_waker(&waker);

    for context in 1..=CALLBACK_PENDING_COUNT {
        state.context.set(context);
        super::assert!(future.as_mut().poll(&mut cx).is_pending());
        super::assert_eq!(wakes.0.load(Ordering::SeqCst), usize::from(context));
        super::assert_eq!(state.wakers.borrow().len(), usize::from(context));
    }

    state.context.set(FINAL_CONTEXT);
    state.wakers.borrow().last().unwrap().wake_by_ref();
    super::assert_eq!(
        wakes.0.load(Ordering::SeqCst),
        usize::from(CALLBACK_PENDING_COUNT) + 1
    );
    let Poll::Ready(Ok(result)) = future.as_mut().poll(&mut cx) else {
        return Err("callbacks must finish after exactly five pending polls");
    };
    let leaves = result.flatten();
    super::assert_eq!(leaves.len(), 2);
    super::assert_eq!(
        leaves[0].lifetime(),
        Lifetime::Blockhash {
            blockhash: Hash::from_bytes([3; 32]),
            last_valid_block_height: 3
        }
    );

    super::assert_eq!(
        leaves[1].lifetime(),
        Lifetime::Blockhash {
            blockhash: Hash::from_bytes([9; 32]),
            last_valid_block_height: 9
        }
    );
    super::assert_eq!(
        *state.events.borrow(),
        [
            "pending:false",
            "ready:false:1",
            "pending:true",
            "ready:true:1",
            "pending:true",
            "ready:true:2",
            "pending:false",
            "ready:false:2",
            "pending:true",
            "ready:true:3"
        ]
    );
    Ok(())
}

// Both failure stages must preserve the first accepted candidate before the
// later callback fails; the two counters deliberately exercise different paths.
pub(super) fn later_construction_and_update_errors_return_original_without_a_tree()
-> Result<(), &'static str> {
    for update in [false, true] {
        let state = Rc::new(State::default());
        let mut builder = Local::new(state.clone());

        if update {
            builder.fail_update = 3;
        } else {
            builder.fail_create = 2;
        }

        let mut planner = Planner::new(builder, MaxInstructions::new(1).unwrap());

        super::assert_eq!(
            block_on(planner.plan_async(&input())),
            Err(Error::Builder(format!(
                "original:{update}:{}",
                if update { 3 } else { 2 }
            )))
        );
        super::assert!(
            state
                .events
                .borrow()
                .iter()
                .any(|event| event == "ready:true:1")
        );
    }

    Ok(())
}
pub(super) fn dropping_pending_create_or_update_releases_owned_callback_and_builder()
-> Result<(), &'static str> {
    for polls in [1, 2] {
        let state = Rc::new(State::default());
        let weak = Rc::downgrade(&state);
        let mut planner = Planner::new(Local::new(state.clone()), MaxInstructions::default());
        let plan = input();
        let mut future = Box::pin(planner.plan_async(&plan));
        let waker = Waker::noop();

        for _ in 0..polls {
            super::assert!(
                future
                    .as_mut()
                    .poll(&mut Context::from_waker(waker))
                    .is_pending()
            );
        }

        drop(future);
        super::assert_eq!(state.dropped.get(), polls);
        drop(planner);
        drop(state);
        super::assert!(weak.upgrade().is_none());
    }

    Ok(())
}
#[derive(Clone)]
struct Stalled(usize);
impl PackerFactory for Stalled {
    type Packer = Self;
    fn create(&self) -> Self {
        Self(0)
    }
}
impl MessagePacker for Stalled {
    fn done(&self) -> bool {
        false
    }
    fn pack(
        &mut self,
        candidate: &MessageDraft,
        _: MaxInstructions,
    ) -> Result<MessageDraft, Error> {
        self.0 += 1;
        std::assert!(self.0 <= 2, "unbounded empty-candidate retry");
        Ok(candidate.clone())
    }
}
pub(super) fn async_no_progress_aborts_after_one_construction() -> Result<(), &'static str> {
    let state = Rc::new(State::default());
    let mut planner = Planner::new(Local::new(state.clone()), MaxInstructions::default());
    let plan = Plan::Single(InstructionLeaf::Packer(Stalled(0)));
    super::assert_eq!(block_on(planner.plan_async(&plan)), Err(Error::NoProgress));
    super::assert_eq!(*state.events.borrow(), ["pending:false", "ready:false:1"]);
    Ok(())
}
pub(super) fn stateful_async_updater_failure_after_prior_candidate_aborts_without_retry()
-> Result<(), &'static str> {
    let state = Rc::new(State::default());
    let mut builder = Local::new(state.clone());
    builder.fail_update = 2;
    let failure = Error::Rejected("updater rejected a consumed proposal".into());
    builder.failure = Some(failure.clone());
    let mut planner = Planner::new(builder, MaxInstructions::new(2).unwrap());
    let plan = Plan::sequential(vec![
        instruction(1).into(),
        Plan::Single(InstructionLeaf::Packer(InstructionBatch(vec![
            instruction(2),
        ]))),
    ]);
    super::assert_eq!(block_on(planner.plan_async(&plan)), Err(failure));

    super::assert_eq!(
        state
            .events
            .borrow()
            .iter()
            .filter(|event| event.starts_with("ready:false"))
            .count(),
        1
    );
    Ok(())
}

pub(super) fn async_dart_callback_trace_and_message_bytes_match() -> Result<(), &'static str> {
    use base64::Engine;
    let state = Rc::new(State::default());
    let mut planner = Planner::new(Local::new(state.clone()), MaxInstructions::new(1).unwrap());
    let result = block_on(planner.plan_async(&input())).unwrap();
    let mut oracle = include_str!("../../../instruction-plans/async_oracle.tsv").lines();
    super::assert_eq!(
        oracle.next(),
        Some("create:1,update:1:1,update:2:2,create:2,update:3:1")
    );
    super::assert_eq!(
        state
            .events
            .borrow()
            .iter()
            .filter(|event| event.starts_with("ready:"))
            .cloned()
            .collect::<Vec<_>>(),
        [
            "ready:false:1",
            "ready:true:1",
            "ready:true:2",
            "ready:false:2",
            "ready:true:3"
        ]
    );

    for message in result.flatten() {
        super::assert_eq!(
            base64::engine::general_purpose::STANDARD.encode(message.bytes()),
            oracle.next().unwrap()
        );
    }

    super::assert!(oracle.next().is_none());
    Ok(())
}

pub(super) fn synchronous_wildcard_imports_keep_unambiguous_builder_methods()
-> Result<(), &'static str> {
    use solana_kit_core::plans::*;
    struct Pure;
    impl MessageBuilder for Pure {
        fn create(&mut self) -> Result<MessageDraft, Error> {
            Ok(draft())
        }
    }
    super::assert_eq!(Pure.create().unwrap(), draft());
    super::assert_eq!(Pure.update(draft()).unwrap(), draft());
    super::assert_eq!(
        Planner::new(Pure, MaxInstructions::default())
            .plan(&input())
            .unwrap()
            .flatten()
            .len(),
        1
    );
    Ok(())
}

struct StageCalls(Arc<AtomicUsize>);
impl solana_kit_client::plans::Signer for StageCalls {
    type Error = u8;
    type Sign<'a> = std::future::Ready<Result<solana_kit_core::transaction::Transaction, u8>>;
    fn sign(&self, _: solana_kit_core::transaction::Message) -> Self::Sign<'_> {
        self.0.fetch_add(1, Ordering::SeqCst);
        std::future::ready(Err(1))
    }
}
impl solana_kit_client::plans::Sender for StageCalls {
    type Error = u8;
    type Send<'a> = std::future::Ready<Result<solana_kit_core::Signature, u8>>;
    fn send(&self, _: solana_kit_core::transaction::Transaction) -> Self::Send<'_> {
        self.0.fetch_add(1, Ordering::SeqCst);
        std::future::ready(Err(2))
    }
}
pub(super) fn boundary_awaits_later_planning_error_before_starting_signing_or_sending()
-> Result<(), &'static str> {
    use solana_kit_client::plans::{Boundary, BoundaryOutcome, Cancellation};
    let state = Rc::new(State::default());
    let mut builder = Local::new(state);
    builder.fail_create = 2;
    let mut planner = Planner::new(builder, MaxInstructions::new(1).unwrap());
    let calls = Arc::new(AtomicUsize::new(0));
    let boundary = Boundary {
        signer: StageCalls(calls.clone()),
        sender: StageCalls(calls.clone()),
    };
    let outcome = block_on(boundary.run(&mut planner, &input(), &Cancellation::default()));
    super::assert!(
        matches!(outcome, BoundaryOutcome::PlanningFailed(Error::Builder(error)) if error == "original:false:2")
    );

    super::assert_eq!(calls.load(Ordering::SeqCst), 0);
    Ok(())
}
