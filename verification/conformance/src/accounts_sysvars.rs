//! Independent portable account/sysvar cases; fixture facts are pinned in the unit receipt.

use crate::require;
use alloc::vec;
use solana_kit_core::{
    Address,
    accounts::{Account, MaybeAccount},
    codecs::{Decoder, Encoder, Error, Limits, Number},
    program_error::{InstructionFailure, ProgramError, parse_failure_log},
    sysvars::{Clock, OWNER_ADDRESS, SlotHashes, SysvarCodec},
};

const CLOCK_SLOT: u64 = 284617079;
const CLOCK_TIMESTAMP: i64 = -20000;
const WORD_SIZE: usize = 8;
const ACCOUNT_BALANCE: u64 = 99;
const RENT_EPOCH: u64 = 7;
const CUSTOM_CODE: u32 = 42;

/// Keeps the additional assertion groups separate from the preexisting suite.
pub(crate) fn cases() -> [crate::CaseResult; 4] {
    [
        crate::case("sysvar-clock-signed-fixture", clock()),
        crate::case("account-metadata-and-absence", accounts()),
        crate::case("sysvar-history-u64-bounds", history_bounds()),
        crate::case("program-error-source-facts", program_errors()),
    ]
}

/// Signed timestamp fixture is independent of the encoder under test.
pub(crate) fn clock() -> Result<(), &'static str> {
    let bytes = [
        119, 233, 246, 16, 0, 0, 0, 0, 246, 255, 255, 255, 255, 255, 255, 255, 4, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 224, 177, 255, 255, 255, 255, 255, 255,
    ];
    let codec = SysvarCodec::<Clock>::NEW;
    let value = codec
        .decode(&bytes, Limits::new(bytes.len(), 0))
        .map_err(|_| "clock fixture rejected")?;
    require(
        value.slot == CLOCK_SLOT
            && value.epoch_start_timestamp == -10
            && value.unix_timestamp == CLOCK_TIMESTAMP,
        "clock signed fields differ",
    )?;
    let encoded = codec
        .encode(&value, bytes.len())
        .map_err(|_| "clock encoding failed")?;
    require(encoded == bytes, "clock independent bytes differ")
}

/// Missing accounts preserve absence; present snapshots retain metadata.
pub(crate) fn accounts() -> Result<(), &'static str> {
    let address = Address::from_bytes([5; 32]);
    let limits = Limits::new(WORD_SIZE, 0);
    let account = Account {
        address,
        owner: OWNER_ADDRESS,
        lamports: ACCOUNT_BALANCE,
        executable: false,
        space: 8,
        rent_epoch: Some(RENT_EPOCH),
        data: vec![255; 8],
    };
    let decoded = account
        .decode(&Number::<u64>::LE, OWNER_ADDRESS, limits)
        .map_err(|_| "account decode failed")?;
    require(
        decoded.address == address
            && decoded.data == u64::MAX
            && decoded.lamports == ACCOUNT_BALANCE
            && decoded.rent_epoch == Some(RENT_EPOCH),
        "account metadata differs",
    )?;
    let missing = MaybeAccount::<alloc::vec::Vec<u8>>::Missing(address)
        .decode(&Number::<u64>::LE, OWNER_ADDRESS, limits)
        .map_err(|_| "missing account decoded data")?;
    require(
        missing == MaybeAccount::Missing(address),
        "missing account changed state",
    )
}

/// A u32 count must not be accepted as the pinned u64 history count.
pub(crate) fn history_bounds() -> Result<(), &'static str> {
    let codec = SysvarCodec::<SlotHashes>::NEW;
    require(
        codec.decode(&[0; 8], Limits::new(8, 0)) == Ok(vec![]),
        "empty history rejected",
    )?;
    require(
        codec.decode(&[1, 0, 0, 0], Limits::new(8, 1)) == Err(Error::Truncated),
        "u32 history prefix accepted",
    )?;
    require(
        codec.decode(&[255; 8], Limits::new(8, 1)).is_err(),
        "overflowing history count accepted",
    )
}

/// Error matching requires the exact program/index/code; source logs remain intact.
pub(crate) fn program_errors() -> Result<(), &'static str> {
    let program = Address::from_bytes([0; 32]);
    let line = "Program 11111111111111111111111111111111 failed: custom program error: 0x2a";
    let failure = parse_failure_log(line)
        .map_err(|_| "failure log rejected")?
        .ok_or("failure log unmatched")?;
    require(
        failure.log == line
            && failure.program == program
            && failure.error == ProgramError::Custom(CUSTOM_CODE),
        "failure facts lost",
    )?;
    let error = InstructionFailure {
        index: 0,
        error: failure.error,
    };
    require(
        error.is_program_error(&[program], program, Some(CUSTOM_CODE))
            && !error.is_program_error(&[], program, None),
        "program error attribution differs",
    )
}
