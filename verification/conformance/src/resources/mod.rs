//! Public resource-estimation contracts shared by every maintained runtime.
extern crate std;

mod cancellation;
mod limits;
mod metrics;
mod tuning;
mod wire;

use alloc::{borrow::Cow, string::ToString, vec, vec::Vec};
use core::{
    cell::RefCell,
    future::{Future, Ready, ready},
    pin::pin,
    task::{Context, Poll, Waker},
};
use serde_json::{Value, json};
use solana_kit_client::{
    Client, Limits, Reply, Request, Transport, TransportError, TransportFailureKind,
    resources::{ResourceMessage, VersionContext},
};
use solana_kit_core::{Address, Hash, Instruction, transaction::Lifetime};

const COMPUTE_USAGE: u32 = 7777;
const LOADED_USAGE: u32 = 3333;
const EXPLICIT_COMPUTE: u32 = 250_000;
const EXPLICIT_LOADED: u32 = 65_536;
const BLOCK_HEIGHT: u64 = 1000;
const SLOT: u64 = 42;
const PAYER: Address = Address::from_bytes([1; 32]);
const PROGRAM: Address = Address::from_bytes([9; 32]);
const BLOCKHASH: Hash = Hash::from_bytes([7; 32]);

/// Original resource groups execute on native, browser, WASI, and production wasmi.
pub(crate) fn cases() -> [crate::CaseResult; 6] {
    [
        crate::case("resources-version-limit-edits", limits::run()),
        crate::case("resources-simulation-wire", wire::run()),
        crate::case("resources-explicit-tuning", tuning::run()),
        crate::case("resources-metric-precedence", metrics::run()),
        crate::case("resources-original-failures", metrics::failures()),
        crate::case("resources-cancel-owned-simulation", cancellation::run()),
    ]
}

fn draft(version: VersionContext<'_>) -> ResourceMessage<'_> {
    ResourceMessage {
        version,
        payer: PAYER,
        lifetime: Lifetime::Blockhash {
            blockhash: BLOCKHASH,
            last_valid_block_height: BLOCK_HEIGHT,
        },
        instructions: Cow::Owned(vec![Instruction {
            program_address: PROGRAM,
            accounts: vec![],
            data: vec![],
        }]),
    }
}

fn result() -> Value {
    json!({
        "context": { "slot": SLOT },
        "value": {
            "accounts": null,
            "err": null,
            "fee": null,
            "loadedAccountsDataSize": LOADED_USAGE,
            "loadedAddresses": null,
            "logs": [],
            "preBalances": [],
            "postBalances": [],
            "preTokenBalances": [],
            "postTokenBalances": [],
            "returnData": null,
            "replacementBlockhash": null,
            "unitsConsumed": COMPUTE_USAGE
        }
    })
}

struct Fixture<'a> {
    requests: &'a RefCell<Vec<Value>>,
    response: Value,
}

impl Transport for Fixture<'_> {
    type Exchange<'a>
        = Ready<Result<Reply, TransportError>>
    where
        Self: 'a;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let Ok(request) = serde_json::from_slice::<Value>(&request.body) else {
            return ready(Err(TransportError::new(
                TransportFailureKind::InvalidResponse,
            )));
        };
        let mut response = self.response.clone();

        if request["params"][1]["replaceRecentBlockhash"] == true
            && let Some(value) = response.get_mut("value")
        {
            value["replacementBlockhash"] = json!({
                "blockhash": BLOCKHASH.to_string(), "lastValidBlockHeight": BLOCK_HEIGHT
            });
        }

        let response = json!({ "jsonrpc": "2.0", "id": request["id"], "result": response });
        self.requests.borrow_mut().push(request);
        let body = response.to_string().into_bytes();

        ready(Ok(Reply { status: 200, body }))
    }
}

fn client(requests: &RefCell<Vec<Value>>, response: Value) -> Client<Fixture<'_>> {
    Client::new(Fixture { requests, response }, Limits::default())
}

fn finish<F: Future>(future: F) -> Result<F::Output, &'static str> {
    let mut future = pin!(future);
    let mut context = Context::from_waker(Waker::noop());

    match future.as_mut().poll(&mut context) {
        Poll::Ready(value) => Ok(value),
        Poll::Pending => Err("resource fixture unexpectedly suspended"),
    }
}
