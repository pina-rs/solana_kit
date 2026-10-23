#![cfg(feature = "mwa")]

use base64::{
    Engine,
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
};
use serde_json::{Value, json};
use solana_kit_core::crypto::Keypair;
use solana_kit_wallet::mwa::*;

const SAMPLE_PORT: u32 = 55123;
const HEX_RADIX: u32 = 16;

fn scalar(value: u8) -> [u8; 32] {
    let mut bytes = [0; 32];
    bytes[31] = value;
    bytes
}

fn decode(value: &str) -> Vec<u8> {
    let (pairs, remainder) = value.as_bytes().as_chunks::<2>();
    assert!(remainder.is_empty());
    pairs
        .iter()
        .map(|pair| u8::from_str_radix(core::str::from_utf8(pair).unwrap(), HEX_RADIX).unwrap())
        .collect()
}

fn hex(value: &[u8]) -> String {
    value.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn fixture() -> Value {
    serde_json::from_str(include_str!("fixtures/mwa/dart.json")).unwrap()
}

#[test]
fn dart_crypto_vectors_and_rust_signature_output() {
    let vectors = fixture();
    let association = AssociationKey::from_scalar(&scalar(1)).unwrap();
    let alice = AgreementKey::from_scalar(&scalar(2)).unwrap();
    let bob = AgreementKey::from_scalar(&scalar(3)).unwrap();
    assert_eq!(
        hex(&association.public_key().to_bytes()),
        vectors["association"]
    );
    assert_eq!(hex(&alice.public_key().to_bytes()), vectors["alice"]);
    assert_eq!(hex(&bob.public_key().to_bytes()), vectors["bob"]);
    let hello = decode(vectors["hello"].as_str().unwrap());

    assert_eq!(alice.hello_request(&association).as_slice(), hello);

    assert_eq!(
        verify_hello_request(&hello, &association.public_key()).unwrap(),
        alice.public_key()
    );
    let key = alice.session_key(&bob.public_key(), &association.public_key());
    let reverse_key = bob.session_key(&alice.public_key(), &association.public_key());
    let frame = decode(vectors["frame"].as_str().unwrap());
    assert_eq!(
        decrypt_message(&frame, &key).unwrap(),
        (1, "hello wallet".into())
    );
    assert_eq!(
        decrypt_message(&frame, &reverse_key).unwrap().1,
        "hello wallet"
    );
    let nonce = core::array::from_fn(|index| u8::try_from(index).unwrap());
    assert_eq!(
        encrypt_message("hello wallet", 1, &nonce, &key).unwrap(),
        frame
    );

    let jws: Vec<_> = vectors["jws"].as_str().unwrap().split('.').collect();
    let public = association.public_key();
    assert_eq!(
        public.verify_jws(vectors["jws"].as_str().unwrap()).unwrap()["test"],
        true
    );

    for invalid in [
        "",
        "a.b.c",
        "eyJhbGciOiJub25lIn0.e30.",
        "eyJhbGciOiJFUzI1NiJ9=.e30.",
    ] {
        assert!(public.verify_jws(invalid).is_err());
    }

    association
        .public_key()
        .verify(
            format!("{}.{}", jws[0], jws[1]).as_bytes(),
            &URL_SAFE_NO_PAD.decode(jws[2]).unwrap(),
        )
        .unwrap();
    let rust_jws = association
        .jws(json!({"test":true}).as_object().unwrap())
        .unwrap();
    assert_eq!(rust_jws, vectors["jws"]);
    let rust = json!({
        "hello": hex(&alice.hello_request(&association)),
        "frame": hex(&frame),
        "jws": rust_jws,
    });
    println!("RUST_ORACLE={rust}");
}

#[test]
fn canonical_point_scalar_and_signature_boundaries() {
    assert!(AssociationKey::from_scalar(&[0; 32]).is_err());
    assert!(AgreementKey::from_scalar(&[255; 32]).is_err());

    for length in [0, 1, 33, 64, 66, 129] {
        assert!(PublicPoint::from_bytes(&vec![4; length]).is_err());
    }

    for coordinate in [0, 1, 255] {
        let mut point = [coordinate; 65];
        point[0] = 4;
        assert!(PublicPoint::from_bytes(&point).is_err());
    }

    let mut order_two = [0; 65];
    order_two[0] = 4;
    order_two[32] = 1;
    assert!(PublicPoint::from_bytes(&order_two).is_err());
    let association = AssociationKey::from_scalar(&scalar(1)).unwrap();
    let public = association.public_key();
    let signature = association.sign(b"hello");
    public.verify(b"hello", &signature).unwrap();

    assert_eq!(public.verify(b"changed", &signature), Err(Error::Signature));
    assert_eq!(public.verify(b"hello", &[0; 64]), Err(Error::Signature));
    let mut compressed = public.to_bytes();
    compressed[0] = 2;
    assert!(PublicPoint::from_bytes(&compressed).is_err());
}

#[test]
fn hello_response_and_properties_require_authenticated_frames() {
    let association = AssociationKey::from_scalar(&scalar(1)).unwrap();
    let peer = AgreementKey::from_scalar(&scalar(3)).unwrap().public_key();

    for length in [0, 64] {
        assert!(
            AgreementKey::from_scalar(&scalar(2))
                .unwrap()
                .accept_hello(&vec![0; length], &association.public_key())
                .is_err()
        );
    }

    let (key, properties) = AgreementKey::from_scalar(&scalar(2))
        .unwrap()
        .accept_hello(&peer.to_bytes(), &association.public_key())
        .unwrap();
    assert!(properties.is_none());

    for version in ["{}", "{\"v\":\"legacy\"}"] {
        assert_eq!(
            SessionProperties::from_json(version)
                .unwrap()
                .protocol_version,
            ProtocolVersion::Legacy
        );
    }

    for version in ["{\"v\":1}", "{\"v\":\"1\"}", "{\"v\":\"v1\"}"] {
        assert_eq!(
            SessionProperties::from_json(version)
                .unwrap()
                .protocol_version,
            ProtocolVersion::V1
        );
    }

    for version in ["{\"v\":2}", "{\"v\":null}", "{\"v\":true}", "[]"] {
        assert!(SessionProperties::from_json(version).is_err());
    }

    let frame = encrypt_message("{\"v\":1}", 0, &[1; 12], &key).unwrap();
    let mut session = Session::new(key, 1, 0);
    assert_eq!(
        session.properties(&frame).unwrap().protocol_version,
        ProtocolVersion::V1
    );
    assert_eq!(session.decrypt(&frame), Err(Error::Sequence));
}

#[test]
fn authenticated_ordering_replay_tampering_and_exhaustion() {
    let key = SessionKey::from_bytes(&[7; 16]);
    let frame = encrypt_message("hello", 1, &[1; 12], &key).unwrap();

    for index in 0..frame.len() {
        let mut altered = frame.clone();
        altered[index] ^= 1;
        assert_eq!(decrypt_message(&altered, &key), Err(Error::Authentication));
    }

    for length in 0..32 {
        assert!(decrypt_message(&frame[..length], &key).is_err());
    }

    assert!(decrypt_message(&frame, &SessionKey::from_bytes(&[8; 16])).is_err());
    let gap = encrypt_message("gap", 2, &[2; 12], &key).unwrap();
    let mut session = Session::new(key, u32::MAX, 1);
    assert_eq!(session.decrypt(&gap), Err(Error::Sequence));
    let mut corrupt = frame.clone();
    corrupt[20] ^= 1;
    assert!(session.decrypt(&corrupt).is_err());
    assert_eq!(session.decrypt(&frame).unwrap(), "hello");

    assert_eq!(session.decrypt(&frame), Err(Error::Sequence));
    assert_eq!(session.decrypt(&gap).unwrap(), "gap");
    assert_eq!(&session.encrypt("last", &[3; 12]).unwrap()[..4], &[255; 4]);
    assert_eq!(session.encrypt("overflow", &[4; 12]), Err(Error::Sequence));
    session.close();
}

#[test]
fn associations_preserve_wallet_prefix() {
    let key = AssociationKey::from_scalar(&scalar(1))
        .unwrap()
        .public_key();

    for prefix in ["", "/", "/mobilewalletadapter", "/mobilewalletadapter/"] {
        let base = format!("https://wallet.example{prefix}");
        let local = Association::Local {
            key: key.clone(),
            protocol: "v1".into(),
            port: AssociationPort::new(SAMPLE_PORT).unwrap(),
        };
        let uri = association_uri(&local, Some(&base)).unwrap();
        assert_eq!(parse_association_uri(&uri).unwrap(), local);
    }
}

#[test]
fn association_port_boundaries() {
    let minimum_port = 49152;
    let maximum_port = 65535;

    for port in [0, minimum_port - 1, maximum_port + 1, u32::MAX] {
        assert!(AssociationPort::new(port).is_err());
    }

    for port in [minimum_port, maximum_port] {
        assert_eq!(u32::from(AssociationPort::new(port).unwrap().value()), port);
    }
}

#[test]
fn reflector_ids_and_query_escaping_round_trip() {
    let key = AssociationKey::from_scalar(&scalar(1))
        .unwrap()
        .public_key();
    assert!(ReflectorId::new(9_007_199_254_740_992).is_err());

    for id in [0, 1, 9_007_199_254_740_991] {
        let remote = Association::Remote {
            key: key.clone(),
            protocol: "v1 +/".into(),
            host: "reflector.example:443".into(),
            id: ReflectorId::new(id).unwrap(),
        };
        assert_eq!(
            parse_association_uri(&association_uri(&remote, None).unwrap()).unwrap(),
            remote
        );
    }
}

#[test]
fn associations_reject_wrong_bases_paths_and_ambiguous_parameters() {
    let key = AssociationKey::from_scalar(&scalar(1))
        .unwrap()
        .public_key();

    for base in [
        "http://wallet.example",
        "javascript:bad",
        "https:/missing-host",
    ] {
        let local = Association::Local {
            key: key.clone(),
            protocol: "v1".into(),
            port: AssociationPort::new(SAMPLE_PORT).unwrap(),
        };
        assert!(association_uri(&local, Some(base)).is_err());
    }
    let uri = format!(
        "solana-wallet:/v1/associate/local?association={}&port=55123",
        key.association_token()
    );
    assert!(parse_association_uri(&format!("{uri}&port=55124")).is_err());
    assert!(parse_association_uri(&uri.replace("/v1/", "/v2/")).is_err());
    assert!(parse_association_uri(&uri.replace("55123", "49151")).is_err());
}

#[test]
fn typed_rpc_legacy_and_v1_request_mapping() {
    let params = json!({"chain":"solana:mainnet", "auth_token":"opaque"})
        .as_object()
        .unwrap()
        .clone();
    let request = Request::new(1, Method::Authorize, params, ProtocolVersion::Legacy).unwrap();
    let encoded: Value = serde_json::from_str(&request.to_json().unwrap()).unwrap();
    assert_eq!(encoded["method"], "reauthorize");
    assert_eq!(encoded["params"]["cluster"], "mainnet-beta");
    let request = Request::new(
        2,
        Method::Reauthorize,
        json!({"auth_token":"opaque"}).as_object().unwrap().clone(),
        ProtocolVersion::V1,
    )
    .unwrap();
    assert_eq!(
        serde_json::from_str::<Value>(&request.to_json().unwrap()).unwrap()["method"],
        "authorize"
    );
}

#[test]
fn typed_rpc_rejects_wrong_response_ids_and_invalid_envelopes() {
    assert!(matches!(
        Response::from_json("{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{}}", 1).unwrap(),
        Response::Result(_)
    ));

    assert!(matches!(
        Response::from_json(
            "{\"jsonrpc\":\"2.0\",\"id\":1,\"error\":{\"code\":-1,\"message\":\"denied\"}}",
            1
        )
        .unwrap(),
        Response::Error(_)
    ));
    for invalid in [
        "{\"jsonrpc\":\"2.0\",\"id\":2,\"result\":{}}",
        "{\"id\":1,\"result\":{}}",
        "{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{},\"error\":{}}",
        "null",
    ] {
        assert!(Response::from_json(invalid, 1).is_err());
    }
}

#[test]
fn typed_capabilities_map_both_protocol_versions() {
    let legacy = map_capabilities(
        json!({"supports_clone_authorization":true})
            .as_object()
            .unwrap()
            .clone(),
        ProtocolVersion::Legacy,
    )
    .unwrap();
    assert_eq!(
        legacy["features"],
        json!(["solana:signTransactions", "solana:cloneAuthorization"])
    );
    let mapped = map_capabilities(legacy, ProtocolVersion::V1).unwrap();
    assert_eq!(mapped["supports_sign_and_send_transactions"], true);
}

#[test]
fn siws_matches_dart_and_verifies_exact_signer_message_and_algorithm() {
    let mut payload = SignInPayload {
        domain: Some("example.com".into()),
        address: Some("11111111111111111111111111111111".into()),
        nonce: Some("abcdefgh".into()),
        resources: Some(vec!["https://example.com/read".into()]),
        ..Default::default()
    };
    assert_eq!(payload.message().unwrap(), fixture()["siws"]);
    assert_eq!(
        SignInPayload::default().message().unwrap(),
        " wants you to sign in with your Solana account:"
    );
    let signer = Keypair::from_seed(&[9; 32]);
    payload.address = Some(signer.address().to_string());
    let message = payload.message().unwrap();
    let mut result = SignInResult {
        address: STANDARD.encode(signer.address().as_bytes()),
        signed_message: STANDARD.encode(&message),
        signature: STANDARD.encode(signer.sign(message.as_bytes()).as_bytes()),
        signature_type: None,
    };
    result.verify(&payload, signer.address()).unwrap();

    result.signature_type = Some("secp256k1".into());
    assert_eq!(
        result.verify(&payload, signer.address()),
        Err(Error::SignIn)
    );
    result.signature_type = None;
    payload.nonce = Some("changed".into());
    assert_eq!(
        result.verify(&payload, signer.address()),
        Err(Error::SignIn)
    );
}

#[test]
fn siws_rejects_injection_in_every_field_and_resource() {
    for field in [
        "domain",
        "address",
        "statement",
        "uri",
        "version",
        "chain_id",
        "nonce",
        "issued_at",
        "expiration_time",
        "not_before",
        "request_id",
    ] {
        for newline in ["\n", "\r", "\r\n"] {
            let mut value = json!({});
            value[field] = format!("value{newline}Nonce: injected").into();
            let malicious: SignInPayload = serde_json::from_value(value).unwrap();
            assert_eq!(malicious.message(), Err(Error::SignIn));
        }
    }

    for newline in ["\n", "\r", "\r\n"] {
        assert!(
            SignInPayload {
                resources: Some(vec![format!("read{newline}- admin")]),
                ..Default::default()
            }
            .message()
            .is_err()
        );
    }
}

#[test]
fn json_models_preserve_omission_null_and_solana_address_validation() {
    let identity: AppIdentity =
        serde_json::from_value(json!({"name":"Application", "uri":null})).unwrap();
    assert_eq!(
        serde_json::to_value(identity).unwrap(),
        json!({"name":"Application"})
    );
    let account: MwaAccount = serde_json::from_value(
        json!({"address":STANDARD.encode([0;32]), "chains":["solana:mainnet"]}),
    )
    .unwrap();
    assert_eq!(
        account.solana_address().unwrap(),
        solana_kit_core::Address::from_bytes([0; 32])
    );
    let invalid: MwaAccount = serde_json::from_value(json!({"address":"dGVzdA=="})).unwrap();
    assert_eq!(invalid.solana_address(), Err(Error::Json));
    let capabilities: WalletCapabilities = serde_json::from_value(
        json!({"supported_transaction_versions":["legacy",0],"max_messages_per_request":5}),
    )
    .unwrap();

    let encoded = serde_json::to_value(capabilities).unwrap();
    assert_eq!(
        encoded["supported_transaction_versions"],
        json!(["legacy", 0])
    );
    assert!(encoded.get("features").is_none());
    assert!(serde_json::from_value::<AuthorizationResult>(json!({"accounts":[]})).is_err());
    let options = SignAndSendOptions {
        skip_preflight: Some(false),
        min_context_slot: Some(u64::MAX),
        ..Default::default()
    };
    assert_eq!(
        serde_json::to_value(options).unwrap(),
        json!({"skip_preflight":false,"min_context_slot":u64::MAX})
    );
}

#[test]
fn encrypted_memory_session_handshake_authorization_and_close() {
    let association = AssociationKey::from_scalar(&scalar(1)).unwrap();
    let alice = AgreementKey::from_scalar(&scalar(2)).unwrap();
    let bob = AgreementKey::from_scalar(&scalar(3)).unwrap();
    let peer = verify_hello_request(
        &alice.hello_request(&association),
        &association.public_key(),
    )
    .unwrap();
    let wallet_key = bob.session_key(&peer, &association.public_key());
    let properties = encrypt_message("{\"v\":1}", 0, &[0; 12], &wallet_key).unwrap();
    let mut hello = bob.public_key().to_bytes().to_vec();
    hello.extend(properties);

    let (app_key, properties) = alice
        .accept_hello(&hello, &association.public_key())
        .unwrap();
    let mut app = Session::new(app_key, 1, 0);
    let version = app
        .properties(&properties.unwrap())
        .unwrap()
        .protocol_version;
    let mut wallet = Session::new(wallet_key, 1, 1);
    let request = Request::new(
        1,
        Method::Authorize,
        json!({"chain":"solana:mainnet"})
            .as_object()
            .unwrap()
            .clone(),
        version,
    )
    .unwrap();
    let encrypted = app.encrypt(&request.to_json().unwrap(), &[1; 12]).unwrap();
    let incoming: Value = serde_json::from_str(&wallet.decrypt(&encrypted).unwrap()).unwrap();
    assert_eq!(incoming["method"], "authorize");

    let response = json!({"jsonrpc":"2.0","id":incoming["id"],"result":{"accounts":[{"address":STANDARD.encode([0;32])}],"auth_token":"session-authorization"}});
    let encrypted = wallet.encrypt(&response.to_string(), &[2; 12]).unwrap();
    let Response::Result(result) =
        Response::from_json(&app.decrypt(&encrypted).unwrap(), request.id()).unwrap()
    else {
        panic!("authorization failed");
    };
    let authorized: AuthorizationResult = serde_json::from_value(Value::Object(result)).unwrap();
    assert_eq!(
        authorized.accounts[0].solana_address().unwrap(),
        solana_kit_core::Address::from_bytes([0; 32])
    );
    let request = Request::new(
        2,
        Method::Deauthorize,
        json!({"auth_token":authorized.auth_token})
            .as_object()
            .unwrap()
            .clone(),
        version,
    )
    .unwrap();
    let encrypted = app.encrypt(&request.to_json().unwrap(), &[3; 12]).unwrap();
    let incoming: Value = serde_json::from_str(&wallet.decrypt(&encrypted).unwrap()).unwrap();
    assert_eq!(incoming["method"], "deauthorize");
    app.close();
    wallet.close();
}
