//! Public client contracts and malformed input boundaries.
use solana_kit_core::{
    AccountRole, Address,
    accounts::Account,
    codecs::{Decoder, Encoder, Error, Limits},
};
use solana_kit_system::*;

const SOURCE: Address = Address::from_bytes([1; 32]);
const DESTINATION: Address = Address::from_bytes([2; 32]);
const NONCE_SPACE: u64 = 80;
const UNKNOWN_CODE: u32 = 9;
const LARGE_UNKNOWN: u32 = 256;
const LIMITS: Limits = Limits::new(4096, 4096);

#[test]
fn strict_parser_rejects_program_role_counts_data_and_budget() {
    let data = TransferSol { amount: u64::MAX };
    let instruction = data
        .instruction(
            TransferSolAccounts {
                source: SOURCE,
                destination: DESTINATION,
            },
            LIMITS,
        )
        .unwrap();
    assert_eq!(
        parse_instruction(&instruction, LIMITS).unwrap(),
        SystemInstructionData::TransferSol(data.clone())
    );
    assert_eq!(TransferSol::parse(&instruction, LIMITS).unwrap().0, data);
    let mut malformed = instruction.clone();
    malformed.program_address = SOURCE;
    assert_eq!(
        parse_instruction(&malformed, LIMITS),
        Err(ClientError::ProgramMismatch)
    );
    malformed = instruction.clone();

    malformed.accounts[0].role = AccountRole::Writable;
    assert_eq!(
        parse_instruction(&malformed, LIMITS),
        Err(ClientError::AccountRole(0))
    );
}

#[test]
fn parser_rejects_account_count_tag_trailing_and_budget() {
    let instruction = TransferSol { amount: u64::MAX }
        .instruction(
            TransferSolAccounts {
                source: SOURCE,
                destination: DESTINATION,
            },
            LIMITS,
        )
        .unwrap();
    let mut malformed = instruction.clone();
    malformed.accounts.pop();
    assert_eq!(
        parse_instruction(&malformed, LIMITS),
        Err(ClientError::AccountCount)
    );
    malformed = instruction.clone();
    malformed.accounts.push(malformed.accounts[0]);
    assert_eq!(
        parse_instruction(&malformed, LIMITS),
        Err(ClientError::AccountCount)
    );
    malformed = instruction.clone();

    malformed.data.push(0);
    assert_eq!(
        parse_instruction(&malformed, LIMITS),
        Err(ClientError::Codec(Error::TrailingBytes))
    );
    assert!(parse_instruction(&instruction, Limits::new(11, 0)).is_err());
    assert!(identify_instruction(&[0; 3]).is_err());
    assert!(identify_instruction(&14u32.to_le_bytes()).is_err());
    assert!(Allocate::parse(&instruction, LIMITS).is_err());
}

#[test]
fn builders_enforce_seed_base_payer_and_fixed_sysvars() {
    let mut data = CreateAccountWithSeed {
        base: SOURCE,
        seed: "é".repeat(16),
        amount: 1,
        space: NONCE_SPACE,
        program_address: DESTINATION,
    };
    let accounts = CreateAccountWithSeedAccounts {
        payer: SOURCE,
        new_account: DESTINATION,
        base_account: None,
    };
    assert!(data.instruction(accounts, LIMITS).is_ok());
    data.seed.push('é');
    assert_eq!(
        data.instruction(accounts, LIMITS),
        Err(ClientError::SeedTooLong)
    );
    data.seed.clear();
    data.base = DESTINATION;
    assert_eq!(
        data.instruction(accounts, LIMITS),
        Err(ClientError::BaseMismatch)
    );

    assert_eq!(
        CreateAccountAllowPrefund {
            lamports: 1,
            space: NONCE_SPACE,
            program_address: DESTINATION
        }
        .instruction(
            CreateAccountAllowPrefundAccounts {
                new_account: SOURCE,
                payer: None
            },
            LIMITS
        ),
        Err(ClientError::MissingPayer)
    );
}

