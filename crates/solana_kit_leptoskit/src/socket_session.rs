//! Session polls retain actual cancellable bridge calls between short event-routing borrows.

use crate::{GuestWebSocket, SocketCall, SocketFailure, SocketState};
use leptoskit_bridge::Event;
use solana_kit_client::subscriptions::{ReceivedFrame, TextSocket};
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// Session adapter over the application's actual guest socket event router.
///
/// Pass events to `Session::socket_mut().route` between polls. The application
/// retains its sole Events receiver and executor. Close awaits the native actor
/// join acknowledged by the existing bridge. Drop cancels all outstanding Calls.
pub struct GuestSubscriptionSocket {
    socket: GuestWebSocket,
    writing: Option<SocketCall>,
    close: CloseState,
}

enum CloseState {
    Open,
    Pending(SocketCall),
    Ended(Result<(), SocketFailure>),
}

impl From<GuestWebSocket> for GuestSubscriptionSocket {
    fn from(socket: GuestWebSocket) -> Self {
        Self {
            socket,
            writing: None,
            close: CloseState::Open,
        }
    }
}

impl GuestSubscriptionSocket {
    /// Actual event-routed socket state, independent from a completed close command.
    pub fn state(&self) -> SocketState {
        self.socket.state()
    }

    /// Routes one application-owned event and returns unrelated work intact.
    pub fn route(&mut self, event: Event) -> Result<Option<Event>, SocketFailure> {
        self.socket.route(event)
    }

    fn fail(&mut self, error: SocketFailure) {
        self.socket.cancel(error);
        self.writing = None;
        self.close = CloseState::Ended(Err(error));
    }
}

impl TextSocket for GuestSubscriptionSocket {
    type Error = SocketFailure;

    fn start_send(&mut self, text: &str) -> Result<(), SocketFailure> {
        if !matches!(self.close, CloseState::Open) {
            return Err(SocketFailure::Closed);
        }

        if self.writing.is_some() {
            return Err(SocketFailure::Protocol);
        }

        self.writing = Some(self.socket.send(text)?);

        Ok(())
    }

    fn poll_flush(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), SocketFailure>> {
        let Some(call) = self.writing.as_mut() else {
            return Poll::Ready(Ok(()));
        };
        let result = core::task::ready!(Pin::new(call).poll(cx));
        self.writing = None;

        if let Err(error) = result {
            self.fail(error);
        }

        Poll::Ready(result)
    }

    fn poll_receive(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<ReceivedFrame>, SocketFailure>> {
        self.socket.register_waker(cx.waker());

        if let Poll::Ready(Err(error)) = self.socket.poll_lease(cx) {
            self.fail(error);

            return Poll::Ready(Err(error));
        }

        match self.socket.try_receive_frame() {
            Ok(Some(frame)) => Poll::Ready(Ok(Some(frame))),

            Ok(None) if self.socket.state() == SocketState::Closed => Poll::Ready(Ok(None)),
            Ok(None) => Poll::Pending,
            Err(error) => {
                self.fail(error);

                Poll::Ready(Err(error))
            }
        }
    }

    /// Retains the actual close call so cancellation cannot issue duplicate close commands.
    fn poll_close(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), SocketFailure>> {
        if let CloseState::Ended(result) = self.close {
            return Poll::Ready(result);
        }

        if matches!(self.close, CloseState::Open) {
            core::task::ready!(self.poll_flush(cx))?;

            if self.socket.state() == SocketState::Closed {
                let result = self.socket.terminal_result();
                self.close = CloseState::Ended(result);

                return Poll::Ready(result);
            }

            let call = match self.socket.close() {
                Ok(call) => call,
                Err(error) => {
                    self.fail(error);

                    return Poll::Ready(Err(error));
                }
            };

            self.close = CloseState::Pending(call);
        }

        let CloseState::Pending(call) = &mut self.close else {
            return Poll::Ready(Err(SocketFailure::Protocol));
        };
        let result = core::task::ready!(Pin::new(call).poll(cx));
        self.close = CloseState::Ended(result);

        if let Err(error) = result {
            self.socket.cancel(error);
        }

        Poll::Ready(result)
    }
}
