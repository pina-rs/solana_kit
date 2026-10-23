//! Original public contract bodies executed without host capabilities.

mod accounts;
mod builder_literals;
mod fetch;
mod independent;
mod oracles;
mod pre_initialize;
mod public;

type Body = (&'static str, fn());

macro_rules! body {
    ($module:ident, $body:ident) => {
        (
            concat!("token2022-", stringify!($module), "-", stringify!($body)),
            $module::$body as fn(),
        )
    };
}

/// Stable original body names paired with their unchanged assertions.
pub fn bodies() -> Vec<Body> {
    [
        accounts_0(),
        builder_literals_0(),
        builder_literals_1(),
        builder_literals_2(),
        builder_literals_3(),
        builder_literals_4(),
        builder_literals_5(),
        builder_literals_6(),
        builder_literals_7(),
        builder_literals_8(),
        builder_literals_9(),
        fetch_0(),
        oracles_0(),
        pre_initialize_0(),
        public_0(),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn accounts_0() -> Vec<Body> {
    vec![
        body!(
            accounts,
            canonical_355_padding_owner_and_account_type_are_checked
        ),
        body!(
            accounts,
            affinity_duplicate_and_terminator_entries_are_rejected
        ),
        body!(
            accounts,
            canonical_fee_frame_and_every_truncation_are_literal
        ),
        body!(accounts, initialized_multisig_and_base_accounts_dispatch),
        body!(
            accounts,
            public_extension_sequence_reuses_canonical_termination_and_shared_budget
        ),
    ]
}

fn builder_literals_0() -> Vec<Body> {
    vec![
        body!(builder_literals, initialize_mint_matches_dart_literal),
        body!(builder_literals, initialize_account_matches_dart_literal),
        body!(builder_literals, initialize_multisig_matches_dart_literal),
        body!(builder_literals, transfer_matches_dart_literal),
        body!(builder_literals, approve_matches_dart_literal),
        body!(builder_literals, revoke_matches_dart_literal),
        body!(builder_literals, set_authority_matches_dart_literal),
        body!(builder_literals, mint_to_matches_dart_literal),
        body!(builder_literals, burn_matches_dart_literal),
        body!(builder_literals, close_account_matches_dart_literal),
    ]
}

fn builder_literals_1() -> Vec<Body> {
    vec![
        body!(builder_literals, freeze_account_matches_dart_literal),
        body!(builder_literals, thaw_account_matches_dart_literal),
        body!(builder_literals, transfer_checked_matches_dart_literal),
        body!(builder_literals, approve_checked_matches_dart_literal),
        body!(builder_literals, mint_to_checked_matches_dart_literal),
        body!(builder_literals, burn_checked_matches_dart_literal),
        body!(builder_literals, initialize_account2_matches_dart_literal),
        body!(builder_literals, sync_native_matches_dart_literal),
        body!(builder_literals, initialize_account3_matches_dart_literal),
        body!(builder_literals, initialize_multisig2_matches_dart_literal),
    ]
}

fn builder_literals_2() -> Vec<Body> {
    vec![
        body!(builder_literals, initialize_mint2_matches_dart_literal),
        body!(builder_literals, get_account_data_size_matches_dart_literal),
        body!(
            builder_literals,
            initialize_immutable_owner_matches_dart_literal
        ),
        body!(builder_literals, amount_to_ui_amount_matches_dart_literal),
        body!(builder_literals, ui_amount_to_amount_matches_dart_literal),
        body!(
            builder_literals,
            initialize_mint_close_authority_matches_dart_literal
        ),
        body!(
            builder_literals,
            initialize_transfer_fee_config_matches_dart_literal
        ),
        body!(
            builder_literals,
            transfer_checked_with_fee_matches_dart_literal
        ),
        body!(
            builder_literals,
            withdraw_withheld_tokens_from_mint_matches_dart_literal
        ),
        body!(
            builder_literals,
            withdraw_withheld_tokens_from_accounts_matches_dart_literal
        ),
    ]
}

fn builder_literals_3() -> Vec<Body> {
    vec![
        body!(
            builder_literals,
            harvest_withheld_tokens_to_mint_matches_dart_literal
        ),
        body!(builder_literals, set_transfer_fee_matches_dart_literal),
        body!(
            builder_literals,
            initialize_confidential_transfer_mint_matches_dart_literal
        ),
        body!(
            builder_literals,
            update_confidential_transfer_mint_matches_dart_literal
        ),
        body!(
            builder_literals,
            configure_confidential_transfer_account_matches_dart_literal
        ),
        body!(
            builder_literals,
            approve_confidential_transfer_account_matches_dart_literal
        ),
        body!(
            builder_literals,
            empty_confidential_transfer_account_matches_dart_literal
        ),
        body!(builder_literals, confidential_deposit_matches_dart_literal),
        body!(builder_literals, confidential_withdraw_matches_dart_literal),
        body!(builder_literals, confidential_transfer_matches_dart_literal),
    ]
}

fn builder_literals_4() -> Vec<Body> {
    vec![
        body!(
            builder_literals,
            apply_confidential_pending_balance_matches_dart_literal
        ),
        body!(
            builder_literals,
            enable_confidential_credits_matches_dart_literal
        ),
        body!(
            builder_literals,
            disable_confidential_credits_matches_dart_literal
        ),
        body!(
            builder_literals,
            enable_non_confidential_credits_matches_dart_literal
        ),
        body!(
            builder_literals,
            disable_non_confidential_credits_matches_dart_literal
        ),
        body!(
            builder_literals,
            confidential_transfer_with_fee_matches_dart_literal
        ),
        body!(
            builder_literals,
            configure_confidential_transfer_account_with_registry_matches_dart_literal
        ),
        body!(
            builder_literals,
            initialize_default_account_state_matches_dart_literal
        ),
        body!(
            builder_literals,
            update_default_account_state_matches_dart_literal
        ),
        body!(builder_literals, reallocate_matches_dart_literal),
    ]
}

fn builder_literals_5() -> Vec<Body> {
    vec![
        body!(builder_literals, enable_memo_transfers_matches_dart_literal),
        body!(
            builder_literals,
            disable_memo_transfers_matches_dart_literal
        ),
        body!(builder_literals, create_native_mint_matches_dart_literal),
        body!(
            builder_literals,
            initialize_non_transferable_mint_matches_dart_literal
        ),
        body!(
            builder_literals,
            initialize_interest_bearing_mint_matches_dart_literal
        ),
        body!(
            builder_literals,
            update_rate_interest_bearing_mint_matches_dart_literal
        ),
        body!(builder_literals, enable_cpi_guard_matches_dart_literal),
        body!(builder_literals, disable_cpi_guard_matches_dart_literal),
        body!(
            builder_literals,
            initialize_permanent_delegate_matches_dart_literal
        ),
        body!(
            builder_literals,
            initialize_transfer_hook_matches_dart_literal
        ),
    ]
}

fn builder_literals_6() -> Vec<Body> {
    vec![
        body!(
            builder_literals,
            update_transfer_hook_matches_dart_literal
        ),
        body!(
            builder_literals,
            initialize_confidential_transfer_fee_matches_dart_literal
        ),
        body!(
            builder_literals,
            withdraw_withheld_tokens_from_mint_for_confidential_transfer_fee_matches_dart_literal
        ),
        body!(
            builder_literals,
            withdraw_withheld_tokens_from_accounts_for_confidential_transfer_fee_matches_dart_literal
        ),
        body!(
            builder_literals,
            harvest_withheld_tokens_to_mint_for_confidential_transfer_fee_matches_dart_literal
        ),
        body!(
            builder_literals,
            enable_harvest_to_mint_matches_dart_literal
        ),
        body!(
            builder_literals,
            disable_harvest_to_mint_matches_dart_literal
        ),
        body!(
            builder_literals,
            withdraw_excess_lamports_matches_dart_literal
        ),
        body!(
            builder_literals,
            initialize_metadata_pointer_matches_dart_literal
        ),
        body!(
            builder_literals,
            update_metadata_pointer_matches_dart_literal
        ),
    ]
}

fn builder_literals_7() -> Vec<Body> {
    vec![
        body!(
            builder_literals,
            initialize_group_pointer_matches_dart_literal
        ),
        body!(builder_literals, update_group_pointer_matches_dart_literal),
        body!(
            builder_literals,
            initialize_group_member_pointer_matches_dart_literal
        ),
        body!(
            builder_literals,
            update_group_member_pointer_matches_dart_literal
        ),
        body!(
            builder_literals,
            initialize_confidential_mint_burn_matches_dart_literal
        ),
        body!(
            builder_literals,
            rotate_supply_elgamal_pubkey_matches_dart_literal
        ),
        body!(
            builder_literals,
            update_confidential_mint_burn_decryptable_supply_matches_dart_literal
        ),
        body!(builder_literals, confidential_mint_matches_dart_literal),
        body!(builder_literals, confidential_burn_matches_dart_literal),
        body!(
            builder_literals,
            apply_confidential_pending_burn_matches_dart_literal
        ),
    ]
}

fn builder_literals_8() -> Vec<Body> {
    vec![
        body!(
            builder_literals,
            initialize_scaled_ui_amount_mint_matches_dart_literal
        ),
        body!(
            builder_literals,
            update_multiplier_scaled_ui_mint_matches_dart_literal
        ),
        body!(
            builder_literals,
            initialize_pausable_config_matches_dart_literal
        ),
        body!(builder_literals, pause_matches_dart_literal),
        body!(builder_literals, resume_matches_dart_literal),
        body!(
            builder_literals,
            initialize_token_metadata_matches_dart_literal
        ),
        body!(
            builder_literals,
            update_token_metadata_field_matches_dart_literal
        ),
        body!(
            builder_literals,
            remove_token_metadata_key_matches_dart_literal
        ),
        body!(
            builder_literals,
            update_token_metadata_update_authority_matches_dart_literal
        ),
        body!(builder_literals, emit_token_metadata_matches_dart_literal),
    ]
}

fn builder_literals_9() -> Vec<Body> {
    vec![
        body!(
            builder_literals,
            initialize_token_group_matches_dart_literal
        ),
        body!(
            builder_literals,
            update_token_group_max_size_matches_dart_literal
        ),
        body!(
            builder_literals,
            update_token_group_update_authority_matches_dart_literal
        ),
        body!(
            builder_literals,
            initialize_token_group_member_matches_dart_literal
        ),
        body!(builder_literals, unwrap_lamports_matches_dart_literal),
        body!(
            builder_literals,
            initialize_permissioned_burn_matches_dart_literal
        ),
        body!(builder_literals, permissioned_burn_matches_dart_literal),
        body!(
            builder_literals,
            permissioned_burn_checked_matches_dart_literal
        ),
        body!(
            builder_literals,
            permissioned_confidential_burn_matches_dart_literal
        ),
        body!(builder_literals, batch_matches_dart_literal),
    ]
}

fn fetch_0() -> Vec<Body> {
    vec![
        body!(
            fetch,
            explicit_fetch_preserves_context_metadata_and_missing_identity
        ),
        body!(
            fetch,
            explicit_batch_owner_and_allocation_failures_are_preserved
        ),
    ]
}

fn oracles_0() -> Vec<Body> {
    vec![
        body!(
            oracles,
            all_twelve_structural_base_frames_match_independent_dart
        ),
        body!(oracles, all_116_bare_payloads_match_independent_dart),
    ]
}

fn pre_initialize_0() -> Vec<Body> {
    vec![
        body!(
            pre_initialize,
            all_fifteen_branches_match_independent_literal_outputs_and_crosschecks
        ),
        body!(
            pre_initialize,
            invalid_authority_limits_and_unmatched_extensions_are_explicit
        ),
    ]
}

fn public_0() -> Vec<Body> {
    vec![
        body!(
            public,
            all_100_dispatch_and_400_independent_frames_preserve_accounts
        ),
        body!(
            public,
            typed_builder_defaults_authority_and_two_segment_boundary_are_literal
        ),
        body!(public, rent_default_is_literal),
        body!(public, omitted_middle_accounts_keep_actual_positions),
        body!(
            public,
            all_errors_preserve_unknown_numeric_codes_and_actual_index
        ),
    ]
}

/// Additional independent public failure and boundary groups.
pub fn independent_bodies() -> [Body; 3] {
    [
        (
            "token2022-independent-maximum-variable-tail",
            independent::maximum_variable_tail_keeps_exact_roles_and_item_limit,
        ),
        (
            "token2022-independent-canonical-mint-boundaries",
            independent::canonical_mint_checks_state_owner_and_exact_allocation,
        ),
        (
            "token2022-independent-multisig-thresholds",
            independent::multisig_threshold_edges_and_uninitialized_state_are_rejected,
        ),
    ]
}
