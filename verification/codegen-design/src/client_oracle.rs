//! Independent official constructors compare bytes, positions and privileges.

use solana_address::Address as OfficialAddress;
use solana_instruction::Instruction as OfficialInstruction;
use solana_kit_core::{AccountRole, Address, Instruction, codecs::Limits};
use solana_kit_system::*;
use solana_system_interface::instruction as official;

const NONCE_SPACE: u64 = 80;
const ACCOUNT_SPACE: u64 = 513;
const LIMITS: Limits = Limits::new(4096, 4096);
const SOURCE: Address = Address::from_bytes([1; 32]);
const DESTINATION: Address = Address::from_bytes([2; 32]);
const OWNER: Address = Address::from_bytes([3; 32]);
fn address(value: Address) -> OfficialAddress {
    OfficialAddress::new_from_array(value.to_bytes())
}

fn compare(local: Instruction, official: OfficialInstruction) {
    assert!(parse_instruction(&local, LIMITS).is_ok());
    assert_eq!(
        local.program_address.to_bytes(),
        official.program_id.to_bytes()
    );
    assert_eq!(local.data, official.data);
    assert_eq!(local.accounts.len(), official.accounts.len());
    reject_contract_mutations(&local);

    for (local, official) in local.accounts.iter().zip(&official.accounts) {
        assert_eq!(local.address.to_bytes(), official.pubkey.to_bytes());
        assert_eq!(
            local.role,
            AccountRole::from_privileges(official.is_signer, official.is_writable)
        );
    }
}

/// Every constructor exercises the common parser with wrong programs and positional roles.
fn reject_contract_mutations(instruction: &Instruction) {
    let mut invalid = instruction.clone();
    invalid.program_address = SOURCE;
    assert!(parse_instruction(&invalid, LIMITS).is_err());

    for index in 0..instruction.accounts.len() {
        let mut invalid = instruction.clone();
        let role = invalid.accounts[index].role;
        invalid.accounts[index].role =
            AccountRole::from_privileges(!role.is_signer(), role.is_writable());
        assert!(parse_instruction(&invalid, LIMITS).is_err());
    }
}

