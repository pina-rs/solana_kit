//! Application-owned I/O around the shared bounded protocol owner.
use super::{
    Budget, Clock, ClockError, Connection, Drain, Failure, Subscription, SubscriptionMethod,
};
use core::{
    future::Future,
    task::{Context, Poll},
};

type BoundedSessionResult<T, E> = Result<Option<Result<T, SessionError<E>>>, ClockError>;

/// One complete received WebSocket message. Outbound protocol commands remain text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReceivedFrame {
    /// JSON-RPC acknowledgement or notification.
    Text(String),
    /// Explicit provider binary notification.
    Binary(Vec<u8>),
}

impl ReceivedFrame {
    /// Payload bytes retained by this message.
    pub fn len(&self) -> usize {
        match self {
            Self::Text(text) => text.len(),
            Self::Binary(bytes) => bytes.len(),
        }
    }

    /// Whether the message payload is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Persistent framed I/O with pending progress retained by the socket owner.
///
/// Polls borrow only for that call. A quiet receive cannot lock out sends.
/// Every Pending registers the current waker. Adapters bound retained frames,
/// preserve parser state, and cancel local I/O on Drop without detaching work.
pub trait TextSocket {
    /// Concrete adapter failure.
    type Error;
    /// Admits one bounded message. Call only after the previous flush completed.
    /// A failed or cancelled write is uncertain and must never be retried.
    fn start_send(&mut self, text: &str) -> Result<(), Self::Error>;
    /// Flushes the admitted message while permitting receive progress.
    fn poll_flush(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
    /// Receives a complete message or clean EOF, retaining incomplete state.
    fn poll_receive(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<ReceivedFrame>, Self::Error>>;
    /// Closes actual I/O and joins owned work. Repeated polls are idempotent.
    fn poll_close(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
}

/// Concrete protocol, socket, EOF or deadline failure.
#[derive(Debug)]
pub enum SessionError<E> {
    /// The application's shared owner is borrowed or its lock is poisoned.
    Unavailable,
    /// Protocol rejection, including false or missing acknowledgements.
    Protocol(Failure),
    /// Actual socket failure.
    Socket(E),
    /// The injected clock failed before its deadline elapsed.
    Clock(ClockError),
    /// Peer ended before the requested lifecycle completed.
    Eof,
    /// Application deadline expired. This session cannot be reused.
    Deadline,
}
impl<E: core::fmt::Display> core::fmt::Display for SessionError<E> {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Unavailable => formatter.write_str("subscription owner unavailable"),
            Self::Protocol(error) => error.fmt(formatter),
            Self::Socket(error) => write!(formatter, "subscription socket failed: {error}"),
            Self::Clock(error) => error.fmt(formatter),
            Self::Eof => formatter.write_str("subscription socket ended unexpectedly"),
            Self::Deadline => formatter.write_str("subscription deadline expired"),
        }
    }
}
impl<E: core::error::Error + 'static> core::error::Error for SessionError<E> {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Protocol(error) => Some(error),
            Self::Socket(error) => Some(error),
            Self::Clock(error) => Some(error),
            _ => None,
        }
    }
}
impl<E> From<Failure> for SessionError<E> {
    fn from(error: Failure) -> Self {
        Self::Protocol(error)
    }
}
/// One typed notification poll with concrete transport failure preserved.
pub type NotificationPoll<T, E> = Poll<Result<Drain<T>, SessionError<E>>>;

/// One application-owned socket and generation for all nine typed families.
///
/// Register or cancel handles between polls even while the peer is quiet. No
/// task, channel, retry, reconnect or second correlation registry is created.
pub struct Session<S> {
    pub(crate) connection: Connection,
    pub(crate) socket: S,
    pub(crate) generation: u64,
    writing: bool,
    closed: bool,
}
impl<S: TextSocket> Session<S> {
    /// Borrows the actual adapter briefly for application-owned event routing.
    /// Do not replace it or feed events from a different socket generation.
    pub fn socket_mut(&mut self) -> &mut S {
        &mut self.socket
    }

