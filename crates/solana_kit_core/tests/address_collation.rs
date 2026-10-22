//! Literal pinned Dart compiler outputs, including case-sensitive address order.

use solana_kit_core::{
    AccountMeta, AccountRole, Address, Hash, Instruction,
    transaction::{Lifetime, LookupTable, Message, Version},
};

const UPGRADED_SIGNER: u8 = 7;
const READONLY_SIGNER: u8 = 25;
const SHARED_LOOKUP: u8 = 17;
const WRITABLE_LOOKUP: u8 = 50;
const READONLY_BYTE_TABLE: u8 = 99;
const READONLY_SYSVAR_TABLE: u8 = 177;

fn address(byte: u8) -> Address {
    Address::from_bytes([byte; 32])
}

fn sysvar() -> Address {
    "SysvarRecentB1ockHashes11111111111111111111"
        .parse()
        .unwrap()
}

fn meta(byte: u8, role: AccountRole) -> AccountMeta {
    AccountMeta {
        address: address(byte),
        role,
    }
}

fn instructions(scenario: usize) -> Vec<Instruction> {
    use AccountRole::{Readonly, ReadonlySigner, Writable, WritableSigner};

    if scenario == 0 {
        return vec![
            Instruction {
                program_address: address(9),
                accounts: vec![],
                data: vec![],
            },
            Instruction {
                program_address: sysvar(),
                accounts: vec![],
                data: vec![],
            },
        ];
    }

    vec![
        Instruction {
            program_address: address(9),
            accounts: vec![
                meta(1, ReadonlySigner),
                meta(UPGRADED_SIGNER, ReadonlySigner),
                meta(READONLY_SIGNER, ReadonlySigner),
                meta(4, Readonly),
                meta(SHARED_LOOKUP, Readonly),
                meta(WRITABLE_LOOKUP, Writable),
                meta(UPGRADED_SIGNER, ReadonlySigner),
            ],
            data: vec![1, 2],
        },
        Instruction {
            program_address: sysvar(),
            accounts: vec![
                meta(UPGRADED_SIGNER, WritableSigner),
                meta(4, Writable),
                meta(SHARED_LOOKUP, Writable),
                meta(READONLY_BYTE_TABLE, Readonly),
                meta(READONLY_SYSVAR_TABLE, Readonly),
            ],
            data: vec![3],
        },
    ]
}

fn tables(version: Version, scenario: usize) -> Vec<LookupTable> {
    if version != Version::V0 || scenario != 2 {
        return vec![];
    }

    vec![
        LookupTable {
            address: sysvar(),
            addresses: vec![
                address(WRITABLE_LOOKUP),
                address(SHARED_LOOKUP),
                address(READONLY_SYSVAR_TABLE),
            ],
        },
        LookupTable {
            address: address(9),
            addresses: vec![
                address(READONLY_BYTE_TABLE),
                address(SHARED_LOOKUP),
                address(SHARED_LOOKUP),
            ],
        },
    ]
}

fn compile(version: Version, scenario: usize, reverse: bool) -> Message {
    let mut tables = tables(version, scenario);

    if reverse {
        tables.reverse();
    }

    Message::compile(
        version,
        address(1),
        Lifetime::Blockhash {
            blockhash: Hash::from_bytes([8; 32]),
            last_valid_block_height: 99,
        },
        &instructions(scenario),
        &tables,
    )
    .unwrap()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[test]
fn pinned_dart_full_compiler_messages() {
    for row in include_str!("fixtures/address_collation.tsv").lines() {
        let fields: Vec<_> = row.split('\t').collect();

        let version = match fields[1] {
            "legacy" => Version::Legacy,
            "v0" => Version::V0,
            "v1" => Version::V1,
            _ => panic!("unknown fixture version"),
        };

        let scenario: usize = fields[2].parse().unwrap();
        let message = compile(version, scenario, false);
        let accounts: Vec<_> = message
            .static_accounts()
            .iter()
            .map(ToString::to_string)
            .collect();

        assert_eq!(
            accounts.join(","),
            fields[3],
            "{} scenario {scenario}",
            fields[1]
        );
        assert_eq!(
            hex(message.bytes()),
            fields[4],
            "{} scenario {scenario}",
            fields[1]
        );
        assert_eq!(compile(version, scenario, true), message);
        assert_eq!(
            Message::decode(message.bytes()).unwrap().bytes(),
            message.bytes()
        );
    }
}
