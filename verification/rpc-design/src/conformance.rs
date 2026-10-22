//! Small portable public-protocol cases for later host harness integration.
use solana_kit_client::{
    Client, Error, Limits, Reply, Request, Transport, TransportError, TransportFailureKind,
    methods::Balance,
};
use solana_kit_core::Address;
use std::{
    future::{Future, Ready, ready},
    pin::pin,
    task::{Context, Poll, Waker},
};

const SUCCESS: u16 = 200;
const SLOT: u64 = 9;
const BODY: &str =
    r#"{"jsonrpc":"2.0","id":1,"result":{"context":{"slot":9},"value":18446744073709551615}}"#;
struct Fixture {
    wrong_id: bool,
}

impl Transport for Fixture {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;
    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let expected = br#"{"jsonrpc":"2.0","id":1,"method":"getBalance","params":["11111111111111111111111111111111"]}"#;
        let body = if self.wrong_id {
            BODY.replace("\"id\":1", "\"id\":2")
        } else {
            BODY.into()
        };

        if request.body != expected {
            return ready(Err(TransportError::new(
                TransportFailureKind::InvalidResponse,
            )));
        }

        ready(Ok(Reply {
            status: SUCCESS,
            body: body.into_bytes(),
        }))
    }
}

/// Executes exact request/u64 success and wrong-correlation controls without I/O.
/// This module is ready for host runners; native execution proves only native.
pub fn verify() -> Result<(), &'static str> {
    for wrong_id in [false, true] {
        let client = Client::new(Fixture { wrong_id }, Limits::default());
        let mut call = pin!(client.call(Balance(Address::from_bytes([0; 32]), None)));
        let mut cx = Context::from_waker(Waker::noop());

        let accepted = match (wrong_id, call.as_mut().poll(&mut cx)) {
            (false, Poll::Ready(Ok(value))) => {
                value.value == u64::MAX && value.context.slot == SLOT
            }
            (true, Poll::Ready(Err(Error::Correlation))) => true,
            _ => false,
        };

        if !accepted {
            return Err("portable RPC protocol fixture mismatch");
        }
    }

    Ok(())
}

#[test]
fn portable_protocol_cases_execute() {
    verify().unwrap();
}
