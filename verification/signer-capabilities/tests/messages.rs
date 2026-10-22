use signer_capability_consumer::{message_batches, message_snapshots};
use solana_kit_core::crypto::Keypair;
use solana_kit_signers::{
    SignerCapability,
    message::{
        SignableMessage, SingularMessageSigner,
        native::{Composition, Registration},
    },
};

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[tokio::test]
async fn pinned_whole_slice_keypair_goldens_and_explicit_singular_adapter() {
    let key = Keypair::from_seed(&[7; 32]);
    let handle = SignerCapability::new(&key);
    let pipeline = Composition::new(vec![Registration::partial(&handle)]).unwrap();
    let inputs = [
        SignableMessage::new([]),
        SignableMessage::from_text("é🦀\0"),
        SignableMessage::new([0, 255, 128]),
        SignableMessage::new(b"exact bare bytes"),
    ];
    let results = pipeline.sign(&inputs).await.unwrap();
    let oracle = include_str!("../fixtures/dart-messages.tsv");
    let mut lines = oracle.lines();
    assert_eq!(lines.next(), Some(key.address().to_string().as_str()));

    for result in &results {
        let expected = format!(
            "{}\t{}",
            hex(result.content()),
            hex(result.signatures()[&key.address()].as_bytes())
        );
        assert_eq!(lines.next(), Some(expected.as_str()));
    }

    assert_eq!(lines.next(), Some("65786163742062617265206279746573\t1"));
    assert_eq!(lines.next(), None);
    let adapter = SingularMessageSigner(Keypair::from_seed(&[7; 32]));
    let handle = SignerCapability::new(&adapter);
    let adapter = Composition::new(vec![Registration::partial(&handle)]).unwrap();
    assert_eq!(adapter.sign(&inputs).await.unwrap(), results);
    assert!(message_snapshots());
    assert!(message_batches().await);
}
