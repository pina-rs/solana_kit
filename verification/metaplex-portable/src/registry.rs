//! Exact contract entrypoints; maintained by metaplex-contracts.
#[path = "../../../crates/solana_kit_mpl_bubblegum/tests/contracts.rs"]
mod bubblegum;
#[path = "../../../crates/solana_kit_account_compression/tests/instructions.rs"]
mod compression_ix;
#[path = "../../../crates/solana_kit_account_compression/tests/state.rs"]
mod compression_state;
#[path = "../../../crates/solana_kit_account_compression/tests/tree.rs"]
mod compression_tree;
#[path = "../../../crates/solana_kit_mpl_core/tests/accounts.rs"]
mod core_acc;
#[path = "../../../crates/solana_kit_mpl_core/tests/errors.rs"]
mod core_errors;
#[path = "../../../crates/solana_kit_mpl_core/tests/external.rs"]
mod core_ext;
#[path = "../../../crates/solana_kit_mpl_core/tests/instructions.rs"]
mod core_ix;
#[path = "../../../crates/solana_kit_mpl_core/tests/pda.rs"]
mod core_pda;
#[path = "../../../crates/solana_kit_mpl_core/tests/plugins.rs"]
mod core_plugins;
#[path = "../../../crates/solana_kit_mpl_token_metadata/tests/accounts.rs"]
mod metadata_accounts;
#[path = "../../../crates/solana_kit_mpl_token_metadata/tests/errors.rs"]
mod metadata_errors;
#[path = "../../../crates/solana_kit_mpl_token_metadata/tests/helpers.rs"]
mod metadata_helpers;
#[path = "../../../crates/solana_kit_mpl_token_metadata/tests/instructions.rs"]
mod metadata_instructions;
#[path = "../../../crates/solana_kit_mpl_token_metadata/tests/pdas.rs"]
mod metadata_pdas;
#[path = "../../../crates/solana_kit_mpl_token_metadata/tests/variants.rs"]
mod metadata_variants;

