//! Persistent poll progress over the existing browser socket and event handlers.
use super::{BrowserWebSocket, Deadline, Phase, WsError};
use solana_kit_client::subscriptions::{ReceivedFrame, TextSocket};
use std::task::{Context, Poll};

/// Browser Session adapter retaining operation deadlines between transient polls.
///
/// Sends hand bounded text to the browser synchronously. Browser buffering and
/// framing remain browser owned. Pending receives and close await actual socket
/// events or the configured timer; no task or polling loop is installed.
pub struct BrowserSubscriptionSocket {
    socket: BrowserWebSocket,
    receive: Option<Deadline>,
    close: Option<Deadline>,
}
impl From<BrowserWebSocket> for BrowserSubscriptionSocket {
    fn from(socket: BrowserWebSocket) -> Self {
        Self {
            socket,
            receive: None,
            close: None,
        }
    }
}
impl TextSocket for BrowserSubscriptionSocket {
    type Error = WsError;

    fn start_send(&mut self, text: &str) -> Result<(), WsError> {
        self.socket.send(text)
    }
    fn poll_flush(&mut self, _: &mut Context<'_>) -> Poll<Result<(), WsError>> {
        let state = self.socket.shared.state.borrow();
        Poll::Ready(if state.phase == Phase::Open {
            Ok(())
        } else {
            Err(state.failure.unwrap_or(WsError::Closed))
        })
    }
    fn poll_receive(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<ReceivedFrame>, WsError>> {
        if self.receive.is_none() {
            self.receive = Some(Deadline::new(&self.socket.shared, &self.socket.window)?);
        }

        let result = self.socket.poll_received(cx).map(|result| result.map(Some));

        if result.is_ready() {
            self.receive = None;
        }

        result
    }
    fn poll_close(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), WsError>> {
        if self.socket.shared.state.borrow().phase == Phase::Closed {
            self.receive = None;
            self.close = None;

            return self.socket.poll_phase(cx, Phase::Closed);
        }

        if self.close.is_none() {
            self.receive = None;
            self.socket.begin_close()?;
            self.close = Some(Deadline::new(&self.socket.shared, &self.socket.window)?);
        }

        let result = self.socket.poll_phase(cx, Phase::Closed);

        if result.is_ready() {
            self.close = None;
        }

        result
    }
}
