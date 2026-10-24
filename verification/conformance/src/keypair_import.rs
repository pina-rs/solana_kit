//! Validated key import bodies shared with the native public API tests.

#[path = "../../../crates/solana_kit_core/tests/keypair_import/assertions.rs"]
mod assertions;

pub(crate) fn cases() -> alloc::vec::Vec<crate::CaseResult> {
    let bodies: [(&str, fn()); 6] = [
        (
            "keypair-import-rfc8032-vectors",
            assertions::imports_both_rfc8032_vectors_and_matches_seed_signatures,
        ),
        (
            "keypair-import-lengths",
            assertions::rejects_wrong_lengths_before_public_key_validation,
        ),
        (
            "keypair-import-public-half-positions",
            assertions::rejects_each_public_half_position_for_both_vectors,
        ),
        (
            "keypair-import-cross-key-public-halves",
            assertions::rejects_a_different_valid_public_key_in_both_directions,
        ),
        (
            "keypair-import-independent-ownership",
            assertions::import_preserves_input_and_owns_its_signer_independently,
        ),
        (
            "keypair-import-redacted-errors",
            assertions::errors_expose_only_length_or_a_public_key_mismatch,
        ),
    ];

    bodies
        .into_iter()
        .map(|(name, body)| {
            body();
            crate::case(name, Ok(()))
        })
        .collect()
}
