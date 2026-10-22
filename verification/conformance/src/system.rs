//! Generated System data cases executed by the combined portable suite.

use crate::{CaseResult, case};

use alloc::string::String;
use solana_kit_core::{
    Address,
    codecs::{Decoder, Encoder, Error, Limits},
};
use solana_kit_system::{
    CreateAccount, CreateAccountCodec, CreateAccountWithSeed, CreateAccountWithSeedCodec,
    SystemInstructionDataCodec,
};

const LIMITS: Limits = Limits::new(256, 64);
const LAMPORTS: u64 = 256;
const SPACE: u64 = 513;

/// Appends generated program data after the deterministic core cases.
pub(crate) fn cases() -> [CaseResult; 3] {
    [
        case("system-create-account-literal", create_account()),
        case("system-create-seeded-utf8", create_seeded()),
        case("system-strict-data-bounds", malformed()),
    ]
}

/// Checks the literal52-byte CreateAccount proof through the public codec.
pub fn create_account() -> Result<(), &'static str> {
    let data = CreateAccount {
        lamports: LAMPORTS,
        space: SPACE,
        program_address: Address::from_bytes([4; 32]),
    };
    let expected = hex_literal::hex!(
        "00000000000100000000000001020000000000000404040404040404040404040404040404040404040404040404040404040404"
    );
    let encoded = CreateAccountCodec
        .encode(&data, expected.len())
        .map_err(|_| "System create encode")?;

    if encoded != expected || CreateAccountCodec.decode(&expected, LIMITS) != Ok(data) {
        return Err("System create literal mismatch");
    }

    Ok(())
}

/// Checks the literal94-byte seed proof, including the u64 UTF-8 byte count.
pub fn create_seeded() -> Result<(), &'static str> {
    let data = CreateAccountWithSeed {
        base: Address::from_bytes([3; 32]),
        seed: String::from("é"),
        amount: LAMPORTS,
        space: SPACE,
        program_address: Address::from_bytes([4; 32]),
    };
    let expected = hex_literal::hex!(
        "0300000003030303030303030303030303030303030303030303030303030303030303030200000000000000c3a9000100000000000001020000000000000404040404040404040404040404040404040404040404040404040404040404"
    );
    let encoded = CreateAccountWithSeedCodec
        .encode(&data, expected.len())
        .map_err(|_| "System seed encode")?;

    if encoded != expected || CreateAccountWithSeedCodec.decode(&expected, LIMITS) != Ok(data) {
        return Err("System seeded literal mismatch");
    }

    Ok(())
}

/// Rejects unsupported tags, trailing data and insufficient byte budgets.
pub fn malformed() -> Result<(), &'static str> {
    let unsupported = SystemInstructionDataCodec.decode(&[14, 0, 0, 0], LIMITS);
    let trailing = SystemInstructionDataCodec.decode(&[4, 0, 0, 0, 0], LIMITS);

    if unsupported != Err(Error::InvalidDiscriminator) || trailing != Err(Error::TrailingBytes) {
        return Err("System malformed data accepted");
    }

    if SystemInstructionDataCodec.decode(&[4, 0, 0, 0], Limits::new(0, 0))
        != Err(Error::LimitExceeded)
    {
        return Err("System byte budget ignored");
    }

    let mut malformed = alloc::vec![0; 44];
    malformed[0] = 3;
    malformed[36..44].copy_from_slice(&u64::MAX.to_le_bytes());

    if CreateAccountWithSeedCodec
        .decode(&malformed, LIMITS)
        .is_ok()
    {
        return Err("System oversized u64 string length accepted");
    }

    malformed[36..44].copy_from_slice(&2_u64.to_le_bytes());

    if CreateAccountWithSeedCodec
        .decode(&malformed, LIMITS)
        .is_ok()
    {
        return Err("System truncated string accepted");
    }

    Ok(())
}
