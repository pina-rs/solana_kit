#![cfg(feature = "crypto")]

mod support;

use solana_kit_core::{
    crypto::{Keypair, verify},
    transaction::Error as TransactionError,
};
use solana_kit_signers::{
    Error, MessageSigner, SignatureOutput, SignerCapability, TransactionSigners,
};

#[tokio::test]
async fn deterministic_keypair_exact_bytes_and_native_spawn() {
    let keypair = Keypair::from_seed(&[7; 32]);
    let output = keypair.sign_message(b"exact bare bytes").await.unwrap();
    let golden: Vec<_> =
        include_str!("../../../verification/signer-capabilities/fixtures/dart-golden.txt")
            .lines()
            .collect();
    assert_eq!(keypair.address().to_string(), golden[0]);
    let signature_hex: String = output
        .signature()
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    assert_eq!(signature_hex, golden[1]);
    assert_eq!(output.signature(), keypair.sign(b"exact bare bytes"));

    verify(&keypair.address(), output.bytes(), &output.signature()).unwrap();
    assert_eq!(
        output.validate(keypair.address(), b"tamper"),
        Err(Error::ChangedSnapshot)
    );

    let handle = tokio::spawn(async move {
        let transaction = support::transaction(keypair.address(), 1);
        tokio::task::yield_now().await;
        TransactionSigners::new(&[&SignerCapability::new(&keypair)])
            .unwrap()
            .sign(&transaction)
            .await
            .unwrap()
    });
    handle.await.unwrap().verify().unwrap();
}

#[tokio::test]
async fn signature_and_identity_failure_paths() {
    let keypair = Keypair::from_seed(&[7; 32]);
    let other = Keypair::from_seed(&[8; 32]);
    let transaction = support::transaction(keypair.address(), 1);
    let changed = support::transaction(keypair.address(), 2);
    let output = SignatureOutput::new(
        keypair.address(),
        transaction.message().bytes(),
        keypair.sign(transaction.message().bytes()),
    );
    assert_eq!(
        output.apply(other.address(), &transaction),
        Err(Error::ForeignSigner)
    );
    assert_eq!(
        output.apply(keypair.address(), &changed),
        Err(Error::ChangedSnapshot)
    );
    let tampered = SignatureOutput::new(
        keypair.address(),
        transaction.message().bytes(),
        other.sign(transaction.message().bytes()),
    );

    assert_eq!(
        tampered.apply(keypair.address(), &transaction),
        Err(Error::Transaction(TransactionError::InvalidSignature))
    );
    let capability = SignerCapability::new(&keypair);
    assert!(TransactionSigners::new(&[&capability, &capability]).is_ok());
    let duplicate = Keypair::from_seed(&[7; 32]);
    assert!(matches!(
        TransactionSigners::new(&[&capability, &SignerCapability::new(&duplicate)]),
        Err(Error::ConflictingSigner)
    ));
    let empty: [&SignerCapability<'_, Keypair>; 0] = [];
    assert_eq!(
        TransactionSigners::new(&empty)
            .unwrap()
            .sign(&transaction)
            .await,
        Err(Error::Transaction(TransactionError::MissingSignature))
    );
}
