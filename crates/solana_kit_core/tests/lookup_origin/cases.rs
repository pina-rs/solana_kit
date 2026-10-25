//! Public compiler callers shared with portable verification; execution is separate.
extern crate alloc;

use alloc::{vec, vec::Vec};
use solana_kit_core::{
    AccountLookupMeta, AccountMeta, AccountRole, Address, Hash, Instruction, InstructionAccount,
    InstructionWithAccounts, LookupRole,
    transaction::{Error, Lifetime, LookupTable, Message, Version},
};

const INSTRUCTION_TAG: u8 = 42;
const UNUSED_ADDRESS: u8 = 5;
const LATE_PROGRAM: u8 = 6;
const NONCE_PAYER: u8 = 9;

fn address(byte: u8) -> Address {
    Address::from_bytes([byte; 32])
}

fn lifetime() -> Lifetime {
    Lifetime::Blockhash {
        blockhash: Hash::from_bytes([9; 32]),
        last_valid_block_height: 10,
    }
}

fn lookup(table: u8, index: u8) -> AccountLookupMeta {
    AccountLookupMeta {
        address: address(2),
        lookup_table_address: address(table),
        address_index: index,
        role: LookupRole::Readonly,
    }
}

fn instruction(accounts: Vec<InstructionAccount>) -> InstructionWithAccounts<InstructionAccount> {
    InstructionWithAccounts {
        program_address: address(1),
        accounts,
        data: vec![INSTRUCTION_TAG],
    }
}

fn compile(accounts: Vec<InstructionAccount>, tables: &[LookupTable]) -> Result<Message, Error> {
    Message::compile_with_lookup_accounts(
        Version::V0,
        address(0),
        lifetime(),
        &[instruction(accounts)],
        tables,
    )
}

fn tables() -> Vec<LookupTable> {
    vec![
        LookupTable {
            address: address(3),
            addresses: vec![address(2)],
        },
        LookupTable {
            address: address(4),
            addresses: vec![address(UNUSED_ADDRESS), address(2)],
        },
    ]
}

/// Exact selected origin survives heterogeneous privilege merging and wire decode.
pub fn explicit_origin_wire() {
    let hint = lookup(4, 1);
    let writable = AccountMeta {
        address: address(2),
        role: AccountRole::Writable,
    };
    let message = compile(vec![hint.into(), writable.into(), hint.into()], &tables()).unwrap();
    let mut expected = vec![128, 1, 0, 1, 2];
    expected.extend_from_slice(&[0; 32]);
    expected.extend_from_slice(&[1; 32]);
    expected.extend_from_slice(&[9; 32]);
    expected.extend_from_slice(&[1, 1, 3, 2, 2, 2, 1, 42, 1]);
    expected.extend_from_slice(&[4; 32]);
    expected.extend_from_slice(&[1, 1, 0]);

    assert_eq!(message.bytes(), expected);
    assert_eq!(message.static_accounts(), &[address(0), address(1)]);
    assert_eq!(message.instructions()[0].accounts(), &[2, 2, 2]);

    let decoded = Message::decode(&expected).unwrap();
    assert_eq!(decoded.lookups()[0].address(), address(4));
    assert_eq!(decoded.lookups()[0].writable_indices(), &[1]);
    assert!(decoded.lookups()[0].readonly_indices().is_empty());

    for accounts in [
        vec![writable.into(), hint.into()],
        vec![hint.into(), writable.into()],
    ] {
        let message = compile(accounts, &tables()).unwrap();
        assert_eq!(message.lookups()[0].address(), address(4));
        assert_eq!(message.lookups()[0].writable_indices(), &[1]);
    }
}

/// Explicit origins cannot silently use another snapshot, table or index.
pub fn invalid_origins() {
    assert_eq!(
        compile(vec![lookup(4, 1).into()], &[]),
        Err(Error::InvalidLookup)
    );
    assert_eq!(
        compile(vec![lookup(4, 0).into()], &tables()),
        Err(Error::InvalidLookup)
    );
    assert_eq!(
        compile(vec![lookup(4, 2).into()], &tables()),
        Err(Error::InvalidLookup)
    );
    assert_eq!(
        compile(vec![lookup(3, 0).into(), lookup(4, 1).into()], &tables()),
        Err(Error::InvalidLookup)
    );
    let repeated = vec![LookupTable {
        address: address(4),
        addresses: vec![address(2), address(2)],
    }];
    assert_eq!(
        compile(vec![lookup(4, 0).into(), lookup(4, 1).into()], &repeated),
        Err(Error::InvalidLookup)
    );
    let message = compile(vec![lookup(4, 1).into()], &repeated).unwrap();
    assert_eq!(message.lookups()[0].readonly_indices(), &[1]);

    for version in [Version::Legacy, Version::V1] {
        assert_eq!(
            Message::compile_with_lookup_accounts(
                version,
                address(0),
                lifetime(),
                &[instruction(vec![lookup(4, 1).into()])],
                &[]
            ),
            Err(Error::InvalidLookup)
        );
    }
}

