#![cfg(feature = "mwa")]

use serde_json::{Value, json};
use solana_kit_wallet::mwa::*;

fn wire_payload() -> Value {
    json!({
        "domain":"app.example", "address":"11111111111111111111111111111111",
        "statement":"Sign in to this application.", "uri":"https://app.example/login",
        "version":"1", "chainId":"solana:mainnet", "nonce":"abcdefgh",
        "issuedAt":"2026-10-23T00:00:00Z", "expirationTime":"2026-10-24T00:00:00Z",
        "notBefore":"2026-10-22T00:00:00Z", "requestId":"request-42",
        "resources":["https://app.example/read", "https://app.example/write"]
    })
}

fn expected_payload() -> SignInPayload {
    SignInPayload {
        domain: Some("app.example".into()),
        address: Some("11111111111111111111111111111111".into()),
        statement: Some("Sign in to this application.".into()),
        uri: Some("https://app.example/login".into()),
        version: Some("1".into()),
        chain_id: Some("solana:mainnet".into()),
        nonce: Some("abcdefgh".into()),
        issued_at: Some("2026-10-23T00:00:00Z".into()),
        expiration_time: Some("2026-10-24T00:00:00Z".into()),
        not_before: Some("2026-10-22T00:00:00Z".into()),
        request_id: Some("request-42".into()),
        resources: Some(vec![
            "https://app.example/read".into(),
            "https://app.example/write".into(),
        ]),
    }
}

#[test]
fn siws_wire_keys_preserve_full_payload_message_and_authorize_envelope() {
    let wire = wire_payload();
    let expected = expected_payload();
    let incoming: SignInPayload = serde_json::from_value(wire.clone()).unwrap();
    assert!(incoming == expected);
    assert_eq!(serde_json::to_value(&expected).unwrap(), wire);
    assert_eq!(
        incoming.message().unwrap(),
        concat!(
            "app.example wants you to sign in with your Solana account:\n",
            "11111111111111111111111111111111\n\n",
            "Sign in to this application.\n\n",
            "URI: https://app.example/login\nVersion: 1\nChain ID: solana:mainnet\n",
            "Nonce: abcdefgh\nIssued At: 2026-10-23T00:00:00Z\n",
            "Expiration Time: 2026-10-24T00:00:00Z\nNot Before: 2026-10-22T00:00:00Z\n",
            "Request ID: request-42\nResources:\n",
            "- https://app.example/read\n- https://app.example/write"
        )
    );

    for absent in [
        json!({}),
        json!({"chainId":null, "issuedAt":null, "expirationTime":null,
            "notBefore":null, "requestId":null}),
    ] {
        let payload: SignInPayload = serde_json::from_value(absent).unwrap();
        assert!(payload == SignInPayload::default());
        assert_eq!(serde_json::to_value(payload).unwrap(), json!({}));
    }

    let params = json!({
        "chain":"solana:mainnet", "auth_token":"opaque",
        "sign_in_payload":serde_json::to_value(expected).unwrap()
    });
    let request = Request::new(
        7,
        Method::Authorize,
        params.as_object().unwrap().clone(),
        ProtocolVersion::V1,
    )
    .unwrap();
    let encoded: Value = serde_json::from_str(&request.to_json().unwrap()).unwrap();
    assert_eq!(
        encoded,
        json!({"id":7, "jsonrpc":"2.0", "method":"authorize", "params":{
            "chain":"solana:mainnet", "auth_token":"opaque", "sign_in_payload":wire
        }})
    );
    let nested: SignInPayload =
        serde_json::from_value(encoded["params"]["sign_in_payload"].clone()).unwrap();
    assert!(nested == incoming);
}
