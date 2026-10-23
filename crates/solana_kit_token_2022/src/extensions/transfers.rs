use crate::*;
use solana_kit_core::{
    Address, Instruction,
    codecs::{Error, Limits},
};

pub(super) fn initialize(
    extension: &Extension,
    mint: Address,
    program: Address,
    limits: Limits,
) -> Result<Option<Instruction>, Error> {
    let instruction = match extension {
        Extension::ConfidentialTransferMint(value) => {
            initialize_confidential_transfer_mint(value, mint, program, limits)?
        }
        Extension::TransferFeeConfig(value) => {
            initialize_transfer_fee_config(value, mint, program, limits)?
        }
        Extension::InterestBearingConfig(value) => {
            initialize_interest_bearing_config(value, mint, program, limits)?
        }
        Extension::ScaledUiAmountConfig(value) => {
            initialize_scaled_ui_amount_config(value, mint, program, limits)?
        }
        Extension::ConfidentialTransferFee(value) => {
            initialize_confidential_transfer_fee(value, mint, program, limits)?
        }
        _ => return Ok(None),
    };

    Ok(Some(instruction))
}

fn initialize_confidential_transfer_mint(
    value: &ExtensionConfidentialTransferMintPayload,
    mint: Address,
    program: Address,
    limits: Limits,
) -> Result<Instruction, Error> {
    Ok(init!(
        program,
        mint,
        limits,
        InitializeConfidentialTransferMint {
            authority: value.authority,
            auto_approve_new_accounts: value.auto_approve_new_accounts,
            auditor_elgamal_pubkey: value.auditor_elgamal_pubkey
        },
        InitializeConfidentialTransferMintAccounts
    ))
}

fn initialize_transfer_fee_config(
    value: &ExtensionTransferFeeConfigPayload,
    mint: Address,
    program: Address,
    limits: Limits,
) -> Result<Instruction, Error> {
    Ok(init!(
        program,
        mint,
        limits,
        InitializeTransferFeeConfig {
            transfer_fee_config_authority: Some(value.transfer_fee_config_authority),
            withdraw_withheld_authority: Some(value.withdraw_withheld_authority),
            transfer_fee_basis_points: value.newer_transfer_fee.transfer_fee_basis_points,
            maximum_fee: value.newer_transfer_fee.maximum_fee
        },
        InitializeTransferFeeConfigAccounts
    ))
}

fn initialize_interest_bearing_config(
    value: &ExtensionInterestBearingConfigPayload,
    mint: Address,
    program: Address,
    limits: Limits,
) -> Result<Instruction, Error> {
    Ok(init!(
        program,
        mint,
        limits,
        InitializeInterestBearingMint {
            rate_authority: Some(value.rate_authority),
            rate: value.current_rate
        },
        InitializeInterestBearingMintAccounts
    ))
}

fn initialize_scaled_ui_amount_config(
    value: &ExtensionScaledUiAmountConfigPayload,
    mint: Address,
    program: Address,
    limits: Limits,
) -> Result<Instruction, Error> {
    Ok(init!(
        program,
        mint,
        limits,
        InitializeScaledUiAmountMint {
            authority: Some(value.authority),
            multiplier: value.multiplier
        },
        InitializeScaledUiAmountMintAccounts
    ))
}

fn initialize_confidential_transfer_fee(
    value: &ExtensionConfidentialTransferFeePayload,
    mint: Address,
    program: Address,
    limits: Limits,
) -> Result<Instruction, Error> {
    Ok(init!(
        program,
        mint,
        limits,
        InitializeConfidentialTransferFee {
            authority: value.authority,
            withdraw_withheld_authority_el_gamal_pubkey: value.elgamal_pubkey
        },
        InitializeConfidentialTransferFeeAccounts
    ))
}
