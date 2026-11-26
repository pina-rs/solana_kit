//! One exclusively borrowed notification consumer over a typed Session.
//!
//! The stream polls the existing [`Session::poll_next`] contract, so ticks,
//! flushes, socket progress and protocol failures stay in one place. A retained
//! clock sleep exists only while waiting for the next item and only when the
//! protocol owner reports a pending acknowledgement deadline.
use super::{
    Clock, ClockError, Drain, Session, SessionError, Subscription, SubscriptionMethod, TextSocket,
};
use core::{
    future::Future,
    ops::ControlFlow,
    pin::Pin,
    task::{Context, Poll},
};
use futures_core::{Stream, stream::FusedStream};

/// Exclusive typed notifications borrowed from one [`Session`].
///
/// Create it with [`Session::notifications`] after registering the handle with
/// [`Session::subscribe`]. While this value is alive, the session is borrowed
/// mutably, so exactly one consumer drives the shared owner. Use a separate
/// session to drive an independent subscription concurrently. The existing
/// poll-style methods remain available between streams.
///
/// A one-shot family ends after yielding its completing notification. A
/// terminal error is yielded once, then the stream ends. Dropping the stream
/// releases the borrows. Use [`Session::unsubscribe`] to await remote cleanup.
#[must_use = "poll the stream or drop it to release the session borrow"]
pub struct SubscriptionStream<'a, S, R, C>
where
    C: Clock + 'a,
{
    session: &'a mut Session<S>,
    handle: &'a Subscription<R>,
    clock: &'a C,

    cleanup_timeout_millis: u64,
    timer: Option<AckTimer<C::Sleep<'a>>>,
    done: bool,
}

struct AckTimer<F> {
    deadline: u64,
    sleep: Pin<Box<F>>,
}

