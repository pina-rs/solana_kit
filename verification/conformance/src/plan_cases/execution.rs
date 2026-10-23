extern crate std;
use solana_kit_client::plans::{Cancellation, ExecutionMode, Executor, LeafOutcome, execute};
use solana_kit_core::{
    Address, Hash, Instruction, Signature,
    plans::{Plan, Status},
    transaction::{Lifetime, Message, Version},
};
use std::prelude::v1::*;
use std::vec;
use std::{
    cell::{Cell, RefCell},
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, Waker},
};

fn message(index: u8) -> Message {
    Message::compile(
        Version::V0,
        Address::from_bytes([1; 32]),
        Lifetime::Blockhash {
            blockhash: Hash::from_bytes([2; 32]),
            last_valid_block_height: 4,
        },
        &[Instruction {
            program_address: Address::from_bytes([9; 32]),
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
fn nested() -> Plan<Message> {
    Plan::sequential(vec![
        Plan::Parallel(vec![
            Plan::sequential(vec![Plan::Single(message(0)), Plan::Single(message(1))]),
            Plan::Parallel(vec![Plan::Single(message(2)), Plan::Single(message(3))]),
        ]),
        Plan::Single(message(4)),
    ])
}
#[derive(Default)]
struct State {
    started: RefCell<Vec<u8>>,
    ready: [Cell<bool>; 5],
    wakers: RefCell<Vec<Waker>>,
    dropped: Cell<usize>,
}
struct LocalOwner {
    state: Rc<State>,
    error: Rc<str>,
}
struct LocalFuture {
    state: Rc<State>,
    index: u8,
    error: Rc<str>,
}
impl Drop for LocalFuture {
    fn drop(&mut self) {
        self.state.dropped.set(self.state.dropped.get() + 1);
    }
}
impl Future for LocalFuture {
    type Output = LeafOutcome<u8, Rc<str>>;
    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.state.ready[usize::from(self.index)].get() {
            self.state.wakers.borrow_mut().push(context.waker().clone());

            return Poll::Pending;
        }

        let result = if self.index == 2 {
            Err(self.error.clone())
        } else {
            Ok(signature())
        };

        Poll::Ready(LeafOutcome {
            context: self.index + 100,
            result,
        })
    }
}
impl Executor for LocalOwner {
    type Context = u8;
    type Error = Rc<str>;
    type Execute<'a> = LocalFuture;
    fn execute(&self, message: Message) -> Self::Execute<'_> {
        let index = message.instructions()[0].data()[0];
        self.state.started.borrow_mut().push(index);
        LocalFuture {
            state: self.state.clone(),
            index,
            error: self.error.clone(),
        }
    }
}
fn local_owner() -> LocalOwner {
    LocalOwner {
        state: Rc::default(),
        error: Rc::from("original"),
    }
}
fn release(owner: &LocalOwner, indices: &[usize]) {
    for index in indices {
        owner.state.ready[*index].set(true);
    }

    for waker in owner.state.wakers.borrow_mut().drain(..) {
        waker.wake();
    }
}
fn poll_once<F: Future>(future: Pin<&mut F>) -> Poll<F::Output> {
    future.poll(&mut Context::from_waker(Waker::noop()))
}
pub(super) fn nested_pending_failure_settles_started_siblings_and_cancels_both_serial_tails()
-> Result<(), &'static str> {
    let owner = local_owner();
    let cancellation = Cancellation::default();
    let input = nested();
    let mut future = Box::pin(execute(
        &owner,
        input.clone(),
        ExecutionMode::Ordered,
        &cancellation,
    ));
    super::assert!(poll_once(future.as_mut()).is_pending());
    super::assert_eq!(*owner.state.started.borrow(), [0, 2, 3]);
    super::assert_eq!(owner.state.wakers.borrow().len(), 3);
    release(&owner, &[2]);
    super::assert!(poll_once(future.as_mut()).is_pending());

    release(&owner, &[0, 3]);
    let Poll::Ready(Err(error)) = poll_once(future.as_mut()) else {
        return Err("must settle");
    };
    let result = error.result().unwrap();
    super::assert_eq!(result.clone().map(&mut |leaf| leaf.planned_message), input);
    let leaves = result.flatten();
    super::assert_eq!(
        leaves.iter().map(|leaf| leaf.context).collect::<Vec<_>>(),
        [100, 0, 102, 103, 0]
    );
    super::assert_eq!(leaves[0].status, Status::Successful(signature()));
    super::assert_eq!(leaves[1].status, Status::Canceled);
    let Status::Failed(cause) = &leaves[2].status else {
        return Err("original failure absent");
    };
    super::assert!(Rc::ptr_eq(cause, &owner.error));
    super::assert_eq!(leaves[3].status, Status::Successful(signature()));
    super::assert_eq!(leaves[4].status, Status::Canceled);
    super::assert_eq!(owner.state.dropped.get(), 3);
    super::assert_eq!(*owner.state.started.borrow(), [0, 2, 3]);
    Ok(())
}
pub(super) fn nested_abort_and_drop_observe_actual_local_future_ownership()
-> Result<(), &'static str> {
    let owner = local_owner();
    let cancellation = Cancellation::default();
    let mut future = Box::pin(execute(
        &owner,
        nested(),
        ExecutionMode::Ordered,
        &cancellation,
    ));
    super::assert!(poll_once(future.as_mut()).is_pending());
    cancellation.cancel();
    release(&owner, &[0, 2, 3]);
    let Poll::Ready(Err(error)) = poll_once(future.as_mut()) else {
        return Err("must settle");
    };

    super::assert_eq!(error.result().unwrap().summarize().canceled.len(), 2);
    super::assert_eq!(*owner.state.started.borrow(), [0, 2, 3]);
    super::assert_eq!(owner.state.dropped.get(), 3);
    let other = local_owner();
    let cancellation = Cancellation::default();
    let mut future = Box::pin(execute(
        &other,
        nested(),
        ExecutionMode::Ordered,
        &cancellation,
    ));
    super::assert!(poll_once(future.as_mut()).is_pending());
    drop(future);
    super::assert_eq!(other.state.dropped.get(), 3);
    Ok(())
}
pub(super) fn concurrent_leaves_cross_nested_sequence_edges_and_keep_error_tree()
-> Result<(), &'static str> {
    let owner = local_owner();
    let cancellation = Cancellation::default();
    let input = nested();
    let mut future = Box::pin(execute(
        &owner,
        input.clone(),
        ExecutionMode::ConcurrentLeaves,
        &cancellation,
    ));
    super::assert!(poll_once(future.as_mut()).is_pending());
    super::assert_eq!(*owner.state.started.borrow(), [0, 1, 2, 3, 4]);
    release(&owner, &[2]);
    super::assert!(poll_once(future.as_mut()).is_pending());

    release(&owner, &[0, 1, 3, 4]);
    let Poll::Ready(Err(error)) = poll_once(future.as_mut()) else {
        return Err("must settle");
    };
    let result = error.result().unwrap();
    super::assert_eq!(result.clone().map(&mut |leaf| leaf.planned_message), input);
    super::assert_eq!(result.summarize().successful.len(), 4);
    super::assert_eq!(result.summarize().failed.len(), 1);
    super::assert_eq!(result.summarize().canceled.len(), 0);
    super::assert_eq!(owner.state.dropped.get(), 5);
    Ok(())
}
