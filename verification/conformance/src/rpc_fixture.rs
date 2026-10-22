//! Immediate in-memory exchanges exercise public protocol framing without an executor.
extern crate std;

use solana_kit_client::{
    Client, Error, Limits, Reply, Request, Transport, TransportError, TransportFailureKind,
    methods::Method,
};
use std::{
    format,
    future::{Future, Ready, ready},
    pin::pin,
    string::String,
    task::{Context, Poll, Waker},
};

struct Fixture {
    request: String,
    response: String,
}

impl Transport for Fixture {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        if request.body != self.request.as_bytes() {
            return ready(Err(TransportError::new(
                TransportFailureKind::InvalidResponse,
            )));
        }

        ready(Ok(Reply {
            status: 200,
            body: self.response.as_bytes().to_vec(),
        }))
    }
}

pub(crate) fn call<M: Method>(
    method: M,
    name: &str,
    params: &str,
    result: &str,
) -> Result<M::Response, Error> {
    framed(
        method,
        name,
        params,
        &format!(r#"{{"jsonrpc":"2.0","id":1,"result":{result}}}"#),
    )
}

pub(crate) fn framed<M: Method>(
    method: M,
    name: &str,
    params: &str,
    response: &str,
) -> Result<M::Response, Error> {
    let client = Client::new(
        Fixture {
            request: format!(r#"{{"jsonrpc":"2.0","id":1,"method":"{name}","params":{params}}}"#),
            response: response.into(),
        },
        Limits::default(),
    );
    let mut future = pin!(client.call(method));

    match future
        .as_mut()
        .poll(&mut Context::from_waker(Waker::noop()))
    {
        Poll::Ready(value) => value,
        Poll::Pending => Err(Error::Transport(TransportError::new(
            TransportFailureKind::Unavailable,
        ))),
    }
}
