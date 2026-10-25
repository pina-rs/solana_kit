//! All36 literal headers/payloads and ordered privileges, including proof-tail boundaries.
use super::fixtures::{LIMITS, payload};
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Decoder, Encoder, Limits},
};
use solana_kit_mpl_bubblegum::*;

fn role(flag: char) -> AccountRole {
    match flag {
        'r' | 'e' => AccountRole::Readonly,
        'w' => AccountRole::Writable,
        's' | 'S' => AccountRole::ReadonlySigner,
        'W' => AccountRole::WritableSigner,
        _ => panic!("unknown literal account privilege"),
    }
}

pub fn all_instructions() {
    let mut names = Vec::new();

    for row in include_str!("../fixtures/instructions.tsv").lines().skip(1) {
        let columns: Vec<_> = row.split('\t').collect();
        assert_eq!(columns.len(), 5);
        assert!(!names.contains(&columns[0]));
        names.push(columns[0]);
        instruction_case(&columns);
    }

    assert_eq!(names.len(), 36);
}

fn instruction_case(columns: &[&str]) {
    let mut data = columns[1]
        .split(',')
        .map(|byte| byte.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    let header = data.clone();
    data.extend(payload(columns[2]));

    let accounts = columns[3]
        .chars()
        .filter(|value| !value.is_whitespace())
        .enumerate()
        .map(|(index, flag)| AccountMeta {
            address: Address::from_bytes([u8::try_from(index + 11).unwrap(); 32]),
            role: role(flag),
        })
        .collect();
    let instruction = Instruction {
        program_address: PROGRAM,
        accounts,
        data,
    };
    let value = parse_instruction(&instruction, LIMITS).unwrap();
    assert_eq!(identify_instruction(&instruction.data).unwrap(), header);
    assert_eq!(
        MplBubblegumInstructionDataCodec
            .encode(&value, LIMITS.max_bytes)
            .unwrap(),
        instruction.data
    );
    assert_eq!(
        MplBubblegumInstructionDataCodec
            .decode(&instruction.data, LIMITS)
            .unwrap(),
        value
    );
    malformed(&instruction);
    privileges(&instruction, columns[0]);

    explicit_program(&instruction);
    proof_tail(&instruction, columns[4]);
    optional_accounts(&instruction, columns[0]);
}

fn malformed(instruction: &Instruction) {
    const BYTE_BUDGET: usize = 8192;

    for end in 0..instruction.data.len() {
        let mut changed = instruction.clone();
        changed.data.truncate(end);
        assert!(parse_instruction(&changed, LIMITS).is_err());
    }

    let mut changed = instruction.clone();
    changed.data.push(0);
    assert!(parse_instruction(&changed, LIMITS).is_err());
    changed = instruction.clone();
    changed.data[0] ^= 0x80;
    assert!(parse_instruction(&changed, LIMITS).is_err());
    assert!(parse_instruction(instruction, Limits::new(instruction.data.len() - 1, 2048)).is_err());
    assert!(
        parse_instruction(
            instruction,
            Limits::new(BYTE_BUDGET, instruction.accounts.len() - 1)
        )
        .is_err()
    );
}

/// Owner-or-delegate instructions admit two signer policies; every other role stays exact.
fn privileges(instruction: &Instruction, operation: &str) {
    for index in 0..instruction.accounts.len() {
        for role in [
            AccountRole::Readonly,
            AccountRole::Writable,
            AccountRole::ReadonlySigner,
            AccountRole::WritableSigner,
        ] {
            if role == instruction.accounts[index].role {
                continue;
            }

            let mut changed = instruction.clone();
            changed.accounts[index].role = role;
            let conditional = ["burn", "transfer"].contains(&operation) && [1, 2].contains(&index);
            let valid =
                conditional && [AccountRole::Readonly, AccountRole::ReadonlySigner].contains(&role);
            assert_eq!(parse_instruction(&changed, LIMITS).is_ok(), valid);
        }
    }

    let mut changed = instruction.clone();
    changed.accounts.pop();
    assert!(parse_instruction(&changed, LIMITS).is_err());
}

fn explicit_program(instruction: &Instruction) {
    let mut changed = instruction.clone();
    let selected = Address::from_bytes([99; 32]);
    changed.program_address = selected;
    assert!(parse_instruction(&changed, LIMITS).is_err());
    assert!(parse_instruction_with_program(selected, &changed, LIMITS).is_ok());
}

fn proof_tail(instruction: &Instruction, policy: &str) {
    let mut changed = instruction.clone();
    changed.accounts.push(AccountMeta {
        address: Address::from_bytes([0xa5; 32]),
        role: AccountRole::Readonly,
    });
    assert_eq!(
        parse_instruction(&changed, LIMITS).is_ok(),
        policy != "false"
    );
    changed.accounts.last_mut().unwrap().role = AccountRole::Writable;
    assert_eq!(
        parse_instruction(&changed, LIMITS).is_ok(),
        policy == "creators"
    );
    changed.accounts.last_mut().unwrap().role = AccountRole::ReadonlySigner;
    assert_eq!(
        parse_instruction(&changed, LIMITS).is_ok(),
        policy == "creators"
    );
}

fn optional_accounts(instruction: &Instruction, operation: &str) {
    let row = include_str!("../fixtures/optional-accounts.tsv")
        .lines()
        .skip(1)
        .find_map(|row| row.split_once('\t').filter(|(name, _)| *name == operation));
    let Some((_, list)) = row else {
        return;
    };
    let indices = list
        .split(',')
        .map(|index| index.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    for selected in 0..(1usize << indices.len()) {
        optional_subset(instruction, &indices, selected);
    }

    for index in indices {
        let mut changed = instruction.clone();
        changed.accounts[index] = AccountMeta {
            address: PROGRAM,
            role: AccountRole::ReadonlySigner,
        };
        assert!(parse_instruction(&changed, LIMITS).is_err());
    }
}

/// Every absence subset must preserve the explicit deployment's account sentinel identity.
fn optional_subset(instruction: &Instruction, indices: &[usize], selected: usize) {
    let mut changed = instruction.clone();

    for (bit, index) in indices.iter().enumerate() {
        if selected & (1 << bit) != 0 {
            changed.accounts[*index] = AccountMeta {
                address: PROGRAM,
                role: AccountRole::Readonly,
            };
        }
    }

    assert!(parse_instruction(&changed, LIMITS).is_ok());
    let custom = Address::from_bytes([99; 32]);
    changed.program_address = custom;

    for index in indices {
        if changed.accounts[*index].address == PROGRAM {
            changed.accounts[*index].address = custom;
        }
    }

    assert!(parse_instruction_with_program(custom, &changed, LIMITS).is_ok());
}

#[test]
fn complete_instruction_matrix() {
    all_instructions();
}
