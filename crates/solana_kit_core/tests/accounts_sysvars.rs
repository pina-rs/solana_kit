//! Independent fixed bytes from the pinned Dart layouts and explicitly specified boundaries.

use solana_kit_core::{
    Address,
    accounts::{Account, AccountError, MaybeAccount},
    codecs::{Decoder, Encoder, Error, Limits, Number},
    program_error::{
        InstructionFailure, KnownProgramError, LogParseError, ProgramError, parse_failure_log,
    },
    sysvars::*,
};

const CLOCK: [u8; 40] = [
    119, 233, 246, 16, 0, 0, 0, 0, 246, 255, 255, 255, 255, 255, 255, 255, 4, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 224, 177, 255, 255, 255, 255, 255, 255,
];
const LIMITS: Limits = Limits::new(200000, 20000);

/// Requires independent decode and encode equality plus all truncated prefixes and trailing bytes.
fn fixture<T: core::fmt::Debug + PartialEq>(value: T, bytes: &[u8])
where
    SysvarCodec<T>: Decoder<T> + Encoder<T>,
{
    let codec = SysvarCodec::<T>::NEW;
    assert_eq!(codec.decode(bytes, LIMITS), Ok(value));
    let decoded = codec.decode(bytes, LIMITS).unwrap();
    assert_eq!(codec.encode(&decoded, bytes.len()).unwrap(), bytes);

    for end in 0..bytes.len() {
        assert!(codec.decode(&bytes[..end], LIMITS).is_err());
    }

    let mut extra = bytes.to_vec();
    extra.push(0);
    assert_eq!(codec.decode(&extra, LIMITS), Err(Error::TrailingBytes));
    let mut prefixed = vec![77];
    prefixed.extend_from_slice(bytes);
    assert_eq!(
        codec.decode_prefix(&prefixed, 1, LIMITS).unwrap().1,
        bytes.len() + 1
    );
}

#[test]
fn fixed_layout_independent_bytes() {
    fixture(
        Clock {
            slot: 284617079,
            epoch_start_timestamp: -10,
            epoch: 4,
            leader_schedule_epoch: 0,
            unix_timestamp: -20000,
        },
        &CLOCK,
    );
    fixture(
        Rent {
            lamports_per_byte_year: 1,
            exemption_threshold: 2.0,
            burn_percent: 255,
        },
        &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 255],
    );
    fixture(
        EpochSchedule {
            slots_per_epoch: 1,
            leader_schedule_slot_offset: 2,
            warmup: true,
            first_normal_epoch: 3,
            first_normal_slot: 4,
        },
        &[
            1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1, 3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0,
            0, 0, 0, 0,
        ],
    );
    fixture(
        LastRestartSlot {
            last_restart_slot: u64::MAX,
        },
        &[255; 8],
    );
}

#[test]
fn reward_points_and_strict_boolean_tags() {
    let mut rewards = [255; 81];
    rewards[80] = 1;
    fixture(
        EpochRewards {
            distribution_starting_block_height: u64::MAX,
            num_partitions: u64::MAX,
            parent_blockhash: solana_kit_core::Hash::from_bytes([255; 32]),
            total_points: u128::MAX,
            total_rewards: u64::MAX,
            distributed_rewards: u64::MAX,
            active: true,
        },
        &rewards,
    );
    rewards[80] = 2;
    assert_eq!(
        SysvarCodec::<EpochRewards>::NEW.decode(&rewards, LIMITS),
        Err(Error::InvalidDiscriminator)
    );
    let mut schedule = [0; 33];
    schedule[16] = 2;
    assert_eq!(
        SysvarCodec::<EpochSchedule>::NEW.decode(&schedule, LIMITS),
        Err(Error::InvalidDiscriminator)
    );
}

#[test]
fn histories_u64_prefixes_and_bounds() {
    let mut hashes = vec![0; 48];
    hashes[0] = 1;
    hashes[8] = 5;
    hashes[16..].fill(7);
    fixture(
        vec![SlotHashEntry {
            slot: 5,
            hash: solana_kit_core::Hash::from_bytes([7; 32]),
        }],
        &hashes,
    );
}

