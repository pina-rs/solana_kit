//! Transient-borrow duplex polling over one codec and two readiness slots.

use super::*;
use crate::reactor::Wait;
use solana_kit_client::subscriptions::ReceivedFrame;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, ready},
};
use wasip2::io::poll::Pollable;

const CONTROL_FRAMES_PER_POLL: usize = 32;

enum Frame {
    Message(ReceivedFrame),
    Control,
    Closed,
    Pending,
}

fn next_frame(socket: &mut WebSocket<Stream>, limit: usize) -> Result<Frame, SocketError> {
    match socket.read() {
        Ok(Message::Text(text)) if text.len() <= limit => {
            Ok(Frame::Message(ReceivedFrame::Text(text.to_string())))
        }

        Ok(Message::Binary(bytes)) if bytes.len() <= limit => {
            Ok(Frame::Message(ReceivedFrame::Binary(bytes.to_vec())))
        }

        Ok(Message::Text(_) | Message::Binary(_)) => Err(SocketError::Limit),
        Ok(Message::Close(_)) | Err(Error::ConnectionClosed) => Ok(Frame::Closed),
        Ok(Message::Ping(_) | Message::Pong(_)) => Ok(Frame::Control),
        Ok(Message::Frame(_)) => Err(SocketError::Protocol),

        Err(Error::Io(error)) if error.kind() == ErrorKind::WouldBlock => Ok(Frame::Pending),
        Err(error) => Err(protocol_error(error)),
    }
}

#[derive(Default)]
pub(super) struct Pending {
    deadline: Option<u64>,
    wait: Option<Wait>,
}

impl Pending {
    fn poll(
        &mut self,
        context: &mut Context<'_>,
        timeout: Duration,
    ) -> Poll<Result<(), SocketError>> {
        let deadline = match self.deadline {
            Some(value) => value,
            None => {
                let value = deadline(timeout)?;
                self.deadline = Some(value);
                value
            }
        };

        if monotonic_clock::now() >= deadline {
            return Poll::Ready(Err(SocketError::Deadline));
        }

        if let Some(wait) = self.wait.as_mut() {
            ready!(Pin::new(wait).poll(context)).map_err(|()| SocketError::Deadline)?;
            self.wait = None;
        }

        Poll::Ready(Ok(()))
    }

    fn arm(
        &mut self,
        reactor: &Reactor,
        signal: Pollable,
        context: &mut Context<'_>,
    ) -> Poll<Result<(), SocketError>> {
        let mut wait = reactor
            .wait(signal, self.deadline.expect("polled operation"))
            .map_err(|()| SocketError::ReadinessLimit)?;
        // Keep even immediately-ready signals registered until the driver polls.
        // A wake-only Pending would strand Reactor::block_on's noop waker.
        wait.register(context);
        self.wait = Some(wait);

        Poll::Pending
    }
}

impl WasiWebSocket {
    /// Queues exactly one bounded text message without awaiting readiness.
    ///
    /// # Errors
    /// Rejects a second send before poll_flush completes. A pending send is uncertain.
    pub fn start_send(&mut self, text: &str) -> Result<(), SocketError> {
        if self.closing || self.writing.deadline.is_some() {
            return Err(SocketError::Configuration(
                "send already pending or closing",
            ));
        }

        if text.len() > self.config.max_frame_bytes {
            return Err(SocketError::Limit);
        }

        self.writing.deadline = Some(deadline(self.config.operation_timeout)?);
        let socket = self.socket.as_mut().ok_or(SocketError::Closed)?;
        let result = socket
            .write(Message::Text(text.into()))
            .or_else(would_block);

        if result.is_err() {
            self.shutdown();
        }

        result
    }

    /// Flushes the current send with transient ownership; receives may be polled too.
    pub fn poll_flush(&mut self, context: &mut Context<'_>) -> Poll<Result<(), SocketError>> {
        let result = self.flush_progress(context);
        self.finish_poll(result)
    }

    fn flush_progress(&mut self, context: &mut Context<'_>) -> Poll<Result<(), SocketError>> {
        ready!(self.writing.poll(context, self.config.operation_timeout))?;
        let socket = self.socket.as_mut().ok_or(SocketError::Closed)?;

        match socket.flush() {
            Ok(()) | Err(Error::ConnectionClosed) => {
                self.writing = Pending::default();
                Poll::Ready(Ok(()))
            }

            Err(Error::Io(error)) if error.kind() == ErrorKind::WouldBlock => {
                self.writing
                    .arm(&self.reactor, socket.get_ref().signal(), context)
            }

            Err(error) => Poll::Ready(Err(protocol_error(error))),
        }
    }

