//! Native-only oracle: official enum serialization never enters an SDK graph.

use solana_address::Address as OfficialAddress;
use solana_kit_core::{
    Address,
    codecs::{Decoder, Encoder, Error, Limits},
};
use solana_kit_system::*;
use solana_system_interface::instruction::SystemInstruction as Official;

const LIMITS: Limits = Limits::new(4096, 4096);
const AMOUNT: u64 = u64::MAX;
const SPACE: u64 = 513;
const OWNER: Address = Address::from_bytes([4; 32]);
const BASE: Address = Address::from_bytes([3; 32]);

fn owner() -> OfficialAddress {
    OfficialAddress::new_from_array(OWNER.to_bytes())
}

fn base() -> OfficialAddress {
    OfficialAddress::new_from_array(BASE.to_bytes())
}

fn ordinary_pairs() -> Vec<(Official, SystemInstructionData)> {
    vec![
        (
            Official::CreateAccount {
                lamports: AMOUNT,
                space: SPACE,
                owner: owner(),
            },
            SystemInstructionData::CreateAccount(CreateAccount {
                lamports: AMOUNT,
                space: SPACE,
                program_address: OWNER,
            }),
        ),
        (
            Official::Assign { owner: owner() },
            SystemInstructionData::Assign(Assign {
                program_address: OWNER,
            }),
        ),
        (
            Official::Transfer { lamports: AMOUNT },
            SystemInstructionData::TransferSol(TransferSol { amount: AMOUNT }),
        ),
    ]
}

fn nonce_and_allocation_pairs() -> Vec<(Official, SystemInstructionData)> {
    vec![
        (
            Official::AdvanceNonceAccount,
            SystemInstructionData::AdvanceNonceAccount(AdvanceNonceAccount {}),
        ),
        (
            Official::WithdrawNonceAccount(AMOUNT),
            SystemInstructionData::WithdrawNonceAccount(WithdrawNonceAccount {
                withdraw_amount: AMOUNT,
            }),
        ),
        (
            Official::InitializeNonceAccount(owner()),
            SystemInstructionData::InitializeNonceAccount(InitializeNonceAccount {
                nonce_authority: OWNER,
            }),
        ),
        (
            Official::AuthorizeNonceAccount(owner()),
            SystemInstructionData::AuthorizeNonceAccount(AuthorizeNonceAccount {
                new_nonce_authority: OWNER,
            }),
        ),
        (
            Official::Allocate { space: SPACE },
            SystemInstructionData::Allocate(Allocate { space: SPACE }),
        ),
        (
            Official::UpgradeNonceAccount,
            SystemInstructionData::UpgradeNonceAccount(UpgradeNonceAccount {}),
        ),
        (
            Official::CreateAccountAllowPrefund {
                lamports: AMOUNT,
                space: SPACE,
                owner: owner(),
            },
            SystemInstructionData::CreateAccountAllowPrefund(CreateAccountAllowPrefund {
                lamports: AMOUNT,
                space: SPACE,
                program_address: OWNER,
            }),
        ),
    ]
}

fn seeded_pairs(seed: &str) -> Vec<(Official, SystemInstructionData)> {
    let seed = seed.to_owned();

    vec![
        (
            Official::CreateAccountWithSeed {
                base: base(),
                seed: seed.clone(),
                lamports: AMOUNT,
                space: SPACE,
                owner: owner(),
            },
            SystemInstructionData::CreateAccountWithSeed(CreateAccountWithSeed {
                base: BASE,
                seed: seed.clone(),
                amount: AMOUNT,
                space: SPACE,
                program_address: OWNER,
            }),
        ),
        (
            Official::AllocateWithSeed {
                base: base(),
                seed: seed.clone(),
                space: SPACE,
                owner: owner(),
            },
            SystemInstructionData::AllocateWithSeed(AllocateWithSeed {
                base: BASE,
                seed: seed.clone(),
                space: SPACE,
                program_address: OWNER,
            }),
        ),
    ]
}

fn assignment_and_transfer_pairs(seed: &str) -> Vec<(Official, SystemInstructionData)> {
    let seed = seed.to_owned();

    vec![
        (
            Official::AssignWithSeed {
                base: base(),
                seed: seed.clone(),
                owner: owner(),
            },
            SystemInstructionData::AssignWithSeed(AssignWithSeed {
                base: BASE,
                seed: seed.clone(),
                program_address: OWNER,
            }),
        ),
        (
            Official::TransferWithSeed {
                lamports: AMOUNT,
                from_seed: seed.clone(),
                from_owner: owner(),
            },
            SystemInstructionData::TransferSolWithSeed(TransferSolWithSeed {
                amount: AMOUNT,
                from_seed: seed,
                from_owner: OWNER,
            }),
        ),
    ]
}

#[test]
fn all_fourteen_official_variants_match_and_strictly_roundtrip() {
    for seed in ["", "é", "a\0🦀", "012345678901234567890123456789012"] {
        let pairs = ordinary_pairs()
            .into_iter()
            .chain(nonce_and_allocation_pairs())
            .chain(seeded_pairs(seed))
            .chain(assignment_and_transfer_pairs(seed));

        for (official, value) in pairs {
            let expected = bincode::serialize(&official).expect("official bytes");
            let actual = SystemInstructionDataCodec
                .encode(&value, LIMITS.max_bytes)
                .expect("bounded encode");
            assert_eq!(actual, expected);
            assert_eq!(
                SystemInstructionDataCodec.encoded_len(&value),
                Ok(expected.len())
            );
            assert_eq!(
                SystemInstructionDataCodec.decode(&expected, LIMITS),
                Ok(value.clone())
            );
            assert_eq!(
                bincode::deserialize::<Official>(&actual).expect("official decode"),
                official
            );

            assert_eq!(
                SystemInstructionDataCodec.encode(&value, actual.len() - 1),
                Err(Error::LimitExceeded)
            );

            for end in 0..expected.len() {
                assert!(
                    SystemInstructionDataCodec
                        .decode(&expected[..end], LIMITS)
                        .is_err()
                );
            }

            let mut trailing = actual;
            trailing.push(0);
            assert_eq!(
                SystemInstructionDataCodec.decode(&trailing, LIMITS),
                Err(Error::TrailingBytes)
            );
        }
    }
}

#[test]
fn unsupported_tags_and_string_limits_fail() {
    for tag in [14u32, 255, u32::MAX] {
        assert_eq!(
            SystemInstructionDataCodec.decode(&tag.to_le_bytes(), LIMITS),
            Err(Error::InvalidDiscriminator)
        );
    }

    let mut bytes = bincode::serialize(&seeded_pairs("é")[0].0).expect("official seeded bytes");
    assert_eq!(
        SystemInstructionDataCodec.decode(&bytes, Limits::new(LIMITS.max_bytes, 1)),
        Err(Error::LimitExceeded)
    );
    bytes[44] = 255;
    assert_eq!(
        SystemInstructionDataCodec.decode(&bytes, LIMITS),
        Err(Error::InvalidUtf8)
    );
    bytes[36..44].copy_from_slice(&u64::MAX.to_le_bytes());
    assert!(SystemInstructionDataCodec.decode(&bytes, LIMITS).is_err());
}