#[test]
fn historical_blockhash_fee_fixture() {
    let mut recent = vec![0; 48];
    recent[0] = 1;
    recent[8..40].fill(9);
    recent[40] = 11;
    fixture(
        vec![RecentBlockhashEntry {
            blockhash: solana_kit_core::Hash::from_bytes([9; 32]),
            fee_calculator: FeeCalculator {
                lamports_per_signature: 11,
            },
        }],
        &recent,
    );
}

#[test]
fn stake_history_fixture() {
    let stake = [
        1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0,
        0, 0, 5, 0, 0, 0, 0, 0, 0, 0,
    ];
    fixture(
        vec![StakeHistoryEntry {
            epoch: 2,
            stake_history: StakeHistoryData {
                effective: 3,
                activating: 4,
                deactivating: 5,
            },
        }],
        &stake,
    );
}

#[test]
fn history_count_boundaries() {
    let mut hashes = vec![0; 48];
    hashes[0] = 1;
    let codec = SysvarCodec::<SlotHashes>::NEW;
    assert_eq!(
        codec.decode(&hashes, Limits::new(48, 0)),
        Err(Error::LimitExceeded)
    );
    assert!(codec.decode(&[255; 8], LIMITS).is_err());
    assert_eq!(codec.decode(&[0; 8], LIMITS), Ok(vec![]));
    assert_eq!(codec.decode(&[1, 0, 0, 0], LIMITS), Err(Error::Truncated));
}

#[test]
fn slot_history_constants_and_malformed_layout() {
    let mut bytes = vec![0; SLOT_HISTORY_SIZE];
    bytes[0] = 1;
    bytes[2] = 64;
    bytes[9] = 1;
    bytes[131083] = 16;
    bytes[131089] = 7;
    let codec = SysvarCodec::<SlotHistory>::NEW;
    let value = codec.decode(&bytes, LIMITS).unwrap();
    assert_eq!(value.bits.len(), 16384);
    assert_eq!(value.bits[0], 1);
    assert_eq!(value.next_slot, 7);
    assert_eq!(codec.encode(&value, SLOT_HISTORY_SIZE).unwrap(), bytes);
    assert_eq!(
        codec.decode(&bytes, Limits::new(SLOT_HISTORY_SIZE, 16383)),
        Err(Error::LimitExceeded)
    );

    for offset in [0, 1, 131081] {
        let mut malformed = bytes.clone();
        malformed[offset] ^= 2;
        assert!(codec.decode(&malformed, LIMITS).is_err());
    }

    assert!(
        codec
            .decode(&bytes[..SLOT_HISTORY_SIZE - 1], LIMITS)
            .is_err()
    );
    assert!(
        codec
            .encode(
                &SlotHistory {
                    bits: vec![0],
                    next_slot: 0
                },
                SLOT_HISTORY_SIZE
            )
            .is_err()
    );
}

#[test]
fn account_metadata_missing_owner_size_and_codec_failures() {
    let address = Address::from_bytes([3; 32]);
    let account = Account {
        address,
        owner: OWNER_ADDRESS,
        lamports: 42,
        executable: false,
        space: 8,
        rent_epoch: Some(u64::MAX),
        data: vec![255; 8],
    };
    let decoded = account
        .clone()
        .decode(&Number::<u64>::LE, OWNER_ADDRESS, LIMITS)
        .unwrap();
    assert_eq!(decoded.data, u64::MAX);
    assert_eq!(decoded.address, address);
    assert_eq!(decoded.owner, OWNER_ADDRESS);
    assert_eq!(decoded.lamports, 42);
    assert_eq!(decoded.space, 8);
    assert_eq!(decoded.rent_epoch, Some(u64::MAX));
    assert!(!decoded.executable);
}

#[test]
fn account_decode_failures() {
    let address = Address::from_bytes([3; 32]);
    let account = Account {
        address,
        owner: OWNER_ADDRESS,
        lamports: 42,
        executable: false,
        space: 8,
        rent_epoch: None,
        data: vec![255; 8],
    };

    assert!(matches!(
        account.clone().decode(&Number::<u64>::LE, address, LIMITS),
        Err(AccountError::OwnerMismatch { .. })
    ));
    let mut wrong = account.clone();
    wrong.space = 7;
    assert_eq!(
        wrong.decode(&Number::<u64>::LE, OWNER_ADDRESS, LIMITS),
        Err(AccountError::DataSizeMismatch(address))
    );
    assert!(matches!(
        account.decode(&Number::<u32>::LE, OWNER_ADDRESS, LIMITS),
        Err(AccountError::Decode {
            source: Error::TrailingBytes,
            ..
        })
    ));
}