    /// Polls one text message without retaining a mutable borrow between polls.
    ///
    /// One owner can start/flush sends while receive remains pending. Original
    /// receive deadlines and partial parser state survive cancelled wrappers.
    pub fn poll_receive(
        &mut self,
        context: &mut Context<'_>,
    ) -> Poll<Result<Option<String>, SocketError>> {
        match ready!(self.poll_receive_frame(context))? {
            Some(ReceivedFrame::Text(text)) => Poll::Ready(Ok(Some(text))),
            Some(ReceivedFrame::Binary(_)) => {
                self.shutdown();
                Poll::Ready(Err(SocketError::Binary))
            }
            None => Poll::Ready(Ok(None)),
        }
    }

    /// Polls one bounded text or binary message using the original codec and readiness.
    ///
    /// The text-only receive methods continue to reject binary. Payload bounds
    /// are checked before copying parser-owned bytes into the returned frame.
    pub fn poll_receive_frame(
        &mut self,
        context: &mut Context<'_>,
    ) -> Poll<Result<Option<ReceivedFrame>, SocketError>> {
        let result = self.receive_progress(context);

        if matches!(result, Poll::Ready(Ok(None))) {
            self.shutdown();
        }

        self.finish_poll(result)
    }

    // A bounded control-frame turn prevents a ping flood monopolizing the caller.
    fn receive_progress(
        &mut self,
        context: &mut Context<'_>,
    ) -> Poll<Result<Option<ReceivedFrame>, SocketError>> {
        ready!(self.reading.poll(context, self.config.operation_timeout))?;
        let socket = self.socket.as_mut().ok_or(SocketError::Closed)?;

        for _ in 0..CONTROL_FRAMES_PER_POLL {
            match next_frame(socket, self.config.max_message_bytes)? {
                Frame::Message(frame) => {
                    self.finish_receive();

                    return Poll::Ready(Ok(Some(frame)));
                }
                Frame::Closed => {
                    return flush_close(socket, &mut self.reading, &self.reactor, context);
                }
                Frame::Control => {}
                Frame::Pending => {
                    return self
                        .reading
                        .arm(&self.reactor, socket.get_ref().signal(), context)
                        .map_ok(|()| None);
                }
            }
        }

        Poll::Ready(Err(SocketError::Limit))
    }

    fn finish_receive(&mut self) {
        if !self.closing {
            self.reading = Pending::default();
        }
    }

    /// Starts and polls graceful close under one original finite deadline.
    pub fn poll_close(&mut self, context: &mut Context<'_>) -> Poll<Result<(), SocketError>> {
        if self.socket.is_none() {
            return Poll::Ready(Ok(()));
        }

        if !self.closing {
            self.closing = true;
            let limit = deadline(self.config.operation_timeout)?;
            self.reading = Pending {
                deadline: Some(limit),
                wait: None,
            };
            self.writing = Pending {
                deadline: Some(limit),
                wait: None,
            };
            let socket = self.socket.as_mut().ok_or(SocketError::Closed)?;

            if let Err(error) = socket.close(None).or_else(would_block) {
                self.shutdown();

                return Poll::Ready(Err(error));
            }
        }

        ready!(self.poll_flush(context))?;

        for _ in 0..CONTROL_FRAMES_PER_POLL {
            if ready!(self.poll_receive_frame(context))?.is_none() {
                return Poll::Ready(Ok(()));
            }
        }

        self.shutdown();
        Poll::Ready(Err(SocketError::Limit))
    }

    /// Immediately cancels both readiness slots and closes actual socket resources.
    /// No remote close or unsubscribe acknowledgement is implied.
    pub fn shutdown(&mut self) {
        self.reading = Pending::default();
        self.writing = Pending::default();
        self.socket = None;
    }

    fn finish_poll<T>(
        &mut self,
        result: Poll<Result<T, SocketError>>,
    ) -> Poll<Result<T, SocketError>> {
        if matches!(result, Poll::Ready(Err(_))) {
            self.shutdown();
        }

        result
    }
}

fn flush_close(
    socket: &mut WebSocket<Stream>,
    reading: &mut Pending,
    reactor: &Reactor,
    context: &mut Context<'_>,
) -> Poll<Result<Option<ReceivedFrame>, SocketError>> {
    match socket.flush() {
        Ok(()) | Err(Error::ConnectionClosed) => Poll::Ready(Ok(None)),

        Err(Error::Io(error)) if error.kind() == ErrorKind::WouldBlock => reading
            .arm(reactor, socket.get_ref().signal(), context)
            .map_ok(|()| None),
        Err(error) => Poll::Ready(Err(protocol_error(error))),
    }
}
