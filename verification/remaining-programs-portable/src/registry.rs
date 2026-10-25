//! Exact contract entrypoints; maintained by remaining-contracts.
#[path = "../../../crates/solana_kit_attestation_service/tests/contracts/accounts.rs"]
mod att_acc;
#[path = "../../../crates/solana_kit_attestation_service/tests/contracts/instructions.rs"]
mod att_ix;
#[path = "../../../crates/solana_kit_attestation_service/tests/contracts/pdas.rs"]
mod att_pda;
#[path = "../../../crates/solana_kit_attestation_service/tests/contracts/schema.rs"]
mod att_schema;
#[path = "../../../crates/solana_kit_squads/tests/accounts.rs"]
mod squads_acc;
#[path = "../../../crates/solana_kit_squads/tests/helpers.rs"]
mod squads_helpers;
#[path = "../../../crates/solana_kit_squads/tests/instructions.rs"]
mod squads_ix;
#[path = "../../../crates/solana_kit_subscriptions/tests/contracts.rs"]
mod subscriptions;

pub const COUNT: usize = 83;
pub const CASES: [(&str, fn()); COUNT] = [
    ("att_acc::credential", att_acc::credential as fn()),
    ("att_acc::schema", att_acc::schema as fn()),
    ("att_acc::attestation", att_acc::attestation as fn()),
    (
        "att_acc::close_event_and_all_errors",
        att_acc::close_event_and_all_errors as fn(),
    ),
    (
        "att_ix::create_credential",
        att_ix::create_credential as fn(),
    ),
    ("att_ix::create_schema", att_ix::create_schema as fn()),
    (
        "att_ix::change_schema_status",
        att_ix::change_schema_status as fn(),
    ),
    (
        "att_ix::change_authorized_signers",
        att_ix::change_authorized_signers as fn(),
    ),
    (
        "att_ix::change_schema_description",
        att_ix::change_schema_description as fn(),
    ),
    (
        "att_ix::change_schema_version",
        att_ix::change_schema_version as fn(),
    ),
    (
        "att_ix::create_attestation",
        att_ix::create_attestation as fn(),
    ),
    (
        "att_ix::close_attestation",
        att_ix::close_attestation as fn(),
    ),
    ("att_ix::tokenize_schema", att_ix::tokenize_schema as fn()),
    (
        "att_ix::create_tokenized_attestation",
        att_ix::create_tokenized_attestation as fn(),
    ),
    (
        "att_ix::close_tokenized_attestation",
        att_ix::close_tokenized_attestation as fn(),
    ),
    ("att_ix::emit_event", att_ix::emit_event as fn()),
    (
        "att_pda::all_seven_pda_recipes_match_literal_vectors",
        att_pda::all_seven_pda_recipes_match_literal_vectors as fn(),
    ),
    (
        "att_pda::custom_program_seed_order_utf8_and_version_boundaries",
        att_pda::custom_program_seed_order_utf8_and_version_boundaries as fn(),
    ),
    (
        "att_schema::every_schema_value_has_exact_literal_bytes",
        att_schema::every_schema_value_has_exact_literal_bytes as fn(),
    ),
    (
        "att_schema::schema_names_tags_and_exact_keys_are_checked",
        att_schema::schema_names_tags_and_exact_keys_are_checked as fn(),
    ),
    (
        "att_schema::malformed_unicode_binary_strings_and_vector_budgets",
        att_schema::malformed_unicode_binary_strings_and_vector_budgets as fn(),
    ),
    (
        "att_schema::schema_wire_order_is_independent_of_named_input_order",
        att_schema::schema_wire_order_is_independent_of_named_input_order as fn(),
    ),
    ("squads_acc::batch", squads_acc::batch as fn()),
    (
        "squads_acc::vault_batch_transaction",
        squads_acc::vault_batch_transaction as fn(),
    ),
    (
        "squads_acc::config_transaction",
        squads_acc::config_transaction as fn(),
    ),
    ("squads_acc::multisig", squads_acc::multisig as fn()),
    (
        "squads_acc::program_config",
        squads_acc::program_config as fn(),
    ),
    (
        "squads_acc::proposal_every_status",
        squads_acc::proposal_every_status as fn(),
    ),
    (
        "squads_acc::spending_limit_every_period",
        squads_acc::spending_limit_every_period as fn(),
    ),
    (
        "squads_acc::transaction_buffer",
        squads_acc::transaction_buffer as fn(),
    ),
    (
        "squads_acc::vault_transaction",
        squads_acc::vault_transaction as fn(),
    ),
    (
        "squads_acc::every_vote_permission_byte_and_error_code_preserve_their_wire_values",
        squads_acc::every_vote_permission_byte_and_error_code_preserve_their_wire_values as fn(),
    ),
    (
        "squads_ix::program_config_init",
        squads_ix::program_config_init as fn(),
    ),
    (
        "squads_ix::program_config_set_authority",
        squads_ix::program_config_set_authority as fn(),
    ),
    (
        "squads_ix::program_config_set_multisig_creation_fee",
        squads_ix::program_config_set_multisig_creation_fee as fn(),
    ),
    (
        "squads_ix::program_config_set_treasury",
        squads_ix::program_config_set_treasury as fn(),
    ),
    (
        "squads_ix::multisig_create",
        squads_ix::multisig_create as fn(),
    ),
    (
        "squads_ix::multisig_create_v2",
        squads_ix::multisig_create_v2 as fn(),
    ),
    (
        "squads_ix::multisig_add_member",
        squads_ix::multisig_add_member as fn(),
    ),
    (
        "squads_ix::multisig_remove_member",
        squads_ix::multisig_remove_member as fn(),
    ),
    (
        "squads_ix::multisig_set_time_lock",
        squads_ix::multisig_set_time_lock as fn(),
    ),
    (
        "squads_ix::multisig_change_threshold",
        squads_ix::multisig_change_threshold as fn(),
    ),
    (
        "squads_ix::multisig_set_config_authority",
        squads_ix::multisig_set_config_authority as fn(),
    ),
    (
        "squads_ix::multisig_set_rent_collector",
        squads_ix::multisig_set_rent_collector as fn(),
    ),
    (
        "squads_ix::multisig_add_spending_limit",
        squads_ix::multisig_add_spending_limit as fn(),
    ),
    (
        "squads_ix::multisig_remove_spending_limit",
        squads_ix::multisig_remove_spending_limit as fn(),
    ),
    (
        "squads_ix::config_transaction_create",
        squads_ix::config_transaction_create as fn(),
    ),
    (
        "squads_ix::config_transaction_execute",
        squads_ix::config_transaction_execute as fn(),
    ),
    (
        "squads_ix::vault_transaction_create",
        squads_ix::vault_transaction_create as fn(),
    ),
    (
        "squads_ix::transaction_buffer_create",
        squads_ix::transaction_buffer_create as fn(),
    ),
    (
        "squads_ix::transaction_buffer_close",
        squads_ix::transaction_buffer_close as fn(),
    ),
    (
        "squads_ix::transaction_buffer_extend",
        squads_ix::transaction_buffer_extend as fn(),
    ),
    (
        "squads_ix::vault_transaction_create_from_buffer",
        squads_ix::vault_transaction_create_from_buffer as fn(),
    ),
    (
        "squads_ix::vault_transaction_execute",
        squads_ix::vault_transaction_execute as fn(),
    ),
    ("squads_ix::batch_create", squads_ix::batch_create as fn()),
    (
        "squads_ix::batch_add_transaction",
        squads_ix::batch_add_transaction as fn(),
    ),
    (
        "squads_ix::batch_execute_transaction",
        squads_ix::batch_execute_transaction as fn(),
    ),
    (
        "squads_ix::proposal_create",
        squads_ix::proposal_create as fn(),
    ),
    (
        "squads_ix::proposal_activate",
        squads_ix::proposal_activate as fn(),
    ),
    (
        "squads_ix::proposal_approve",
        squads_ix::proposal_approve as fn(),
    ),
    (
        "squads_ix::proposal_reject",
        squads_ix::proposal_reject as fn(),
    ),
    (
        "squads_ix::proposal_cancel",
        squads_ix::proposal_cancel as fn(),
    ),
    (
        "squads_ix::proposal_cancel_v2",
        squads_ix::proposal_cancel_v2 as fn(),
    ),
    (
        "squads_ix::spending_limit_use",
        squads_ix::spending_limit_use as fn(),
    ),
    (
        "squads_ix::config_transaction_accounts_close",
        squads_ix::config_transaction_accounts_close as fn(),
    ),
    (
        "squads_ix::vault_transaction_accounts_close",
        squads_ix::vault_transaction_accounts_close as fn(),
    ),
    (
        "squads_ix::vault_batch_transaction_account_close",
        squads_ix::vault_batch_transaction_account_close as fn(),
    ),
    (
        "squads_ix::batch_accounts_close",
        squads_ix::batch_accounts_close as fn(),
    ),
    (
        "squads_helpers::every_pda_recipe_uses_distinct_nonzero_addresses_and_complete_integer_seeds",
        squads_helpers::every_pda_recipe_uses_distinct_nonzero_addresses_and_complete_integer_seeds
            as fn(),
    ),
    (
        "squads_helpers::vault_message_checks_each_header_partition_index_and_budget",
        squads_helpers::vault_message_checks_each_header_partition_index_and_budget as fn(),
    ),
    (
        "subscriptions::accounts::all_delegation_account_layouts_and_fetched_metadata",
        subscriptions::accounts::all_delegation_account_layouts_and_fetched_metadata as fn(),
    ),
    (
        "subscriptions::accounts::authority_plan_and_empty_event_authority_accounts",
        subscriptions::accounts::authority_plan_and_empty_event_authority_accounts as fn(),
    ),
    (
        "subscriptions::accounts::explicit_account_tags_fixed_utf8_and_revoke_tails",
        subscriptions::accounts::explicit_account_tags_fixed_utf8_and_revoke_tails as fn(),
    ),
    (
        "subscriptions::events::fixed_recurring_and_subscription_transfer_events",
        subscriptions::events::fixed_recurring_and_subscription_transfer_events as fn(),
    ),
    (
        "subscriptions::events::create_cancel_resume_and_plan_update_events",
        subscriptions::events::create_cancel_resume_and_plan_update_events as fn(),
    ),
    (
        "subscriptions::instructions::all_authority_and_revoke_instructions",
        subscriptions::instructions::all_authority_and_revoke_instructions as fn(),
    ),
    (
        "subscriptions::instructions::all_fixed_recurring_and_subscription_transfer_instructions",
        subscriptions::instructions::all_fixed_recurring_and_subscription_transfer_instructions
            as fn(),
    ),
    (
        "subscriptions::instructions::all_plan_and_subscription_lifecycle_instructions",
        subscriptions::instructions::all_plan_and_subscription_lifecycle_instructions as fn(),
    ),
    (
        "subscriptions::pdas::all_six_primary_pda_recipes_use_full_nonzero_seed_bytes",
        subscriptions::pdas::all_six_primary_pda_recipes_use_full_nonzero_seed_bytes as fn(),
    ),
    (
        "subscriptions::pdas::all_declared_defaults_and_explicit_overrides_resolve",
        subscriptions::pdas::all_declared_defaults_and_explicit_overrides_resolve as fn(),
    ),
    (
        "subscriptions::schedule::schedule_exact_boundaries_checked_amounts_and_no_partial_mutation",
        subscriptions::schedule::schedule_exact_boundaries_checked_amounts_and_no_partial_mutation
            as fn(),
    ),
    (
        "subscriptions::schedule::schedule_invalid_periods_overflow_and_full_plan_approval_contract",
        subscriptions::schedule::schedule_invalid_periods_overflow_and_full_plan_approval_contract
            as fn(),
    ),
    (
        "subscriptions::schedule::every_primary_error_name_code_message_and_unknown_code",
        subscriptions::schedule::every_primary_error_name_code_message_and_unknown_code as fn(),
    ),
];