    /// Starts an empty generation around an already connected socket.
    pub fn new(socket: S, generation: u64, budget: Budget) -> Self {
        Self {
            connection: Connection::new(generation, budget),
            socket,
            generation,
            writing: false,
            closed: false,
        }
    }
    /// Installs routing before exposing the subscribe frame to I/O.
    /// Await `flush` before reconciliation to send registration first.
    pub fn subscribe<R: SubscriptionMethod>(
        &mut self,
        request: R,
        deadline: u64,
    ) -> Result<Subscription<R>, SessionError<S::Error>> {
        self.open()?;
        self.connection
            .subscribe(request, deadline)
            .map_err(Into::into)
    }
    /// Earliest pending protocol acknowledgement tick in the injected clock domain.
    #[must_use]
    pub fn next_deadline(&self) -> Option<u64> {
        self.connection.next_deadline()
    }
    /// Progresses bounded outgoing commands without locking incoming progress.
    pub fn poll_flush(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), SessionError<S::Error>>> {
        self.open()?;

        loop {
            core::task::ready!(self.poll_write(cx))?;

            let Some(frame) = self.connection.outbound() else {
                return Poll::Ready(Ok(()));
            };
            let text =
                std::str::from_utf8(&frame).map_err(|_| SessionError::Protocol(Failure::Method))?;

            if let Err(error) = self.socket.start_send(text) {
                self.terminate();

                return Poll::Ready(Err(SessionError::Socket(error)));
            }

            self.writing = true;
        }
    }
    /// Drives one typed handle and retains bounded queues for other handles.
    /// Wake at acknowledgement deadlines even when the peer remains quiet.
    pub fn poll_next<R: SubscriptionMethod>(
        &mut self,
        handle: &Subscription<R>,
        cx: &mut Context<'_>,
        now: u64,
        cleanup_deadline: u64,
    ) -> NotificationPoll<R::Response, S::Error> {
        self.open()?;
        self.connection.register_waker(cx.waker());

        for _ in 0..32 {
            self.connection.tick(now, cleanup_deadline)?;

            match self.connection.next(handle)? {
                Drain::Pending => (),

                Drain::Closed if !self.connection.settled(handle)? => (),
                value => return Poll::Ready(Ok(value)),
            }

            if let Poll::Ready(Err(error)) = self.poll_flush(cx) {
                return Poll::Ready(Err(error));
            }

            core::task::ready!(self.poll_frame(cx))?;
        }

        cx.waker().wake_by_ref();
        Poll::Pending
    }
    /// Cancels delivery and queues remote cleanup, including before subscribe ack.
    pub fn cancel<R>(
        &mut self,
        handle: &Subscription<R>,
        deadline: u64,
    ) -> Result<(), SessionError<S::Error>> {
        self.open()?;
        self.connection
            .unsubscribe(handle, deadline)
            .map_err(Into::into)
    }
    /// Frees a terminal registration. Cancellation alone is not remote cleanup.
    pub fn forget<R>(&mut self, handle: Subscription<R>) -> Result<(), SessionError<S::Error>> {
        self.connection.forget(handle).map_err(Into::into)
    }
    /// Sends queued commands within an injected deadline.
    pub async fn flush<C: Clock>(
        &mut self,
        clock: &C,
        deadline: u64,
    ) -> Result<(), SessionError<S::Error>> {
        let result = bounded(
            core::future::poll_fn(|cx| self.poll_flush(cx)),
            clock.sleep_until(deadline),
        )
        .await;
        self.finish_bounded(result)
    }
    /// Awaits a typed value with a real wake for quiet acknowledgement expiry.
    pub async fn next<R: SubscriptionMethod, C: Clock>(
        &mut self,
        handle: &Subscription<R>,
        clock: &C,
        deadline: u64,
    ) -> Result<Drain<R::Response>, SessionError<S::Error>> {
        loop {
            let now = clock.now();
            let wake = self
                .connection
                .next_deadline()
                .filter(|tick| *tick > now)
                .unwrap_or(deadline)
                .min(deadline);
            let result = bounded(
                core::future::poll_fn(|cx| self.poll_next(handle, cx, clock.now(), deadline)),
                clock.sleep_until(wake),
            )
            .await;

            if !matches!(result, Ok(None)) || clock.now() >= deadline {
                return self.finish_bounded(result);
            }
        }
    }
    /// Awaits a true unsubscribe acknowledgement, then frees the handle.
    pub async fn unsubscribe<R: SubscriptionMethod, C: Clock>(
        &mut self,
        handle: Subscription<R>,
        clock: &C,
        deadline: u64,
    ) -> Result<(), SessionError<S::Error>> {
        self.cancel(&handle, deadline)?;
        self.next(&handle, clock, deadline).await?;
        self.forget(handle)
    }
    /// Stops admission and polls actual socket teardown. Idempotent on Pending.
    pub fn poll_shutdown(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), SessionError<S::Error>>> {
        self.terminate();
        self.socket.poll_close(cx).map_err(SessionError::Socket)
    }
    /// Consumes the owner after bounded awaited socket teardown.
    pub async fn shutdown<C: Clock>(
        mut self,
        clock: &C,
        deadline: u64,
    ) -> Result<(), SessionError<S::Error>> {
        let result = bounded(
            core::future::poll_fn(|cx| self.poll_shutdown(cx)),
            clock.sleep_until(deadline),
        )
        .await;
        self.finish_bounded(result)
    }
    fn open(&self) -> Result<(), SessionError<S::Error>> {
        if self.closed {
            Err(Failure::Closed.into())
        } else {
            Ok(())
        }
    }
    fn poll_write(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), SessionError<S::Error>>> {
        if !self.writing {
            return Poll::Ready(Ok(()));
        }

        let result = core::task::ready!(self.socket.poll_flush(cx));
        self.writing = false;

        if result.is_err() {
            self.terminate();
        }

        Poll::Ready(result.map_err(SessionError::Socket))
    }
    fn poll_frame(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), SessionError<S::Error>>> {
        let result = match core::task::ready!(self.socket.poll_receive(cx)) {
            Ok(Some(ReceivedFrame::Text(frame))) => self
                .connection
                .ingest(self.generation, frame.as_bytes())
                .map_err(Into::into),
            Ok(Some(ReceivedFrame::Binary(frame))) => self
                .connection
                .ingest_binary(self.generation, &frame)
                .map_err(Into::into),
            Ok(None) => Err(SessionError::Eof),
            Err(error) => Err(SessionError::Socket(error)),
        };

        if result.is_err() {
            self.terminate();
        }

        Poll::Ready(result)
    }
    pub(crate) fn terminate(&mut self) {
        self.closed = true;
        self.connection.terminate();
    }
    fn finish_bounded<T>(
        &mut self,
        result: BoundedSessionResult<T, S::Error>,
    ) -> Result<T, SessionError<S::Error>> {
        match result {
            Ok(Some(result)) => result,
            Ok(None) => {
                self.terminate();
                Err(SessionError::Deadline)
            }
            Err(error) => {
                self.terminate();
                Err(SessionError::Clock(error))
            }
        }
    }
}

pub(crate) async fn bounded<F: Future, D: Future<Output = Result<(), ClockError>>>(
    future: F,
    deadline: D,
) -> Result<Option<F::Output>, ClockError> {
    let mut future = core::pin::pin!(future);
    let mut deadline = core::pin::pin!(deadline);
    core::future::poll_fn(|cx| {
        if let Poll::Ready(result) = deadline.as_mut().poll(cx) {
            return Poll::Ready(result.map(|()| None));
        }

        future.as_mut().poll(cx).map(|output| Ok(Some(output)))
    })
    .await
}
