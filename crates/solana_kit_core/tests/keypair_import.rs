#![cfg(feature = "crypto")]

extern crate alloc;

#[path = "keypair_import/assertions.rs"]
mod assertions;

#[test]
fn imports_both_rfc8032_vectors_and_matches_seed_signatures() {
    assertions::imports_both_rfc8032_vectors_and_matches_seed_signatures();
}

#[test]
fn rejects_wrong_lengths_before_public_key_validation() {
    assertions::rejects_wrong_lengths_before_public_key_validation();
}

#[test]
fn rejects_each_public_half_position_for_both_vectors() {
    assertions::rejects_each_public_half_position_for_both_vectors();
}

#[test]
fn rejects_a_different_valid_public_key_in_both_directions() {
    assertions::rejects_a_different_valid_public_key_in_both_directions();
}

#[test]
fn import_preserves_input_and_owns_its_signer_independently() {
    assertions::import_preserves_input_and_owns_its_signer_independently();
}

#[test]
fn errors_expose_only_length_or_a_public_key_mismatch() {
    assertions::errors_expose_only_length_or_a_public_key_mismatch();
}
