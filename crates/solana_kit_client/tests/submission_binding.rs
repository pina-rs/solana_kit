//! Hostile acknowledgement identities cannot become low-level submission success.

use solana_kit_client::{
    Error,
    methods::{Method, submission::*},
};
use solana_kit_core::Signature;

#[path = "fixtures/submission_wire.rs"]
mod submission_wire;

const PRIMARY_BYTE: u8 = 7;

#[test]
fn every_wire_version_binds_both_encodings_to_the_primary_signature() {
    for bytes in [
        submission_wire::wire(255, PRIMARY_BYTE),
        submission_wire::wire(0, PRIMARY_BYTE),
        submission_wire::wire(1, PRIMARY_BYTE),
        submission_wire::full_v1(),
    ] {
        for encoding in [WireEncoding::Base58, WireEncoding::Base64] {
            let request = SendTransaction {
                wire: SignedWire::new(&bytes).unwrap(),
                encoding,
                config: SendConfig::default(),
            };
            assert_eq!(request.wire.bytes(), bytes);
            acknowledgements(&request);
            invalid_signatures(&request);
        }
    }
}

/// Success names the exact primary signature; a different canonical value fails.
fn acknowledgements(request: &SendTransaction<'_>) {
    let expected = Signature::from_bytes([PRIMARY_BYTE; 64]);
    let foreign = Signature::from_bytes([8; 64]);
    assert_eq!(
        request.decode(&format!("\"{expected}\"")).unwrap(),
        expected
    );
    let error = request.decode(&format!("\"{foreign}\"")).unwrap_err();
    let Error::SubmissionSignatureMismatch {
        expected: retained,
        actual: reported,
    } = error
    else {
        panic!("expected identity mismatch");
    };
    assert_eq!(retained, expected);

    assert_eq!(*reported, foreign);
}

/// Malformed provider text cannot bypass canonical signature parsing.
fn invalid_signatures(request: &SendTransaction<'_>) {
    for invalid in ["\"arbitrary\"", "null", "42", "[]"] {
        assert!(matches!(request.decode(invalid), Err(Error::Schema(_))));
    }
}

#[test]
fn malformed_and_unsigned_wire_cannot_construct_a_submission() {
    for bytes in [
        vec![],
        vec![0, 1],
        vec![0, 1, 2],
        vec![0; 4096],
        vec![0x81; 4096],
        submission_wire::wire(255, 0),
    ] {
        assert!(SignedWire::new(&bytes).is_err());
    }

    let mut bytes = submission_wire::wire(1, PRIMARY_BYTE);
    bytes.pop();
    assert!(SignedWire::new(&bytes).is_err());

    let mut multisig = vec![2];
    multisig.extend([PRIMARY_BYTE; 64]);
    multisig.extend([0; 64]);
    multisig.extend([2, 1, 0, 2]);
    multisig.extend([1; 32]);

    multisig.extend([2; 32]);
    multisig.extend([3; 32]);
    multisig.push(0);
    assert!(matches!(
        SignedWire::new(&multisig),
        Err(solana_kit_core::transaction::Error::MissingSignature)
    ));
}
