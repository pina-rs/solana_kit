//! Exact contract entrypoints; maintained by program-contracts.
#[path = "../../../crates/solana_kit_address_lookup_table/tests/accounts.rs"]
mod alt_accounts;
#[path = "../../../crates/solana_kit_address_lookup_table/tests/bank.rs"]
mod alt_bank;
#[path = "../../../crates/solana_kit_address_lookup_table/tests/contracts.rs"]
mod alt_contracts;
#[path = "../../../crates/solana_kit_address_lookup_table/tests/fullwidth.rs"]
mod alt_fullwidth;
#[path = "../../../crates/solana_kit_config/tests/contracts.rs"]
mod config_contracts;
#[path = "../../../crates/solana_kit_config/tests/encoding.rs"]
mod config_encoding;
#[path = "../../../crates/solana_kit_config/tests/metadata.rs"]
mod config_metadata;
#[path = "../../../crates/solana_kit_loader/tests/helpers.rs"]
mod loader_helpers;
#[path = "../../../crates/solana_kit_loader/tests/protocol.rs"]
mod loader_protocol;
#[path = "../../../crates/solana_kit_loader/tests/v3_metadata.rs"]
mod loader_v3_metadata;
#[path = "../../../crates/solana_kit_loader/tests/v4_metadata.rs"]
mod loader_v4_metadata;
#[path = "../../../crates/solana_kit_loader/tests/values.rs"]
mod loader_values;
#[path = "../../../crates/solana_kit_loader/tests/wires.rs"]
mod loader_wires;
#[path = "../../../crates/solana_kit_stake/tests/activation.rs"]
mod stake_activation;
#[path = "../../../crates/solana_kit_stake/tests/helpers.rs"]
mod stake_helpers;
#[path = "../../../crates/solana_kit_stake/tests/stake_metadata.rs"]
mod stake_metadata;
#[path = "../../../crates/solana_kit_stake/tests/options.rs"]
mod stake_options;
#[path = "../../../crates/solana_kit_stake/tests/protocol.rs"]
mod stake_protocol;
#[path = "../../../crates/solana_kit_stake/tests/values.rs"]
mod stake_values;
#[path = "../../../crates/solana_kit_stake/tests/wires.rs"]
mod stake_wires;
#[path = "../../../crates/solana_kit_stake/tests/withdrawal.rs"]
mod stake_withdrawal;

