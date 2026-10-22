//! Caller-owned local browser WebSocket operations.
mod config;
mod state;
pub use config::WsConfig;
use state::{Handlers, Phase, Shared, State};
use std::{cell::RefCell, collections::VecDeque, fmt, future::poll_fn, rc::Rc, task::Poll};
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{WebSocket, Window};

/// Redacted failure categories. Browser events do not reveal TLS or handshake causes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WsError {
    /// Invalid endpoint, subprotocol, budget or deadline.
    Config,
    /// A browser Window or timer capability is unavailable.
    Unavailable,
    /// The browser rejected socket construction.
    Constructor,
    /// An opaque browser network or protocol error occurred.
    Browser,
    /// One complete text message exceeded its byte budget.
    MessageLimit,
    /// The bounded receive queue overflowed and the socket ended.
    QueueLimit,
    /// A binary ArrayBuffer or Blob message is unsupported by this text adapter.
    Binary,
    /// Sending would exceed the configured bufferedAmount budget. Nothing was sent.
    Backpressure,
    /// An operation deadline expired and ended the socket.
    Deadline,
    /// The owner has ended or is closing.
    Closed,
    /// The peer closed outside an explicit caller close. Its reason is redacted.
    PeerClosed {
        /// Browser-reported close code.
        code: u16,
        /// Browser-reported clean handshake flag.
        clean: bool,
    },
}
impl fmt::Display for WsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "browser WebSocket failure: {self:?}")
    }
}
impl std::error::Error for WsError {}

/// One browser Window socket. No executor, receive task, retries or Send promise.
///
/// Drop detaches every handler and closes local I/O. The browser controls TLS,
/// cookies, framing and network buffers. Authentication may use caller URL query
/// parameters or subprotocols. Arbitrary headers cannot be set by this API.
pub struct BrowserWebSocket {
    shared: Rc<Shared>,
    _handlers: Handlers,
    window: Window,
}
impl fmt::Debug for BrowserWebSocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BrowserWebSocket").finish_non_exhaustive()
    }
}
impl BrowserWebSocket {
    /// Connects once. Cancelling the future closes the in-progress socket.
    pub async fn connect(endpoint: &str, config: WsConfig) -> Result<Self, WsError> {
        let endpoint = config::validate(&config, endpoint)?;
        let window = web_sys::window().ok_or(WsError::Unavailable)?;

        let protocols = config.protocols.iter().map(wasm_bindgen::JsValue::from);
        let protocols = protocols.collect::<js_sys::Array>();
        let socket = WebSocket::new_with_str_sequence(&endpoint, &protocols)
            .map_err(|_| WsError::Constructor)?;
        socket.set_binary_type(web_sys::BinaryType::Arraybuffer);
        let shared = Rc::new(Shared {
            socket,
            state: RefCell::new(State {
                phase: Phase::Connecting,
                queue: VecDeque::new(),
                bytes: 0,
                failure: None,
                waker: None,
            }),
            config,
        });
        let handlers = Handlers::install(&shared);

        let owner = Self {
            shared,
            _handlers: handlers,
            window,
        };
        owner.wait(Phase::Open).await?;
        Ok(owner)
    }
    /// Sends one bounded UTF-8 message synchronously without retrying.
    /// Backpressure and message-limit errors leave the open socket usable.
    pub fn send(&mut self, text: &str) -> Result<(), WsError> {
        let state = self.shared.state.borrow();

        if state.phase != Phase::Open {
            return Err(state.failure.unwrap_or(WsError::Closed));
        }

        if text.len() > self.shared.config.message_bytes {
            return Err(WsError::MessageLimit);
        }

        let remaining = self
            .shared
            .config
            .buffered_bytes
            .saturating_sub(self.shared.socket.buffered_amount());

        if text.len() > remaining as usize {
            return Err(WsError::Backpressure);
        }

        drop(state);
        self.shared.socket.send_with_str(text).map_err(|_| {
            self.shared.stop(Some(WsError::Browser));
            WsError::Browser
        })
    }
    /// Awaits one complete text message. Cancellation preserves the socket and queue.
    /// A deadline ends the socket and discards all retained messages.
    pub async fn receive(&mut self) -> Result<String, WsError> {
        let _deadline = Deadline::new(&self.shared, &self.window)?;
        poll_fn(|cx| {
            let waker = cx.waker().clone();
            let mut state = self.shared.state.borrow_mut();

            if state.phase != Phase::Open {
                return Poll::Ready(Err(state.failure.unwrap_or(WsError::Closed)));
            }

            if let Some(text) = state.queue.pop_front() {
                state.bytes -= text.len();
                return Poll::Ready(Ok(text));
            }

            let previous = state.waker.replace(waker);
            drop(state);
            drop(previous);

            Poll::Pending
        })
        .await
    }
    /// Consumes the socket, discards queued data and waits for the close event.
    /// Cancelling this future drops and closes the owner immediately.
    pub async fn close(self) -> Result<(), WsError> {
        {
            let mut state = self.shared.state.borrow_mut();

            if state.phase == Phase::Closed {
                return state.failure.map_or(Ok(()), Err);
            }

            state.phase = Phase::Closing;
            state.queue.clear();
            state.bytes = 0;
        }
        self.shared.socket.close().map_err(|_| WsError::Browser)?;
        self.wait(Phase::Closed).await
    }
    async fn wait(&self, target: Phase) -> Result<(), WsError> {
        let _deadline = Deadline::new(&self.shared, &self.window)?;
        poll_fn(|cx| {
            let waker = cx.waker().clone();
            let mut state = self.shared.state.borrow_mut();

            if state.phase == Phase::Closed {
                return Poll::Ready(state.failure.map_or(Ok(()), Err));
            }

            if state.phase == target {
                return Poll::Ready(Ok(()));
            }

            let previous = state.waker.replace(waker);
            drop(state);
            drop(previous);

            Poll::Pending
        })
        .await
    }
}
impl Drop for BrowserWebSocket {
    fn drop(&mut self) {
        self.shared.stop(Some(WsError::Closed));
        self.shared.detach();
    }
}
struct Deadline {
    shared: Rc<Shared>,
    window: Window,
    timer: i32,
    _callback: Closure<dyn FnMut()>,
}
impl Deadline {
    fn new(shared: &Rc<Shared>, window: &Window) -> Result<Self, WsError> {
        let weak = Rc::downgrade(shared);
        let callback = Closure::new(move || {
            if let Some(shared) = weak.upgrade() {
                shared.stop(Some(WsError::Deadline));
            }
        });
        let timeout = i32::try_from(shared.config.operation_timeout.as_millis())
            .map_err(|_| WsError::Config)?;
        let timer = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                timeout,
            )
            .map_err(|_| WsError::Unavailable)?;
        Ok(Self {
            shared: shared.clone(),
            window: window.clone(),
            timer,
            _callback: callback,
        })
    }
}
impl Drop for Deadline {
    fn drop(&mut self) {
        self.window.clear_timeout_with_handle(self.timer);
        let cancelled = self.shared.state.borrow_mut().waker.take();
        drop(cancelled);
    }
}
