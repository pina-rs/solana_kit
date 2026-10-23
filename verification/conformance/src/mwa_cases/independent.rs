extern crate std;
use base64::{
    Engine,
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
};
use serde_json::json;
use solana_kit_core::crypto::Keypair;
use solana_kit_wallet::mwa::*;
use std::format;
use std::prelude::v1::*;
use std::vec;

const RETRY_SEQUENCE: u32 = 91;
const WALLET_SEQUENCE: u32 = 20;
const PUBLIC_POINT_BYTES: usize = 65;

fn scalar(value: u8) -> [u8; 32] {
    let mut bytes = [0; 32];
    bytes[31] = value;
    bytes
}

fn key() -> SessionKey {
    SessionKey::from_bytes(&[19; 16])
}

pub(super) fn every_tampered_receive_preserves_state_for_successful_retry()
-> Result<(), &'static str> {
    let frame = encrypt_message("retry ∞", RETRY_SEQUENCE, &[1; 12], &key()).unwrap();

    for offset in 0..frame.len() {
        let mut session = Session::new(key(), RETRY_SEQUENCE, RETRY_SEQUENCE);
        let mut tampered = frame.clone();
        tampered[offset] ^= 0x80;
        check_eq!(session.decrypt(&tampered), Err(Error::Authentication));
        check_eq!(session.decrypt(&frame).unwrap(), "retry ∞");
        check_eq!(session.decrypt(&frame), Err(Error::Sequence));
    }
    Ok(())
}

pub(super) fn gap_does_not_advance_and_receive_counter_cannot_wrap() -> Result<(), &'static str> {
    let mut session = Session::new(key(), u32::MAX, u32::MAX - 1);
    let penultimate = encrypt_message("before", u32::MAX - 1, &[2; 12], &key()).unwrap();
    let last = encrypt_message("last", u32::MAX, &[3; 12], &key()).unwrap();
    check_eq!(session.decrypt(&last), Err(Error::Sequence));
    check_eq!(session.decrypt(&penultimate).unwrap(), "before");
    check_eq!(session.decrypt(&last).unwrap(), "last");
    check_eq!(session.decrypt(&last), Err(Error::Sequence));
    let wrapped = encrypt_message("wrapped", 0, &[4; 12], &key()).unwrap();

    check_eq!(session.decrypt(&wrapped), Err(Error::Sequence));
    session.encrypt("last send", &[5; 12]).unwrap();
    check_eq!(session.encrypt("exhausted", &[6; 12]), Err(Error::Sequence));
    Ok(())
}

pub(super) fn directional_ordering_does_not_bind_the_sender_or_prevent_reflection()
-> Result<(), &'static str> {
    let mut app = Session::new(key(), 1, 1);
    let mut wallet = Session::new(key(), 1, 1);
    let request = app.encrypt("app request", &[7; 12]).unwrap();
    let response = wallet.encrypt("wallet reply", &[8; 12]).unwrap();
    check_eq!(wallet.decrypt(&request).unwrap(), "app request");
    check_eq!(app.decrypt(&response).unwrap(), "wallet reply");
    check_eq!(app.decrypt(&request), Err(Error::Sequence));
    let mut reflected = Session::new(key(), 1, 1);

    let own_frame = reflected.encrypt("reflected request", &[9; 12]).unwrap();
    check_eq!(reflected.decrypt(&own_frame).unwrap(), "reflected request");
    Ok(())
}

pub(super) fn cross_direction_iv_reuse_is_a_caller_obligation_separate_from_sequence()
-> Result<(), &'static str> {
    let mut app = Session::new(key(), 1, 1);
    let mut wallet = Session::new(key(), WALLET_SEQUENCE, 1);
    let app_plain = b"known-data";
    let wallet_plain = b"secret-msg";
    let app_frame = app
        .encrypt(core::str::from_utf8(app_plain).unwrap(), &[10; 12])
        .unwrap();
    let wallet_frame = wallet
        .encrypt(core::str::from_utf8(wallet_plain).unwrap(), &[10; 12])
        .unwrap();
    let recovered: Vec<_> = app_frame[16..26]
        .iter()
        .zip(&wallet_frame[16..26])
        .zip(app_plain)
        .map(|((left, right), known)| left ^ right ^ known)
        .collect();
    check_eq!(recovered, wallet_plain);
    Ok(())
}

pub(super) fn hello_authenticates_association_and_ephemeral_point_before_agreement()
-> Result<(), &'static str> {
    let association = AssociationKey::from_scalar(&scalar(1)).unwrap();
    let wrong = AssociationKey::from_scalar(&scalar(2)).unwrap();
    let ephemeral = AgreementKey::from_scalar(&scalar(3)).unwrap();
    let mut hello = ephemeral.hello_request(&association);
    check!(verify_hello_request(&hello, &wrong.public_key()).is_err());
    hello[..PUBLIC_POINT_BYTES].copy_from_slice(
        &AgreementKey::from_scalar(&scalar(4))
            .unwrap()
            .public_key()
            .to_bytes(),
    );
    check!(verify_hello_request(&hello, &association.public_key()).is_err());
    let mut invalid = [0; 129];

    invalid[0] = 4;
    let signature = association.sign(&invalid[..65]);
    invalid[65..].copy_from_slice(&signature);
    check_eq!(
        verify_hello_request(&invalid, &association.public_key()),
        Err(Error::PublicKey)
    );
    let peer = AgreementKey::from_scalar(&scalar(5)).unwrap();
    let peer_key = peer.session_key(&ephemeral.public_key(), &association.public_key());
    let frame = encrypt_message("bound salt", 0, &[11; 12], &peer_key).unwrap();
    let (wrong_salt, _) = ephemeral
        .accept_hello(&peer.public_key().to_bytes(), &wrong.public_key())
        .unwrap();
    check_eq!(
        decrypt_message(&frame, &wrong_salt),
        Err(Error::Authentication)
    );
    Ok(())
}