#[test]
fn missing_accounts_preserve_the_address() {
    let address = Address::from_bytes([3; 32]);
    let missing = MaybeAccount::<Vec<u8>>::Missing(address);
    assert_eq!(
        missing.clone().require(),
        Err(AccountError::Missing(address))
    );
    assert_eq!(
        missing.decode(&Number::<u64>::LE, OWNER_ADDRESS, LIMITS),
        Ok(MaybeAccount::Missing(address))
    );
}

#[test]
fn exact_sysvar_address_constants() {
    for (address, text) in [
        (OWNER_ADDRESS, "Sysvar1111111111111111111111111111111111111"),
        (CLOCK_ADDRESS, "SysvarC1ock11111111111111111111111111111111"),
        (
            EPOCH_REWARDS_ADDRESS,
            "SysvarEpochRewards1111111111111111111111111",
        ),
        (
            EPOCH_SCHEDULE_ADDRESS,
            "SysvarEpochSchedu1e111111111111111111111111",
        ),
        (FEES_ADDRESS, "SysvarFees111111111111111111111111111111111"),
        (
            INSTRUCTIONS_ADDRESS,
            "Sysvar1nstructions1111111111111111111111111",
        ),
        (
            LAST_RESTART_SLOT_ADDRESS,
            "SysvarLastRestartS1ot1111111111111111111111",
        ),
        (
            RECENT_BLOCKHASHES_ADDRESS,
            "SysvarRecentB1ockHashes11111111111111111111",
        ),
        (RENT_ADDRESS, "SysvarRent111111111111111111111111111111111"),
        (
            REWARDS_ADDRESS,
            "SysvarRewards111111111111111111111111111111",
        ),
        (
            SLOT_HASHES_ADDRESS,
            "SysvarS1otHashes111111111111111111111111111",
        ),
        (
            SLOT_HISTORY_ADDRESS,
            "SysvarS1otHistory11111111111111111111111111",
        ),
        (
            STAKE_HISTORY_ADDRESS,
            "SysvarStakeHistory1111111111111111111111111",
        ),
    ] {
        assert_eq!(address.to_string(), text);
    }
}

#[test]
fn program_error_matching_and_lossless_log_extraction() {
    let program = Address::from_bytes([0; 32]);
    let failure = InstructionFailure {
        index: 0,
        error: ProgramError::Custom(42),
    };
    assert!(failure.is_program_error(&[program], program, None));
    assert!(failure.is_program_error(&[program], program, Some(42)));
    assert!(!failure.is_program_error(&[program], program, Some(43)));
    assert!(!failure.is_program_error(&[], program, None));
    assert!(!failure.is_program_error(&[program], OWNER_ADDRESS, None));
    let line = "Program 11111111111111111111111111111111 failed: custom program error: 0x2a";
    let parsed = parse_failure_log(line).unwrap().unwrap();
    assert_eq!(parsed.program, program);
    assert_eq!(parsed.log, line);
    assert_eq!(parsed.error, ProgramError::Custom(42));
    let known = parse_failure_log(
        "Program 11111111111111111111111111111111 failed: invalid instruction data",
    )
    .unwrap()
    .unwrap();
    assert_eq!(
        known.error,
        ProgramError::Known(KnownProgramError::InvalidInstructionData)
    );
    assert_eq!(
        parse_failure_log("Program 11111111111111111111111111111111 success"),
        Ok(None)
    );
    assert_eq!(
        parse_failure_log("Program broken failed: bad"),
        Err(LogParseError::InvalidAddress)
    );
    assert_eq!(
        parse_failure_log(
            "Program 11111111111111111111111111111111 failed: custom program error: 0x100000000"
        ),
        Err(LogParseError::InvalidCustomCode)
    );
    let unknown =
        parse_failure_log("Program 11111111111111111111111111111111 failed: future failure")
            .unwrap()
            .unwrap();
    assert_eq!(
        unknown.error,
        ProgramError::Unknown("future failure".into())
    );
}
