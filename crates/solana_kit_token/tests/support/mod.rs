//! Independent literal and complete-frame checks shared by the caller matrix.
use solana_kit_core::{
    AccountRole, Instruction,
    codecs::{Decoder, Encoder, Limits},
};
use solana_kit_token::parse_instruction_with_program;
use std::fmt::Debug;
const BUDGET: usize = 4096;
const TEXT_TAG: u8 = 24;
const BATCH_TAG: u8 = 255;

fn row(key: &str) -> Vec<&'static str> {
    include_str!("../fixtures/token-oracle.tsv")
        .lines()
        .find(|line| line.split('\t').next() == Some(key))
        .expect("oracle row")
        .split('\t')
        .collect()
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub(super) fn bytes(key: &str, bytes: &[u8]) {
    assert_eq!(hex(bytes), row(key)[1], "{key}");
}

pub(super) fn instruction(key: &str, instruction: &Instruction) {
    let expected = row(key);
    bytes(key, &instruction.data);
    assert_eq!(instruction.program_address.to_string(), expected[2]);
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
    assert_eq!(actual, expected[3], "{key}");
    invocation_negatives(key, instruction);
}

fn invocation_negatives(key: &str, instruction: &Instruction) {
    for end in 0..instruction.data.len() {
        let mut truncated = instruction.clone();
        truncated.data.truncate(end);

        let parsed = parse_instruction_with_program(
            instruction.program_address,
            &truncated,
            Limits::new(BUDGET, BUDGET),
        );
        assert_eq!(
            parsed.is_ok(),
            valid_prefix(&instruction.data, end),
            "{key} prefix {end}"
        );
    }

    let mut trailing = instruction.clone();
    trailing.data.push(0);

    if ![TEXT_TAG, BATCH_TAG].contains(&instruction.data[0]) {
        assert!(
            parse_instruction_with_program(
                instruction.program_address,
                &trailing,
                Limits::new(BUDGET, BUDGET)
            )
            .is_err()
        );
    }

    assert!(
        parse_instruction_with_program(
            instruction.program_address,
            instruction,
            Limits::new(instruction.data.len() - 1, BUDGET)
        )
        .is_err()
    );
}

fn valid_prefix(data: &[u8], end: usize) -> bool {
    if end == 0 {
        return false;
    }

    if data[0] == TEXT_TAG {
        return std::str::from_utf8(&data[1..end]).is_ok();
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

pub(super) fn fixed_negatives<C, T>(codec: &C, bytes: &[u8])
where
    C: Decoder<T> + Encoder<T>,
    T: Debug,
{
    for end in 0..bytes.len() {
        assert!(
            codec
                .decode(&bytes[..end], Limits::new(BUDGET, BUDGET))
                .is_err()
        );
    }

    let mut trailing = bytes.to_vec();
    trailing.push(0);

    assert!(
        codec
            .decode(&trailing, Limits::new(BUDGET, BUDGET))
            .is_err()
    );
    assert!(
        codec
            .decode(bytes, Limits::new(bytes.len() - 1, BUDGET))
            .is_err()
    );
}