fn signed_compact(association: &AssociationKey, header: &str, payload: &str) -> String {
    let input = format!(
        "{}.{}",
        URL_SAFE_NO_PAD.encode(header),
        URL_SAFE_NO_PAD.encode(payload)
    );
    format!(
        "{input}.{}",
        URL_SAFE_NO_PAD.encode(association.sign(input.as_bytes()))
    )
}

pub(super) fn jws_rejects_correctly_signed_wrong_algorithm_shape_and_key_binding()
-> Result<(), &'static str> {
    let association = AssociationKey::from_scalar(&scalar(1)).unwrap();
    let public = association.public_key();

    for header in [
        r#"{"alg":"none"}"#,
        r#"{"alg":"ES384"}"#,
        r#"{"alg":"ES256","crit":["x"]}"#,
    ] {
        check!(
            public
                .verify_jws(&signed_compact(&association, header, "{}"))
                .is_err()
        );
    }

    for payload in ["null", "[]", "1", "true", "\"text\""] {
        check!(
            public
                .verify_jws(&signed_compact(&association, r#"{"alg":"ES256"}"#, payload))
                .is_err()
        );
    }

    let valid = association
        .jws(json!({"nonce":"abc"}).as_object().unwrap())
        .unwrap();
    let wrong = AssociationKey::from_scalar(&scalar(17))
        .unwrap()
        .public_key();
    check!(wrong.verify_jws(&valid).is_err());
    check!(public.verify_jws(&format!("{valid}.")).is_err());
    check!(public.verify_jws(&format!("{valid}=")).is_err());
    check_eq!(public.verify_jws(&valid).unwrap()["nonce"], "abc");
    Ok(())
}

pub(super) fn uri_decoded_duplicates_and_authority_redirection_are_rejected()
-> Result<(), &'static str> {
    let local = Association::Local {
        key: AssociationKey::from_scalar(&scalar(18))
            .unwrap()
            .public_key(),
        protocol: "v1 +&=λ".into(),
        port: AssociationPort::new(50000).unwrap(),
    };
    let uri = association_uri(
        &local,
        Some("https://wallet.example/prefix?discard=1#discard"),
    )
    .unwrap();
    check_eq!(parse_association_uri(&uri).unwrap(), local);
    check!(!uri.contains("discard"));

    for suffix in ["&%70ort=50001", "&v=v1", "&%61ssociation=AA", "&port=50000"] {
        check!(parse_association_uri(&format!("{uri}{suffix}")).is_err());
    }

    for host in [
        "trusted.example@evil.example",
        "trusted.example/path",
        "trusted.example?host=evil",
        "trusted.example#evil",
        "",
        "trusted.example:99999",
    ] {
        let remote = Association::Remote {
            key: AssociationKey::from_scalar(&scalar(18))
                .unwrap()
                .public_key(),
            protocol: "v1".into(),
            host: host.into(),
            id: ReflectorId::new(1).unwrap(),
        };
        check!(
            association_uri(&remote, None).is_err(),
            "accepted authority {host}"
        );
    }
    Ok(())
}

pub(super) fn response_id_and_outcome_boundaries_are_strict() -> Result<(), &'static str> {
    for id in ["null", "\"1\"", "1.0", "1.5", "-1", "4294967296", "true"] {
        let text = format!(r#"{{"jsonrpc":"2.0","id":{id},"result":{{}}}}"#);
        check!(Response::from_json(&text, 1).is_err(), "accepted {id}");
    }

    for outcome in [
        r#""result":null"#,
        r#""result":[]"#,
        r#""result":{},"error":null"#,
        r#""error":{"code":-1.5,"message":"denied"}"#,
        r#""error":{"code":-1,"message":null}"#,
    ] {
        let text = format!(r#"{{"jsonrpc":"2.0","id":1,{outcome}}}"#);
        check!(Response::from_json(&text, 1).is_err());
    }
    Ok(())
}

pub(super) fn siws_checks_expected_signer_claim_context_and_strict_signature()
-> Result<(), &'static str> {
    let signer = Keypair::from_seed(&[21; 32]);
    let wrong_signer = Keypair::from_seed(&[22; 32]);
    let payload = SignInPayload {
        domain: Some("app.example".into()),
        address: Some(signer.address().to_string()),
        uri: Some("https://app.example/login".into()),
        nonce: Some("fresh-nonce".into()),
        resources: Some(vec!["https://app.example/read".into()]),
        ..Default::default()
    };
    let message = payload.message().unwrap();
    let mut result = SignInResult {
        address: STANDARD.encode(signer.address().as_bytes()),
        signed_message: STANDARD.encode(&message),
        signature: STANDARD.encode(signer.sign(message.as_bytes()).as_bytes()),
        signature_type: Some("ed25519".into()),
    };
    result.verify(&payload, signer.address()).unwrap();
    check_eq!(
        result.verify(&payload, wrong_signer.address()),
        Err(Error::SignIn)
    );

    for field in ["domain", "address", "uri", "nonce", "resources"] {
        let mut changed = serde_json::to_value(&payload).unwrap();
        changed[field] = if field == "resources" {
            json!(["https://app.example/admin"])
        } else {
            json!("different")
        };

        let changed: SignInPayload = serde_json::from_value(changed).unwrap();
        check_eq!(
            result.verify(&changed, signer.address()),
            Err(Error::SignIn)
        );
    }

    result.signature = STANDARD.encode([0; 64]);
    check_eq!(
        result.verify(&payload, signer.address()),
        Err(Error::SignIn)
    );
    Ok(())
}
