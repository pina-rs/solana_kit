//! Accepted public account and error contracts shared by all hosts.
pub mod accounts;
pub mod errors;

pub type Contract = (&'static str, fn());

pub fn cases() -> Vec<Contract> {
    let mut cases = Vec::new();
    cases.extend(group_0());
    cases.extend(group_1());
    cases.extend(group_2());
    cases.extend(group_3());
    cases.extend(group_4());
    cases.extend(group_5());
    cases.extend(group_6());
    cases.extend(group_7());
    cases.extend(group_8());
    cases.extend(group_9());
    cases
}

fn group_0() -> Vec<Contract> {
    use crate::accounts::original::*;
    vec![
        (
            "core-contract-0-0",
            complete_options_and_full_width_metadata_survive_fetch,
        ),
        (
            "core-contract-0-1",
            multiple_fetch_preserves_duplicates_and_missing_neighbors,
        ),
        (
            "core-contract-0-2",
            missing_and_inconsistent_responses_are_distinct,
        ),
        (
            "core-contract-0-3",
            standalone_parsers_preserve_allocation_and_reject_malformed_encoding,
        ),
        (
            "core-contract-0-4",
            legacy_base58_plain_base64_and_absent_rent_preserve_facts,
        ),
        (
            "core-contract-0-5",
            opaque_parsed_json_and_binary_fallback_keep_their_shapes,
        ),
        (
            "core-contract-0-6",
            borrowed_decode_retains_snapshot_cause_and_typed_missing_state,
        ),
        (
            "core-contract-0-7",
            equality_hash_debug_and_existence_have_native_contracts,
        ),
        (
            "core-contract-0-8",
            parsed_multiple_and_decoded_assertions_preserve_positions,
        ),
    ]
}

fn group_1() -> Vec<Contract> {
    use crate::accounts::probe::*;
    vec![
        (
            "core-contract-1-0",
            all_four_fetches_have_literal_options_context_and_local_pending,
        ),
        (
            "core-contract-1-1",
            cardinality_rejects_both_directions_for_both_multiple_helpers,
        ),
        (
            "core-contract-1-2",
            protocol_and_transport_errors_remain_errors,
        ),
        (
            "core-contract-1-3",
            standalone_and_node_boundaries_differ_only_in_documented_fields,
        ),
        (
            "core-contract-1-4",
            parsed_unknown_json_is_exact_and_metadata_keeps_its_identity,
        ),
        (
            "core-contract-1-5",
            custom_decoder_preserves_originals_exact_causes_and_limits,
        ),
        (
            "core-contract-1-6",
            native_value_construction_hash_equality_debug_and_missing_assertions,
        ),
        (
            "core-contract-1-7",
            assertions_retain_order_duplicates_and_compressed_state,
        ),
    ]
}

fn group_2() -> Vec<Contract> {
    use crate::accounts::gaps::*;
    vec![
        (
            "core-contract-2-0",
            parsed_program_and_nested_space_are_optional_in_reference,
        ),
        (
            "core-contract-2-1",
            standalone_numeric_metadata_strings_are_accepted_in_reference,
        ),
    ]
}

fn group_3() -> Vec<Contract> {
    use crate::accounts::contracts::*;
    vec![
        (
            "core-contract-3-0",
            metadata_only_parser_returns_actual_external_header_fields,
        ),
        (
            "core-contract-3-1",
            standalone_numeric_strings_follow_source_syntax_with_checked_u64_bounds,
        ),
        (
            "core-contract-3-2",
            optional_parsed_metadata_keeps_exact_payload_and_available_facts,
        ),
        (
            "core-contract-3-3",
            each_single_decoded_assertion_preserves_its_exact_address,
        ),
    ]
}

fn group_4() -> Vec<Contract> {
    use crate::accounts::idioms::*;
    vec![(
        "core-contract-4-0",
        arbitrary_typed_data_and_optional_metadata_construct_compare_hash_and_format,
    )]
}

fn group_5() -> Vec<Contract> {
    use crate::accounts::independent::*;
    vec![
        (
            "core-contract-5-0",
            header_requires_each_fact_and_preserves_rent_presence,
        ),
        (
            "core-contract-5-1",
            fresh_dart_grammar_and_complete_unicode_oracle_drive_all_standalone_numbers,
        ),
        (
            "core-contract-5-2",
            raw_json_numeric_types_and_malformed_documents_remain_errors,
        ),
        (
            "core-contract-5-3",
            parsed_optional_cross_product_keeps_exact_raw_and_independent_allocations,
        ),
        (
            "core-contract-5-4",
            strict_node_never_inherits_metadata_string_coercion_or_optional_facts,
        ),
        (
            "core-contract-5-5",
            slice_lengths_and_compressed_states_stay_explicit,
        ),
    ]
}

fn group_6() -> Vec<Contract> {
    use crate::accounts::assertion_gap::*;
    vec![(
        "core-contract-6-0",
        decoded_assertion_preserves_failing_account_address,
    )]
}

fn group_7() -> Vec<Contract> {
    use crate::errors::*;
    vec![
        ("core-contract-7-0", nested_key_boundaries),
        (
            "core-contract-7-1",
            exact_context_boundaries_and_atomic_rejection,
        ),
        ("core-contract-7-2", exact_aggregate_storage_boundary),
        (
            "core-contract-7-3",
            nested_sensitive_families_are_rejected_and_public_tokens_survive,
        ),
        (
            "core-contract-7-4",
            malformed_codec_failures_are_distinct_public_values,
        ),
        (
            "core-contract-7-5",
            rpc_i64_extremes_and_local_fnonce_projection,
        ),
        (
            "core-contract-7-6",
            rejected_nested_object_allocation_is_bounded_by_node_budget,
        ),
    ]
}

fn group_8() -> Vec<Contract> {
    use crate::errors::bounds::*;
    vec![
        (
            "core-contract-8-0",
            oversized_objects_in_lists_have_a_fixed_allocation_bound,
        ),
        (
            "core-contract-8-1",
            near_exhausted_prior_facts_preserve_atomicity_and_replacement,
        ),
        (
            "core-contract-8-2",
            small_maps_keep_exact_keys_values_and_early_privacy_errors,
        ),
        (
            "core-contract-8-3",
            nested_object_node_budget_is_exact_at_each_valid_depth,
        ),
        (
            "core-contract-8-4",
            later_objects_use_the_remaining_list_node_budget,
        ),
    ]
}

fn group_9() -> Vec<Contract> {
    use crate::errors::more::*;
    vec![
        ("core-contract-9-0", exact_message_render_boundary),
        ("core-contract-9-1", utf8_uri_escaping_is_exact),
        (
            "core-contract-9-2",
            future_payloads_and_native_debug_redaction,
        ),
        (
            "core-contract-9-3",
            preflight_defaults_preserve_source_identity,
        ),
        (
            "core-contract-9-4",
            checked_enum_arithmetic_does_not_invoke_callback,
        ),
    ]
}
