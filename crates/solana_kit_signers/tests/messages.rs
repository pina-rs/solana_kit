mod message_support;
const PRIMARY_SEED: u8 = 7;
const FOREIGN_SEED: u8 = 9;

use message_support::{Backend, Behavior, address, signature};
use solana_kit_core::{Address, Signature};
use solana_kit_signers::{
    Signer, SignerCapability,
    message::{
        MessageError, MessageModifyingSigner, SignableMessage,
        local::{Composition, Registration},
    },
};
use std::{
    cell::Cell,
    future::{Ready, ready},
};

#[test]
fn immutable_bare_snapshots_preserve_utf8_empty_arbitrary_and_existing_signatures() {
    let mut bytes = vec![0, 255, 128];
    let snapshot = SignableMessage::new(&bytes);
    bytes[0] = 42;
    assert_eq!(snapshot.content(), [0, 255, 128]);
    assert_eq!(
        SignableMessage::from_text("é🦀\0").content(),
        [195, 169, 240, 159, 166, 128, 0]
    );
    assert!(SignableMessage::new([]).content().is_empty());
    assert_eq!(
        SignableMessage::new(vec![1; 10_000]).content().len(),
        10_000
    );
    let signed = SignableMessage::with_signatures(
        b"x",
        [(address(PRIMARY_SEED), signature(PRIMARY_SEED, b"x"))],
    )
    .unwrap();
    assert_eq!(
        signed.signatures()[&address(PRIMARY_SEED)],
        signature(PRIMARY_SEED, b"x")
    );
    assert_eq!(
        SignableMessage::with_signatures(
            b"x",
            [(address(PRIMARY_SEED), signature(PRIMARY_SEED, b"x")); 2]
        ),
        Err(MessageError::DuplicateSignature)
    );
}

#[tokio::test]
async fn actual_bulk_256_success_and_257_preflight_rejection() {
    let backend = Backend::new(PRIMARY_SEED, Behavior::Good);
    let handle = SignerCapability::new(&backend);
    let pipeline = Composition::new(vec![Registration::partial(&handle)]).unwrap();
    let inputs: Vec<_> = (0..256)
        .map(|n| SignableMessage::new([n as u8, 0, 255]))
        .collect();
    let results = pipeline.sign(&inputs).await.unwrap();
    assert_eq!((backend.calls.get(), backend.items.get()), (1, 256));
    assert_eq!(results.len(), 256);

    for (input, result) in inputs.iter().zip(results) {
        assert_eq!(result.content(), input.content());
        assert_eq!(
            result.signatures()[&address(PRIMARY_SEED)],
            signature(PRIMARY_SEED, input.content())
        );
    }

    assert_eq!(
        pipeline.sign(&vec![SignableMessage::new([]); 257]).await,
        Err(MessageError::TooManyMessages)
    );
    assert_eq!((backend.calls.get(), backend.items.get()), (1, 256));
    assert!(pipeline.sign(&[]).await.unwrap().is_empty());
    assert_eq!((backend.calls.get(), backend.items.get()), (2, 256));
}

#[tokio::test]
async fn malicious_bulk_outputs_reject_at_the_message_boundary() {
    let cases = [
        (Behavior::Short, MessageError::Cardinality),
        (Behavior::Long, MessageError::Cardinality),
        (Behavior::Foreign, MessageError::ForeignSigner),
        (Behavior::Changed, MessageError::ChangedSnapshot),
        (Behavior::Missing, MessageError::MissingSignature),
        (Behavior::Zero, MessageError::InvalidSignature),
        (Behavior::Reject, MessageError::Rejected),
    ];

    for (behavior, expected) in cases {
        let backend = Backend::new(PRIMARY_SEED, behavior);
        let handle = SignerCapability::new(&backend);
        let pipeline = Composition::new(vec![Registration::partial(&handle)]).unwrap();
        assert_eq!(
            pipeline
                .sign(&[SignableMessage::new(b"one"), SignableMessage::new(b"two")])
                .await,
            Err(expected)
        );
        assert_eq!((backend.calls.get(), backend.items.get()), (1, 2));
    }

    let backend = Backend::new(PRIMARY_SEED, Behavior::Invalid);
    let handle = SignerCapability::new(&backend);
    let pipeline = Composition::new(vec![Registration::partial(&handle)]).unwrap();
    let result = pipeline.sign(&[SignableMessage::new(b"one")]).await;
    assert_eq!(result.is_err(), cfg!(feature = "crypto"));
}

struct Modifier {
    seed: u8,
    calls: Cell<usize>,
    replacement: Vec<SignableMessage>,
}

impl Signer for Modifier {
    fn address(&self) -> Address {
        address(self.seed)
    }
}