#[test]
fn all_fourteen_builders_match_official_constructors() {
    for amount in [0, 1, u64::MAX] {
        ordinary_0(amount);
        ordinary_1(amount);
        ordinary_2(amount);
        ordinary_3(amount);
        ordinary_4(amount);

        for seed in ["", "é\0🌻", "01234567890123456789012345678901"] {
            seeded_0(amount, seed);

            seeded_1(amount, seed);
        }
    }

    optional_accounts();
}
fn ordinary_0(amount: u64) {
    let (a, b, c) = (address(SOURCE), address(DESTINATION), address(OWNER));
    compare(
        CreateAccount {
            lamports: amount,
            space: ACCOUNT_SPACE,
            program_address: OWNER,
        }
        .instruction(
            CreateAccountAccounts {
                payer: SOURCE,
                new_account: DESTINATION,
            },
            LIMITS,
        )
        .unwrap(),
        official::create_account(&a, &b, amount, ACCOUNT_SPACE, &c),
    );
    compare(
        Assign {
            program_address: OWNER,
        }
        .instruction(AssignAccounts { account: SOURCE }, LIMITS)
        .unwrap(),
        official::assign(&a, &c),
    );
}
fn ordinary_1(amount: u64) {
    let (a, b, _c) = (address(SOURCE), address(DESTINATION), address(OWNER));
    compare(
        TransferSol { amount }
            .instruction(
                TransferSolAccounts {
                    source: SOURCE,
                    destination: DESTINATION,
                },
                LIMITS,
            )
            .unwrap(),
        official::transfer(&a, &b, amount),
    );
    compare(
        AdvanceNonceAccount {}
            .instruction(
                AdvanceNonceAccountAccounts {
                    nonce_account: SOURCE,
                    nonce_authority: DESTINATION,
                },
                LIMITS,
            )
            .unwrap(),
        official::advance_nonce_account(&a, &b),
    );
}
fn ordinary_2(amount: u64) {
    let (a, b, c) = (address(SOURCE), address(DESTINATION), address(OWNER));
    compare(
        WithdrawNonceAccount {
            withdraw_amount: amount,
        }
        .instruction(
            WithdrawNonceAccountAccounts {
                nonce_account: SOURCE,
                recipient_account: OWNER,
                nonce_authority: DESTINATION,
            },
            LIMITS,
        )
        .unwrap(),
        official::withdraw_nonce_account(&a, &b, &c, amount),
    );
    compare(
        InitializeNonceAccount {
            nonce_authority: OWNER,
        }
        .instruction(
            InitializeNonceAccountAccounts {
                nonce_account: DESTINATION,
            },
            LIMITS,
        )
        .unwrap(),
        official::create_nonce_account(&a, &b, &c, amount).remove(1),
    );
}
fn ordinary_3(amount: u64) {
    let (a, b, c) = (address(SOURCE), address(DESTINATION), address(OWNER));
    compare(
        AuthorizeNonceAccount {
            new_nonce_authority: OWNER,
        }
        .instruction(
            AuthorizeNonceAccountAccounts {
                nonce_account: SOURCE,
                nonce_authority: DESTINATION,
            },
            LIMITS,
        )
        .unwrap(),
        official::authorize_nonce_account(&a, &b, &c),
    );
    compare(
        Allocate { space: amount }
            .instruction(
                AllocateAccounts {
                    new_account: SOURCE,
                },
                LIMITS,
            )
            .unwrap(),
        official::allocate(&a, amount),
    );
}
fn ordinary_4(amount: u64) {
    let (a, b, c) = (address(SOURCE), address(DESTINATION), address(OWNER));
    compare(
        UpgradeNonceAccount {}
            .instruction(
                UpgradeNonceAccountAccounts {
                    nonce_account: SOURCE,
                },
                LIMITS,
            )
            .unwrap(),
        official::upgrade_nonce_account(a),
    );
    compare(
        CreateAccountAllowPrefund {
            lamports: amount,
            space: ACCOUNT_SPACE,
            program_address: OWNER,
        }
        .instruction(
            CreateAccountAllowPrefundAccounts {
                new_account: DESTINATION,
                payer: Some(SOURCE),
            },
            LIMITS,
        )
        .unwrap(),
        official::create_account_allow_prefund(&b, Some((&a, amount)), ACCOUNT_SPACE, &c),
    );
}
fn seeded_0(amount: u64, seed: &str) {
    let (a, b, c) = (address(SOURCE), address(DESTINATION), address(OWNER));
    compare(
        CreateAccountWithSeed {
            base: OWNER,
            seed: seed.into(),
            amount,
            space: ACCOUNT_SPACE,
            program_address: DESTINATION,
        }
        .instruction(
            CreateAccountWithSeedAccounts {
                payer: SOURCE,
                new_account: DESTINATION,
                base_account: Some(OWNER),
            },
            LIMITS,
        )
        .unwrap(),
        official::create_account_with_seed(&a, &b, &c, seed, amount, ACCOUNT_SPACE, &b),
    );
    compare(
        AllocateWithSeed {
            base: OWNER,
            seed: seed.into(),
            space: amount,
            program_address: DESTINATION,
        }
        .instruction(
            AllocateWithSeedAccounts {
                new_account: SOURCE,
                base_account: OWNER,
            },
            LIMITS,
        )
        .unwrap(),
        official::allocate_with_seed(&a, &c, seed, amount, &b),
    );
}
fn seeded_1(amount: u64, seed: &str) {
    let (a, b, c) = (address(SOURCE), address(DESTINATION), address(OWNER));
    compare(
        AssignWithSeed {
            base: OWNER,
            seed: seed.into(),
            program_address: DESTINATION,
        }
        .instruction(
            AssignWithSeedAccounts {
                account: SOURCE,
                base_account: OWNER,
            },
            LIMITS,
        )
        .unwrap(),
        official::assign_with_seed(&a, &c, seed, &b),
    );
    compare(
        TransferSolWithSeed {
            amount,
            from_seed: seed.into(),
            from_owner: OWNER,
        }
        .instruction(
            TransferSolWithSeedAccounts {
                source: SOURCE,
                base_account: OWNER,
                destination: DESTINATION,
            },
            LIMITS,
        )
        .unwrap(),
        official::transfer_with_seed(&a, &c, seed.into(), &c, &b, amount),
    );
}
fn optional_accounts() {
    let (a, b, c) = (address(SOURCE), address(DESTINATION), address(OWNER));
    compare(
        CreateAccountAllowPrefund {
            lamports: 0,
            space: NONCE_SPACE,
            program_address: OWNER,
        }
        .instruction(
            CreateAccountAllowPrefundAccounts {
                new_account: DESTINATION,
                payer: None,
            },
            LIMITS,
        )
        .unwrap(),
        official::create_account_allow_prefund(&b, None, NONCE_SPACE, &c),
    );
    compare(
        CreateAccountWithSeed {
            base: SOURCE,
            seed: "é".into(),
            amount: 1,
            space: NONCE_SPACE,
            program_address: OWNER,
        }
        .instruction(
            CreateAccountWithSeedAccounts {
                payer: SOURCE,
                new_account: DESTINATION,
                base_account: None,
            },
            LIMITS,
        )
        .unwrap(),
        official::create_account_with_seed(&a, &b, &a, "é", 1, NONCE_SPACE, &c),
    );
    let duplicates = TransferSol { amount: 1 }
        .instruction(
            TransferSolAccounts {
                source: SOURCE,
                destination: SOURCE,
            },
            LIMITS,
        )
        .unwrap();
    compare(duplicates, official::transfer(&a, &a, 1));
}

