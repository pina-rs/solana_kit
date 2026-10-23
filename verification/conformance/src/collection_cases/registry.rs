use alloc::vec::Vec;

const CONTRACTS: usize = 54;

pub(crate) fn cases() -> Vec<crate::CaseResult> {
    let mut cases = Vec::with_capacity(CONTRACTS);
    cases.extend(absence());
    cases.extend(boolean());
    cases.extend(budgets());
    cases.extend(counts());
    cases.extend(markers());

    cases.extend(minimum());
    cases.extend(collection_layouts());
    cases.extend(native_option_values());
    cases.extend(codec_framing());
    cases.extend(codec_framing_accounting());
    cases.extend(composite_codecs());
    cases.extend(composite_accounting());
    cases
}

fn execute(body: fn()) -> Result<(), &'static str> {
    body();
    Ok(())
}

fn absence() -> [crate::CaseResult; 2] {
    use super::absence::*;

    [
        crate::case(
            "collection-absence-0",
            execute(dirty_writes_preserve_surroundings_for_every_presence_policy),
        ),
        crate::case(
            "collection-absence-1",
            execute(zero_present_frames_fail_exactly_and_absent_constants_are_validated),
        ),
    ]
}

fn boolean() -> [crate::CaseResult; 2] {
    use super::boolean::*;

    [
        crate::case(
            "collection-boolean-0",
            execute(custom_boolean_storage_rejects_every_other_word),
        ),
        crate::case(
            "collection-boolean-1",
            execute(untagged_absent_patterns_reserve_present_wire_values),
        ),
    ]
}

fn budgets() -> [crate::CaseResult; 3] {
    use super::budgets::*;

    [
        crate::case(
            "collection-budgets-0",
            execute(failed_item_consumes_a_claim_without_speculative_rollback),
        ),
        crate::case(
            "collection-budgets-1",
            execute(zero_width_counted_items_share_the_remaining_budget),
        ),
        crate::case(
            "collection-budgets-2",
            execute(failed_frame_advances_outer_offset_and_retains_inner_claims),
        ),
    ]
}

fn counts() -> [crate::CaseResult; 1] {
    use super::counts::*;

    [crate::case(
        "collection-counts-0",
        execute(wide_count_mapping_checks_before_zero_width_allocation),
    )]
}

fn markers() -> [crate::CaseResult; 2] {
    use super::markers::*;

    [
        crate::case(
            "collection-markers-0",
            execute(multibyte_markers_match_only_complete_item_boundaries),
        ),
        crate::case(
            "collection-markers-1",
            execute(marker_valued_item_is_the_documented_reserved_representation),
        ),
    ]
}

fn minimum() -> [crate::CaseResult; 2] {
    use super::minimum::*;

    [
        crate::case(
            "collection-minimum-0",
            execute(false_minimum_is_rejected_after_the_actual_decoder_progress),
        ),
        crate::case(
            "collection-minimum-1",
            execute(nonprogress_and_configuration_failures_are_bounded),
        ),
    ]
}

fn collection_layouts() -> [crate::CaseResult; 8] {
    use super::collection_layouts::*;

    [
        crate::case(
            "collection-collection_layouts-0",
            execute(existing_byte_delimiters_and_option_defaults_are_preserved),
        ),
        crate::case(
            "collection-collection_layouts-1",
            execute(boundary_markers_preserve_existing_byte_delimiters),
        ),
        crate::case(
            "collection-collection_layouts-2",
            execute(marker_limits_reject_nonprogress_and_truncated_items),
        ),
        crate::case(
            "collection-collection_layouts-3",
            execute(configured_absence_consumes_and_validates_literal_patterns),
        ),
        crate::case(
            "collection-collection_layouts-4",
            execute(literal_absence_rejects_missing_and_invalid_bytes),
        ),
        crate::case(
            "collection-collection_layouts-5",
            execute(zero_absence_and_custom_boolean_storage_follow_source),
        ),
        crate::case(
            "collection-collection_layouts-6",
            execute(validated_wide_count_and_exhaustion_policy_share_claims),
        ),
        crate::case(
            "collection-collection_layouts-7",
            execute(fresh_dart_literals_match_both_optional_value_families),
        ),
    ]
}

fn native_option_values() -> [crate::CaseResult; 4] {
    use super::native_option_values::*;

    [
        crate::case(
            "collection-native_option_values-0",
            execute(native_variants_preserve_presence_nested_absence_and_value_extraction),
        ),
        crate::case(
            "collection-native_option_values-1",
            execute(nullable_adaptation_is_identity_without_collapsing_nested_values),
        ),
        crate::case(
            "collection-native_option_values-2",
            execute(native_fallback_runs_only_for_absence),
        ),
        crate::case(
            "collection-native_option_values-3",
            execute(native_equality_hash_and_numeric_debug_have_explicit_bounds),
        ),
    ]
}

