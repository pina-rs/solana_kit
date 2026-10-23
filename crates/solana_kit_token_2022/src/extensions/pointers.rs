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
        Extension::MetadataPointer(value) => {
            initialize_metadata_pointer(value, mint, program, limits)?
        }
        Extension::GroupPointer(value) => initialize_group_pointer(value, mint, program, limits)?,
        Extension::GroupMemberPointer(value) => {
            initialize_group_member_pointer(value, mint, program, limits)?
        }
        Extension::TransferHook(value) => initialize_transfer_hook(value, mint, program, limits)?,
        _ => return Ok(None),
    };

    Ok(Some(instruction))
}

fn initialize_metadata_pointer(
    value: &ExtensionMetadataPointerPayload,
    mint: Address,
    program: Address,
    limits: Limits,
) -> Result<Instruction, Error> {
    Ok(init!(
        program,
        mint,
        limits,
        InitializeMetadataPointer {
            authority: value.authority,
            metadata_address: value.metadata_address
        },
        InitializeMetadataPointerAccounts
    ))
}

fn initialize_group_pointer(
    value: &ExtensionGroupPointerPayload,
    mint: Address,
    program: Address,
    limits: Limits,
) -> Result<Instruction, Error> {
    Ok(init!(
        program,
        mint,
        limits,
        InitializeGroupPointer {
            authority: value.authority,
            group_address: value.group_address
        },
        InitializeGroupPointerAccounts
    ))
}

fn initialize_group_member_pointer(
    value: &ExtensionGroupMemberPointerPayload,
    mint: Address,
    program: Address,
    limits: Limits,
) -> Result<Instruction, Error> {
    Ok(init!(
        program,
        mint,
        limits,
        InitializeGroupMemberPointer {
            authority: value.authority,
            member_address: value.member_address
        },
        InitializeGroupMemberPointerAccounts
    ))
}

fn initialize_transfer_hook(
    value: &ExtensionTransferHookPayload,
    mint: Address,
    program: Address,
    limits: Limits,
) -> Result<Instruction, Error> {
    Ok(init!(
        program,
        mint,
        limits,
        InitializeTransferHook {
            authority: Some(value.authority),
            program_id: Some(value.program_id)
        },
        InitializeTransferHookAccounts
    ))
}