#[test]
fn nonce_matches_official_legacy_current_serialization_and_prefix_difference() {
    for version in [NonceVersion::Legacy, NonceVersion::Current] {
        for initialized in [false, true] {
            compare_nonce(version, initialized);
        }
    }
}

fn compare_nonce(version: NonceVersion, initialized: bool) {
    use solana_kit_core::codecs::{Decoder, Encoder};
    use solana_nonce::versions::Versions;
    const AUTHORITY_END: usize = 40;
    const HASH_END: usize = 72;
    let limits = Limits::new(NONCE_SIZE, 0);
    let mut bytes = [0u8; NONCE_SIZE];
    bytes[..4].copy_from_slice(&(version as u32).to_le_bytes());
    bytes[4..8].copy_from_slice(&u32::from(initialized).to_le_bytes());
    bytes[8..AUTHORITY_END].fill(37);
    bytes[AUTHORITY_END..HASH_END].fill(91);

    bytes[HASH_END..].copy_from_slice(&u64::MAX.to_le_bytes());

    let official: Versions = bincode::deserialize(&bytes).unwrap();
    let local = NonceCodec.decode(&bytes, limits).unwrap();
    assert_eq!(local.version, version);
    assert_eq!(NonceCodec.encode(&local, NONCE_SIZE).unwrap(), bytes);
    compare_nonce_state(local.state, &official);
    let official_bytes = bincode::serialize(&official).unwrap();
    assert_eq!(official_bytes, bytes[..official_bytes.len()]);
    assert_eq!(
        local.durable_data().is_some(),
        initialized && version == NonceVersion::Current
    );

    for end in 0..NONCE_SIZE {
        assert!(NonceCodec.decode(&bytes[..end], limits).is_err());
    }
}

fn compare_nonce_state(local: NonceState, official: &solana_nonce::versions::Versions) {
    use solana_nonce::state::State;

    match (local, official.state()) {
        (NonceState::Initialized(local), State::Initialized(official)) => {
            assert_eq!(local.authority.to_bytes(), official.authority.to_bytes());
            assert_eq!(local.blockhash.to_bytes(), official.blockhash().to_bytes());
            assert_eq!(
                local.lamports_per_signature,
                official.get_lamports_per_signature()
            );
            assert_eq!(bincode::serialize(&official).unwrap().len(), NONCE_SIZE - 8);
        }
        (NonceState::Uninitialized(_), State::Uninitialized) => {
            assert_eq!(bincode::serialize(&official).unwrap().len(), 8);
        }
        _ => panic!("nonce state disagreement"),
    }
}
