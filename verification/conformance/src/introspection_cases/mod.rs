// Complete public introspection contracts adapted into three bounded groups.

macro_rules! check {
    ($condition:expr $(,)?) => {
        crate::require($condition, stringify!($condition))?
    };
}

macro_rules! check_eq {
    ($left:expr, $right:expr $(,)?) => {
        crate::require($left == $right, stringify!($left == $right))?
    };
}

macro_rules! check_ne {
    ($left:expr, $right:expr $(,)?) => {
        crate::require($left != $right, stringify!($left != $right))?
    };
}

mod adversarial;
mod fixtures;
mod oracle;
mod parsed;
mod program_boundary;
mod raw;
mod security;
mod wire;

type Check = Result<(), &'static str>;

pub(crate) fn cases() -> [crate::CaseResult; 3] {
    [
        crate::case("introspection-wire-metadata", wire_contracts()),
        crate::case("introspection-accounts-traces", account_contracts()),
        crate::case("introspection-boundary-validation", boundaries()),
    ]
}

fn wire_contracts() -> Check {
    wire::exact_legacy_v0_v1_envelopes_roundtrip_in_both_rpc_binary_encodings()?;
    wire::direct_message_helpers_and_wire_lookup_resolution_share_the_same_account_table()?;
    wire::metadata_independent_wire_decode_preserves_v0_while_resolution_requires_loaded_addresses(
    )?;
    oracle::actual_pinned_dart_bytes_equal_all_three_independent_literal_envelopes()?;
    adversarial::wire_only_preserves_signed_v0_under_missing_null_empty_and_contradictory_metadata()
}

fn account_contracts() -> Check {
    raw::legacy_roles_duplicate_positions_bytes_and_unresolved_lifetime_are_exact()?;
    raw::walk_interleaves_unsorted_groups_and_retains_depth_presence()?;
    raw::lookup_partitions_preserve_loaded_privileges_and_order()?;
    raw::unavailable_metadata_never_establishes_execution_or_recorded_inner_calls()?;
    raw::failed_execution_remains_failed_while_attempted_instructions_are_inspectable()?;
    raw::raw_v1_json_retains_the_typed_message_without_synthesizing_wire()?;
    oracle::value_identity_includes_trace_depth_presence_and_ordered_instruction_content()?;
    parsed::parsed_payload_is_borrowed_exactly_and_partial_data_retains_loaded_roles()?;

    parsed::parsed_inner_calls_walk_after_their_outer_call()?;
    adversarial::nested_trace_depth_and_opaque_payload_borrow_stay_exact()
}

fn boundaries() -> Check {
    wire::malformed_truncated_trailing_header_index_and_signature_counts_fail_closed()?;
    wire::binary_reported_version_must_equal_the_actual_signed_envelope()?;
    security::rejects_header_signature_and_instruction_reference_mismatches()?;
    security::rejects_missing_null_partial_extra_and_swapped_loaded_partitions()?;
    security::rejects_duplicate_empty_and_orphan_inner_groups()?;
    parsed::parsed_addresses_signers_sources_and_opaque_payload_shapes_are_checked()?;
    adversarial::wire_only_rejects_all_reported_version_mismatches_and_trailing_bytes()?;
    adversarial::duplicate_and_empty_lookup_descriptors_and_inner_index_overflows_fail()?;

    adversarial::parsed_metadata_must_match_exact_loaded_address_and_source_ownership()?;
    adversarial::conflicting_duplicate_address_roles_cannot_be_resolved_by_address()?;
    adversarial::v1_json_duplicate_accounts_and_instruction_bound_fail_closed()?;
    program_boundary::raw_json_must_reject_payer_as_invoked_program_like_core_wire()?;
    program_boundary::raw_json_must_reject_loaded_invoked_program_like_core_wire()?;
    program_boundary::payer_and_static_boundary_follow_core_rules_for_every_version()?;
    program_boundary::inner_loaded_programs_preserve_account_roles_and_trace()
}