impl<'a, S, R, C> SubscriptionStream<'a, S, R, C>
where
    S: TextSocket,
    R: SubscriptionMethod,
    C: Clock + 'a,
{
    fn poll_deadline(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), ClockError>> {
        // A protocol poll can add a deadline, including for an abandoned handle.
        let Some(deadline) = self.session.next_deadline() else {
            self.timer = None;

            return Poll::Pending;
        };

        if self
            .timer
            .as_ref()
            .is_some_and(|timer| timer.deadline != deadline)
        {
            // Release the previous readiness slot before asking the clock for another.
            self.timer = None;
        }

        let clock = self.clock;
        let timer = self.timer.get_or_insert_with(|| AckTimer {
            deadline,
            sleep: Box::pin(clock.sleep_until(deadline)),
        });
        timer.sleep.as_mut().poll(cx)
    }

    fn poll_pending(&mut self, cx: &mut Context<'_>) -> Poll<Option<<Self as Stream>::Item>> {
        let elapsed = core::task::ready!(self.poll_deadline(cx));
        self.timer = None;

        if let Err(error) = elapsed {
            self.session.terminate();
            self.done = true;

            return Poll::Ready(Some(Err(SessionError::Clock(error))));
        }

        // A quiet socket may have no other wake to drive the next tick.
        cx.waker().wake_by_ref();
        Poll::Pending
    }

    fn poll_callbacks(
        &mut self,
        cx: &mut Context<'_>,
        callback: &mut impl FnMut(R::Response) -> ControlFlow<()>,
    ) -> Poll<Result<(), SessionError<S::Error>>> {
        // Give the executor a turn even when every socket read is ready.
        for _ in 0..32 {
            let Some(item) = core::task::ready!(Pin::new(&mut *self).poll_next(cx)) else {
                return Poll::Ready(Ok(()));
            };

            if callback(item?).is_break() {
                return Poll::Ready(Ok(()));
            }
        }

        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

impl<S, R, C> Stream for SubscriptionStream<'_, S, R, C>
where
    S: TextSocket,
    R: SubscriptionMethod,
    C: Clock,
{
    type Item = Result<R::Response, SessionError<S::Error>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        if this.done {
            return Poll::Ready(None);
        }

        let now = this.clock.now();
        let cleanup = now.saturating_add(this.cleanup_timeout_millis);

        let item = match this.session.poll_next(this.handle, cx, now, cleanup) {
            Poll::Ready(Ok(Drain::Notification(value))) => Some(Ok(value)),
            Poll::Ready(Ok(Drain::Closed)) => None,
            Poll::Ready(Err(error)) => Some(Err(error)),
            Poll::Ready(Ok(Drain::Pending)) | Poll::Pending => {
                return this.poll_pending(cx);
            }
        };

        this.timer = None;
        this.done = item.as_ref().is_none_or(Result::is_err);
        Poll::Ready(item)
    }
}

impl<S, R, C> FusedStream for SubscriptionStream<'_, S, R, C>
where
    S: TextSocket,
    R: SubscriptionMethod,
    C: Clock,
{
    fn is_terminated(&self) -> bool {
        self.done
    }
}

impl<S: TextSocket> Session<S> {
    /// Borrows one typed notification stream for an already registered handle.
    ///
    /// Call [`Session::subscribe`] first. Polling this stream drives flushes,
    /// protocol ticks and socket progress through [`Session::poll_next`]. The
    /// stream holds a mutable session borrow and a shared handle borrow. Drop
    /// it before registering, cancelling or reading another handle on this
    /// session. Use separate sessions for independently driven subscriptions.
    ///
    /// `cleanup_timeout_millis` is added to the current clock tick and passed as
    /// the cleanup deadline for cancelled or abandoned registrations seen by
    /// this stream. The socket adapter controls receive and operation timeouts.
    ///
    /// A one-shot subscription yields its completing notification and then
    /// ends. A protocol, socket, EOF or clock failure is yielded once as `Err`
    /// and the stream is fused afterwards. Once the borrow ends, await
    /// [`Session::unsubscribe`] for remote cleanup or [`Session::forget`] to
    /// release a completed one-shot registration.
    ///
    /// # Example
    ///
    /// ```
    /// use futures::StreamExt;
    /// use solana_kit_client::subscriptions::{Clock, Root, Session, SessionError, TextSocket};
    ///
    /// async fn first_root<S: TextSocket>(
    ///     session: &mut Session<S>,
    ///     clock: &impl Clock,
    /// ) -> Result<Option<u64>, SessionError<S::Error>> {
    ///     let handle = session.subscribe(Root, clock.now() + 2_000)?;
    ///     let root = session.notifications(&handle, clock, 500).next().await.transpose()?;
    ///     session.unsubscribe(handle, clock, clock.now() + 2_000).await?;
    ///     Ok(root)
    /// }
    /// ```
    pub fn notifications<'a, R, C>(
        &'a mut self,
        handle: &'a Subscription<R>,
        clock: &'a C,
        cleanup_timeout_millis: u64,
    ) -> SubscriptionStream<'a, S, R, C>
    where
        R: SubscriptionMethod,
        C: Clock,
    {
        SubscriptionStream {
            session: self,
            handle,
            clock,
            cleanup_timeout_millis,
            timer: None,
            done: false,
        }
    }

    /// Runs a callback for each typed notification until it breaks or ends.
    ///
    /// The callback runs in the caller's task and can borrow local state.
    /// Return `ControlFlow::Break(())` to stop or `ControlFlow::Continue(())` to
    /// wait for the next item. Subscription failures are returned by the future.
    ///
    /// Breaking or dropping the future releases the session borrow. Await
    /// [`Session::unsubscribe`] for remote cleanup or [`Session::forget`] to
    /// release a completed one-shot registration.
    ///
    /// # Example
    ///
    /// ```
    /// use core::ops::ControlFlow;
    /// use solana_kit_client::subscriptions::{Clock, Root, Session, SessionError, TextSocket};
    ///
    /// async fn watch<S: TextSocket>(
    ///     session: &mut Session<S>,
    ///     clock: &impl Clock,
    /// ) -> Result<(), SessionError<S::Error>> {
    ///     let handle = session.subscribe(Root, clock.now() + 2_000)?;
    ///     session
    ///         .run_callback(&handle, clock, 500, |root| {
    ///             println!("root {root}");
    ///             ControlFlow::Break(())
    ///         })
    ///         .await?;
    ///     session.unsubscribe(handle, clock, clock.now() + 2_000).await
    /// }
    /// ```
    pub async fn run_callback<R, C, F>(
        &mut self,
        handle: &Subscription<R>,
        clock: &C,
        cleanup_timeout_millis: u64,
        mut callback: F,
    ) -> Result<(), SessionError<S::Error>>
    where
        R: SubscriptionMethod,
        C: Clock,
        F: FnMut(R::Response) -> ControlFlow<()>,
    {
        let mut stream = self.notifications(handle, clock, cleanup_timeout_millis);

        core::future::poll_fn(|cx| stream.poll_callbacks(cx, &mut callback)).await
    }
}
