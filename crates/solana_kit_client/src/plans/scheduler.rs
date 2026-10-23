//! Owned leaf futures and an explicit tree scheduler.
use super::Cancellation;
use solana_kit_core::{
    Signature,
    plans::{LeafResult, Plan, Status, TransactionPlan, TransactionPlanResult},
    transaction::Message,
};
use std::{
    future::{Future, poll_fn},
    pin::Pin,
    task::{Context, Poll},
};

/// Result of one caller-owned async operation, including partial failure context.
#[derive(Debug)]
pub struct LeafOutcome<C, E> {
    /// Progress accumulated before success or failure.
    pub context: C,
    /// Explicit successful signature or the original error.
    pub result: Result<Signature, E>,
}

/// Async operation owner. No universal Send requirement or hidden runtime.
pub trait Executor {
    /// Caller-defined progress retained in every terminal leaf.
    type Context: Default;
    /// Original operation error.
    type Error;
    /// Concrete operation future; may contain local Rc state or remain Send.
    type Execute<'a>: Future<Output = LeafOutcome<Self::Context, Self::Error>> + 'a
    where
        Self: 'a;
    /// Starts one operation, owning the immutable planned message.
    fn execute(&self, message: Message) -> Self::Execute<'_>;
}

/// Traversal policy. ConcurrentLeaves explicitly ignores sequential dependencies.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExecutionMode {
    /// Honor sequence edges, run parallel branches concurrently, halt new work on failure.
    Ordered,
    /// Start all leaves concurrently, retain bundle flags, and let siblings finish on failure.
    ConcurrentLeaves,
}

/// A complete execution failure, including every successful, failed, or canceled leaf.
#[derive(Debug)]
pub enum ExecutionError<C, E> {
    /// Ordered execution cannot provide atomic multi-transaction bundle semantics.
    NonDivisible,
    /// The full result tree, also usable as a passthrough result.
    Failed(Box<TransactionPlanResult<C, E>>),
}
impl<C, E> ExecutionError<C, E> {
    /// Returns the complete partial result when execution started.
    #[must_use]
    pub fn result(&self) -> Option<&TransactionPlanResult<C, E>> {
        match self {
            Self::Failed(result) => Some(result),
            Self::NonDivisible => None,
        }
    }
}
impl<C, E: core::fmt::Display> core::fmt::Display for ExecutionError<C, E> {
    fn fmt(&self, output: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self::Failed(result) = self else {
            return output.write_str("non-divisible plans require an atomic bundle executor");
        };
        output.write_str("transaction plan execution failed")?;

        for (index, leaf) in result.flatten().into_iter().enumerate() {
            if let Status::Failed(error) = &leaf.status {
                write!(output, "\n[Tx #{}] {error}", index + 1)?;
            }
        }

        Ok(())
    }
}
impl<C: core::fmt::Debug, E: core::fmt::Debug + core::fmt::Display> core::error::Error
    for ExecutionError<C, E>
{
}

enum LeafState<F, C, E> {
    Waiting(Message),
    Running {
        message: Message,
        future: Pin<Box<F>>,
    },
    Complete(LeafResult<C, E>),
}

/// Executes using only caller polling. Failure never erases earlier results.
///
/// Cancellation prevents new work and retains active-operation results. Dropping
/// this future immediately drops all active futures; no task is detached.
///
/// # Errors
/// Rejects unsupported bundles before starting work. Other errors retain the full result tree.
pub async fn execute<O: Executor>(
    owner: &O,
    plan: TransactionPlan,
    mode: ExecutionMode,
    cancellation: &Cancellation,
) -> Result<TransactionPlanResult<O::Context, O::Error>, ExecutionError<O::Context, O::Error>> {
    if mode == ExecutionMode::Ordered
        && !plan.every(&|node| {
            !matches!(
                node,
                Plan::Sequential {
                    divisible: false,
                    ..
                }
            )
        })
    {
        return Err(ExecutionError::NonDivisible);
    }

    let mut state = plan.map(&mut LeafState::Waiting);
    let mut scheduler = Scheduler {
        owner,
        mode,
        cancellation,
        failed: false,
    };
    poll_fn(|context| scheduler.poll(&mut state, context)).await;
    let result = state.map(&mut |leaf| match leaf {
        LeafState::Complete(result) => result,
        LeafState::Waiting(_) | LeafState::Running { .. } => {
            unreachable!("completed plan contains an unfinished leaf")
        }
    });

    if result.summarize().is_successful() {
        Ok(result)
    } else {
        Err(ExecutionError::Failed(Box::new(result)))
    }
}

