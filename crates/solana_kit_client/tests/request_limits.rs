//! Bounded preparation preserves public framing and error precedence.

use serde::{Serialize, Serializer};
use serde_json::value::RawValue;
use solana_kit_client::methods::Method;
use solana_kit_client::{Client, Error, Limits, Reply, Request, Transport, TransportError};
use std::{
    cell::RefCell,
    future::{Ready, ready},
    rc::Rc,
};

const RAW_PARAMS: &str = "[18446744073709551615,1.2300E+00400]";
const EXACT_FRAME: &[u8] = concat!(
    r#"{"jsonrpc":"2.0","id":1,"method":"limitProbe","params":"#,
    "[18446744073709551615,1.2300E+00400]}"
)
.as_bytes();

struct Capture(Rc<RefCell<Option<Request>>>);

impl Transport for Capture {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        self.0.replace(Some(request));

        ready(Ok(Reply {
            status: 200,
            body: br#"{"jsonrpc":"2.0","id":1,"result":7}"#.to_vec(),
        }))
    }
}

#[derive(Serialize)]
struct Params<'a>(&'a RawValue);

impl Method for Params<'_> {
    type Response = u64;
    const NAME: &'static str = "limitProbe";

    fn decode(&self, result: &str) -> Result<Self::Response, Error> {
        serde_json::from_str(result).map_err(Error::Schema)
    }
}

#[derive(Serialize)]
struct Scalar<'a>(&'a str);

impl Method for Scalar<'_> {
    type Response = u64;
    const NAME: &'static str = "limitProbe";

    fn decode(&self, result: &str) -> Result<Self::Response, Error> {
        serde_json::from_str(result).map_err(Error::Schema)
    }
}

enum Failure {
    BeforeOutput,
    AfterScalar,
    AfterContainer,
}

impl Serialize for Failure {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::BeforeOutput => {}
            Self::AfterScalar => {
                true.serialize(serializer)?;
            }
            Self::AfterContainer => {
                [0_u8; 0].serialize(serializer)?;
            }
        }

        Err(serde::ser::Error::custom("producer failure"))
    }
}

impl Method for Failure {
    type Response = u64;
    const NAME: &'static str = "limitProbe";

    fn decode(&self, result: &str) -> Result<Self::Response, Error> {
        serde_json::from_str(result).map_err(Error::Schema)
    }
}

fn call<M: Method>(method: M, limits: Limits) -> (Result<M::Response, Error>, Option<Request>) {
    let captured = Rc::new(RefCell::new(None));
    let client = Client::new(Capture(captured.clone()), limits);
    let result = futures::executor::block_on(client.call(method));
    let request = captured.borrow_mut().take();

    (result, request)
}

fn params(source: &str) -> Params<'_> {
    Params(serde_json::from_str(source).expect("valid fixture params"))
}

#[test]
fn full_frame_overhead_and_exact_numeric_bytes_count_toward_the_budget() {
    let limits = Limits {
        bytes: EXACT_FRAME.len(),
        depth: 2,
    };
    let (result, request) = call(params(RAW_PARAMS), limits);
    let request = request.expect("one exactly admitted frame");

    assert_eq!(result.unwrap(), 7);
    assert_eq!(request.body, EXACT_FRAME);
    assert_eq!(request.max_reply_bytes, EXACT_FRAME.len());

    let (result, request) = call(
        params(RAW_PARAMS),
        Limits {
            bytes: EXACT_FRAME.len() - 1,
            ..limits
        },
    );

    assert!(matches!(result, Err(Error::BodyLimit)), "{result:?}");
    assert!(request.is_none());
}

#[test]
fn depth_counts_the_outer_frame_and_byte_denial_still_takes_precedence() {
    let (result, request) = call(
        params("[]"),
        Limits {
            bytes: 256,
            depth: 1,
        },
    );
    assert!(matches!(result, Err(Error::DepthLimit)), "{result:?}");
    assert!(request.is_none());

    let (result, request) = call(params("[[[0]]]"), Limits { bytes: 1, depth: 0 });
    assert!(matches!(result, Err(Error::BodyLimit)), "{result:?}");
    assert!(request.is_none());
}

#[test]
fn invalid_scalar_params_remain_envelope_errors_even_with_smaller_byte_limits() {
    let value = "x".repeat(4096);

    for bytes in [0, 1, 32] {
        let (result, request) = call(Scalar(&value), Limits { bytes, depth: 0 });

        assert!(matches!(result, Err(Error::Envelope)), "{result:?}");
        assert!(request.is_none());
    }
}

#[test]
fn ordinary_producer_errors_stay_json_errors_before_envelope_validation() {
    for failure in [
        Failure::BeforeOutput,
        Failure::AfterScalar,
        Failure::AfterContainer,
    ] {
        let (result, request) = call(failure, Limits::default());
        let Err(Error::Json(error)) = result else {
            panic!("original serialization error expected: {result:?}");
        };

        assert!(error.to_string().contains("producer failure"));
        assert!(request.is_none());
    }
}
