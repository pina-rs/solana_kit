//! Compute estimation uses the shared bounded RPC frame and typed simulation result.

mod cancellation;

use futures::executor::block_on;
use serde_json::{Value, json};
use solana_kit_client::{
    Client, Error, Limits, Presence, Reply, Request, Transport, TransportError,
    TransportFailureKind,
    methods::{Commitment, Contextual, ReadConfig, submission::TransactionFailure},
};
use solana_kit_helius::transactions::{ComputeUnitsError, get_compute_units};
use std::{
    cell::RefCell,
    future::{Ready, ready},
    rc::Rc,
};

const WIRE: &[u8] = &[0, 1, 254, 255];
const SLOT: u64 = 9_007_199_254_740_993;
const MIN_SLOT: u64 = SLOT - 1;
const PRIVATE_DETAIL: &str = "private-execution-detail";
const LARGE_LOG_BYTES: usize = 1024;
const SMALL_BODY_LIMIT: usize = 512;
const REQUEST_JSON_DEPTH: usize = 3;

type Estimate = Result<Contextual<u64>, ComputeUnitsError>;
type Requests = Rc<RefCell<Vec<Request>>>;

struct Rpc {
    body: Vec<u8>,
    requests: Requests,
}

impl Transport for Rpc {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        self.requests.borrow_mut().push(request);
        ready(Ok(Reply {
            status: 200,
            body: self.body.clone(),
        }))
    }
}

fn read() -> ReadConfig {
    ReadConfig {
        commitment: Some(Commitment::Confirmed),
        min_context_slot: Some(MIN_SLOT),
    }
}

fn response(units: Option<Value>, error: Value) -> Value {
    let mut execution = json!({
        "err": error, "fee": null, "accounts": null, "loadedAddresses": null,
        "logs": null, "preBalances": null, "postBalances": null,
        "preTokenBalances": null, "postTokenBalances": null, "returnData": null,
        "replacementBlockhash": {
            "blockhash": "11111111111111111111111111111111",
            "lastValidBlockHeight": u64::MAX
        }
    });

    if let Some(units) = units {
        execution["unitsConsumed"] = units;
    }

    json!({"context":{"slot":SLOT,"apiVersion":"fixture-api"},"value":execution})
}

fn envelope(result: Value) -> Vec<u8> {
    json!({"jsonrpc":"2.0","id":1,"result":result})
        .to_string()
        .into_bytes()
}

fn estimate(body: Vec<u8>, limits: Limits) -> (Estimate, Requests) {
    let requests = Rc::new(RefCell::new(Vec::new()));
    let client = Client::new(
        Rpc {
            body,
            requests: Rc::clone(&requests),
        },
        limits,
    );
    let result = block_on(get_compute_units(&client, WIRE, read()));
    (result, requests)
}

fn call(result: Value) -> Estimate {
    let (result, requests) = estimate(envelope(result), Limits::default());
    assert_eq!(requests.borrow().len(), 1);
    result
}

fn assert_frame(request: &Request) {
    let body: Value = serde_json::from_slice(&request.body).expect("actual RPC request");
    assert_eq!(
        body,
        json!({"jsonrpc":"2.0","id":1,"method":"simulateTransaction",
            "params":["AAH+/w==",{"encoding":"base64","commitment":"confirmed",
                "minContextSlot":MIN_SLOT,"replaceRecentBlockhash":true,"sigVerify":false}]})
    );
    assert_eq!(request.max_reply_bytes, Limits::default().bytes);
}

/// Exact reports, explicit failures, bounded admission and cancellation share one portable group.
pub fn estimation() {
    exact_reports();
    execution_failures();
    unavailable_and_malformed_units();
    rpc_boundaries();
    transport_failure();
    cancellation::dropped_exchange();
}

fn exact_reports() {
    for units in [0, SLOT, u64::MAX] {
        let (result, requests) = estimate(
            envelope(response(Some(json!(units)), Value::Null)),
            Limits::default(),
        );
        let result = result.expect("reported exact compute units");
        assert_eq!(result.value, units);
        assert_eq!(result.context.slot, SLOT);
        assert_eq!(
            result.context.api_version,
            Presence::Present("fixture-api".into())
        );
        let requests = requests.borrow();
        assert_eq!(requests.len(), 1);
        assert_frame(&requests[0]);
    }

    let mut stale = response(Some(json!(0)), Value::Null);
    stale["context"]["slot"] = json!(0);
    assert!(matches!(
        call(stale),
        Err(ComputeUnitsError::StaleContext { context, minimum_slot })
            if context.slot == 0 && minimum_slot == MIN_SLOT
    ));
}