fn codec_framing() -> [crate::CaseResult; 17] {
    use super::codec_framing::*;

    [
        crate::case(
            "collection-codec_framing-0",
            execute(fixed_fields_truncate_extend_and_clear_dirty_padding),
        ),
        crate::case(
            "collection-codec_framing-1",
            execute(fixed_failures_retain_documented_cursor_and_budget),
        ),
        crate::case(
            "collection-codec_framing-2",
            execute(selected_prefixes_use_encoded_bytes_and_explicit_consumption),
        ),
        crate::case(
            "collection-codec_framing-3",
            execute(nested_prefix_and_scratch_claims_share_one_budget),
        ),
        crate::case(
            "collection-codec_framing-4",
            execute(resize_changes_extent_without_changing_write_or_read_movement),
        ),
        crate::case(
            "collection-codec_framing-5",
            execute(reverse_fixed_width_preserves_nonzero_boundaries),
        ),
        crate::case(
            "collection-codec_framing-6",
            execute(offset_callbacks_see_whole_buffer_and_allow_backwards_movement),
        ),
        crate::case(
            "collection-codec_framing-7",
            execute(offset_failures_do_not_rollback_writes_or_refund_claims),
        ),
        crate::case(
            "collection-codec_framing-8",
            execute(dart_sentinel_matches_empty_and_overlap_semantics),
        ),
        crate::case(
            "collection-codec_framing-9",
            execute(existing_padding_and_exact_sentinel_contracts_remain),
        ),
        crate::case(
            "collection-codec_framing-10",
            execute(portable_framing_functions_execute_natively),
        ),
        crate::case(
            "collection-codec_framing-11",
            execute(actual_dart_oracle_replays_all_rows),
        ),
        crate::case(
            "collection-codec_framing-12",
            execute(oversized_scratch_and_extent_fail_without_panicking),
        ),
        crate::case(
            "collection-codec_framing-13",
            execute(malformed_numeric_prefix_domains_are_validated_in_typed_adapters),
        ),
        crate::case(
            "collection-codec_framing-14",
            execute(offset_contexts_observe_entry_exit_and_post_write_bytes),
        ),
        crate::case(
            "collection-codec_framing-15",
            execute(seek_and_callback_failures_preserve_checked_positions),
        ),
        crate::case(
            "collection-codec_framing-16",
            execute(prefix_frames_keep_exact_write_contract_with_metadata_resize),
        ),
    ]
}

fn codec_framing_accounting() -> [crate::CaseResult; 1] {
    use super::codec_framing_accounting::*;

    [crate::case(
        "collection-codec_framing_accounting-0",
        execute(exact_framing_receipt_preserves_source_ids_hashes_and_residuals),
    )]
}

fn composite_codecs() -> [crate::CaseResult; 11] {
    use super::composite_codecs::*;

    [
        crate::case(
            "collection-composite_codecs-0",
            execute(actual_dart_composition_goldens),
        ),
        crate::case(
            "collection-composite_codecs-1",
            execute(mappings_and_pairing_preserve_failures),
        ),
        crate::case(
            "collection-composite_codecs-2",
            execute(observers_preserve_failure_cursor_state),
        ),
        crate::case(
            "collection-composite_codecs-3",
            execute(bits_literals_unit_and_tagged_choices_reject_bad_boundaries),
        ),
        crate::case(
            "collection-composite_codecs-4",
            execute(nested_counts_and_dependent_records_share_allocation_claims),
        ),
        crate::case(
            "collection-composite_codecs-5",
            execute(dependent_records_share_allocation_claims),
        ),
        crate::case(
            "collection-composite_codecs-6",
            execute(delimiters_padding_offsets_and_exact_frames),
        ),
        crate::case(
            "collection-composite_codecs-7",
            execute(exact_sentinel_frames_and_overlap_reject),
        ),
        crate::case(
            "collection-composite_codecs-8",
            execute(padding_skips_input_and_zeroes_written_offsets),
        ),
        crate::case(
            "collection-composite_codecs-9",
            execute(isolated_portable_composite_cases),
        ),
        crate::case(
            "collection-composite_codecs-10",
            execute(sentinel_roundtrip_matches_first_delimiter_across_all_small_binary_inputs),
        ),
    ]
}

fn composite_accounting() -> [crate::CaseResult; 1] {
    use super::composite_accounting::*;

    [crate::case(
        "collection-composite_accounting-0",
        execute(exact_composite_receipt_preserves_every_original_field_and_remaining_id),
    )]
}
