//! Opaque remote data keeps JSON numeric spellings without relaxing typed schemas.
use solana_kit_client::{
    Client, Error, Limits, Presence, Reply, Request, Transport, TransportError, methods::Balance,
};
use solana_kit_core::Address;
use std::future::{Ready, ready};

struct Script(String);

impl Transport for Script {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, _: Request) -> Self::Exchange<'_> {
        ready(Ok(Reply {
            status: 200,
            body: self.0.as_bytes().to_vec(),
        }))
    }
}

fn call(
    body: String,
    limits: Limits,
) -> Result<solana_kit_client::methods::Contextual<u64>, Error> {
    let client = Client::new(Script(body), limits);
    let method = Balance(Address::from_bytes([0; 32]), None);
    futures::executor::block_on(client.call(method))
}

fn remote(data: &str) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","id":1,"error":{{"code":-32002,"message":"opaque failure","data":{data}}}}}"#
    )
}

#[test]
fn arbitrary_numeric_lexemes_remain_exact_in_public_remote_errors() {
    for data in [
        "1e400",
        "-1e400",
        "18446744073709551616000000000000000000000000000001",
        "-92233720368547758090000000000000000000000000000001",
        "1.2345678901234567890123456789012345678901234567890",
        "1.2300E+00400",
        r#"{ "nested": [1e400, -1e400, {"decimal": 0.00012345678901234567890}] }"#,
        r#"{"$serde_json::private::Number":"ordinary object","nested":1e400}"#,
    ] {
        let Err(Error::Remote(error)) = call(remote(data), Limits::default()) else {
            panic!("remote error expected for {data}")
        };
        assert_eq!(error.code, -32002);
        assert_eq!(error.message, "opaque failure");
        let Presence::Present(value) = error.data else {
            panic!("present opaque data expected")
        };
        assert_eq!(value.get(), data);
    }
}

#[test]
fn synthetic_number_keys_never_bypass_decoded_duplicate_detection() {
    for data in [
        r#"{"$serde_json::private::Number":1e400,"$serde_json::private::Number":2}"#,
        r#"{"$serde_json::private::Number":1,"\u0024serde_json::private::Number":2}"#,
        r#"[{"x":1e400,"\u0078":-1e400}]"#,
        r#"{"nested":{"$serde_json::private::Number":{"a":1,"a":2}}}"#,
    ] {
        assert!(matches!(
            call(remote(data), Limits::default()),
            Err(Error::Json(_))
        ));
    }
}

#[test]
fn malformed_opaque_numbers_and_json_remain_invalid() {
    for data in [
        "01", "+1", "1e", "1e+", "1.", "NaN", "Infinity", "[1e400,]", "{\"a\":}",
    ] {
        assert!(matches!(
            call(remote(data), Limits::default()),
            Err(Error::Json(_))
        ));
    }
}

#[test]
fn arbitrary_precision_does_not_coerce_typed_integers_or_remote_codes() {
    for number in [
        "1e400",
        "-1e400",
        "18446744073709551616",
        "1.0",
        "1e0",
        "-1",
    ] {
        let body = format!(
            r#"{{"jsonrpc":"2.0","id":1,"result":{{"context":{{"slot":1}},"value":{number}}}}}"#
        );
        assert!(matches!(
            call(body, Limits::default()),
            Err(Error::Schema(_))
        ));
    }

    for code in [
        "1e400",
        "9223372036854775808",
        "-9223372036854775809",
        "1.0",
    ] {
        let body = remote("1e400").replace("-32002", code);
        assert!(matches!(call(body, Limits::default()), Err(Error::Json(_))));
    }
}

#[test]
fn opaque_numeric_values_still_obey_body_and_depth_guards_first() {
    let body = remote("{\"nested\":[1e400]}");
    let exact = Limits {
        bytes: body.len(),
        depth: 4,
    };
    assert!(matches!(call(body.clone(), exact), Err(Error::Remote(_))));
    assert!(matches!(
        call(
            body.clone(),
            Limits {
                bytes: body.len() - 1,
                ..exact
            }
        ),
        Err(Error::BodyLimit)
    ));
    assert!(matches!(
        call(body, Limits { depth: 3, ..exact }),
        Err(Error::DepthLimit)
    ));
}
