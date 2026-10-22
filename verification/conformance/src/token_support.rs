//! Literal Token oracle comparisons without host services.

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use core::fmt::Debug;
use solana_kit_core::{
    AccountRole, Instruction,
    codecs::{Decoder, Encoder, Limits},
};
use solana_kit_token::parse_instruction_with_program;

const BUDGET: usize = 4096;
const TEXT_TAG: u8 = 24;
const BATCH_TAG: u8 = 255;
const ORACLE: &str =
    include_str!("../../../crates/solana_kit_token/tests/fixtures/token-oracle.tsv");

fn row(key: &str) -> Result<Vec<&'static str>, &'static str> {
    ORACLE
        .lines()
        .find(|line| line.split('\t').next() == Some(key))
        .map(|line| line.split('\t').collect())
        .ok_or("missing Token oracle row")
}

pub(super) fn bytes(key: &str, bytes: &[u8]) -> Result<(), &'static str> {
    let actual: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
    crate::require(actual == row(key)?[1], "Token literal bytes differ")
}

pub(super) fn instruction(key: &str, instruction: &Instruction) -> Result<(), &'static str> {
    let expected = row(key)?;
    bytes(key, &instruction.data)?;
    crate::require(
        instruction.program_address.to_string() == expected[2],
        "Token program differs",
    )?;
    let actual = instruction
        .accounts
        .iter()
        .map(|meta| {
            let role = match meta.role {
                AccountRole::Readonly => 0,
                AccountRole::Writable => 1,
                AccountRole::ReadonlySigner => 2,
                AccountRole::WritableSigner => 3,
            };

            format!("{}:{role}", meta.address)
        })
        .collect::<Vec<_>>()
        .join(",");
    crate::require(actual == expected[3], "Token ordered account roles differ")?;
    invocation_negatives(instruction)
}

fn invocation_negatives(instruction: &Instruction) -> Result<(), &'static str> {
    for end in 0..instruction.data.len() {
        let mut truncated = instruction.clone();
        truncated.data.truncate(end);
        let parsed = parse_instruction_with_program(
            instruction.program_address,
            &truncated,
            Limits::new(BUDGET, BUDGET),
        );
        crate::require(
            parsed.is_ok() == valid_prefix(&instruction.data, end),
            "Token prefix classification differs",
        )?;
    }

    let mut trailing = instruction.clone();
    trailing.data.push(0);

    if ![TEXT_TAG, BATCH_TAG].contains(&instruction.data[0]) {
        crate::require(
            parse_instruction_with_program(
                instruction.program_address,
                &trailing,
                Limits::new(BUDGET, BUDGET),
            )
            .is_err(),
            "Token trailing byte accepted",
        )?;
    }

    crate::require(
        parse_instruction_with_program(
            instruction.program_address,
            instruction,
            Limits::new(instruction.data.len() - 1, BUDGET),
        )
        .is_err(),
        "Token byte limit ignored",
    )
}

fn valid_prefix(data: &[u8], end: usize) -> bool {
    if end == 0 {
        return false;
    }

    if data[0] == TEXT_TAG {
        return core::str::from_utf8(&data[1..end]).is_ok();
    }

    if data[0] != BATCH_TAG {
        return false;
    }

    let mut position = 1;

    while position < end {
        let Some(length) = data.get(position + 1) else {
            return false;
        };
        position += 2 + usize::from(*length);
    }

    position == end
}

pub(super) fn fixed_negatives<C, T>(codec: &C, bytes: &[u8]) -> Result<(), &'static str>
where
    C: Decoder<T> + Encoder<T>,
    T: Debug,
{
    for end in 0..bytes.len() {
        crate::require(
            codec
                .decode(&bytes[..end], Limits::new(BUDGET, BUDGET))
                .is_err(),
            "Token truncated account accepted",
        )?;
    }

    let mut trailing = bytes.to_vec();
    trailing.push(0);
    crate::require(
        codec
            .decode(&trailing, Limits::new(BUDGET, BUDGET))
            .is_err(),
        "Token account trailing byte accepted",
    )?;
    crate::require(
        codec
            .decode(bytes, Limits::new(bytes.len() - 1, BUDGET))
            .is_err(),
        "Token account limit ignored",
    )
}
