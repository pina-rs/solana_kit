//! Ordinary downstream exercise with no repository rustflags or host providers.

use solana_kit_wallet::mwa::{
    AgreementKey, AssociationKey, AssociationPort, Error, Session, SessionKey, decrypt_message,
    encrypt_message, verify_hello_request,
};

/// Executes deterministic cryptography and negative protocol boundaries.
pub fn exercise() -> u32 {
    let mut scalar = [0; 32];
    scalar[31] = 1;
    let association = AssociationKey::from_scalar(&scalar).unwrap();
    scalar[31] = 2;
    let agreement = AgreementKey::from_scalar(&scalar).unwrap();
    let hello = agreement.hello_request(&association);
    assert_eq!(
        verify_hello_request(&hello, &association.public_key()).unwrap(),
        agreement.public_key()
    );
    assert!(AssociationKey::from_scalar(&[0; 32]).is_err());

    let key = SessionKey::from_bytes(&[7; 16]);
    let frame = encrypt_message("consumer", 1, &[1; 12], &key).unwrap();
    assert_eq!(decrypt_message(&frame, &key).unwrap().1, "consumer");
    let mut session = Session::new(key, 1, 1);
    assert_eq!(session.decrypt(&frame).unwrap(), "consumer");
    assert_eq!(session.decrypt(&frame), Err(Error::Sequence));
    assert!(AssociationPort::new(0).is_err());
    let jws = association.jws(&Default::default()).unwrap();

    assert!(
        association
            .public_key()
            .verify_jws(&jws)
            .unwrap()
            .is_empty()
    );
    session.close();
    7
}

/// Keeps the same consumer behavior reachable in the compiled plain Wasm module.
#[unsafe(no_mangle)]
pub extern "C" fn mwa_consumer_checks() -> u32 {
    exercise()
}
