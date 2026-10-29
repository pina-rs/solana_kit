//! Every original publisher test body runs through five appended portable groups.

#[path = "../../../crates/solana_kit_dapp_publisher/tests/attestation/assertions.rs"]
mod assertions;
#[path = "../../../crates/solana_kit_dapp_publisher/tests/prepared/assertions.rs"]
mod prepared;
#[path = "../../../crates/solana_kit_dapp_publisher/tests/attestation/signatures.rs"]
mod signatures;

use crate::{CaseResult, case};
use prepared::{occupied_slots, policy_cases as policies};

/// Appends groups only after each named set of complete assertions has executed.
pub(crate) fn cases() -> [CaseResult; 5] {
    let groups: [(&str, fn()); 5] = [
        (
            "publisher-attestation-exact-json-and-verified-base64",
            attestation,
        ),
        (
            "publisher-prepared-three-envelope-bytes-and-signatures",
            envelopes,
        ),
        (
            "publisher-prepared-policy-invariants-and-rejections",
            policy,
        ),
        ("publisher-occupied-signature-preservation", occupied),
        ("publisher-shared-caller-budgets", budgets),
    ];

    groups.map(|(name, body)| {
        body();
        case(name, Ok(()))
    })
}

/// Four original bodies retain literal JSON, identifiers and detached-signature checks.
fn attestation() {
    assertions::ordered_json_preserves_full_width_slots_and_leading_request_zeroes();
    assertions::request_identifiers_reject_length_unicode_and_injection_before_allocation();
    signatures::detached_signature_is_verified_and_prefixed_to_the_original_json();
    signatures::wrong_identity_changed_message_and_reserved_zero_signature_are_rejected();
}

/// Three complete bodies retain all message, signature and unsigned-slot assertions.
fn envelopes() {
    prepared::legacy_v0_and_v1_preserve_independent_message_bytes_and_existing_mint_signature();
    prepared::incomplete_collections_are_not_reported_as_fully_signed_transactions();
    prepared::attached_signatures_do_not_change_the_unsigned_slot_encoding_contract();
}

/// Nine original bodies exercise successful policy composition and each rejection boundary.
fn policy() {
    prepared::common_expectations_reject_changed_blockhash_payer_and_nonrequired_local_signer();
    prepared::invalid_existing_and_new_signatures_cannot_cross_the_seal();
    prepared::lookup_descriptors_require_a_future_authenticated_snapshot_contract();

    policies::only_canonical_outer_programs_are_allowed_and_metadata_is_required();
    policies::mint_identity_and_collection_must_match_independent_caller_expectations();
    policies::missing_truncated_unknown_and_trailing_create_data_reject_before_signing();
    policies::a_missing_optional_collection_cannot_match_the_expected_collection();
    policies::collection_verification_requires_each_derived_and_explicit_account();
    policies::a_changed_existing_signature_is_rejected_even_when_collection_policy_still_matches();
}

/// Three original bodies cover all envelopes and duplicate occupied/empty signer identities.
fn occupied() {
    occupied_slots::conflicting_signature_bytes_are_rejected_for_an_occupied_caller_slot();
    occupied_slots::identical_occupied_signatures_preserve_the_entire_wire_transaction();
    occupied_slots::matching_occupied_signature_can_fill_a_duplicate_empty_identity_slot();
}

/// Three original bodies enforce independent wire, metadata and attestation output limits.
fn budgets() {
    prepared::each_transaction_allocation_is_admitted_before_parsing_or_nested_decoding();
    prepared::metadata_allocations_cannot_reuse_the_transaction_reservation();
    assertions::the_complete_attestation_output_is_admitted_before_json_allocation();
}
