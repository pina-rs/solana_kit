#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate alloc;

mod program;

pub use program::*;

use alloc::{string::String, vec::Vec};
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Decoder, Encoder, Error, Limits, Utf8},
};

/// Decodes the entire raw payload, preserving nulls and rejecting malformed UTF-8.
/// A leading BOM is removed, matching the pinned Dart codec's default.
///
/// # Errors
/// Rejects malformed UTF-8 or an exceeded byte or allocation budget.
pub fn decode_memo(bytes: &[u8], limits: Limits) -> Result<String, Error> {
    let mut text = Utf8.decode(bytes, limits)?;

    if text.starts_with('\u{feff}') {
        text.drain(..3);
    }

    Ok(text)
}

/// Builds a v4 Memo invocation with raw UTF-8 and ordered readonly signers.
///
/// # Errors
/// Rejects a byte budget smaller than the payload or an exceeded signer budget.
pub fn add_memo(memo: &str, signers: &[Address], limits: Limits) -> Result<Instruction, Error> {
    add_memo_with_program(PROGRAM, memo, signers, limits)
}

/// Builds a Memo invocation for one of the three supported program identities.
/// Duplicate signer positions are retained.
///
/// # Errors
/// Rejects a foreign program, an exceeded signer budget or an oversized payload.
pub fn add_memo_with_program(
    program: Address,
    memo: &str,
    signers: &[Address],
    limits: Limits,
) -> Result<Instruction, Error> {
    if !SUPPORTED_PROGRAMS.contains(&program) {
        return Err(Error::InvalidValue);
    }

    if signers.len() > limits.max_items {
        return Err(Error::LimitExceeded);
    }

    let data = Utf8.encode(memo, limits.max_bytes)?;
    let accounts = signers
        .iter()
        .map(|address| AccountMeta {
            address: *address,
            role: AccountRole::ReadonlySigner,
        })
        .collect();

    Ok(Instruction {
        program_address: program,
        accounts,
        data,
    })
}

/// A fully parsed invocation, including its ordered required signers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedMemo {
    /// Decoded text, with a leading BOM removed.
    pub memo: String,
    /// Ordered required signers. Duplicate positions remain present.
    pub signers: Vec<Address>,
}

/// Parses a supported Memo invocation and requires every account to sign.
/// Writable signer privileges are accepted because the Memo program ignores them.
///
/// # Errors
/// Rejects a foreign program, non-signer account, malformed text or exceeded budget.
pub fn parse_instruction(instruction: &Instruction, limits: Limits) -> Result<ParsedMemo, Error> {
    if !SUPPORTED_PROGRAMS.contains(&instruction.program_address)
        || instruction
            .accounts
            .iter()
            .any(|account| !account.role.is_signer())
    {
        return Err(Error::InvalidValue);
    }

    let text_items = limits
        .max_items
        .checked_sub(instruction.accounts.len())
        .ok_or(Error::LimitExceeded)?;
    let memo = decode_memo(&instruction.data, Limits::new(limits.max_bytes, text_items))?;
    let signers = instruction
        .accounts
        .iter()
        .map(|account| account.address)
        .collect();

    Ok(ParsedMemo { memo, signers })
}

/// A memo found by the extraction helper.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtractedMemo {
    /// Text decoded from the complete raw payload.
    pub memo: String,
    /// Original bytes, including any leading BOM.
    pub bytes: Vec<u8>,
    /// Historical or current program identity.
    pub program_address: Address,
    /// Position in the original instruction slice.
    pub index: usize,
}

/// Extracts all supported memo versions in source order, ignoring account metadata.
/// Limits apply to each payload and to the number of extracted memos.
///
/// # Errors
/// Rejects malformed text or exceeded byte, text-allocation or result-count budgets.
pub fn memos_from_instructions(
    instructions: &[Instruction],
    limits: Limits,
) -> Result<Vec<ExtractedMemo>, Error> {
    let mut output = Vec::new();

    for (index, instruction) in instructions.iter().enumerate() {
        if !SUPPORTED_PROGRAMS.contains(&instruction.program_address) {
            continue;
        }

        if output.len() == limits.max_items {
            return Err(Error::LimitExceeded);
        }

        output.push(ExtractedMemo {
            memo: decode_memo(&instruction.data, limits)?,
            bytes: instruction.data.clone(),
            program_address: instruction.program_address,
            index,
        });
    }

    Ok(output)
}
