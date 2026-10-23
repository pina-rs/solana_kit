//! Source-generated typed invocation wrappers.
/// Canonical pinned program identity.
pub const PROGRAM: solana_kit_core::Address = solana_kit_core::Address::from_bytes([
    6, 221, 246, 225, 238, 117, 143, 222, 24, 66, 93, 188, 228, 108, 205, 218, 182, 26, 252, 77,
    131, 185, 13, 39, 254, 189, 249, 40, 216, 161, 139, 252,
]);
mod initialize_mint;
pub use initialize_mint::*;
mod initialize_account;
pub use initialize_account::*;
mod initialize_multisig;
pub use initialize_multisig::*;
mod transfer;
pub use transfer::*;
mod approve;
pub use approve::*;
mod revoke;
pub use revoke::*;
mod set_authority;
pub use set_authority::*;
mod mint_to;
pub use mint_to::*;
mod burn;
pub use burn::*;
mod close_account;
pub use close_account::*;
mod freeze_account;
pub use freeze_account::*;
mod thaw_account;
pub use thaw_account::*;
mod transfer_checked;
pub use transfer_checked::*;
mod approve_checked;
pub use approve_checked::*;
mod mint_to_checked;
pub use mint_to_checked::*;
mod burn_checked;
pub use burn_checked::*;
mod initialize_account2;
pub use initialize_account2::*;
mod sync_native;
pub use sync_native::*;
mod initialize_account3;
pub use initialize_account3::*;
mod initialize_multisig2;
pub use initialize_multisig2::*;
mod initialize_mint2;
pub use initialize_mint2::*;
mod get_account_data_size;
pub use get_account_data_size::*;
mod initialize_immutable_owner;
pub use initialize_immutable_owner::*;
mod amount_to_ui_amount;
pub use amount_to_ui_amount::*;
mod ui_amount_to_amount;
pub use ui_amount_to_amount::*;
mod initialize_mint_close_authority;
pub use initialize_mint_close_authority::*;
mod initialize_transfer_fee_config;
pub use initialize_transfer_fee_config::*;
mod transfer_checked_with_fee;
pub use transfer_checked_with_fee::*;
mod withdraw_withheld_tokens_from_mint;
pub use withdraw_withheld_tokens_from_mint::*;
mod withdraw_withheld_tokens_from_accounts;
pub use withdraw_withheld_tokens_from_accounts::*;
mod harvest_withheld_tokens_to_mint;
pub use harvest_withheld_tokens_to_mint::*;
mod set_transfer_fee;
pub use set_transfer_fee::*;
mod initialize_confidential_transfer_mint;
pub use initialize_confidential_transfer_mint::*;
mod update_confidential_transfer_mint;
pub use update_confidential_transfer_mint::*;
mod configure_confidential_transfer_account;
pub use configure_confidential_transfer_account::*;
mod approve_confidential_transfer_account;
pub use approve_confidential_transfer_account::*;
mod empty_confidential_transfer_account;
pub use empty_confidential_transfer_account::*;
mod confidential_deposit;
pub use confidential_deposit::*;
mod confidential_withdraw;
pub use confidential_withdraw::*;
mod confidential_transfer;
pub use confidential_transfer::*;
mod apply_confidential_pending_balance;
pub use apply_confidential_pending_balance::*;
mod enable_confidential_credits;
pub use enable_confidential_credits::*;
mod disable_confidential_credits;
pub use disable_confidential_credits::*;
mod enable_non_confidential_credits;
pub use enable_non_confidential_credits::*;
mod disable_non_confidential_credits;
pub use disable_non_confidential_credits::*;
mod confidential_transfer_with_fee;
pub use confidential_transfer_with_fee::*;
mod configure_confidential_transfer_account_with_registry;
pub use configure_confidential_transfer_account_with_registry::*;
mod initialize_default_account_state;
pub use initialize_default_account_state::*;
mod update_default_account_state;
pub use update_default_account_state::*;
mod reallocate;
pub use reallocate::*;
mod enable_memo_transfers;
pub use enable_memo_transfers::*;
mod disable_memo_transfers;
pub use disable_memo_transfers::*;
mod create_native_mint;
pub use create_native_mint::*;
mod initialize_non_transferable_mint;
pub use initialize_non_transferable_mint::*;
mod initialize_interest_bearing_mint;
pub use initialize_interest_bearing_mint::*;
mod update_rate_interest_bearing_mint;
pub use update_rate_interest_bearing_mint::*;
mod enable_cpi_guard;
pub use enable_cpi_guard::*;
mod disable_cpi_guard;
pub use disable_cpi_guard::*;
mod initialize_permanent_delegate;
pub use initialize_permanent_delegate::*;
mod initialize_transfer_hook;
pub use initialize_transfer_hook::*;
mod update_transfer_hook;
pub use update_transfer_hook::*;
mod initialize_confidential_transfer_fee;
pub use initialize_confidential_transfer_fee::*;
mod withdraw_withheld_tokens_from_mint_for_confidential_transfer_fee;
pub use withdraw_withheld_tokens_from_mint_for_confidential_transfer_fee::*;
mod withdraw_withheld_tokens_from_accounts_for_confidential_transfer_fee;
pub use withdraw_withheld_tokens_from_accounts_for_confidential_transfer_fee::*;
mod harvest_withheld_tokens_to_mint_for_confidential_transfer_fee;
pub use harvest_withheld_tokens_to_mint_for_confidential_transfer_fee::*;
mod enable_harvest_to_mint;
pub use enable_harvest_to_mint::*;
mod disable_harvest_to_mint;
pub use disable_harvest_to_mint::*;
mod withdraw_excess_lamports;
pub use withdraw_excess_lamports::*;
mod initialize_metadata_pointer;
pub use initialize_metadata_pointer::*;
mod update_metadata_pointer;
pub use update_metadata_pointer::*;
mod initialize_group_pointer;
pub use initialize_group_pointer::*;
mod update_group_pointer;
pub use update_group_pointer::*;
mod initialize_group_member_pointer;
pub use initialize_group_member_pointer::*;
mod update_group_member_pointer;
pub use update_group_member_pointer::*;
mod initialize_confidential_mint_burn;
pub use initialize_confidential_mint_burn::*;
mod rotate_supply_elgamal_pubkey;
pub use rotate_supply_elgamal_pubkey::*;
mod update_confidential_mint_burn_decryptable_supply;
pub use update_confidential_mint_burn_decryptable_supply::*;
mod confidential_mint;
pub use confidential_mint::*;
mod confidential_burn;
pub use confidential_burn::*;
mod apply_confidential_pending_burn;
pub use apply_confidential_pending_burn::*;
mod initialize_scaled_ui_amount_mint;
pub use initialize_scaled_ui_amount_mint::*;
mod update_multiplier_scaled_ui_mint;
pub use update_multiplier_scaled_ui_mint::*;
mod initialize_pausable_config;
pub use initialize_pausable_config::*;
mod pause;
pub use pause::*;
mod resume;
pub use resume::*;
mod initialize_token_metadata;
pub use initialize_token_metadata::*;
mod update_token_metadata_field;
pub use update_token_metadata_field::*;
mod remove_token_metadata_key;
pub use remove_token_metadata_key::*;
mod update_token_metadata_update_authority;
pub use update_token_metadata_update_authority::*;
mod emit_token_metadata;
pub use emit_token_metadata::*;
mod initialize_token_group;
pub use initialize_token_group::*;
mod update_token_group_max_size;
pub use update_token_group_max_size::*;
mod update_token_group_update_authority;
pub use update_token_group_update_authority::*;
mod initialize_token_group_member;
pub use initialize_token_group_member::*;
mod unwrap_lamports;
pub use unwrap_lamports::*;
mod initialize_permissioned_burn;
pub use initialize_permissioned_burn::*;
mod permissioned_burn;
pub use permissioned_burn::*;
mod permissioned_burn_checked;
pub use permissioned_burn_checked::*;
mod permissioned_confidential_burn;
pub use permissioned_confidential_burn::*;
mod batch;
use crate::{invocation::ParsedInstruction, *};
pub use batch::*;
use solana_kit_core::{
    Address, Instruction,
    codecs::{Decoder, Error, Limits},
};
/// Parses the canonical program, complete data frame and exact positional accounts.
pub fn parse_instruction(
    instruction: &Instruction,
    limits: Limits,
) -> Result<ParsedInstruction, Error> {
    parse_instruction_with_program(PROGRAM, instruction, limits)
}
/// Parses with an explicit expected program identity.
pub fn parse_instruction_with_program(
    program: Address,
    instruction: &Instruction,
    limits: Limits,
) -> Result<ParsedInstruction, Error> {
    if instruction.program_address != program {
        return Err(Error::InvalidValue);
    }

    let data = Token2022InstructionDataCodec.decode(&instruction.data, limits)?;

    match &data {
        Token2022InstructionData::InitializeMint(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializeAccount(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializeMultisig(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::Transfer(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::Approve(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::Revoke(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::SetAuthority(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::MintTo(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::Burn(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::CloseAccount(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::FreezeAccount(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::ThawAccount(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::TransferChecked(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::ApproveChecked(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::MintToChecked(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::BurnChecked(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializeAccount2(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::SyncNative(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializeAccount3(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializeMultisig2(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializeMint2(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::GetAccountDataSize(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializeImmutableOwner(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::AmountToUiAmount(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::UiAmountToAmount(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializeMintCloseAuthority(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializeTransferFeeConfig(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::TransferCheckedWithFee(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::WithdrawWithheldTokensFromMint(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::WithdrawWithheldTokensFromAccounts(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::HarvestWithheldTokensToMint(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::SetTransferFee(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializeConfidentialTransferMint(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::UpdateConfidentialTransferMint(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::ConfigureConfidentialTransferAccount(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::ApproveConfidentialTransferAccount(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::EmptyConfidentialTransferAccount(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::ConfidentialDeposit(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::ConfidentialWithdraw(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::ConfidentialTransfer(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::ApplyConfidentialPendingBalance(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::EnableConfidentialCredits(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::DisableConfidentialCredits(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::EnableNonConfidentialCredits(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::DisableNonConfidentialCredits(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::ConfidentialTransferWithFee(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::ConfigureConfidentialTransferAccountWithRegistry(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializeDefaultAccountState(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::UpdateDefaultAccountState(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::Reallocate(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::EnableMemoTransfers(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::DisableMemoTransfers(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::CreateNativeMint(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializeNonTransferableMint(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializeInterestBearingMint(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::UpdateRateInterestBearingMint(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::EnableCpiGuard(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::DisableCpiGuard(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializePermanentDelegate(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializeTransferHook(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::UpdateTransferHook(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializeConfidentialTransferFee(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::WithdrawWithheldTokensFromMintForConfidentialTransferFee(
            value,
        ) => value.validate_accounts(program, &instruction.accounts, limits)?,
        Token2022InstructionData::WithdrawWithheldTokensFromAccountsForConfidentialTransferFee(
            value,
        ) => value.validate_accounts(program, &instruction.accounts, limits)?,
        Token2022InstructionData::HarvestWithheldTokensToMintForConfidentialTransferFee(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::EnableHarvestToMint(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::DisableHarvestToMint(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::WithdrawExcessLamports(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializeMetadataPointer(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::UpdateMetadataPointer(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializeGroupPointer(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::UpdateGroupPointer(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializeGroupMemberPointer(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::UpdateGroupMemberPointer(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializeConfidentialMintBurn(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::RotateSupplyElgamalPubkey(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::UpdateConfidentialMintBurnDecryptableSupply(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::ConfidentialMint(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::ConfidentialBurn(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::ApplyConfidentialPendingBurn(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializeScaledUiAmountMint(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::UpdateMultiplierScaledUiMint(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializePausableConfig(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::Pause(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::Resume(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializeTokenMetadata(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::UpdateTokenMetadataField(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::RemoveTokenMetadataKey(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::UpdateTokenMetadataUpdateAuthority(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::EmitTokenMetadata(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializeTokenGroup(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::UpdateTokenGroupMaxSize(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::UpdateTokenGroupUpdateAuthority(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializeTokenGroupMember(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::UnwrapLamports(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::InitializePermissionedBurn(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::PermissionedBurn(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::PermissionedBurnChecked(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::PermissionedConfidentialBurn(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
        Token2022InstructionData::Batch(value) => {
            value.validate_accounts(program, &instruction.accounts, limits)?
        }
    }

    Ok(ParsedInstruction {
        data,
        accounts: instruction.accounts.clone(),
    })
}
