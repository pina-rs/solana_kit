//! Event ownership without detached work or callback cycles.
use super::{WsConfig, WsError};
use solana_kit_client::subscriptions::ReceivedFrame;
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
    pub queue: VecDeque<ReceivedFrame>,
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
    // Reserve both queue budgets before copying data out of browser-owned storage.
    fn message(&self, event: MessageEvent) {
        if self.state.borrow().phase != Phase::Open {
            return;
        }

        let incoming = match Incoming::new(event.data(), self.config.message_bytes) {
            Ok(incoming) => incoming,
            Err(error) => {
                self.stop(Some(error));
                return;
            }
        };

        let waker = {
            let mut state = self.state.borrow_mut();

            if state.queue.len() >= self.config.queue_messages
                || incoming.len() > self.config.queue_bytes.saturating_sub(state.bytes)
            {
                drop(state);
                self.stop(Some(WsError::QueueLimit));

                return;
            }

            let frame = match incoming.copy() {
                Ok(frame) => frame,
                Err(error) => {
                    drop(state);
                    self.stop(Some(error));

                    return;
                }
            };

            state.bytes += frame.len();

            state.queue.push_back(frame);
            state.waker.take()
        };
        wake(waker);
    }
}
enum Incoming {
    Text(js_sys::JsString, usize),
    Binary(js_sys::Uint8Array, usize),
}
impl Incoming {
    fn new(data: wasm_bindgen::JsValue, max: usize) -> Result<Self, WsError> {
        let incoming = if let Some(text) = data.dyn_ref::<js_sys::JsString>() {
            let bytes = utf8_len(text, max)?;
            Self::Text(text.clone(), bytes)
        } else if data.is_instance_of::<js_sys::ArrayBuffer>() {
            let bytes = js_sys::Uint8Array::new(&data);
            let len = usize::try_from(bytes.length()).map_err(|_| WsError::MessageLimit)?;
            Self::Binary(bytes, len)
        } else {
            return Err(WsError::Binary);
        };

        if incoming.len() > max {
            return Err(WsError::MessageLimit);
        }

        Ok(incoming)
    }

    fn len(&self) -> usize {
        match self {
            Self::Text(_, len) | Self::Binary(_, len) => *len,
        }
    }

    fn copy(self) -> Result<ReceivedFrame, WsError> {
        match self {
            Self::Text(text, _) => text
                .as_string()
                .map(ReceivedFrame::Text)
                .ok_or(WsError::Binary),
            Self::Binary(bytes, _) => Ok(ReceivedFrame::Binary(bytes.to_vec())),
        }
    }
}

fn utf8_len(text: &js_sys::JsString, max: usize) -> Result<usize, WsError> {
    let mut index = 0;
    let mut bytes = 0_usize;

    while index < text.length() {
        let (width, units) = utf8_width(text, index);
        bytes = bytes.checked_add(width).ok_or(WsError::MessageLimit)?;

        if bytes > max {
            return Err(WsError::MessageLimit);
        }

        index += units;
    }

    Ok(bytes)
}

// A paired surrogate consumes two code units; isolated surrogates become replacement characters.
fn utf8_width(text: &js_sys::JsString, index: u32) -> (usize, u32) {
    const ASCII_MAX: f64 = 127.0;
    const TWO_BYTE_MAX: f64 = 2047.0;
    const HIGH_SURROGATE: std::ops::RangeInclusive<f64> = 55296.0..=56319.0;
    const LOW_SURROGATE: std::ops::RangeInclusive<f64> = 56320.0..=57343.0;
    let unit = text.char_code_at(index);

    if unit <= ASCII_MAX {
        return (1, 1);
    }

    if unit <= TWO_BYTE_MAX {
        return (2, 1);
    }

    let paired = HIGH_SURROGATE.contains(&unit)
        && index + 1 < text.length()
        && LOW_SURROGATE.contains(&text.char_code_at(index + 1));

    if paired { (4, 2) } else { (3, 1) }
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
