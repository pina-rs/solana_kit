use solana_kit_associated_token_account::*;
use solana_kit_core::{
    AccountRole, Address, Instruction,
    codecs::{Error, Limits},
};

const LIMITS: Limits = Limits::new(1, 7);

fn create() -> CreateAccounts {
    CreateAccounts {
        payer: Address::from_bytes([1; 32]),
        associated_token: Address::from_bytes([2; 32]),
        owner: Address::from_bytes([3; 32]),
        mint: Address::from_bytes([4; 32]),
        system_program: SYSTEM_PROGRAM,
        token_program: TOKEN_PROGRAM,
    }
}

fn recover() -> RecoverNestedAccounts {
    RecoverNestedAccounts {
        nested_associated_token: Address::from_bytes([1; 32]),
        nested_mint: Address::from_bytes([2; 32]),
        destination_associated_token: Address::from_bytes([3; 32]),
        owner_associated_token: Address::from_bytes([4; 32]),
        owner_mint: Address::from_bytes([5; 32]),
        wallet: Address::from_bytes([6; 32]),
        token_program: TOKEN_2022_PROGRAM,
    }
}

#[test]
fn recovery_preserves_every_account_address_in_source_order() {
    let instruction = AssociatedTokenInstruction::RecoverNested(recover())
        .instruction(LIMITS)
        .unwrap();
    let actual: Vec<_> = instruction
        .accounts
        .iter()
        .map(|account| account.address)
        .collect();
    let mut expected: Vec<_> = (1..=6)
        .map(|byte| Address::from_bytes([byte; 32]))
        .collect();
    expected.push(TOKEN_2022_PROGRAM);
    assert_eq!(actual, expected);
    let mut extra = instruction;
    extra.accounts.push(extra.accounts[0]);
    assert_eq!(
        parse_instruction(&extra, Limits::new(1, 8)),
        Err(Error::InvalidValue)
    );
}

#[test]
fn all_three_constructors_and_parsers_preserve_exact_order_roles_and_bytes() {
    let values = [
        AssociatedTokenInstruction::Create(create()),
        AssociatedTokenInstruction::CreateIdempotent(create()),
        AssociatedTokenInstruction::RecoverNested(recover()),
    ];
    let roles = [
        vec![3, 1, 0, 0, 0, 0],
        vec![3, 1, 0, 0, 0, 0],
        vec![1, 0, 1, 0, 0, 3, 0],
    ];

    for (tag, value) in values.into_iter().enumerate() {
        let instruction = value.instruction(LIMITS).unwrap();
        assert_eq!(instruction.data, [u8::try_from(tag).unwrap()]);
        assert_eq!(
            instruction.program_address.to_string(),
            "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        );
        let actual: Vec<_> = instruction
            .accounts
            .iter()
            .map(|account| {
                u8::from(account.role.is_writable()) + 2 * u8::from(account.role.is_signer())
            })
            .collect();
        assert_eq!(actual, roles[tag]);
        assert_eq!(parse_instruction(&instruction, LIMITS).unwrap(), value);
        malformed_controls(value);
    }

    let instruction = AssociatedTokenInstruction::Create(create())
        .instruction(LIMITS)
        .unwrap();
    assert_eq!(
        instruction
            .accounts
            .iter()
            .map(|account| account.address)
            .collect::<Vec<_>>(),
        [
            Address::from_bytes([1; 32]),
            Address::from_bytes([2; 32]),
            Address::from_bytes([3; 32]),
            Address::from_bytes([4; 32]),
            SYSTEM_PROGRAM,
            TOKEN_PROGRAM
        ]
    );
    assert_eq!(
        error_message(0),
        Some("Associated token account owner does not match address derivation")
    );
    assert_eq!(error_message(1), None);
}

fn malformed_controls(value: AssociatedTokenInstruction) {
    let instruction = value.instruction(LIMITS).unwrap();
    let mut bad = instruction.clone();
    bad.program_address = SYSTEM_PROGRAM;
    assert_eq!(parse_instruction(&bad, LIMITS), Err(Error::InvalidValue));
    bad = instruction.clone();
    bad.data.clear();
    assert_eq!(parse_instruction(&bad, LIMITS), Err(Error::Truncated));
    bad.data = vec![instruction.data[0], 0];
    assert_eq!(
        parse_instruction(&bad, Limits::new(2, 7)),
        Err(Error::TrailingBytes)
    );
    budget_controls(value);
    metadata_controls(&instruction);

    for tag in 3..=255 {
        bad = instruction.clone();
        bad.data = vec![tag];
        assert_eq!(
            parse_instruction(&bad, LIMITS),
            Err(Error::InvalidDiscriminator)
        );
    }

    bad = instruction;
    bad.accounts.pop();
    assert_eq!(parse_instruction(&bad, LIMITS), Err(Error::InvalidValue));
}

fn budget_controls(value: AssociatedTokenInstruction) {
    let instruction = value.instruction(LIMITS).unwrap();
    assert_eq!(
        value.instruction(Limits::new(0, 7)),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        value.instruction(Limits::new(1, 0)),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        parse_instruction(&instruction, Limits::new(0, 7)),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        parse_instruction(&instruction, Limits::new(1, 0)),
        Err(Error::LimitExceeded)
    );
}

fn metadata_controls(instruction: &Instruction) {
    let roles = [
        AccountRole::Readonly,
        AccountRole::Writable,
        AccountRole::ReadonlySigner,
        AccountRole::WritableSigner,
    ];
    let cases = (0..instruction.accounts.len())
        .flat_map(|index| roles.into_iter().map(move |role| (index, role)));

    for (index, role) in cases {
        if role == instruction.accounts[index].role {
            continue;
        }

        let mut bad = instruction.clone();
        bad.accounts[index].role = role;
        assert_eq!(parse_instruction(&bad, LIMITS), Err(Error::InvalidValue));
    }
}

#[cfg(feature = "crypto")]
#[test]
fn pda_matches_independent_dart_literal_and_alternate_token_program() {
    let seeds = AssociatedTokenSeeds {
        owner: "2ojv9BAiHUrvsm9gxDe7fJSzbNZSJcxZvf8dqmWGHG8S"
            .parse()
            .unwrap(),
        mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
            .parse()
            .unwrap(),
        token_program: TOKEN_PROGRAM,
    };
    let pda = seeds.find().unwrap();
    assert_eq!(pda.bump(), 254);
    assert_eq!(
        pda.address().to_string(),
        "9KaA7vEBUdRCcBWxfuMjxYwKfvu8Us3Cg5gkhVFt2LNk"
    );
    assert_eq!(
        AssociatedTokenSeeds {
            token_program: TOKEN_2022_PROGRAM,
            ..seeds
        }
        .find()
        .unwrap()
        .address()
        .to_string(),
        "C9vSMEWDPoHa5GpA2w2g4sDsP4iB7PwyL1oTrEhXZiNA"
    );
    assert_eq!(pda, seeds.find_with_program(PROGRAM).unwrap());
    assert_ne!(
        pda.address(),
        seeds.find_with_program(SYSTEM_PROGRAM).unwrap().address()
    );
    assert_eq!(
        TOKEN_PROGRAM.to_string(),
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
    );
    assert_eq!(
        TOKEN_2022_PROGRAM.to_string(),
        "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
    );
}