pub const COUNT: usize = 158;
pub const CASES: [(&str, fn()); COUNT] = [
    (
        "compression_ix::initialize_transfer_append_and_close_match_primary_bytes",
        compression_ix::initialize_transfer_append_and_close_match_primary_bytes as fn(),
    ),
    (
        "compression_ix::every_proof_instruction_matches_primary_payload_and_remaining_roles",
        compression_ix::every_proof_instruction_matches_primary_payload_and_remaining_roles
            as fn(),
    ),
    (
        "compression_state::application_and_change_log_records_and_all_event_variants",
        compression_state::application_and_change_log_records_and_all_event_variants as fn(),
    ),
    (
        "compression_state::complete_header_records_version_and_account_types",
        compression_state::complete_header_records_version_and_account_types as fn(),
    ),
    (
        "compression_state::every_error_code_and_unknown_code_is_lossless",
        compression_state::every_error_code_and_unknown_code_is_lossless as fn(),
    ),
    (
        "compression_tree::primary_allocation_vectors_and_inverse_canopies",
        compression_tree::primary_allocation_vectors_and_inverse_canopies as fn(),
    ),
    (
        "compression_tree::invalid_and_overflowing_canopies_fail_explicitly",
        compression_tree::invalid_and_overflowing_canopies_fail_explicitly as fn(),
    ),
    (
        "compression_tree::canonical_addresses_and_default_roles_are_source_grounded",
        compression_tree::canonical_addresses_and_default_roles_are_source_grounded as fn(),
    ),
    (
        "core_acc::every_account_has_independent_nonzero_wire_and_strict_header",
        core_acc::every_account_has_independent_nonzero_wire_and_strict_header as fn(),
    ),
    (
        "core_acc::fetched_snapshots_preserve_extensions_and_reject_owner_or_allocation_drift",
        core_acc::fetched_snapshots_preserve_extensions_and_reject_owner_or_allocation_drift
            as fn(),
    ),
    (
        "core_acc::registry_bounds_tags_and_overlaps_are_checked_before_returning_views",
        core_acc::registry_bounds_tags_and_overlaps_are_checked_before_returning_views as fn(),
    ),
    (
        "core_acc::external_registry_data_pairs_and_disjoint_payload_ranges_are_required",
        core_acc::external_registry_data_pairs_and_disjoint_payload_ranges_are_required as fn(),
    ),
    (
        "core_errors::all_57_known_error_names_messages_and_unknown_codes_remain_exact",
        core_errors::all_57_known_error_names_messages_and_unknown_codes_remain_exact as fn(),
    ),
    (
        "core_ext::every_external_plugin_init_update_and_key_variant_has_a_literal_contract",
        core_ext::every_external_plugin_init_update_and_key_variant_has_a_literal_contract
            as fn(),
    ),
    (
        "core_ext::every_extra_account_variant_and_optional_plugin_list_has_literal_wire",
        core_ext::every_extra_account_variant_and_optional_plugin_list_has_literal_wire as fn(),
    ),
    (
        "core_ext::all_unit_enum_values_are_literal_and_unknown_values_reject",
        core_ext::all_unit_enum_values_are_literal_and_unknown_values_reject as fn(),
    ),
    ("core_ix::create_v1", core_ix::create_v1 as fn()),
    (
        "core_ix::create_collection_v1",
        core_ix::create_collection_v1 as fn(),
    ),
    ("core_ix::add_plugin_v1", core_ix::add_plugin_v1 as fn()),
    (
        "core_ix::add_collection_plugin_v1",
        core_ix::add_collection_plugin_v1 as fn(),
    ),
    (
        "core_ix::remove_plugin_v1",
        core_ix::remove_plugin_v1 as fn(),
    ),
    (
        "core_ix::remove_collection_plugin_v1",
        core_ix::remove_collection_plugin_v1 as fn(),
    ),
    (
        "core_ix::update_plugin_v1",
        core_ix::update_plugin_v1 as fn(),
    ),
    (
        "core_ix::update_collection_plugin_v1",
        core_ix::update_collection_plugin_v1 as fn(),
    ),
    (
        "core_ix::approve_plugin_authority_v1",
        core_ix::approve_plugin_authority_v1 as fn(),
    ),
    (
        "core_ix::approve_collection_plugin_authority_v1",
        core_ix::approve_collection_plugin_authority_v1 as fn(),
    ),
    (
        "core_ix::revoke_plugin_authority_v1",
        core_ix::revoke_plugin_authority_v1 as fn(),
    ),
    (
        "core_ix::revoke_collection_plugin_authority_v1",
        core_ix::revoke_collection_plugin_authority_v1 as fn(),
    ),
    ("core_ix::burn_v1", core_ix::burn_v1 as fn()),
    (
        "core_ix::burn_collection_v1",
        core_ix::burn_collection_v1 as fn(),
    ),
    ("core_ix::transfer_v1", core_ix::transfer_v1 as fn()),
    ("core_ix::update_v1", core_ix::update_v1 as fn()),
    (
        "core_ix::update_collection_v1",
        core_ix::update_collection_v1 as fn(),
    ),
    ("core_ix::compress_v1", core_ix::compress_v1 as fn()),
    ("core_ix::decompress_v1", core_ix::decompress_v1 as fn()),
    ("core_ix::collect", core_ix::collect as fn()),
    ("core_ix::create_v2", core_ix::create_v2 as fn()),
    (
        "core_ix::create_collection_v2",
        core_ix::create_collection_v2 as fn(),
    ),
    ("core_ix::add_external_v1", core_ix::add_external_v1 as fn()),
    (
        "core_ix::add_collection_external_v1",
        core_ix::add_collection_external_v1 as fn(),
    ),
    (
        "core_ix::remove_external_v1",
        core_ix::remove_external_v1 as fn(),
    ),
    (
        "core_ix::remove_collection_external_v1",
        core_ix::remove_collection_external_v1 as fn(),
    ),
    (
        "core_ix::update_external_v1",
        core_ix::update_external_v1 as fn(),
    ),
    (
        "core_ix::update_collection_external_v1",
        core_ix::update_collection_external_v1 as fn(),
    ),
    (
        "core_ix::write_external_v1",
        core_ix::write_external_v1 as fn(),
    ),
    (
        "core_ix::write_collection_external_v1",
        core_ix::write_collection_external_v1 as fn(),
    ),
    ("core_ix::update_v2", core_ix::update_v2 as fn()),
    ("core_ix::execute_v1", core_ix::execute_v1 as fn()),
    (
        "core_ix::update_collection_info_v1",
        core_ix::update_collection_info_v1 as fn(),
    ),
    (
        "core_ix::add_collections_v1",
        core_ix::add_collections_v1 as fn(),
    ),
    (
        "core_ix::remove_collections_v1",
        core_ix::remove_collections_v1 as fn(),
    ),
    ("core_ix::add_assets_v1", core_ix::add_assets_v1 as fn()),
    (
        "core_ix::remove_assets_v1",
        core_ix::remove_assets_v1 as fn(),
    ),
    ("core_ix::add_groups_v1", core_ix::add_groups_v1 as fn()),
    (
        "core_ix::remove_groups_v1",
        core_ix::remove_groups_v1 as fn(),
    ),
    ("core_ix::create_group_v1", core_ix::create_group_v1 as fn()),
    ("core_ix::close_group_v1", core_ix::close_group_v1 as fn()),
    ("core_ix::update_group_v1", core_ix::update_group_v1 as fn()),
    (
        "core_ix::remaining_privileges_unknown_tags_and_optional_payloads",
        core_ix::remaining_privileges_unknown_tags_and_optional_payloads as fn(),
    ),
    (
        "core_pda::independent_dart_vectors_cover_default_custom_and_on_curve_bump_search",
        core_pda::independent_dart_vectors_cover_default_custom_and_on_curve_bump_search as fn(),
    ),
    (
        "core_pda::context_and_custom_seed_failures_are_explicit",
        core_pda::context_and_custom_seed_failures_are_explicit as fn(),
    ),
    (
        "core_pda::custom_program_precedence_all_seed_contexts_and_direct_addresses_are_explicit",
        core_pda::custom_program_precedence_all_seed_contexts_and_direct_addresses_are_explicit
            as fn(),
    ),
    (
        "core_plugins::every_plugin_variant_has_an_independent_payload_vector",
        core_plugins::every_plugin_variant_has_an_independent_payload_vector as fn(),
    ),
    (
        "core_plugins::authority_rule_seed_lifecycle_and_schema_variants_keep_their_literal_tags",
        core_plugins::authority_rule_seed_lifecycle_and_schema_variants_keep_their_literal_tags
            as fn(),
    ),
    (
        "metadata_accounts::metadata",
        metadata_accounts::metadata as fn(),
    ),
    (
        "metadata_accounts::every_allocated_account_key",
        metadata_accounts::every_allocated_account_key as fn(),
    ),
    (
        "metadata_errors::every_error_name_code_and_message",
        metadata_errors::every_error_name_code_and_message as fn(),
    ),
    (
        "metadata_helpers::string_limits_and_puffed_view",
        metadata_helpers::string_limits_and_puffed_view as fn(),
    ),
    (
        "metadata_helpers::edition_bitmap_is_most_significant_bit_first",
        metadata_helpers::edition_bitmap_is_most_significant_bit_first as fn(),
    ),
    (
        "metadata_helpers::payload_keys_preserve_order_and_reject_duplicates",
        metadata_helpers::payload_keys_preserve_order_and_reject_duplicates as fn(),
    ),
    (
        "metadata_instructions::create_metadata_account",
        metadata_instructions::create_metadata_account as fn(),
    ),
    (
        "metadata_instructions::update_metadata_account",
        metadata_instructions::update_metadata_account as fn(),
    ),
    (
        "metadata_instructions::deprecated_create_master_edition",
        metadata_instructions::deprecated_create_master_edition as fn(),
    ),
    (
        "metadata_instructions::deprecated_mint_edition",
        metadata_instructions::deprecated_mint_edition as fn(),
    ),
    (
        "metadata_instructions::update_primary_sale",
        metadata_instructions::update_primary_sale as fn(),
    ),
    (
        "metadata_instructions::deprecated_set_reservation_list",
        metadata_instructions::deprecated_set_reservation_list as fn(),
    ),
    (
        "metadata_instructions::deprecated_create_reservation_list",
        metadata_instructions::deprecated_create_reservation_list as fn(),
    ),
    (
        "metadata_instructions::sign_metadata",
        metadata_instructions::sign_metadata as fn(),
    ),
    (
        "metadata_instructions::deprecated_mint_printing_via_token",
        metadata_instructions::deprecated_mint_printing_via_token as fn(),
    ),
    (
        "metadata_instructions::deprecated_mint_printing",
        metadata_instructions::deprecated_mint_printing as fn(),
    ),
    (
        "metadata_instructions::create_master_edition",
        metadata_instructions::create_master_edition as fn(),
    ),
    (
        "metadata_instructions::mint_edition_via_token",
        metadata_instructions::mint_edition_via_token as fn(),
    ),
    (
        "metadata_instructions::convert_master_edition",
        metadata_instructions::convert_master_edition as fn(),
    ),
    (
        "metadata_instructions::mint_edition_via_vault",
        metadata_instructions::mint_edition_via_vault as fn(),
    ),
    (
        "metadata_instructions::puff_metadata",
        metadata_instructions::puff_metadata as fn(),
    ),
    (
        "metadata_instructions::update_metadata_v2",
        metadata_instructions::update_metadata_v2 as fn(),
    ),
    (
        "metadata_instructions::create_metadata_v2",
        metadata_instructions::create_metadata_v2 as fn(),
    ),
    (
        "metadata_instructions::create_master_edition_v3",
        metadata_instructions::create_master_edition_v3 as fn(),
    ),
    (
        "metadata_instructions::verify_collection",
        metadata_instructions::verify_collection as fn(),
    ),
    (
        "metadata_instructions::utilize",
        metadata_instructions::utilize as fn(),
    ),
    (
        "metadata_instructions::approve_use_authority",
        metadata_instructions::approve_use_authority as fn(),
    ),
    (
        "metadata_instructions::revoke_use_authority",
        metadata_instructions::revoke_use_authority as fn(),
    ),
    (
        "metadata_instructions::unverify_collection",
        metadata_instructions::unverify_collection as fn(),
    ),
    (
        "metadata_instructions::approve_collection_authority",
        metadata_instructions::approve_collection_authority as fn(),
    ),
    (
        "metadata_instructions::revoke_collection_authority",
        metadata_instructions::revoke_collection_authority as fn(),
    ),
    (
        "metadata_instructions::set_and_verify_collection",
        metadata_instructions::set_and_verify_collection as fn(),
    ),
    (
        "metadata_instructions::freeze_delegated_account",
        metadata_instructions::freeze_delegated_account as fn(),
    ),
    (
        "metadata_instructions::thaw_delegated_account",
        metadata_instructions::thaw_delegated_account as fn(),
    ),
    (
        "metadata_instructions::remove_creator_verification",
        metadata_instructions::remove_creator_verification as fn(),
    ),
    (
        "metadata_instructions::burn_nft",
        metadata_instructions::burn_nft as fn(),
    ),
    (
        "metadata_instructions::verify_sized_collection",
        metadata_instructions::verify_sized_collection as fn(),
    ),
    (
        "metadata_instructions::unverify_sized_collection",
        metadata_instructions::unverify_sized_collection as fn(),
    ),
    (
        "metadata_instructions::set_and_verify_sized_collection",
        metadata_instructions::set_and_verify_sized_collection as fn(),
    ),
    (
        "metadata_instructions::create_metadata_v3",
        metadata_instructions::create_metadata_v3 as fn(),
    ),
    (
        "metadata_instructions::set_collection_size",
        metadata_instructions::set_collection_size as fn(),
    ),
    (
        "metadata_instructions::set_token_standard",
        metadata_instructions::set_token_standard as fn(),
    ),
    (
        "metadata_instructions::bubblegum_collection_size",
        metadata_instructions::bubblegum_collection_size as fn(),
    ),
    (
        "metadata_instructions::burn_edition_nft",
        metadata_instructions::burn_edition_nft as fn(),
    ),
    (
        "metadata_instructions::create_escrow_account",
        metadata_instructions::create_escrow_account as fn(),
    ),
    (
        "metadata_instructions::close_escrow_account",
        metadata_instructions::close_escrow_account as fn(),
    ),
    (
        "metadata_instructions::transfer_out_of_escrow",
        metadata_instructions::transfer_out_of_escrow as fn(),
    ),
    (
        "metadata_instructions::burn",
        metadata_instructions::burn as fn(),
    ),
    (
        "metadata_instructions::create",
        metadata_instructions::create as fn(),
    ),
    (
        "metadata_instructions::mint",
        metadata_instructions::mint as fn(),
    ),
    (
        "metadata_instructions::delegate",
        metadata_instructions::delegate as fn(),
    ),
    (
        "metadata_instructions::revoke",
        metadata_instructions::revoke as fn(),
    ),
    (
        "metadata_instructions::lock",
        metadata_instructions::lock as fn(),
    ),
    (
        "metadata_instructions::unlock",
        metadata_instructions::unlock as fn(),
    ),
    (
        "metadata_instructions::migrate",
        metadata_instructions::migrate as fn(),
    ),
    (
        "metadata_instructions::transfer",
        metadata_instructions::transfer as fn(),
    ),
    (
        "metadata_instructions::update",
        metadata_instructions::update as fn(),
    ),
    (
        "metadata_instructions::use_asset",
        metadata_instructions::use_asset as fn(),
    ),
    (
        "metadata_instructions::verify_asset",
        metadata_instructions::verify_asset as fn(),
    ),
    (
        "metadata_instructions::unverify_asset",
        metadata_instructions::unverify_asset as fn(),
    ),
    (
        "metadata_instructions::collect",
        metadata_instructions::collect as fn(),
    ),
    (
        "metadata_instructions::print",
        metadata_instructions::print as fn(),
    ),
    (
        "metadata_instructions::resize",
        metadata_instructions::resize as fn(),
    ),
    (
        "metadata_instructions::close_accounts",
        metadata_instructions::close_accounts as fn(),
    ),
    (
        "metadata_pdas::all_canonical_pda_literals",
        metadata_pdas::all_canonical_pda_literals as fn(),
    ),
    (
        "metadata_pdas::explicit_program_seed_domains_and_delegate_roles",
        metadata_pdas::explicit_program_seed_domains_and_delegate_roles as fn(),
    ),
    (
        "metadata_variants::finite_tags",
        metadata_variants::finite_tags as fn(),
    ),
    (
        "metadata_variants::toggles_and_print_variants",
        metadata_variants::toggles_and_print_variants as fn(),
    ),
    (
        "metadata_variants::every_delegate_variant",
        metadata_variants::every_delegate_variant as fn(),
    ),
    (
        "metadata_variants::every_update_variant",
        metadata_variants::every_update_variant as fn(),
    ),
    (
        "metadata_variants::nested_seeds_proofs_and_authorization",
        metadata_variants::nested_seeds_proofs_and_authorization as fn(),
    ),
    (
        "metadata_variants::creator_options_utf8_boolean_and_budget_failures",
        metadata_variants::creator_options_utf8_boolean_and_budget_failures as fn(),
    ),
    (
        "bubblegum::das::provider_normalization",
        bubblegum::das::provider_normalization as fn(),
    ),
    (
        "bubblegum::das::malformed_provider",
        bubblegum::das::malformed_provider as fn(),
    ),
    (
        "bubblegum::das::exact_proof_binding",
        bubblegum::das::exact_proof_binding as fn(),
    ),
    (
        "bubblegum::das::request_cancellation",
        bubblegum::das::request_cancellation as fn(),
    ),
    (
        "bubblegum::enums::named_tags",
        bubblegum::enums::named_tags as fn(),
    ),
    (
        "bubblegum::hashing::hash_domains",
        bubblegum::hashing::hash_domains as fn(),
    ),
    (
        "bubblegum::hashing::sparse_tree",
        bubblegum::hashing::sparse_tree as fn(),
    ),
    (
        "bubblegum::helpers::proof_accounts",
        bubblegum::helpers::proof_accounts as fn(),
    ),
    (
        "bubblegum::helpers::metadata_constraints",
        bubblegum::helpers::metadata_constraints as fn(),
    ),
    (
        "bubblegum::helpers::raw_royalties",
        bubblegum::helpers::raw_royalties as fn(),
    ),
    (
        "bubblegum::helpers::leaf_authorities",
        bubblegum::helpers::leaf_authorities as fn(),
    ),
    (
        "bubblegum::options::payload_options",
        bubblegum::options::payload_options as fn(),
    ),
    (
        "bubblegum::pdas::complete_pda_recipes",
        bubblegum::pdas::complete_pda_recipes as fn(),
    ),
    (
        "bubblegum::plans::mint_defaults",
        bubblegum::plans::mint_defaults as fn(),
    ),
    (
        "bubblegum::plans::mint_v2_options",
        bubblegum::plans::mint_v2_options as fn(),
    ),
    (
        "bubblegum::plans::tree_defaults",
        bubblegum::plans::tree_defaults as fn(),
    ),
    (
        "bubblegum::state::tree_account",
        bubblegum::state::tree_account as fn(),
    ),
    (
        "bubblegum::state::voucher_accounts",
        bubblegum::state::voucher_accounts as fn(),
    ),
    (
        "bubblegum::state::metadata_branches",
        bubblegum::state::metadata_branches as fn(),
    ),
    (
        "bubblegum::state::program_errors",
        bubblegum::state::program_errors as fn(),
    ),
    (
        "bubblegum::state::leaf_events",
        bubblegum::state::leaf_events as fn(),
    ),
    (
        "bubblegum::wire::all_instructions",
        bubblegum::wire::all_instructions as fn(),
    ),
];