impl MessageModifyingSigner for Modifier {
    type Modify<'a> = Ready<Result<Vec<SignableMessage>, MessageError>>;

    fn modify_messages<'a>(&'a self, messages: &'a [SignableMessage]) -> Self::Modify<'a> {
        self.calls.set(self.calls.get() + 1);
        assert_eq!(messages.len(), 1);
        ready(Ok(self.replacement.clone()))
    }
}

#[tokio::test]
async fn modifications_precede_partials_and_failures_stop_later_backends() {
    let bytes = b"replacement";
    let replacement =
        SignableMessage::with_signatures(bytes, [(address(8), signature(8, bytes))]).unwrap();
    let modifier = Modifier {
        seed: 8,
        calls: Cell::new(0),
        replacement: vec![replacement],
    };
    let partial = Backend::new(PRIMARY_SEED, Behavior::Good);
    let modify_handle = SignerCapability::new(&modifier);
    let partial_handle = SignerCapability::new(&partial);
    let pipeline = Composition::new(vec![
        Registration::partial(&partial_handle),
        Registration::modifying(&modify_handle),
    ])
    .unwrap();
    let original = SignableMessage::new(b"original");
    let result = pipeline
        .sign(std::slice::from_ref(&original))
        .await
        .unwrap();
    assert_eq!(result[0].content(), bytes);
    assert_eq!(result[0].signatures().len(), 2);
    assert_eq!(
        result[0].signatures()[&address(PRIMARY_SEED)],
        signature(PRIMARY_SEED, bytes)
    );
    assert!(original.signatures().is_empty());

    let old =
        SignableMessage::with_signatures(b"old", [(address(8), signature(8, b"old"))]).unwrap();
    let stale =
        SignableMessage::with_signatures(b"new", [(address(8), signature(8, b"old"))]).unwrap();
    let bad = Modifier {
        seed: 8,
        calls: Cell::new(0),
        replacement: vec![stale],
    };
    let bad_handle = SignerCapability::new(&bad);
    assert!(matches!(
        Composition::new(vec![
            Registration::modifying(&bad_handle),
            Registration::modifying(&modify_handle),
            Registration::partial(&partial_handle)
        ]),
        Err(MessageError::ConflictingSigner)
    ));
    let bad_pipeline = Composition::new(vec![
        Registration::modifying(&bad_handle),
        Registration::partial(&partial_handle),
    ])
    .unwrap();
    assert_eq!(
        bad_pipeline.sign(&[old]).await,
        Err(MessageError::ChangedSnapshot)
    );
    assert_eq!(partial.calls.get(), 1);
}

struct Zst;
impl Signer for Zst {
    fn address(&self) -> Address {
        address(PRIMARY_SEED)
    }
}
impl solana_kit_signers::message::MessagePartialSigner for Zst {
    type Sign<'a> =
        Ready<Result<Vec<solana_kit_signers::message::MessageSignatureOutput>, MessageError>>;
    fn sign_messages<'a>(&'a self, messages: &'a [SignableMessage]) -> Self::Sign<'a> {
        ready(Ok(messages
            .iter()
            .map(|m| message_support::output(PRIMARY_SEED, m))
            .collect()))
    }
}

#[tokio::test]
async fn logical_identity_and_registration_bounds_include_coincident_zsts() {
    let first = Box::new(Zst);
    let second = Box::new(Zst);
    assert_eq!(&*first as *const Zst, &*second as *const Zst);
    let first_handle = SignerCapability::new(&*first);
    let second_handle = SignerCapability::new(&*second);
    assert!(matches!(
        Composition::new(vec![
            Registration::partial(&first_handle),
            Registration::partial(&second_handle)
        ]),
        Err(MessageError::ConflictingSigner)
    ));
    let pipeline = Composition::new(
        (0..256)
            .map(|_| Registration::partial(&first_handle))
            .collect(),
    )
    .unwrap();
    assert_eq!(
        pipeline.sign(&[SignableMessage::new(b"x")]).await.unwrap()[0]
            .signatures()
            .len(),
        1
    );
    assert!(matches!(
        Composition::new(
            (0..257)
                .map(|_| Registration::partial(&first_handle))
                .collect()
        ),
        Err(MessageError::TooManySigners)
    ));
    let invalid = SignableMessage::with_signatures(
        b"x",
        [(address(PRIMARY_SEED), Signature::from_bytes([0; 64]))],
    )
    .unwrap();
    assert_eq!(
        pipeline.sign(&[invalid]).await,
        Err(MessageError::InvalidSignature)
    );
}