#[test]
fn nonce_builder_uses_fixed_addresses() {
    let mut nonce = AdvanceNonceAccount {}
        .instruction(
            AdvanceNonceAccountAccounts {
                nonce_account: SOURCE,
                nonce_authority: DESTINATION,
            },
            LIMITS,
        )
        .unwrap();
    assert_eq!(nonce.accounts[1].address, RECENT_BLOCKHASHES);
    nonce.accounts[1].address = DESTINATION;
    assert_eq!(
        parse_instruction(&nonce, LIMITS),
        Err(ClientError::AccountAddress(1))
    );
    assert_eq!(
        SYSTEM_PROGRAM.to_string(),
        "11111111111111111111111111111111"
    );
    assert_eq!(
        RENT.to_string(),
        "SysvarRent111111111111111111111111111111111"
    );
    assert_eq!(
        RECENT_BLOCKHASHES.to_string(),
        "SysvarRecentB1ockHashes11111111111111111111"
    );
}

#[test]
fn nonce_preserves_storage_metadata_and_rejects_bad_wire() {
    let nonce = Nonce {
        version: NonceVersion::Legacy,
        state: NonceState::Uninitialized([91; 72]),
    };
    let bytes = NonceCodec.encode(&nonce, NONCE_SIZE).unwrap();
    assert_eq!(
        NonceCodec
            .decode(&bytes, Limits::new(NONCE_SIZE, 0))
            .unwrap(),
        nonce
    );
    assert!(nonce.durable_data().is_none());
    let mut invalid = bytes.clone();
    invalid[4] = 2;
    assert_eq!(
        NonceCodec.decode(&invalid, LIMITS),
        Err(Error::InvalidDiscriminator)
    );
    invalid = bytes.clone();

    invalid[0] = 2;
    assert_eq!(
        NonceCodec.decode(&invalid, LIMITS),
        Err(Error::InvalidDiscriminator)
    );
    invalid = bytes.clone();
    invalid.push(0);
    assert_eq!(
        NonceCodec.decode(&invalid, LIMITS),
        Err(Error::TrailingBytes)
    );
    assert!(NonceCodec.decode(&bytes, Limits::new(79, 0)).is_err());
}

#[test]
fn nonce_account_preserves_metadata_and_checks_owner_space() {
    let nonce = Nonce {
        version: NonceVersion::Legacy,
        state: NonceState::Uninitialized([91; 72]),
    };
    let bytes = NonceCodec.encode(&nonce, NONCE_SIZE).unwrap();
    let account = Account {
        address: SOURCE,
        owner: SYSTEM_PROGRAM,
        lamports: u64::MAX,
        executable: false,
        space: NONCE_SPACE,
        rent_epoch: Some(7),
        data: bytes,
    };
    let decoded = decode_nonce(account.clone(), LIMITS).unwrap();
    assert_eq!(decoded.data, nonce);
    assert_eq!(decoded.lamports, u64::MAX);
    assert_eq!(decoded.rent_epoch, Some(7));
    let mut wrong = account.clone();

    wrong.owner = SOURCE;
    assert!(decode_nonce(wrong, LIMITS).is_err());
    wrong = account;
    wrong.space = NONCE_SPACE + 1;
    assert!(decode_nonce(wrong, LIMITS).is_err());
}

#[test]
fn error_numbers_and_unknown_codes_remain_exact() {
    for code in 0..9 {
        let error = SystemError::from_code(code);
        assert_eq!(error.code(), code);
        assert!(error.message().is_some());
    }

    for code in [UNKNOWN_CODE, LARGE_UNKNOWN, u32::MAX] {
        assert_eq!(SystemError::from_code(code), SystemError::Unknown(code));
        assert_eq!(SystemError::from_code(code).code(), code);
        assert_eq!(SystemError::from_code(code).message(), None);
    }
}

#[test]
fn explicit_optional_duplicate_positions_remain_present() {
    let data = CreateAccountWithSeed {
        base: SOURCE,
        seed: "é".into(),
        amount: 1,
        space: NONCE_SPACE,
        program_address: DESTINATION,
    };
    let accounts = CreateAccountWithSeedAccounts {
        payer: SOURCE,
        new_account: DESTINATION,
        base_account: Some(SOURCE),
    };
    let instruction = data.instruction(accounts, LIMITS).unwrap();
    assert_eq!(instruction.accounts.len(), 3);
    assert_eq!(
        instruction.accounts[0].address,
        instruction.accounts[2].address
    );
    assert_eq!(instruction.accounts[2].role, AccountRole::ReadonlySigner);
    assert_eq!(
        CreateAccountWithSeed::parse(&instruction, LIMITS).unwrap(),
        (data, accounts)
    );
}
