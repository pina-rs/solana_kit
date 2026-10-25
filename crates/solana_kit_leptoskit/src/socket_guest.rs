//! Application-routed socket events and an idle-owner cancellable lease.
use crate::socket_wire::{self as wire, SocketFailure};
use leptoskit_bridge::{
    CallId, Event, PluginId,
    guest::{Call, Client},
};
use solana_kit_client::subscriptions::ReceivedFrame;
use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
};

/// Observable lifecycle of one fixed host-granted text socket.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketState {
    /// The native handshake has not completed.
    Connecting,
    /// Text operations are admitted.
    Open,
    /// Native work ended or local queue limits were exceeded.
    Closed,
}

/// Text socket whose lease cancels on Drop, including while no operation waits.
///
/// The application alone takes `guest::Events` and passes each event to `route`.
/// Feed `try_receive` text to the matching subscription Connection generation.
/// Queue overflow closes this owner explicitly. No executor or receiver is started.
pub struct GuestWebSocket {
    client: Client,
    plugin: PluginId,
    id: CallId,
    lease: Option<Call>,
    state: SocketState,
    queue: VecDeque<ReceivedFrame>,
    messages: usize,
    frame_bytes: usize,
    failure: Option<SocketFailure>,
    waker: Option<Waker>,
}

impl GuestWebSocket {
    pub(super) fn register_waker(&mut self, waker: &Waker) {
        self.waker = Some(waker.clone());
    }

    pub(super) fn cancel(&mut self, failure: SocketFailure) {
        self.queue.clear();
        self.end(Some(failure));
    }

    pub(super) fn terminal_result(&self) -> Result<(), SocketFailure> {
        self.failure.map_or(Ok(()), Err)
    }

    /// Starts a lease at a fixed installed service with positive local budgets.
    pub fn new(service: &str, messages: usize, frame_bytes: usize) -> Result<Self, SocketFailure> {
        if messages == 0 || frame_bytes == 0 {
            return Err(SocketFailure::Limit);
        }

        let client = Client::new(service).map_err(|_| SocketFailure::Bridge)?;
        let lease = client
            .call(wire::OPEN, &[])
            .map_err(|_| SocketFailure::Bridge)?;
        let id = lease.id().ok_or(SocketFailure::Bridge)?;
        Ok(Self {
            client,
            plugin: client.plugin_id(),
            id,
            lease: Some(lease),
            state: SocketState::Connecting,
            queue: VecDeque::new(),
            messages,
            frame_bytes,
            failure: None,
            waker: None,
        })
    }

    /// Reports handshake and terminal state without consuming an event receiver.
    pub fn state(&self) -> SocketState {
        self.state
    }

    /// Polls the cancellable lifetime Call for refusal or terminal failure.
    /// The application's executor polls this after plugin wakes, alongside Events.
    pub fn poll_lease(&mut self, context: &mut Context<'_>) -> Poll<Result<(), SocketFailure>> {
        let Some(lease) = self.lease.as_mut() else {
            return Poll::Ready(Ok(()));
        };

        let result = match Pin::new(lease).poll(context) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(Ok(bytes)) => wire::parse_reply(&bytes),
            Poll::Ready(Err(_)) => Err(SocketFailure::Bridge),
        };

        self.end(result.err());
        Poll::Ready(result)
    }

    /// Routes matching events and returns unrelated plugin/socket work intact.
    pub fn route(&mut self, event: Event) -> Result<Option<Event>, SocketFailure> {
        if event.plugin != self.plugin || event.kind != wire::EVENT {
            return Ok(Some(event));
        }

        let (id, kind, body) = wire::parse(&event.payload)?;

        if id != self.id {
            return Ok(Some(event));
        }

        if self.state == SocketState::Closed {
            return Ok(None);
        }

        let result = self.route_body(kind, body);

        if let Err(error) = result {
            self.end(Some(error));

            return Err(error);
        }

        if let Some(waker) = self.waker.take() {
            waker.wake();
        }

        Ok(None)
    }

    fn route_body(&mut self, kind: u8, body: &[u8]) -> Result<(), SocketFailure> {
        match kind {
            wire::OPENED if body.is_empty() && self.state == SocketState::Connecting => {
                self.state = SocketState::Open;
                Ok(())
            }

            wire::TEXT | wire::BINARY if self.state == SocketState::Open => {
                self.enqueue(kind, body)
            }
            wire::ENDED => {
                let result = wire::parse_reply(body);
                self.end(result.err());
                Ok(())
            }
            _ => Err(SocketFailure::Protocol),
        }
    }

    fn enqueue(&mut self, kind: u8, body: &[u8]) -> Result<(), SocketFailure> {
        if body.len() > self.frame_bytes || self.queue.len() >= self.messages {
            return Err(SocketFailure::Limit);
        }

        let frame = if kind == wire::TEXT {
            let text = std::str::from_utf8(body).map_err(|_| SocketFailure::Protocol)?;
            ReceivedFrame::Text(text.to_owned())
        } else {
            ReceivedFrame::Binary(body.to_vec())
        };
        self.queue.push_back(frame);
        Ok(())
    }

    fn end(&mut self, failure: Option<SocketFailure>) {
        self.state = SocketState::Closed;
        self.lease.take();
        self.failure = failure;

        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }

    /// Takes one frame. An empty open queue returns None; terminal errors are explicit.
    pub fn try_receive(&mut self) -> Result<Option<String>, SocketFailure> {
        match self.try_receive_frame()? {
            Some(ReceivedFrame::Text(text)) => Ok(Some(text)),
            Some(ReceivedFrame::Binary(_)) => {
                self.end(Some(SocketFailure::Protocol));

                Err(SocketFailure::Protocol)
            }
            None => Ok(None),
        }
    }

    /// Takes one bounded text or binary frame from the application's actual event router.
    pub fn try_receive_frame(&mut self) -> Result<Option<ReceivedFrame>, SocketFailure> {
        if let Some(frame) = self.queue.pop_front() {
            return Ok(Some(frame));
        }

        if let Some(error) = self.failure.take() {
            return Err(error);
        }

        Ok(None)
    }

    /// Submits text once. Dropping its pending Call closes the native socket.
    /// Native sends are never retried after uncertain cancellation.
    pub fn send(&self, text: &str) -> Result<SocketCall, SocketFailure> {
        if self.state != SocketState::Open {
            return Err(SocketFailure::Closed);
        }

        if text.len() > self.frame_bytes {
            return Err(SocketFailure::Limit);
        }

        self.operation(wire::SEND, text.as_bytes())
    }

    /// Starts a real graceful close; its Call must be polled to observe completion.
    /// Dropping this owner always cancels its lease independently of this operation.
    pub fn close(&self) -> Result<SocketCall, SocketFailure> {
        if self.state == SocketState::Closed {
            return Err(SocketFailure::Closed);
        }

        self.operation(wire::CLOSE, &[])
    }

    fn operation(
        &self,
        method: leptoskit_bridge::MethodId,
        body: &[u8],
    ) -> Result<SocketCall, SocketFailure> {
        let payload = wire::envelope(self.id, 0, body);
        self.client
            .call(method, &payload)
            .map(SocketCall)
            .map_err(|_| SocketFailure::Bridge)
    }
}

/// Owned cancellable socket command, containing no borrow of its event router.
pub struct SocketCall(Call);

impl Future for SocketCall {
    type Output = Result<(), SocketFailure>;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.0).poll(context) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok(bytes)) => Poll::Ready(wire::parse_reply(&bytes)),
            Poll::Ready(Err(_)) => Poll::Ready(Err(SocketFailure::Bridge)),
        }
    }
}
