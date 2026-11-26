//! Monotonic browser deadlines on the caller's existing Window timer.
use solana_kit_client::{
    TransportError, TransportFailureKind,
    subscriptions::{Clock, ClockError},
};
use std::{
    cell::RefCell,
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, Waker},
};
use wasm_bindgen::{JsCast, closure::Closure};
use web_sys::{Performance, Window};

/// A browser `performance.now()` millisecond clock starting at construction.
///
/// Each pending sleep owns one Window timeout and cancels it on drop. Browsers
/// may delay background timers. A remaining delay beyond `i32::MAX` milliseconds
/// returns [`ClockError::DeadlineRange`].
pub struct BrowserClock {
    window: Window,
    performance: Performance,
    origin: f64,
}

impl BrowserClock {
    /// Captures the current browser Window and performance origin.
    ///
    /// Fails with [`ClockError::Unavailable`] when the Window or its performance
    /// object is missing. Call it from the window that will drive the session.
    pub fn new() -> Result<Self, ClockError> {
        let window = web_sys::window().ok_or_else(unavailable)?;
        let performance = window.performance().ok_or_else(unavailable)?;
        let origin = performance.now();

        Ok(Self {
            window,
            performance,
            origin,
        })
    }
}

impl Clock for BrowserClock {
    type Sleep<'a> = BrowserClockSleep;

    fn now(&self) -> u64 {
        (self.performance.now() - self.origin) as u64
    }

    fn sleep_until(&self, deadline: u64) -> BrowserClockSleep {
        BrowserClockSleep::new(&self.window, self.now(), deadline)
    }
}

/// One owned Window timeout. Drop clears it before releasing its callback.
pub struct BrowserClockSleep {
    state: SleepState,
}

enum SleepState {
    Waiting(Alarm),
    Complete(Option<Result<(), ClockError>>),
}

impl BrowserClockSleep {
    fn new(window: &Window, now: u64, deadline: u64) -> Self {
        let state = match i32::try_from(deadline.saturating_sub(now)) {
            Ok(0) => SleepState::Complete(Some(Ok(()))),
            Ok(delay) => match Alarm::new(window, delay) {
                Ok(alarm) => SleepState::Waiting(alarm),
                Err(error) => SleepState::Complete(Some(Err(error))),
            },
            Err(_) => SleepState::Complete(Some(Err(ClockError::DeadlineRange))),
        };

        Self { state }
    }
}

impl Future for BrowserClockSleep {
    type Output = Result<(), ClockError>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        match &mut this.state {
            SleepState::Waiting(alarm) => {
                core::task::ready!(alarm.poll(context));
                this.state = SleepState::Complete(None);

                Poll::Ready(Ok(()))
            }
            SleepState::Complete(result) => Poll::Ready(
                result
                    .take()
                    .expect("completed browser clock sleep polled again"),
            ),
        }
    }
}

#[derive(Default)]
struct AlarmState {
    fired: bool,
    waker: Option<Waker>,
}

struct Alarm {
    window: Window,
    timer: i32,
    state: Rc<RefCell<AlarmState>>,
    _callback: Closure<dyn FnMut()>,
}

impl Alarm {
    fn new(window: &Window, delay: i32) -> Result<Self, ClockError> {
        let state = Rc::new(RefCell::new(AlarmState::default()));
        let callback_state = Rc::clone(&state);
        let callback = Closure::new(move || {
            let waker = {
                let mut state = callback_state.borrow_mut();
                state.fired = true;
                state.waker.take()
            };

            if let Some(waker) = waker {
                waker.wake();
            }
        });
        let timer = window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                delay,
            )
            .map_err(|_| unavailable())?;

        Ok(Self {
            window: window.clone(),
            timer,
            state,
            _callback: callback,
        })
    }

    fn poll(&self, context: &mut Context<'_>) -> Poll<()> {
        let waker = context.waker().clone();
        let mut state = self.state.borrow_mut();

        if state.fired {
            return Poll::Ready(());
        }

        // A custom waker may run user code when cloned or dropped.
        let previous = state.waker.replace(waker);
        drop(state);
        drop(previous);
        Poll::Pending
    }
}

impl Drop for Alarm {
    fn drop(&mut self) {
        self.window.clear_timeout_with_handle(self.timer);
    }
}

fn unavailable() -> ClockError {
    ClockError::Unavailable(TransportError::new(TransportFailureKind::Unavailable))
}