#[tokio::test]
async fn exactly_256_distinct_registrations_invoke_each_backend_once() {
    let backends: Vec<_> = (0..256)
        .map(|seed| Backend::new(seed as u8, Behavior::Good))
        .collect();
    let handles: Vec<_> = backends.iter().map(SignerCapability::new).collect();
    let pipeline = Composition::new(handles.iter().map(Registration::partial).collect()).unwrap();
    let result = pipeline
        .sign(&[SignableMessage::new(b"all owners")])
        .await
        .unwrap();
    assert_eq!(result[0].signatures().len(), 256);
    assert!(
        backends
            .iter()
            .all(|b| b.calls.get() == 1 && b.items.get() == 1)
    );
}

#[tokio::test]
async fn modifier_cardinality_foreign_signature_and_removal_fail_before_partials() {
    let partial = Backend::new(PRIMARY_SEED, Behavior::Good);
    let partial_handle = SignerCapability::new(&partial);
    let old = SignableMessage::with_signatures(
        b"old",
        [(address(FOREIGN_SEED), signature(FOREIGN_SEED, b"old"))],
    )
    .unwrap();
    let cases = [
        (Vec::new(), MessageError::Cardinality),
        (
            vec![SignableMessage::new(b"new")],
            MessageError::MissingSignature,
        ),
        (
            vec![
                SignableMessage::with_signatures(
                    b"new",
                    [(address(8), signature(FOREIGN_SEED, b"old"))],
                )
                .unwrap(),
            ],
            MessageError::ChangedSnapshot,
        ),
        (
            vec![
                SignableMessage::with_signatures(
                    b"new",
                    [(address(FOREIGN_SEED), signature(FOREIGN_SEED, b"new"))],
                )
                .unwrap(),
            ],
            MessageError::ForeignSigner,
        ),
        (
            vec![SignableMessage::new(b"old")],
            MessageError::ForeignSigner,
        ),
    ];

    for (replacement, expected) in cases {
        let backend = Modifier {
            seed: 8,
            calls: Cell::new(0),
            replacement,
        };
        let handle = SignerCapability::new(&backend);
        let pipeline = Composition::new(vec![
            Registration::modifying(&handle),
            Registration::partial(&partial_handle),
        ])
        .unwrap();
        assert_eq!(
            pipeline.sign(std::slice::from_ref(&old)).await,
            Err(expected)
        );
        assert_eq!(backend.calls.get(), 1);
        assert_eq!(partial.calls.get(), 0);
    }
}

struct Rekeyable(Cell<u8>);

impl Signer for Rekeyable {
    fn address(&self) -> Address {
        address(self.0.get())
    }
}

impl solana_kit_signers::message::MessagePartialSigner for Rekeyable {
    type Sign<'a> =
        Ready<Result<Vec<solana_kit_signers::message::MessageSignatureOutput>, MessageError>>;

    fn sign_messages<'a>(&'a self, messages: &'a [SignableMessage]) -> Self::Sign<'a> {
        ready(Ok(messages
            .iter()
            .map(|m| message_support::output(self.0.get(), m))
            .collect()))
    }
}

#[test]
fn one_logical_handle_cannot_claim_two_addresses() {
    let backend = Rekeyable(Cell::new(PRIMARY_SEED));
    let handle = SignerCapability::new(&backend);
    let first = Registration::partial(&handle);
    backend.0.set(FOREIGN_SEED);
    let second = Registration::partial(&handle);
    assert!(matches!(
        Composition::new(vec![first, second]),
        Err(MessageError::ConflictingSigner)
    ));
}

struct Singular(Cell<usize>);

impl Signer for Singular {
    fn address(&self) -> Address {
        address(PRIMARY_SEED)
    }
}

impl solana_kit_signers::MessageSigner for Singular {
    type Sign<'a> = Ready<Result<solana_kit_signers::SignatureOutput, solana_kit_signers::Error>>;

    fn sign_message<'a>(&'a self, bytes: &'a [u8]) -> Self::Sign<'a> {
        self.0.set(self.0.get() + 1);

        if bytes == b"reject" {
            return ready(Err(solana_kit_signers::Error::Rejected));
        }

        ready(Ok(solana_kit_signers::SignatureOutput::new(
            self.address(),
            bytes,
            signature(PRIMARY_SEED, bytes),
        )))
    }
}

#[tokio::test]
async fn explicit_singular_adapter_stops_at_first_item_failure() {
    use solana_kit_signers::message::SingularMessageSigner;
    let backend = SingularMessageSigner(Singular(Cell::new(0)));
    let handle = SignerCapability::new(&backend);
    let pipeline = Composition::new(vec![Registration::partial(&handle)]).unwrap();
    let inputs = [
        SignableMessage::new(b"good"),
        SignableMessage::new(b"reject"),
        SignableMessage::new(b"unseen"),
    ];
    assert_eq!(
        pipeline.sign(&inputs).await,
        Err(MessageError::Signer(solana_kit_signers::Error::Rejected))
    );
    assert_eq!(backend.0.0.get(), 2);
}
