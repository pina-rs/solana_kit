use solana_kit_wallet::ui::PanelAction;
use std::{
    collections::VecDeque,
    fmt,
    future::Future,
    sync::{Arc, Mutex},
    task::{Poll, Waker},
};

const MAX_ACTIONS: usize = 64;

/// A bounded action could not be admitted or its receiver has closed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PanelQueueError {
    /// The configured count is zero, excessive, or cannot be allocated.
    Capacity,
    /// The view owner has been disposed.
    Closed,
    /// A prior panic poisoned the owner lock.
    Poisoned,
}

impl fmt::Display for PanelQueueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Capacity => "wallet action capacity exceeded",
            Self::Closed => "wallet view closed",
            Self::Poisoned => "wallet action owner poisoned",
        })
    }
}

impl std::error::Error for PanelQueueError {}

struct State {
    queue: VecDeque<PanelAction>,
    limit: usize,
    closed: bool,
    refresh: bool,
    waker: Option<Waker>,
}

/// Sendable UI callbacks hold only this bounded command queue, never a browser Rc.
#[derive(Clone)]
pub struct PanelCommands(Arc<Mutex<State>>);

/// Unique local receiver. Drop closes admission and discards unexecuted user intents.
pub struct PanelEvents(Arc<Mutex<State>>);

/// Creates one view-to-controller channel, with at most 64 queued user intents.
pub fn panel_channel(limit: usize) -> Result<(PanelCommands, PanelEvents), PanelQueueError> {
    if limit == 0 || limit > MAX_ACTIONS {
        return Err(PanelQueueError::Capacity);
    }

    let mut queue = VecDeque::new();
    queue
        .try_reserve_exact(limit)
        .map_err(|_| PanelQueueError::Capacity)?;
    let state = Arc::new(Mutex::new(State {
        queue,
        limit,
        closed: false,
        refresh: false,
        waker: None,
    }));
    Ok((PanelCommands(Arc::clone(&state)), PanelEvents(state)))
}

impl PanelCommands {
    /// Coalesces backend notifications separately from bounded user intents.
    /// User actions drain first, so metadata churn cannot starve cancellation.
    pub fn refresh(&self) -> Result<(), PanelQueueError> {
        let mut state = self.0.lock().map_err(|_| PanelQueueError::Poisoned)?;

        if state.closed {
            return Err(PanelQueueError::Closed);
        }

        state.refresh = true;
        let waker = state.waker.take();
        drop(state);

        if let Some(waker) = waker {
            waker.wake();
        }

        Ok(())
    }

    /// Admits an action or reports exactly why it was not accepted.
    pub fn send(&self, action: PanelAction) -> Result<(), PanelQueueError> {
        let mut state = self.0.lock().map_err(|_| PanelQueueError::Poisoned)?;

        if state.closed {
            return Err(PanelQueueError::Closed);
        }

        if state.queue.len() == state.limit {
            return Err(PanelQueueError::Capacity);
        }

        state.queue.push_back(action);
        let waker = state.waker.take();
        drop(state);

        if let Some(waker) = waker {
            waker.wake();
        }

        Ok(())
    }
}

impl PanelEvents {
    /// Sleeps without a task/runtime until a UI action arrives or admission closes.
    pub fn next_action(
        &mut self,
    ) -> impl Future<Output = Result<Option<PanelAction>, PanelQueueError>> + '_ {
        std::future::poll_fn(move |context| self.poll_next(context))
    }

    /// Short-borrow polling for application owners that render independently.
    pub fn poll_next(
        &mut self,
        context: &mut std::task::Context<'_>,
    ) -> Poll<Result<Option<PanelAction>, PanelQueueError>> {
        let next = context.waker().clone();

        let mut state = match self.0.lock() {
            Ok(state) => state,
            Err(_) => return Poll::Ready(Err(PanelQueueError::Poisoned)),
        };

        if let Some(action) = state.queue.pop_front() {
            return Poll::Ready(Ok(Some(action)));
        }

        if state.closed {
            return Poll::Ready(Ok(None));
        }

        if state.refresh {
            state.refresh = false;

            return Poll::Ready(Ok(Some(PanelAction::Refresh)));
        }

        let previous = state.waker.replace(next);
        drop(state);
        drop(previous);
        Poll::Pending
    }
}

impl Drop for PanelEvents {
    fn drop(&mut self) {
        if let Ok(mut state) = self.0.lock() {
            state.closed = true;
            state.queue.clear();
            let waker = state.waker.take();
            drop(state);
            drop(waker);
        }
    }
}
