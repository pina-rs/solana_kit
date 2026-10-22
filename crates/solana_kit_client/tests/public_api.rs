//! Downstream consumers use the public client without private parsing seams.
use serde::{Deserialize, Serialize};
use solana_kit_client::{
    Client, Error, Limits, Presence, Reply, Request, Transport, TransportError,
    TransportFailureKind,
    methods::{AccountInfo, Balance, Method, ReadConfig},
};
use solana_kit_core::{Address, accounts::MaybeAccount};
use std::{
    error::Error as _,
    future::{Ready, ready},
};

struct Script(&'static str);
impl Transport for Script {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;
    fn exchange(&self, _: Request) -> Self::Exchange<'_> {
        ready(Ok(Reply {
            status: 200,
            body: self.0.as_bytes().to_vec(),
        }))
    }
}
fn address() -> Address {
    Address::from_bytes([0; 32])
}
fn account(
    result: &str,
) -> Result<
    solana_kit_client::methods::Contextual<solana_kit_core::accounts::MaybeEncodedAccount>,
    Error,
> {
    let request = AccountInfo(address(), ReadConfig::default());
    request.decode(result)
}

#[test]
fn full_account_bytes_preserve_owned_core_metadata_and_explicit_absence() {
    let value = account(r#"{"context":{"slot":8},"value":{"lamports":18446744073709551615,"owner":"11111111111111111111111111111111","executable":false,"rentEpoch":18446744073709551615,"space":3,"data":["AQID","base64"]}}"#).unwrap();
    let MaybeAccount::Present(value) = value.value else {
        panic!("present account expected")
    };
    assert_eq!(value.address, address());
    assert_eq!(value.owner, address());
    assert_eq!(value.lamports, u64::MAX);
    assert_eq!(value.rent_epoch, Some(u64::MAX));
    assert_eq!(value.data, [1, 2, 3]);
    assert_eq!(
        account(r#"{"context":{"slot":8},"value":null}"#)
            .unwrap()
            .value,
        MaybeAccount::Missing(address())
    );
}

#[test]
fn malformed_or_alternate_account_encodings_never_become_empty_success() {
    let template = r#"{"context":{"slot":8},"value":{"lamports":1,"owner":"11111111111111111111111111111111","executable":false,"rentEpoch":1,"space":3,"data":["AQID","base64"]}}"#;
    for malformed in [
        template.replace("AQID", "?"),
        template.replace("AQID", "AB=="),
        template.replace("base64", "jsonParsed"),
        template.replace("\"space\":3", "\"space\":0"),
        template.replace("\"space\":3,", ""),
        template.replace("\"rentEpoch\":1", "\"rentEpoch\":null"),
    ] {
        assert!(account(&malformed).is_err(), "{malformed}");
    }
}

#[derive(Serialize)]
struct ProviderStatus(Vec<String>);
#[derive(Deserialize, Debug, PartialEq)]
struct Status {
    healthy: bool,
}
impl Method for ProviderStatus {
    type Response = Status;
    const NAME: &'static str = "providerStatus";
    fn decode(&self, result: &str) -> Result<Self::Response, Error> {
        serde_json::from_str(result).map_err(Error::Schema)
    }
}

#[test]
fn downstream_typed_extension_reuses_real_envelope_validation() {
    let method = || ProviderStatus(vec![]);
    let good = Client::new(
        Script(r#"{"jsonrpc":"2.0","id":1,"result":{"healthy":true}}"#),
        Limits::default(),
    );
    assert_eq!(
        futures::executor::block_on(good.call(method())).unwrap(),
        Status { healthy: true }
    );
    let wrong = Client::new(
        Script(r#"{"jsonrpc":"2.0","id":2,"result":{"healthy":true}}"#),
        Limits::default(),
    );
    assert!(matches!(
        futures::executor::block_on(wrong.call(method())),
        Err(Error::Correlation)
    ));
}

#[test]
fn duplicate_unknown_keys_and_nested_remote_data_are_rejected() {
    for body in [
        r#"{"jsonrpc":"2.0","id":1,"result":{"context":{"slot":1,"future":0,"future":1},"value":0}}"#,
        r#"{"jsonrpc":"2.0","id":1,"error":{"code":-1,"message":"bad","data":{"x":0,"x":1}}}"#,
    ] {
        let client = Client::new(Script(body), Limits::default());
        assert!(matches!(
            futures::executor::block_on(client.call(Balance(address(), None))),
            Err(Error::Json(_))
        ));
    }
}

#[test]
fn adapter_error_keeps_original_source_and_missing_error_data_stays_missing() {
    let source = std::io::Error::new(std::io::ErrorKind::ConnectionReset, "fixture reset");
    let error = Error::from(TransportError::with_source(
        TransportFailureKind::Io,
        source,
    ));
    assert_eq!(
        error.source().unwrap().source().unwrap().to_string(),
        "fixture reset"
    );
    let client = Client::new(
        Script(r#"{"jsonrpc":"2.0","id":1,"error":{"code":-1,"message":"bad"}}"#),
        Limits::default(),
    );
    let Err(Error::Remote(remote)) =
        futures::executor::block_on(client.call(Balance(address(), None)))
    else {
        panic!("remote error expected")
    };
    assert!(matches!(remote.data, Presence::Missing));
}