fn execution_failures() {
    for units in [None, Some(Value::Null), Some(json!(u64::MAX))] {
        let error = call(response(units, json!({"FutureExecution":PRIVATE_DETAIL})))
            .expect_err("failed execution cannot yield an estimate");
        assert!(!format!("{error:?} {error}").contains(PRIVATE_DETAIL));
        assert!(std::error::Error::source(&error).is_some());

        match error {
            ComputeUnitsError::Execution {
                context,
                failure: TransactionFailure::Unknown(raw),
            } => {
                assert_eq!(context.slot, SLOT);
                assert_eq!(
                    raw.get(),
                    r#"{"FutureExecution":"private-execution-detail"}"#
                );
            }
            other => panic!("wrong execution error: {other:?}"),
        }
    }

    assert!(matches!(
        call(response(
            Some(json!(0)),
            json!({"InstructionError":[0,{"Custom":7}]})
        )),
        Err(ComputeUnitsError::Execution {
            failure: TransactionFailure::Known(_),
            ..
        })
    ));
}

fn unavailable_and_malformed_units() {
    for units in [None, Some(Value::Null)] {
        let error = call(response(units, Value::Null)).expect_err("units must be reported");
        assert!(std::error::Error::source(&error).is_none());
        assert!(
            matches!(error, ComputeUnitsError::MissingUnits { context } if context.slot == SLOT)
        );
    }

    let overflow: Value = serde_json::from_str("18446744073709551616").expect("JSON integer");

    for invalid in [
        json!(-1),
        json!(1.5),
        json!("1"),
        json!(true),
        json!([]),
        overflow,
    ] {
        assert!(matches!(
            call(response(Some(invalid), Value::Null)),
            Err(ComputeUnitsError::Rpc(Error::Schema(_)))
        ));
    }
}

fn rpc_boundaries() {
    let remote = json!({"jsonrpc":"2.0","id":1,
        "error":{"code":-1,"message":PRIVATE_DETAIL}});
    let (result, requests) = estimate(remote.to_string().into_bytes(), Limits::default());
    let error = result.expect_err("original RPC error");
    assert!(!format!("{error:?} {error}").contains(PRIVATE_DETAIL));
    assert!(matches!(error, ComputeUnitsError::Rpc(Error::Remote(_))));
    assert_eq!(requests.borrow().len(), 1);

    for body in [
        b"{".to_vec(),
        br#"{"jsonrpc":"2.0","id":2,"result":{}}"#.to_vec(),
    ] {
        let (result, requests) = estimate(body, Limits::default());
        assert!(matches!(result, Err(ComputeUnitsError::Rpc(_))));
        assert_eq!(requests.borrow().len(), 1);
    }

    body_budgets();
}

fn body_budgets() {
    let mut result = response(Some(json!(0)), Value::Null);
    result["value"]["logs"] = json!(["x".repeat(LARGE_LOG_BYTES)]);
    let (result, requests) = estimate(
        envelope(result),
        Limits {
            bytes: SMALL_BODY_LIMIT,
            depth: Limits::default().depth,
        },
    );
    assert!(matches!(
        result,
        Err(ComputeUnitsError::Rpc(Error::BodyLimit))
    ));
    assert_eq!(requests.borrow().len(), 1);

    let (result, requests) = estimate(
        envelope(response(Some(json!(0)), Value::Null)),
        Limits {
            bytes: 0,
            depth: Limits::default().depth,
        },
    );
    assert!(matches!(
        result,
        Err(ComputeUnitsError::Rpc(Error::BodyLimit))
    ));
    assert!(requests.borrow().is_empty());

    let (result, requests) = estimate(
        envelope(response(Some(json!(0)), Value::Null)),
        Limits {
            bytes: Limits::default().bytes,
            depth: REQUEST_JSON_DEPTH,
        },
    );
    assert!(matches!(
        result,
        Err(ComputeUnitsError::Rpc(Error::DepthLimit))
    ));
    assert_eq!(requests.borrow().len(), 1);
}

fn transport_failure() {
    struct Unavailable;

    impl Transport for Unavailable {
        type Exchange<'a> = Ready<Result<Reply, TransportError>>;

        fn exchange(&self, _: Request) -> Self::Exchange<'_> {
            ready(Err(TransportError::new(TransportFailureKind::Unavailable)))
        }
    }

    let client = Client::new(Unavailable, Limits::default());
    let error =
        block_on(get_compute_units(&client, WIRE, read())).expect_err("original adapter failure");
    assert!(std::error::Error::source(&error).is_some());
    assert!(matches!(
        error,
        ComputeUnitsError::Rpc(Error::Transport(error))
            if error.kind == TransportFailureKind::Unavailable
    ));
}
