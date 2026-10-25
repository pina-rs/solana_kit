use super::WalletFailure;
use std::{
    cell::{Cell, RefCell},
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, Waker},
};

/// A fresh identity avoids wrapping counters when connections are replaced.
pub(super) struct Token {
    valid: Cell<bool>,
    waker: RefCell<Option<Waker>>,
}

impl Token {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            valid: Cell::new(true),
            waker: RefCell::new(None),
        })
    }

    pub fn cancel(&self) {
        self.valid.set(false);
        let waker = self.waker.borrow_mut().take();

        if let Some(waker) = waker {
            waker.wake();
        }
    }

    pub fn valid(&self) -> bool {
        self.valid.get()
    }
}

pub(super) struct Current<'a, T> {
    token: Rc<Token>,
    future: Pin<Box<dyn Future<Output = Result<T, WalletFailure>> + 'a>>,
}

impl<'a, T> Current<'a, T> {
    pub fn new(
        token: Rc<Token>,
        future: impl Future<Output = Result<T, WalletFailure>> + 'a,
    ) -> Self {
        Self {
            token,
            future: Box::pin(future),
        }
    }
}

impl<T> Future for Current<'_, T> {
    type Output = Result<T, WalletFailure>;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.token.valid() {
            return Poll::Ready(Err(WalletFailure::disconnected()));
        }

        *self.token.waker.borrow_mut() = Some(context.waker().clone());
        let result = self.future.as_mut().poll(context);

        if !self.token.valid() {
            return Poll::Ready(Err(WalletFailure::disconnected()));
        }

        result
    }
}
