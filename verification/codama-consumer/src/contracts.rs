//! Literal declared layouts and independent PDA composition exercise generated clients.

use crate::{
    empty_pda_contracts as empty, literal_contracts as literal, record_contracts as records,
    resolver_contracts as resolvers,
};
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Decoder, Encoder, Error, Limits},
    crypto::find_program_address,
};

const LIMITS: Limits = Limits::new(128, 128);
const VALUE: u64 = 0x0102_0304_0506_0708;
const ACCOUNT_TAG: u8 = 7;
const EVENT_TAG: u8 = 9;
const ACCOUNT_WIRE: [u8; 9] = [7, 8, 7, 6, 5, 4, 3, 2, 1];
const EVENT_WIRE: [u8; 9] = [9, 8, 7, 6, 5, 4, 3, 2, 1];
const SOURCE: Address = Address::from_bytes([11; 32]);
const DESTINATION: Address = Address::from_bytes([12; 32]);
const OVERRIDE: Address = Address::from_bytes([13; 32]);
const OTHER_PROGRAM: Address = Address::from_bytes([42; 32]);

fn record_predicates() {
    let state = records::StateRecord {
        magic: ACCOUNT_TAG,
        value: VALUE,
    };
    let event = records::StateChanged {
        magic: EVENT_TAG,
        value: VALUE,
    };
    assert_eq!(
        records::StateRecordCodec
            .encode(&state, ACCOUNT_WIRE.len())
            .unwrap(),
        ACCOUNT_WIRE
    );
    assert_eq!(
        records::StateChangedCodec
            .encode(&event, EVENT_WIRE.len())
            .unwrap(),
        EVENT_WIRE
    );
    assert_eq!(
        records::StateRecordCodec
            .decode(&ACCOUNT_WIRE, LIMITS)
            .unwrap(),
        state
    );
    assert_eq!(
        records::StateChangedCodec
            .decode(&EVENT_WIRE, LIMITS)
            .unwrap(),
        event
    );

    let invalid_state = records::StateRecord {
        magic: EVENT_TAG,
        value: VALUE,
    };
    let invalid_event = records::StateChanged {
        magic: ACCOUNT_TAG,
        value: VALUE,
    };
    assert_eq!(
        records::StateRecordCodec.encode(&invalid_state, ACCOUNT_WIRE.len()),
        Err(Error::InvalidDiscriminator)
    );
    assert_eq!(
        records::StateChangedCodec.encode(&invalid_event, EVENT_WIRE.len()),
        Err(Error::InvalidDiscriminator)
    );
    assert_eq!(
        records::StateRecordCodec.decode(&EVENT_WIRE, LIMITS),
        Err(Error::InvalidDiscriminator)
    );
    assert_eq!(
        records::StateChangedCodec.decode(&ACCOUNT_WIRE, LIMITS),
        Err(Error::InvalidDiscriminator)
    );
    assert!(records::StateRecordCodec.decode(&[], LIMITS).is_err());
    assert!(
        records::StateChangedCodec
            .decode(&EVENT_WIRE[..1], LIMITS)
            .is_err()
    );

    let mut trailing = ACCOUNT_WIRE.to_vec();
    trailing.push(0);
    assert_eq!(
        records::StateRecordCodec.decode(&trailing, LIMITS),
        Err(Error::TrailingBytes)
    );
    assert_eq!(
        records::StateRecordCodec.encode(&state, ACCOUNT_WIRE.len() - 1),
        Err(Error::LimitExceeded)
    );
}

fn positional_optionals() {
    for program in [resolvers::PROGRAM, OTHER_PROGRAM] {
        let inputs = resolvers::SetRolesAccounts {
            program: SOURCE,
            maybe: None,
            accounts: DESTINATION,
            derived: None,
        };
        let derived = find_program_address(&[SOURCE.as_bytes()], &program)
            .unwrap()
            .address();
        let expected = [
            AccountMeta {
                address: SOURCE,
                role: AccountRole::Readonly,
            },
            AccountMeta {
                address: program,
                role: AccountRole::Readonly,
            },
            AccountMeta {
                address: DESTINATION,
                role: AccountRole::Writable,
            },
            AccountMeta {
                address: derived,
                role: AccountRole::Readonly,
            },
        ];
        assert_eq!(inputs.resolve(program).unwrap(), expected);
        let invocation = inputs
            .instruction_with_program(program, &resolvers::SetRoles {}, LIMITS)
            .unwrap();
        assert_eq!(invocation.accounts, expected);
        resolvers::SetRoles::parse_with_program(program, &invocation, LIMITS).unwrap();

        reject_foreign_program(program, &invocation);
        overridden_roles(inputs, program);
    }
}

