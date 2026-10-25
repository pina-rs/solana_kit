//! Transient polls keep quiet reads from borrowing the socket across commands.
use super::{NativeWebSocket, WsError, socket_error};
use crate::{WsConfig, websocket_connect::Socket};
use futures_util::{Sink, Stream};
use solana_kit_client::subscriptions::{ReceivedFrame, TextSocket};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tokio::time::Sleep;
use tokio_tungstenite::tungstenite::{Error, Message};

/// Native socket progress for an application-owned typed subscription Session.
///
/// Uses the original frame/write budgets and finite operation deadlines. Each
/// poll releases its borrow, permitting writes while a quiet read is pending.
/// Drop closes TCP/TLS immediately. No task or executor is created.
pub struct NativeSubscriptionSocket {
    socket: Option<Socket>,
    config: WsConfig,
    outgoing: Option<String>,
    read: Option<Pin<Box<Sleep>>>,
    write: Option<Pin<Box<Sleep>>>,
    close: Option<Pin<Box<Sleep>>>,
}
impl From<NativeWebSocket> for NativeSubscriptionSocket {
    fn from(value: NativeWebSocket) -> Self {
        Self {
            socket: value.socket,
            config: value.config,
            outgoing: None,
            read: None,
            write: None,
            close: None,
        }
    }
}
impl TextSocket for NativeSubscriptionSocket {
    type Error = WsError;
    fn start_send(&mut self, text: &str) -> Result<(), WsError> {
        if self.socket.is_none() {
            return Err(WsError::Closed);
        }

        if self.write.is_some() {
            return Err(WsError::Protocol);
        }

        if text.len() > self.config.max_frame_bytes {
            return Err(WsError::Limit);
        }

        self.outgoing = Some(text.into());
        self.write = Some(Box::pin(tokio::time::sleep(self.config.operation_timeout)));

        Ok(())
    }
    fn poll_flush(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), WsError>> {
        let result = self.flush(cx);
        self.finish(result)
    }
    fn poll_receive(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<ReceivedFrame>, WsError>> {
        if elapsed(&mut self.read, cx, self.config.operation_timeout) {
            self.socket.take();

            return Poll::Ready(Err(WsError::Deadline));
        }

        let result = self.receive(cx);

        if result.is_ready() {
            self.read = None;
        }

        self.finish(result)
    }
    fn poll_close(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), WsError>> {
        if self.socket.is_none() {
            return Poll::Ready(Ok(()));
        }

        if elapsed(&mut self.close, cx, self.config.operation_timeout) {
            self.socket.take();

            return Poll::Ready(Err(WsError::Deadline));
        }

        let socket = self.socket.as_mut().ok_or(WsError::Closed)?;

        match Pin::new(socket).poll_close(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(result) => {
                self.socket.take();
                self.close = None;
                Poll::Ready(result.map_err(socket_error))
            }
        }
    }
}
impl NativeSubscriptionSocket {
    fn flush(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), WsError>> {
        if let Some(timer) = self.write.as_mut()
            && timer.as_mut().poll(cx).is_ready()
        {
            return Poll::Ready(Err(WsError::Deadline));
        }

        let socket = self.socket.as_mut().ok_or(WsError::Closed)?;

        if self.outgoing.is_some() {
            core::task::ready!(Pin::new(&mut *socket).poll_ready(cx)).map_err(socket_error)?;
            let text = self.outgoing.take().ok_or(WsError::Protocol)?;
            Pin::new(&mut *socket)
                .start_send(Message::Text(text.into()))
                .map_err(socket_error)?;
        }

        core::task::ready!(Pin::new(socket).poll_flush(cx)).map_err(socket_error)?;
        self.write = None;
        Poll::Ready(Ok(()))
    }
    // Bound control-frame work so a busy peer cannot starve other executor tasks.
    fn receive(&mut self, cx: &mut Context<'_>) -> Poll<Result<Option<ReceivedFrame>, WsError>> {
        const CONTROL_FRAMES_PER_POLL: usize = 32;
        let socket = self.socket.as_mut().ok_or(WsError::Closed)?;

        for _ in 0..CONTROL_FRAMES_PER_POLL {
            match core::task::ready!(Pin::new(&mut *socket).poll_next(cx)) {
                Some(Ok(message @ (Message::Text(_) | Message::Binary(_)))) => {
                    return Poll::Ready(
                        data_frame(message, self.config.max_message_bytes).map(Some),
                    );
                }
                Some(Ok(Message::Ping(_) | Message::Pong(_))) => (),
                Some(Ok(Message::Close(_))) | None | Some(Err(Error::ConnectionClosed)) => {
                    return Poll::Ready(Ok(None));
                }
                Some(Ok(Message::Frame(_))) => return Poll::Ready(Err(WsError::Protocol)),
                Some(Err(error)) => return Poll::Ready(Err(socket_error(error))),
            }
        }

        cx.waker().wake_by_ref();
        Poll::Pending
    }
    fn finish<T>(&mut self, result: Poll<Result<T, WsError>>) -> Poll<Result<T, WsError>> {
        if matches!(result, Poll::Ready(Err(_))) {
            self.socket.take();
        }

        result
    }
}
// Check the retained network payload before allocating its application-owned copy.
fn data_frame(message: Message, maximum: usize) -> Result<ReceivedFrame, WsError> {
    let length = match &message {
        Message::Text(text) => text.len(),
        Message::Binary(bytes) => bytes.len(),
        _ => return Err(WsError::Protocol),
    };

    if length > maximum {
        return Err(WsError::Limit);
    }

    match message {
        Message::Text(text) => Ok(ReceivedFrame::Text(text.to_string())),
        Message::Binary(bytes) => Ok(ReceivedFrame::Binary(bytes.to_vec())),
        _ => Err(WsError::Protocol),
    }
}

fn elapsed(
    timer: &mut Option<Pin<Box<Sleep>>>,
    cx: &mut Context<'_>,
    duration: std::time::Duration,
) -> bool {
    timer
        .get_or_insert_with(|| Box::pin(tokio::time::sleep(duration)))
        .as_mut()
        .poll(cx)
        .is_ready()
}
