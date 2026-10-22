//! Event ownership without detached work or callback cycles.
use super::{WsConfig, WsError};
use std::{cell::RefCell, collections::VecDeque, rc::Rc, task::Waker};
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{CloseEvent, Event, MessageEvent, WebSocket};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Phase {
    Connecting,
    Open,
    Closing,
    Closed,
}
pub(super) struct State {
    pub phase: Phase,
    pub queue: VecDeque<String>,
    pub bytes: usize,
    pub failure: Option<WsError>,
    pub waker: Option<Waker>,
}
pub(super) struct Shared {
    pub socket: WebSocket,
    pub state: RefCell<State>,
    pub config: WsConfig,
}
impl Shared {
    pub fn stop(&self, failure: Option<WsError>) {
        let waker = {
            let mut state = self.state.borrow_mut();

            if state.phase == Phase::Closed {
                return;
            }

            state.phase = Phase::Closed;
            state.failure = failure;
            state.queue.clear();
            state.bytes = 0;
            state.waker.take()
        };
        self.detach();
        // The browser exposes close as fallible even during terminal cleanup.
        // Its result cannot replace the first terminal cause.
        let _ = self.socket.close();
        wake(waker);
    }
    pub fn detach(&self) {
        self.socket.set_onopen(None);
        self.socket.set_onmessage(None);
        self.socket.set_onerror(None);
        self.socket.set_onclose(None);
    }
    fn open(&self) {
        let waker = {
            let mut state = self.state.borrow_mut();

            if state.phase != Phase::Connecting {
                return;
            }

            state.phase = Phase::Open;
            state.waker.take()
        };
        wake(waker);
    }
    fn message(&self, event: MessageEvent) {
        if self.state.borrow().phase != Phase::Open {
            return;
        }

        let text = match incoming(event.data(), self.config.message_bytes) {
            Ok(text) => text,
            Err(error) => {
                self.stop(Some(error));
                return;
            }
        };

        let waker = {
            let mut state = self.state.borrow_mut();

            if state.queue.len() >= self.config.queue_messages
                || text.len() > self.config.queue_bytes.saturating_sub(state.bytes)
            {
                drop(state);
                self.stop(Some(WsError::QueueLimit));

                return;
            }

            state.bytes += text.len();
            state.queue.push_back(text);
            state.waker.take()
        };
        wake(waker);
    }
}
fn incoming(data: wasm_bindgen::JsValue, max: usize) -> Result<String, WsError> {
    let value = data.dyn_ref::<js_sys::JsString>().ok_or(WsError::Binary)?;

    if value.length() as usize > max {
        return Err(WsError::MessageLimit);
    }

    let text = value.as_string().ok_or(WsError::Binary)?;

    if text.len() > max {
        return Err(WsError::MessageLimit);
    }

    Ok(text)
}
fn wake(waker: Option<Waker>) {
    if let Some(waker) = waker {
        waker.wake();
    }
}
/// Closures stay allocated until owner Drop, including while a terminal callback returns.
pub(super) struct Handlers {
    _open: Closure<dyn FnMut(Event)>,
    _message: Closure<dyn FnMut(MessageEvent)>,
    _error: Closure<dyn FnMut(Event)>,
    _close: Closure<dyn FnMut(CloseEvent)>,
}
impl Handlers {
    // Weak references break the socket-to-closure cycle. The owner retains closures
    // after terminal detachment so an executing callback can return safely.
    pub fn install(shared: &Rc<Shared>) -> Self {
        let weak = Rc::downgrade(shared);
        let open = Closure::new(move |_: Event| {
            if let Some(shared) = weak.upgrade() {
                shared.open();
            }
        });

        let weak = Rc::downgrade(shared);
        let message = Closure::new(move |event: MessageEvent| {
            if let Some(shared) = weak.upgrade() {
                shared.message(event);
            }
        });

        let weak = Rc::downgrade(shared);
        let error = Closure::new(move |_: Event| {
            if let Some(shared) = weak.upgrade() {
                shared.stop(Some(WsError::Browser));
            }
        });

        let weak = Rc::downgrade(shared);
        let close = Closure::new(move |event: CloseEvent| {
            if let Some(shared) = weak.upgrade() {
                let closing = shared.state.borrow().phase == Phase::Closing;
                let failure = (!closing).then_some(WsError::PeerClosed {
                    code: event.code(),
                    clean: event.was_clean(),
                });
                shared.stop(failure);
            }
        });

        let socket = &shared.socket;
        socket.set_onopen(Some(open.as_ref().unchecked_ref()));
        socket.set_onmessage(Some(message.as_ref().unchecked_ref()));
        socket.set_onerror(Some(error.as_ref().unchecked_ref()));
        socket.set_onclose(Some(close.as_ref().unchecked_ref()));

        Self {
            _open: open,
            _message: message,
            _error: error,
            _close: close,
        }
    }
}