fn overridden_roles(inputs: resolvers::SetRolesAccounts, program: Address) {
    let overridden = resolvers::SetRolesAccounts {
        maybe: Some(OVERRIDE),
        derived: Some(DESTINATION),
        ..inputs
    };
    let accounts = overridden.resolve(program).unwrap();
    assert_eq!(
        accounts[1],
        AccountMeta {
            address: OVERRIDE,
            role: AccountRole::Writable
        }
    );
    assert_eq!(accounts[3].address, DESTINATION);
}

fn reject_foreign_program(program: Address, invocation: &Instruction) {
    let mut foreign = invocation.clone();
    foreign.program_address = OVERRIDE;
    assert!(matches!(
        resolvers::SetRoles::parse_with_program(program, &foreign, LIMITS),
        Err(Error::InvalidValue)
    ));
}

fn omitted_suffixes() {
    let empty = resolvers::OmitRolesAccounts {
        first: None,
        second: None,
    };
    assert!(empty.resolve(OTHER_PROGRAM).unwrap().is_empty());
    let one = resolvers::OmitRolesAccounts {
        first: Some(SOURCE),
        second: None,
    };
    let one_account = one.resolve(OTHER_PROGRAM).unwrap();
    assert_eq!(one_account.len(), 1);
    assert_eq!(one_account[0].address, SOURCE);
    one.instruction_with_program(OTHER_PROGRAM, &resolvers::OmitRoles {}, LIMITS)
        .unwrap();
    let both = resolvers::OmitRolesAccounts {
        first: Some(SOURCE),
        second: Some(DESTINATION),
    };
    assert_eq!(both.resolve(OTHER_PROGRAM).unwrap().len(), 2);
    let hole = resolvers::OmitRolesAccounts {
        first: None,
        second: Some(DESTINATION),
    };
    assert_eq!(
        hole.resolve(OTHER_PROGRAM),
        Err(resolvers::DefaultError::Codec(Error::InvalidValue))
    );
}

fn literal_defaults() {
    let default = literal::SetLiteralAccounts {
        system_program: None,
    };
    let expected = [AccountMeta {
        address: literal::PROGRAM,
        role: AccountRole::Readonly,
    }];
    assert_eq!(default.resolve(OTHER_PROGRAM).unwrap(), expected);
    let invocation = default
        .instruction_with_program(OTHER_PROGRAM, &literal::SetLiteral {}, LIMITS)
        .unwrap();
    assert_eq!(invocation.accounts, expected);
    literal::SetLiteral::parse_with_program(OTHER_PROGRAM, &invocation, LIMITS).unwrap();
    let overridden = literal::SetLiteralAccounts {
        system_program: Some(OVERRIDE),
    };
    assert_eq!(
        overridden.resolve(OTHER_PROGRAM).unwrap()[0].address,
        OVERRIDE
    );
}

fn helper_names_and_empty_roles() {
    let first = find_program_address(&[b"one"], &resolvers::PROGRAM).unwrap();
    let second = find_program_address(&[b"two"], &resolvers::PROGRAM).unwrap();
    assert_eq!(resolvers::find_x().unwrap(), first);
    assert_eq!(resolvers::find_x_with_program_pda().unwrap(), second);
    assert_eq!(
        resolvers::find_by_program_with_program(OTHER_PROGRAM, SOURCE).unwrap(),
        find_program_address(&[SOURCE.as_bytes()], &OTHER_PROGRAM).unwrap()
    );

    let inputs = empty::SetValueAccounts {};
    assert!(inputs.resolve(OTHER_PROGRAM).unwrap().is_empty());
    let data = empty::SetValue { value: VALUE };
    let canonical = data.instruction(LIMITS).unwrap();
    assert_eq!(canonical.program_address, empty::PROGRAM);
    let invocation = inputs
        .instruction_with_program(OTHER_PROGRAM, &data, LIMITS)
        .unwrap();
    assert_eq!(invocation.program_address, OTHER_PROGRAM);
    assert!(invocation.accounts.is_empty());
    assert_eq!(invocation.data, ACCOUNT_WIRE);
    assert_eq!(
        empty::find_constant_with_program(OTHER_PROGRAM).unwrap(),
        find_program_address(&[b"empty"], &OTHER_PROGRAM).unwrap()
    );
}

pub(super) fn run() {
    record_predicates();
    positional_optionals();
    omitted_suffixes();
    literal_defaults();
    helper_names_and_empty_roles();
}