/// Signer, payer and invoked-program requirements cannot consume explicit hints.
pub fn static_requirements() {
    let signer = AccountMeta {
        address: address(2),
        role: AccountRole::ReadonlySigner,
    };

    for accounts in [
        vec![lookup(4, 1).into(), signer.into()],
        vec![signer.into(), lookup(4, 1).into()],
    ] {
        assert_eq!(compile(accounts, &tables()), Err(Error::InvalidLookup));
    }

    for byte in [0, 1] {
        let hint = AccountLookupMeta {
            address: address(byte),
            ..lookup(4, 1)
        };
        let table = LookupTable {
            address: address(4),
            addresses: vec![address(UNUSED_ADDRESS), address(byte)],
        };
        assert_eq!(
            compile(vec![hint.into()], &[table]),
            Err(Error::InvalidLookup)
        );
    }

    let hint = AccountLookupMeta {
        address: address(1),
        role: LookupRole::Writable,
        ..lookup(4, 1)
    };
    assert_eq!(
        compile(vec![hint.into()], &tables()),
        Err(Error::WritableProgram)
    );
    late_invoked_program();
}

fn late_invoked_program() {
    let hint = AccountLookupMeta {
        address: address(LATE_PROGRAM),
        ..lookup(4, 1)
    };
    let invoke = Instruction {
        program_address: address(LATE_PROGRAM),
        accounts: vec![],
        data: vec![],
    };
    let use_lookup = instruction(vec![hint.into()]);
    let table = LookupTable {
        address: address(4),
        addresses: vec![address(UNUSED_ADDRESS), address(LATE_PROGRAM)],
    };

    for instructions in [
        vec![use_lookup.clone(), invoke.clone().into()],
        vec![invoke.into(), use_lookup],
    ] {
        assert_eq!(
            Message::compile_with_lookup_accounts(
                Version::V0,
                address(0),
                lifetime(),
                &instructions,
                core::slice::from_ref(&table)
            ),
            Err(Error::InvalidLookup)
        );
    }
}

/// Table bounds are checked even when the selected index fits the wire domain.
pub fn lookup_bounds() {
    let mut table = LookupTable {
        address: address(4),
        addresses: vec![address(UNUSED_ADDRESS); 256],
    };
    table.addresses[255] = address(2);
    let hint = lookup(4, u8::MAX);
    let message = compile(vec![hint.into()], &[table.clone()]).unwrap();
    assert_eq!(message.lookups()[0].readonly_indices(), &[u8::MAX]);

    table.addresses.push(address(LATE_PROGRAM));
    assert_eq!(
        compile(vec![hint.into()], &[table]),
        Err(Error::InvalidLookup)
    );

    let duplicate = tables()[0].clone();
    assert_eq!(
        compile(vec![lookup(3, 0).into()], &[duplicate.clone(), duplicate]),
        Err(Error::InvalidLookup)
    );
}

/// Unannotated empty accounts and ordinary snapshot callers retain their entry point.
pub fn ordinary_source_compatibility() {
    context_free_literal();

    let empty_nonce = Instruction {
        program_address: address(1),
        accounts: Vec::new(),
        data: vec![],
    };
    assert_eq!(
        solana_kit_core::transaction::NonceAdvance::recognize(&empty_nonce),
        None
    );

    let empty = Instruction {
        program_address: address(1),
        accounts: Vec::new(),
        data: vec![],
    };
    assert!(Message::compile(Version::V0, address(0), lifetime(), &[empty], &[]).is_ok());
    assert!(Message::compile(Version::V0, address(0), lifetime(), &[], &[]).is_ok());

    let ordinary = Instruction {
        program_address: address(1),
        accounts: vec![AccountMeta {
            address: address(2),
            role: AccountRole::Readonly,
        }],
        data: vec![],
    };
    let message =
        Message::compile(Version::V0, address(0), lifetime(), &[ordinary], &tables()).unwrap();
    assert_eq!(message.lookups()[0].address(), address(3));
    assert_eq!(message.lookups()[0].readonly_indices(), &[0]);
    ordinary_builder_composition();
}

fn context_free_literal() {
    let ordinary = Instruction {
        program_address: address(1),
        accounts: Vec::new(),
        data: vec![],
    };

    assert!(ordinary.accounts.is_empty());
    assert!(ordinary.data.is_empty());
}

fn ordinary_builder_composition() {
    use solana_kit_core::transaction::DurableNonce;
    let nonce = DurableNonce::new(Hash::from_bytes([9; 32]), address(7), address(8));
    let advance = nonce.advance().instruction();
    let expected_accounts = advance.accounts.clone();
    let instructions = vec![advance.into(), instruction(vec![lookup(4, 1).into()])];
    assert_eq!(
        instructions[0]
            .accounts
            .iter()
            .map(|account| account.account_meta())
            .collect::<Vec<_>>(),
        expected_accounts
    );
    let message = Message::compile_with_lookup_accounts(
        Version::V0,
        address(NONCE_PAYER),
        Lifetime::DurableNonce(nonce),
        &instructions,
        &tables(),
    )
    .unwrap();
    assert_eq!(message.lookups()[0].address(), address(4));
    assert!(message.signers().contains(&address(8)));
}
