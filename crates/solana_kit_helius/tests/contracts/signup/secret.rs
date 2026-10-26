//! Encoded signing keys authenticate locally and bind the actual developer JWT once.

use super::*;
use base64::{Engine, engine::general_purpose::STANDARD};
use solana_kit_core::crypto::{Keypair, verify};

const TIMESTAMP: u64 = 1234;

/// Standard, URL-safe, unpadded and escaped inputs produce independently verified signatures.
pub fn signed_authentication() {
    let signer = Keypair::from_seed(&[1; 32]);
    let mut bytes = [1; 64];
    bytes[32..].copy_from_slice(signer.address().as_bytes());
    let encoded = STANDARD.encode(bytes);

    for secret in [
        encoded.clone(),
        encoded.trim_end_matches('=').into(),
        encoded.replace('+', "-").replace('/', "_"),
        encoded.replace('=', "%3D"),
    ] {
        valid_secret(secret, &signer);
    }

    bytes[63] ^= 1;

    for encoded in [
        "invalid%ZZ".into(),
        STANDARD.encode([1; 63]),
        STANDARD.encode([1; 65]),
        STANDARD.encode(bytes),
    ] {
        invalid_secret(encoded);
    }
}

fn valid_secret(secret: String, signer: &Keypair) {
    let (bootstrap, trace) = client(vec![reply(
        r#"{"token":"minted-jwt","refId":"reference","newUser":true}"#,
    )]);
    let summary = project("business_v4", "start", "end");
    let (session, session_trace) = client(vec![reply(&summary)]);
    let options = SignupRequest::new(
        SignupAuthentication::SecretKey(SensitiveString::new(secret)),
        Plan::Developer,
    );
    let result = futures::executor::block_on(auth_signup(&bootstrap, options, TIMESTAMP, |jwt| {
        assert_eq!(jwt.expose(), "minted-jwt");
        Ok(session)
    }))
    .expect("authenticated signup");

    let SignupResult::UpgradeRequired { identity, .. } = result else {
        panic!("authenticated mismatch");
    };

    assert_eq!(identity.wallet_address, signer.address());
    assert_eq!(session_trace.borrow().len(), 1);
    verify_request(&trace.borrow(), signer);
}

fn verify_request(calls: &[ServiceRequest], signer: &Keypair) {
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].path_segments, ["wallet-signup"]);
    let body = body(&calls[0]);
    let message = body["message"].as_str().expect("signed text");
    assert_eq!(message, auth_message(TIMESTAMP).expect("default message"));
    assert_eq!(body["userID"], signer.address().to_string());
    let signature = body["signature"]
        .as_str()
        .expect("signature")
        .parse()
        .expect("Kit signature");
    verify(&signer.address(), message.as_bytes(), &signature).expect("real Ed25519 signature");
}

fn invalid_secret(encoded: String) {
    let (bootstrap, trace) = client(Vec::new());
    let options = SignupRequest::new(
        SignupAuthentication::SecretKey(SensitiveString::new(encoded)),
        Plan::Developer,
    );
    let result = futures::executor::block_on(auth_signup(
        &bootstrap,
        options,
        1,
        |_| -> Result<ServiceClient<Replay>, Error> { panic!("invalid key cannot bind session") },
    ));
    assert!(matches!(result, Err(SignupError::InvalidSecretKey)));
    assert!(trace.borrow().is_empty());
}