pub const COUNT: usize = 88;
pub const CASES: [(&str, fn()); COUNT] = [
    (
        "alt_accounts::create_and_extend_preserve_distinct_positional_addresses",
        alt_accounts::create_and_extend_preserve_distinct_positional_addresses as fn(),
    ),
    (
        "alt_bank::activation_and_deactivation_use_bounded_bank_history",
        alt_bank::activation_and_deactivation_use_bounded_bank_history as fn(),
    ),
    (
        "alt_bank::fetched_lookup_account_preserves_metadata",
        alt_bank::fetched_lookup_account_preserves_metadata as fn(),
    ),
    (
        "alt_contracts::literal_all_five_variants",
        alt_contracts::literal_all_five_variants as fn(),
    ),
    (
        "alt_contracts::optional_funding_and_privileges",
        alt_contracts::optional_funding_and_privileges as fn(),
    ),
    (
        "alt_contracts::literal_header_none_and_table_boundaries",
        alt_contracts::literal_header_none_and_table_boundaries as fn(),
    ),
    (
        "alt_contracts::absent_authority_padding_is_rust_source_layout",
        alt_contracts::absent_authority_padding_is_rust_source_layout as fn(),
    ),
    (
        "alt_contracts::pda_uses_actual_authority_slot_seeds",
        alt_contracts::pda_uses_actual_authority_slot_seeds as fn(),
    ),
    (
        "alt_fullwidth::fullwidth_create_and_nonzero_authority_are_literal",
        alt_fullwidth::fullwidth_create_and_nonzero_authority_are_literal as fn(),
    ),
    (
        "alt_fullwidth::upstream_error_codes_require_actual_program_and_instruction_index",
        alt_fullwidth::upstream_error_codes_require_actual_program_and_instruction_index as fn(),
    ),
    (
        "config_contracts::literal_keys_data_and_roles",
        config_contracts::literal_keys_data_and_roles as fn(),
    ),
    (
        "config_contracts::compact_boundaries_and_failures",
        config_contracts::compact_boundaries_and_failures as fn(),
    ),
    (
        "config_contracts::signer_duplicates_and_privilege_negatives",
        config_contracts::signer_duplicates_and_privilege_negatives as fn(),
    ),
    (
        "config_metadata::fetched_config_value_equality_hash_and_diagnostics",
        config_metadata::fetched_config_value_equality_hash_and_diagnostics as fn(),
    ),
    (
        "stake_protocol::every_instruction_tag_dispatches_with_exact_u32_width",
        stake_protocol::every_instruction_tag_dispatches_with_exact_u32_width as fn(),
    ),
    (
        "stake_protocol::canonical_account_preserves_all_four_states_and_tuple_payload_order",
        stake_protocol::canonical_account_preserves_all_four_states_and_tuple_payload_order
            as fn(),
    ),
    (
        "stake_protocol::u32_enum_unknown_truncation_budget_and_rollback_are_explicit",
        stake_protocol::u32_enum_unknown_truncation_budget_and_rollback_are_explicit as fn(),
    ),
    (
        "stake_protocol::ordered_sysvars_optional_signer_remaining_and_override_are_preserved",
        stake_protocol::ordered_sysvars_optional_signer_remaining_and_override_are_preserved
            as fn(),
    ),
    (
        "stake_protocol::all_error_numbers_and_failure_attribution_are_lossless",
        stake_protocol::all_error_numbers_and_failure_attribution_are_lossless as fn(),
    ),
    (
        "stake_wires::all_seventeen_stake_literals_and_malformed_boundaries",
        stake_wires::all_seventeen_stake_literals_and_malformed_boundaries as fn(),
    ),
    (
        "stake_metadata::initialize_metadata_contract",
        stake_metadata::initialize_metadata_contract as fn(),
    ),
    (
        "stake_metadata::authorize_metadata_contract",
        stake_metadata::authorize_metadata_contract as fn(),
    ),
    (
        "stake_metadata::delegatestake_metadata_contract",
        stake_metadata::delegatestake_metadata_contract as fn(),
    ),
    (
        "stake_metadata::split_metadata_contract",
        stake_metadata::split_metadata_contract as fn(),
    ),
    (
        "stake_metadata::withdraw_metadata_contract",
        stake_metadata::withdraw_metadata_contract as fn(),
    ),
    (
        "stake_metadata::deactivate_metadata_contract",
        stake_metadata::deactivate_metadata_contract as fn(),
    ),
    (
        "stake_metadata::setlockup_metadata_contract",
        stake_metadata::setlockup_metadata_contract as fn(),
    ),
    (
        "stake_metadata::merge_metadata_contract",
        stake_metadata::merge_metadata_contract as fn(),
    ),
    (
        "stake_metadata::authorizewithseed_metadata_contract",
        stake_metadata::authorizewithseed_metadata_contract as fn(),
    ),
    (
        "stake_metadata::initializechecked_metadata_contract",
        stake_metadata::initializechecked_metadata_contract as fn(),
    ),
    (
        "stake_metadata::authorizechecked_metadata_contract",
        stake_metadata::authorizechecked_metadata_contract as fn(),
    ),
    (
        "stake_metadata::authorizecheckedwithseed_metadata_contract",
        stake_metadata::authorizecheckedwithseed_metadata_contract as fn(),
    ),
    (
        "stake_metadata::setlockupchecked_metadata_contract",
        stake_metadata::setlockupchecked_metadata_contract as fn(),
    ),
    (
        "stake_metadata::getminimumdelegation_metadata_contract",
        stake_metadata::getminimumdelegation_metadata_contract as fn(),
    ),
    (
        "stake_metadata::deactivatedelinquent_metadata_contract",
        stake_metadata::deactivatedelinquent_metadata_contract as fn(),
    ),
    (
        "stake_metadata::movestake_metadata_contract",
        stake_metadata::movestake_metadata_contract as fn(),
    ),
    (
        "stake_metadata::movelamports_metadata_contract",
        stake_metadata::movelamports_metadata_contract as fn(),
    ),
    (
        "stake_helpers::creation_defaults_and_atomic_plan_match_source",
        stake_helpers::creation_defaults_and_atomic_plan_match_source as fn(),
    ),
    (
        "stake_helpers::delegation_defaults_and_account_metadata_remain_exact",
        stake_helpers::delegation_defaults_and_account_metadata_remain_exact as fn(),
    ),
    (
        "stake_helpers::fetched_stake_preserves_metadata_padding_and_original_errors",
        stake_helpers::fetched_stake_preserves_metadata_padding_and_original_errors as fn(),
    ),
    (
        "stake_options::all_lockup_option_combinations_have_literal_frames",
        stake_options::all_lockup_option_combinations_have_literal_frames as fn(),
    ),
    (
        "stake_options::checked_lockup_options_and_invalid_tags_are_independent",
        stake_options::checked_lockup_options_and_invalid_tags_are_independent as fn(),
    ),
    (
        "stake_options::explicit_lockup_plan_preserves_nondefault_values",
        stake_options::explicit_lockup_plan_preserves_nondefault_values as fn(),
    ),
    (
        "stake_activation::bootstrap_and_epoch_boundaries_preserve_upstream_precedence",
        stake_activation::bootstrap_and_epoch_boundaries_preserve_upstream_precedence as fn(),
    ),
    (
        "stake_activation::warmup_uses_cluster_share_and_the_rate_change_epoch",
        stake_activation::warmup_uses_cluster_share_and_the_rate_change_epoch as fn(),
    ),
    (
        "stake_activation::cooldown_retains_only_effective_stake_and_uses_cluster_history",
        stake_activation::cooldown_retains_only_effective_stake_and_uses_cluster_history as fn(),
    ),
    (
        "stake_activation::missing_history_distinguishes_initial_fallback_from_internal_gaps",
        stake_activation::missing_history_distinguishes_initial_fallback_from_internal_gaps
            as fn(),
    ),
    (
        "stake_activation::minimum_progress_zero_portions_and_empty_accounts_are_distinct",
        stake_activation::minimum_progress_zero_portions_and_empty_accounts_are_distinct as fn(),
    ),
    (
        "stake_activation::saturation_and_maximum_epoch_keep_integer_rate_semantics",
        stake_activation::saturation_and_maximum_epoch_keep_integer_rate_semantics as fn(),
    ),
    (
        "stake_activation::ambiguous_history_and_impossible_non_bootstrap_epochs_are_rejected",
        stake_activation::ambiguous_history_and_impossible_non_bootstrap_epochs_are_rejected
            as fn(),
    ),
    (
        "stake_withdrawal::inactive_closure_does_not_make_the_rent_gap_withdrawable",
        stake_withdrawal::inactive_closure_does_not_make_the_rent_gap_withdrawable as fn(),
    ),
    (
        "stake_withdrawal::uninitialized_and_empty_accounts_require_their_account_signer",
        stake_withdrawal::uninitialized_and_empty_accounts_require_their_account_signer as fn(),
    ),
    (
        "stake_withdrawal::warming_stake_reserves_the_entire_delegation",
        stake_withdrawal::warming_stake_reserves_the_entire_delegation as fn(),
    ),
    (
        "stake_withdrawal::cooldown_history_and_rate_select_the_actual_reserved_principal",
        stake_withdrawal::cooldown_history_and_rate_select_the_actual_reserved_principal as fn(),
    ),
    (
        "stake_withdrawal::lockup_requires_both_boundaries_or_the_matching_custodian",
        stake_withdrawal::lockup_requires_both_boundaries_or_the_matching_custodian as fn(),
    ),
    (
        "stake_withdrawal::invalid_observations_never_become_successful_zero_amounts",
        stake_withdrawal::invalid_observations_never_become_successful_zero_amounts as fn(),
    ),
    (
        "stake_withdrawal::selected_rent_reserve_does_not_read_the_stored_metadata_reserve",
        stake_withdrawal::selected_rent_reserve_does_not_read_the_stored_metadata_reserve as fn(),
    ),
    (
        "stake_values::source_value_traits_retain_payload_and_variant_identity",
        stake_values::source_value_traits_retain_payload_and_variant_identity as fn(),
    ),
    (
        "loader_protocol::pinned_loader_byte_lengths_and_tags_remain_distinct",
        loader_protocol::pinned_loader_byte_lengths_and_tags_remain_distinct as fn(),
    ),
    (
        "loader_protocol::every_empty_variant_and_unknown_code_uses_exact_width",
        loader_protocol::every_empty_variant_and_unknown_code_uses_exact_width as fn(),
    ),
    (
        "loader_protocol::optional_account_omission_and_exact_privileges_follow_source",
        loader_protocol::optional_account_omission_and_exact_privileges_follow_source as fn(),
    ),
    (
        "loader_protocol::actual_headers_cover_optional_authority_and_tail",
        loader_protocol::actual_headers_cover_optional_authority_and_tail as fn(),
    ),
    (
        "loader_protocol::actual_v4_status_headers_reject_unknown_u64_tags",
        loader_protocol::actual_v4_status_headers_reject_unknown_u64_tags as fn(),
    ),
    (
        "loader_wires::every_v3_instruction_uses_exact_literal_data",
        loader_wires::every_v3_instruction_uses_exact_literal_data as fn(),
    ),
    (
        "loader_wires::every_v4_instruction_uses_exact_literal_data",
        loader_wires::every_v4_instruction_uses_exact_literal_data as fn(),
    ),
    (
        "loader_v3_metadata::initializebuffer_metadata_contract",
        loader_v3_metadata::initializebuffer_metadata_contract as fn(),
    ),
    (
        "loader_v3_metadata::write_metadata_contract",
        loader_v3_metadata::write_metadata_contract as fn(),
    ),
    (
        "loader_v3_metadata::deploywithmaxdatalen_metadata_contract",
        loader_v3_metadata::deploywithmaxdatalen_metadata_contract as fn(),
    ),
    (
        "loader_v3_metadata::upgrade_metadata_contract",
        loader_v3_metadata::upgrade_metadata_contract as fn(),
    ),
    (
        "loader_v3_metadata::setauthority_metadata_contract",
        loader_v3_metadata::setauthority_metadata_contract as fn(),
    ),
    (
        "loader_v3_metadata::close_metadata_contract",
        loader_v3_metadata::close_metadata_contract as fn(),
    ),
    (
        "loader_v3_metadata::extendprogram_metadata_contract",
        loader_v3_metadata::extendprogram_metadata_contract as fn(),
    ),
    (
        "loader_v3_metadata::setauthoritychecked_metadata_contract",
        loader_v3_metadata::setauthoritychecked_metadata_contract as fn(),
    ),
    (
        "loader_v4_metadata::write_metadata_contract",
        loader_v4_metadata::write_metadata_contract as fn(),
    ),
    (
        "loader_v4_metadata::truncate_metadata_contract",
        loader_v4_metadata::truncate_metadata_contract as fn(),
    ),
    (
        "loader_v4_metadata::deploy_metadata_contract",
        loader_v4_metadata::deploy_metadata_contract as fn(),
    ),
    (
        "loader_v4_metadata::retract_metadata_contract",
        loader_v4_metadata::retract_metadata_contract as fn(),
    ),
    (
        "loader_v4_metadata::transferauthority_metadata_contract",
        loader_v4_metadata::transferauthority_metadata_contract as fn(),
    ),
    (
        "loader_v4_metadata::finalize_metadata_contract",
        loader_v4_metadata::finalize_metadata_contract as fn(),
    ),
    (
        "loader_helpers::deploy_chunks_and_capacity_defaults_match_source_order",
        loader_helpers::deploy_chunks_and_capacity_defaults_match_source_order as fn(),
    ),
    (
        "loader_helpers::upgrade_follows_all_writes_and_preserves_spill_identity",
        loader_helpers::upgrade_follows_all_writes_and_preserves_spill_identity as fn(),
    ),
    (
        "loader_helpers::fetched_loader_retains_metadata_and_validates_owner_size_and_header",
        loader_helpers::fetched_loader_retains_metadata_and_validates_owner_size_and_header
            as fn(),
    ),
    (
        "loader_values::authority_header_literals_values_and_diagnostics_are_exact",
        loader_values::authority_header_literals_values_and_diagnostics_are_exact as fn(),
    ),
    (
        "config_encoding::independent_empty_and_nonempty_wire_roundtrips",
        config_encoding::independent_empty_and_nonempty_wire_roundtrips as fn(),
    ),
    (
        "config_encoding::both_store_helpers_retain_order_roles_and_program",
        config_encoding::both_store_helpers_retain_order_roles_and_program as fn(),
    ),
    (
        "config_encoding::writer_preserves_emitted_bytes_cursor_and_truncation",
        config_encoding::writer_preserves_emitted_bytes_cursor_and_truncation as fn(),
    ),
    (
        "config_encoding::malformed_input_keeps_exact_diagnostics",
        config_encoding::malformed_input_keeps_exact_diagnostics as fn(),
    ),
    (
        "config_encoding::caller_budgets_and_unrepresentable_counts_keep_error_precedence",
        config_encoding::caller_budgets_and_unrepresentable_counts_keep_error_precedence as fn(),
    ),
];
