//! Literal server fixtures execute the public client and inspect the whole request.
use solana_kit_client::{
    Client, Error, Limits, Reply, Request, Transport, TransportError, methods::Method,
};
use solana_kit_core::Address;
use std::future::{Ready, ready};

pub const ADDRESS: &str = "11111111111111111111111111111111";
pub const MAX: &str = "18446744073709551615";

pub fn address() -> Address {
    ADDRESS.parse().unwrap()
}

struct Script {
    expected: String,
    response: String,
}

impl Transport for Script {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;
    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        assert_eq!(std::str::from_utf8(&request.body).unwrap(), self.expected);
        assert_eq!(request.max_reply_bytes, 8192);
        ready(Ok(Reply {
            status: 200,
            body: self.response.as_bytes().to_vec(),
        }))
    }
}

pub fn call<R: Method>(
    method: R,
    name: &str,
    params: &str,
    result: &str,
) -> Result<R::Response, Error> {
    envelope(
        method,
        name,
        params,
        &format!(r#"{{"jsonrpc":"2.0","id":1,"result":{result}}}"#),
    )
}

pub fn envelope<R: Method>(
    method: R,
    name: &str,
    params: &str,
    response: &str,
) -> Result<R::Response, Error> {
    let expected = format!(r#"{{"jsonrpc":"2.0","id":1,"method":"{name}","params":{params}}}"#);
    let client = Client::new(
        Script {
            expected,
            response: response.to_owned(),
        },
        Limits::default(),
    );
    futures::executor::block_on(client.call(method))
}
