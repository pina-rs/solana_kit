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
        Extension::DefaultAccountState(value) => {
            initialize_default_account_state(value, mint, program, limits)?
        }
        Extension::PausableConfig(value) => {
            initialize_pausable_config(value, mint, program, limits)?
        }
        Extension::PermissionedBurn(value) => {
            initialize_permissioned_burn(value, mint, program, limits)?
        }
        Extension::NonTransferable(value) => {
            initialize_non_transferable(value, mint, program, limits)?
        }
        Extension::PermanentDelegate(value) => {
            initialize_permanent_delegate(value, mint, program, limits)?
        }
        Extension::MintCloseAuthority(value) => {
            initialize_mint_close_authority(value, mint, program, limits)?
        }
        _ => return Ok(None),
    };

    Ok(Some(instruction))
}

fn initialize_default_account_state(
    value: &ExtensionDefaultAccountStatePayload,
    mint: Address,
    program: Address,
    limits: Limits,
) -> Result<Instruction, Error> {
    Ok(init!(
        program,
        mint,
        limits,
        InitializeDefaultAccountState {
            state: value.state.clone()
        },
        InitializeDefaultAccountStateAccounts
    ))
}

fn initialize_pausable_config(
    value: &ExtensionPausableConfigPayload,
    mint: Address,
    program: Address,
    limits: Limits,
) -> Result<Instruction, Error> {
    Ok(init!(
        program,
        mint,
        limits,
        InitializePausableConfig {
            authority: value.authority
        },
        InitializePausableConfigAccounts
    ))
}

fn initialize_permissioned_burn(
    value: &ExtensionPermissionedBurnPayload,
    mint: Address,
    program: Address,
    limits: Limits,
) -> Result<Instruction, Error> {
    Ok(init!(
        program,
        mint,
        limits,
        InitializePermissionedBurn {
            authority: value.authority.ok_or(Error::InvalidValue)?
        },
        InitializePermissionedBurnAccounts
    ))
}

fn initialize_non_transferable(
    _value: &ExtensionNonTransferablePayload,
    mint: Address,
    program: Address,
    limits: Limits,
) -> Result<Instruction, Error> {
    Ok(init!(
        program,
        mint,
        limits,
        InitializeNonTransferableMint {},
        InitializeNonTransferableMintAccounts
    ))
}

fn initialize_permanent_delegate(
    value: &ExtensionPermanentDelegatePayload,
    mint: Address,
    program: Address,
    limits: Limits,
) -> Result<Instruction, Error> {
    Ok(init!(
        program,
        mint,
        limits,
        InitializePermanentDelegate {
            delegate: value.delegate
        },
        InitializePermanentDelegateAccounts
    ))
}

fn initialize_mint_close_authority(
    value: &ExtensionMintCloseAuthorityPayload,
    mint: Address,
    program: Address,
    limits: Limits,
) -> Result<Instruction, Error> {
    Ok(init!(
        program,
        mint,
        limits,
        InitializeMintCloseAuthority {
            close_authority: Some(value.close_authority)
        },
        InitializeMintCloseAuthorityAccounts
    ))
}