type State<'a, O> =
    Plan<LeafState<<O as Executor>::Execute<'a>, <O as Executor>::Context, <O as Executor>::Error>>;
struct Scheduler<'a, O> {
    owner: &'a O,
    mode: ExecutionMode,
    cancellation: &'a Cancellation,
    failed: bool,
}
impl<'a, O: Executor> Scheduler<'a, O> {
    fn start(&self, state: &mut State<'a, O>) {
        match state {
            Plan::Single(leaf) => self.start_leaf(leaf),
            Plan::Parallel(plans) => plans.iter_mut().for_each(|plan| self.start(plan)),
            Plan::Sequential { plans, .. } => self.start_sequence(plans),
        }
    }

    fn start_leaf(&self, leaf: &mut LeafState<O::Execute<'a>, O::Context, O::Error>) {
        let LeafState::Waiting(message) = leaf else {
            return;
        };
        let message = message.clone();

        if self.cancellation.is_canceled() || (self.failed && self.mode == ExecutionMode::Ordered) {
            *leaf = LeafState::Complete(LeafResult {
                planned_message: message,
                context: O::Context::default(),
                status: Status::Canceled,
            });

            return;
        }

        *leaf = LeafState::Running {
            future: Box::pin(self.owner.execute(message.clone())),
            message,
        };
    }

    fn start_sequence(&self, plans: &mut [State<'a, O>]) {
        if self.mode == ExecutionMode::ConcurrentLeaves {
            plans.iter_mut().for_each(|plan| self.start(plan));

            return;
        }

        if let Some(plan) = plans.iter_mut().find(|plan| !complete(plan)) {
            self.start(plan);
        }
    }

    fn poll(&mut self, state: &mut State<'a, O>, context: &mut Context<'_>) -> Poll<()> {
        self.start(state);

        match state {
            Plan::Single(leaf) => self.poll_leaf(leaf, context),

            Plan::Sequential { plans, .. } if self.mode == ExecutionMode::Ordered => {
                self.poll_sequence(plans, context)
            }

            Plan::Sequential { plans, .. } | Plan::Parallel(plans) => {
                self.poll_parallel(plans, context)
            }
        }
    }

    fn poll_leaf(
        &mut self,
        leaf: &mut LeafState<O::Execute<'a>, O::Context, O::Error>,
        context: &mut Context<'_>,
    ) -> Poll<()> {
        let LeafState::Running { message, future } = leaf else {
            return Poll::Ready(());
        };
        let Poll::Ready(outcome) = future.as_mut().poll(context) else {
            return Poll::Pending;
        };

        let status = match outcome.result {
            Ok(signature) => Status::Successful(signature),
            Err(error) => {
                self.failed = true;
                Status::Failed(error)
            }
        };
        *leaf = LeafState::Complete(LeafResult {
            planned_message: message.clone(),
            context: outcome.context,
            status,
        });

        Poll::Ready(())
    }

    fn poll_sequence(&mut self, plans: &mut [State<'a, O>], context: &mut Context<'_>) -> Poll<()> {
        for plan in plans {
            if self.poll(plan, context).is_pending() {
                return Poll::Pending;
            }
        }

        Poll::Ready(())
    }

    fn poll_parallel(&mut self, plans: &mut [State<'a, O>], context: &mut Context<'_>) -> Poll<()> {
        let mut ready = true;

        for plan in plans {
            ready &= self.poll(plan, context).is_ready();
        }

        if ready {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
fn complete<Operation, Progress, Failure>(
    plan: &Plan<LeafState<Operation, Progress, Failure>>,
) -> bool {
    plan.every(&|node| {
        !matches!(
            node,
            Plan::Single(LeafState::Waiting(_) | LeafState::Running { .. })
        )
    })
}
