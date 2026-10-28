//! Immediate Rc-backed exchanges keep these fixtures independent of an executor.

use alloc::{format, rc::Rc, string::String, vec::Vec};
use core::{
    cell::{Cell, RefCell},
    future::{Future, Ready, ready},
    pin::pin,
    task::{Context, Poll, Waker},
};
use serde_json::Value;
use solana_kit_client::{
    Client, Error, Limits, Reply, Request, Transport, TransportError, methods::Method,
};

pub struct Recording {
    pub requests: Rc<RefCell<Vec<Value>>>,
    pub body: String,
    pub status: u16,
    pub budget: Rc<Cell<usize>>,
}

impl Transport for Recording {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let parsed = serde_json::from_slice(&request.body);
        self.budget.set(request.max_reply_bytes);

        match parsed {
            Ok(frame) => self.requests.borrow_mut().push(frame),
            Err(_) => {
                return ready(Err(TransportError::new(
                    solana_kit_client::TransportFailureKind::InvalidResponse,
                )));
            }
        }

        ready(Ok(Reply {
            status: self.status,
            body: self.body.as_bytes().to_vec(),
        }))
    }
}

pub fn poll_ready<F: Future>(future: F) -> Result<F::Output, &'static str> {
    let mut future = pin!(future);

    match future
        .as_mut()
        .poll(&mut Context::from_waker(Waker::noop()))
    {
        Poll::Ready(value) => Ok(value),
        Poll::Pending => Err("immediate Surfpool exchange unexpectedly pending"),
    }
}

pub fn call<M: Method>(
    method: M,
    name: &str,
    params: &str,
    result: &str,
) -> Result<M::Response, &'static str> {
    let requests = Rc::new(RefCell::new(Vec::new()));
    let budget = Rc::new(Cell::new(0));
    let client = Client::new(
        Recording {
            requests: Rc::clone(&requests),
            body: format!(r#"{{"jsonrpc":"2.0","id":1,"result":{result}}}"#),
            status: 200,
            budget: Rc::clone(&budget),
        },
        Limits::default(),
    );
    let response =
        poll_ready(client.call(method))?.map_err(|_| "Surfpool typed exchange failed")?;
    let expected: Value = serde_json::from_str(&format!(
        r#"{{"jsonrpc":"2.0","id":1,"method":"{name}","params":{params}}}"#
    ))
    .map_err(|_| "invalid request oracle")?;
    require(
        *requests.borrow() == [expected],
        "Surfpool request method or positional parameters changed",
    )?;
    require(
        budget.get() == Limits::default().bytes,
        "Surfpool did not forward the reply budget",
    )?;
    Ok(response)
}

pub fn framed<M: Method>(
    method: M,
    status: u16,
    body: &str,
    limits: Limits,
) -> Result<Result<M::Response, Error>, &'static str> {
    let client = Client::new(
        Recording {
            requests: Rc::new(RefCell::new(Vec::new())),
            body: body.into(),
            status,
            budget: Rc::new(Cell::new(0)),
        },
        limits,
    );
    poll_ready(client.call(method))
}

pub fn require(condition: bool, explanation: &'static str) -> Result<(), &'static str> {
    condition.then_some(()).ok_or(explanation)
}
