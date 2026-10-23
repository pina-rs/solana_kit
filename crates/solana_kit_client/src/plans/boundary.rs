//! Concrete associated futures compose caller-owned signing and sending.
use super::{Cancellation, ExecutionError, ExecutionMode, Executor, LeafOutcome, execute};
use solana_kit_core::{
    Signature,
    plans::{
        AsyncMessageBuilder, InstructionPlan, PackerFactory, Planner, TransactionPlan,
        TransactionPlanResult,
    },
    transaction::{Message, Transaction},
};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// Caller-owned signing capability.
pub trait Signer {
    /// Original signer failure.
    type Error;
    /// Concrete signing future, preserving local or Send behavior.
    type Sign<'a>: Future<Output = Result<Transaction, Self::Error>> + 'a
    where
        Self: 'a;
    /// Signs an owned immutable message.
    fn sign(&self, message: Message) -> Self::Sign<'_>;
}
/// Caller-owned submission capability, without retries or polling.
pub trait Sender {
    /// Original submission failure.
    type Error;
    /// Concrete submission future.
    type Send<'a>: Future<Output = Result<Signature, Self::Error>> + 'a
    where
        Self: 'a;
    /// Sends an owned transaction and reports its explicit signature.
    fn send(&self, transaction: Transaction) -> Self::Send<'_>;
}

/// Typed execution-stage error retaining the original cause.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StageError<E> {
    /// Signer failed.
    Signing(E),
    /// Sender failed after signing completed.
    Sending(E),
    /// The signer changed the planned immutable message or lifetime.
    ChangedMessage,
}
impl<E: core::fmt::Display> core::fmt::Display for StageError<E> {
    fn fmt(&self, output: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Signing(error) => write!(output, "signing failed: {error}"),
            Self::Sending(error) => write!(output, "sending failed: {error}"),
            Self::ChangedMessage => output.write_str("signer changed the planned message"),
        }
    }
}
impl<E: core::fmt::Display + core::fmt::Debug> core::error::Error for StageError<E> {}

/// Typed partial progress; an unsigned returned transaction cannot erase an error.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BoundaryContext {
    /// The exact transaction returned by the signer, retained when sending fails.
    pub transaction: Option<Transaction>,
}

/// A workflow's planning failure or its complete execution outcome.
#[derive(Debug)]
pub enum BoundaryOutcome<E> {
    /// No execution started.
    PlanningFailed(solana_kit_core::plans::Error),
    /// Planning succeeded; result includes every partial leaf on execution failure.
    Executed(Box<BoundaryExecution<E>>),
}

/// The immutable plan and complete execution outcome after successful planning.
#[derive(Debug)]
pub struct BoundaryExecution<E> {
    /// Immutable planned tree.
    pub plan: TransactionPlan,
    /// Success or complete structured execution failure.
    pub result: Result<
        TransactionPlanResult<BoundaryContext, StageError<E>>,
        ExecutionError<BoundaryContext, StageError<E>>,
    >,
}

/// Explicit composition of signing and sending operations.
pub struct Boundary<S, T> {
    /// Caller-owned signer.
    pub signer: S,
    /// Caller-owned sender.
    pub sender: T,
}
impl<S: Signer, T: Sender<Error = S::Error>> Boundary<S, T> {
    /// Plans, signs and sends, retaining exact stage errors and the complete result tree.
    pub async fn run<B: AsyncMessageBuilder, F: PackerFactory>(
        &self,
        planner: &mut Planner<B>,
        instructions: &InstructionPlan<F>,
        cancellation: &Cancellation,
    ) -> BoundaryOutcome<S::Error> {
        let plan = match planner.plan_async(instructions).await {
            Ok(plan) => plan,
            Err(error) => return BoundaryOutcome::PlanningFailed(error),
        };

        let result = execute(self, plan.clone(), ExecutionMode::Ordered, cancellation).await;

        BoundaryOutcome::Executed(Box::new(BoundaryExecution { plan, result }))
    }
}
impl<S: Signer, T: Sender<Error = S::Error>> Executor for Boundary<S, T> {
    type Context = BoundaryContext;
    type Error = StageError<S::Error>;
    type Execute<'a>
        = BoundaryFuture<'a, S, T>
    where
        Self: 'a;
    fn execute(&self, message: Message) -> Self::Execute<'_> {
        BoundaryFuture {
            planned: message.clone(),
            sender: &self.sender,
            state: BoundaryState::Signing(Box::pin(self.signer.sign(message))),
        }
    }
}

enum BoundaryState<SF, TF> {
    Signing(Pin<Box<SF>>),
    Sending {
        future: Pin<Box<TF>>,
        transaction: Box<Transaction>,
    },
    Complete,
}
/// Named future retaining concrete signer and sender futures without unsafe projection.
pub struct BoundaryFuture<'a, S: Signer + 'a, T: Sender<Error = S::Error> + 'a> {
    planned: Message,
    sender: &'a T,
    state: BoundaryState<S::Sign<'a>, T::Send<'a>>,
}
impl<S: Signer, T: Sender<Error = S::Error>> Future for BoundaryFuture<'_, S, T> {
    type Output = LeafOutcome<BoundaryContext, StageError<S::Error>>;
    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        if let BoundaryState::Signing(future) = &mut this.state {
            let result = core::task::ready!(future.as_mut().poll(context));

            return this.start_sending(result, context);
        }

        this.poll_sending(context)
    }
}
impl<S: Signer, T: Sender<Error = S::Error>> BoundaryFuture<'_, S, T> {
    fn start_sending(
        &mut self,
        signed: Result<Transaction, S::Error>,
        context: &mut Context<'_>,
    ) -> Poll<LeafOutcome<BoundaryContext, StageError<S::Error>>> {
        let transaction = match signed {
            Ok(transaction) => transaction,
            Err(error) => return self.finish(None, Err(StageError::Signing(error))),
        };

        if transaction.message() != &self.planned {
            return self.finish(Some(transaction), Err(StageError::ChangedMessage));
        }

        self.state = BoundaryState::Sending {
            future: Box::pin(self.sender.send(transaction.clone())),
            transaction: Box::new(transaction),
        };
        self.poll_sending(context)
    }

    fn poll_sending(
        &mut self,
        context: &mut Context<'_>,
    ) -> Poll<LeafOutcome<BoundaryContext, StageError<S::Error>>> {
        let BoundaryState::Sending {
            future,
            transaction,
        } = &mut self.state
        else {
            unreachable!("sending state required");
        };
        let Poll::Ready(result) = future.as_mut().poll(context) else {
            return Poll::Pending;
        };
        let transaction = transaction.as_ref().clone();

        self.finish(Some(transaction), result.map_err(StageError::Sending))
    }

    fn finish(
        &mut self,
        transaction: Option<Transaction>,
        result: Result<Signature, StageError<S::Error>>,
    ) -> Poll<LeafOutcome<BoundaryContext, StageError<S::Error>>> {
        self.state = BoundaryState::Complete;
        Poll::Ready(LeafOutcome {
            context: BoundaryContext { transaction },
            result,
        })
    }
}
