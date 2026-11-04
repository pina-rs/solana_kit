//! Typed `jsonParsed` projection for every sysvar variant declared by the
//! pinned reference model, including deprecated and legacy shapes.
#![cfg(feature = "parsed-accounts")]

mod parsed_accounts_support;

use parsed_accounts_support::{decode, exact, parsed_account};
use solana_kit_client::{
    Presence,
    accounts::assert_account_decoded,
    parsed::{
        ClockInfo, EpochRewardsInfo, EpochScheduleInfo, FeeCalculator, FeesInfo,
        LastRestartSlotInfo, ParsedAccountData, RecentBlockhashEntry, RentInfo, SlotHashEntry,
        SlotHistoryInfo, StakeHistoryData, StakeHistoryEntry, SysvarAccount,
    },
};
use solana_kit_core::Hash;

const CLOCK: &str = r#"{
  "type": "clock",
  "info": {
    "epoch": 0,
    "epochStartTimestamp": 1704907181,
    "leaderScheduleEpoch": 1,
    "slot": 90829,
    "unixTimestamp": 1704998009
  }
}"#;

const EPOCH_REWARDS_AGAVE: &str = r#"{
  "type": "epochRewards",
  "info": {
    "active": true,
    "distributedRewards": "100",
    "distributionStartingBlockHeight": 500,
    "numPartitions": 512,
    "parentBlockhash": "Gy5GKD5p7UmUWF2BEm3TAUP4PSLTw9puSUbuoH5xPdzk",
    "totalPoints": "0",
    "totalRewards": "18446744073709551615"
  }
}"#;

const EPOCH_REWARDS_REFERENCE: &str = r#"{
  "type": "epochRewards",
  "info": {
    "distributedRewards": "18446744073709551615",
    "distributionCompleteBlockHeight": 1000,
    "totalRewards": "200"
  }
}"#;

const EPOCH_SCHEDULE: &str = r#"{
  "type": "epochSchedule",
  "info": {
    "firstNormalEpoch": 0,
    "firstNormalSlot": 0,
    "leaderScheduleSlotOffset": 432000,
    "slotsPerEpoch": 18446744073709551615,
    "warmup": false
  }
}"#;

const FEES: &str = r#"{
  "type": "fees",
  "info": {"feeCalculator": {"lamportsPerSignature": "5000"}}
}"#;

const LAST_RESTART_SLOT: &str = r#"{
  "type": "lastRestartSlot",
  "info": {"lastRestartSlot": 18446744073709551615}
}"#;

const RECENT_BLOCKHASHES: &str = r#"{
  "type": "recentBlockhashes",
  "info": [
    {
      "blockhash": "Gy5GKD5p7UmUWF2BEm3TAUP4PSLTw9puSUbuoH5xPdzk",
      "feeCalculator": {"lamportsPerSignature": "5000"}
    },
    {
      "blockhash": "FvNKRQk7TuFXVmXEM4XEubXdYREaeiWX56SL97taEhAQ",
      "feeCalculator": {"lamportsPerSignature": "0"}
    }
  ]
}"#;

const RENT_MODERN: &str = r#"{
  "type": "rent",
  "info": {"lamportsPerByte": "18446744073709551615"}
}"#;

const RENT_LEGACY: &str = r#"{
  "type": "rent",
  "info": {"burnPercent": 50, "exemptionThreshold": 2, "lamportsPerByteYear": "3480"}
}"#;

const SLOT_HASHES: &str = r#"{
  "type": "slotHashes",
  "info": [
    {"hash": "BAwX3h9EtGqBGnvXqgBoTZL19iHY8PKeCS9AVWFsVLQ8", "slot": 149694},
    {"hash": "7HdyQAb5kaZ9RjTuX8uejYXj86J3P2foNfDkqLAFNYFF", "slot": 149693}
  ]
}"#;

const SLOT_HISTORY: &str = r#"{
  "type": "slotHistory",
  "info": {"bits": "1111100000", "nextSlot": 150104}
}"#;

const STAKE_HISTORY: &str = r#"{
  "type": "stakeHistory",
  "info": [
    {
      "epoch": 683,
      "stakeHistory": {
        "activating": 0,
        "deactivating": 1,
        "effective": 18446744073709551615
      }
    }
  ]
}"#;

fn sysvar(projected: ParsedAccountData) -> SysvarAccount {
    match projected {
        ParsedAccountData::Sysvar(account) => account,
        _ => panic!("sysvar expected"),
    }
}

#[test]
fn clock_projects_slots_epochs_and_signed_timestamps() {
    let SysvarAccount::Clock(ClockInfo {
        epoch,
        epoch_start_timestamp,
        leader_schedule_epoch,
        slot,
        unix_timestamp,
    }) = sysvar(decode("sysvar", CLOCK).unwrap())
    else {
        panic!("clock expected")
    };

    assert_eq!(
        (epoch, epoch_start_timestamp, leader_schedule_epoch),
        (0, 1_704_907_181, 1)
    );
    assert_eq!((slot, unix_timestamp), (90_829, 1_704_998_009));
}

#[test]
fn epoch_rewards_keeps_absence_of_the_source_completion_height_explicit() {
    let account = parsed_account("sysvar", EPOCH_REWARDS_AGAVE);
    let raw = exact(&account).to_owned();
    let projected = assert_account_decoded(&account)
        .unwrap()
        .decode_typed(parsed_accounts_support::LIMITS)
        .unwrap();

    let SysvarAccount::EpochRewards(EpochRewardsInfo {
        distributed_rewards,
        distribution_complete_block_height,
        total_rewards,
    }) = sysvar(projected)
    else {
        panic!("epoch rewards expected")
    };

    assert_eq!(distributed_rewards, 100);
    assert_eq!(
        distribution_complete_block_height,
        Presence::Missing,
        "Agave omits the source-only completion height"
    );
    assert_eq!(total_rewards, u64::MAX);
    assert_eq!(exact(&account), raw, "unmodeled Agave members stay in raw");

    let SysvarAccount::EpochRewards(info) =
        sysvar(decode("sysvar", EPOCH_REWARDS_REFERENCE).unwrap())
    else {
        panic!("epoch rewards expected")
    };
    assert_eq!(info.distributed_rewards, u64::MAX);
    assert_eq!(
        info.distribution_complete_block_height,
        Presence::Present(1000)
    );
    assert_eq!(info.total_rewards, 200);
}

#[test]
fn epoch_schedule_projects_u64_slot_widths_without_float_conversion() {
    let SysvarAccount::EpochSchedule(EpochScheduleInfo {
        first_normal_epoch,
        first_normal_slot,
        leader_schedule_slot_offset,
        slots_per_epoch,
        warmup,
    }) = sysvar(decode("sysvar", EPOCH_SCHEDULE).unwrap())
    else {
        panic!("epoch schedule expected")
    };

    assert_eq!((first_normal_epoch, first_normal_slot), (0, 0));
    assert_eq!(leader_schedule_slot_offset, 432_000);
    assert_eq!(slots_per_epoch, u64::MAX);
    assert!(!warmup);
}

#[test]
fn fees_and_recent_blockhashes_share_the_exact_fee_rate_shape() {
    let SysvarAccount::Fees(FeesInfo { fee_calculator }) = sysvar(decode("sysvar", FEES).unwrap())
    else {
        panic!("fees expected")
    };
    assert_eq!(fee_calculator.lamports_per_signature, 5000);

    let SysvarAccount::RecentBlockhashes(entries) =
        sysvar(decode("sysvar", RECENT_BLOCKHASHES).unwrap())
    else {
        panic!("recent blockhashes expected")
    };

    assert_eq!(
        entries,
        vec![
            RecentBlockhashEntry {
                blockhash: "Gy5GKD5p7UmUWF2BEm3TAUP4PSLTw9puSUbuoH5xPdzk"
                    .parse::<Hash>()
                    .unwrap(),
                fee_calculator: FeeCalculator {
                    lamports_per_signature: 5000,
                },
            },
            RecentBlockhashEntry {
                blockhash: "FvNKRQk7TuFXVmXEM4XEubXdYREaeiWX56SL97taEhAQ"
                    .parse::<Hash>()
                    .unwrap(),
                fee_calculator: FeeCalculator {
                    lamports_per_signature: 0,
                },
            },
        ]
    );
}

#[test]
fn rent_accepts_the_modern_and_legacy_wire_shapes() {
    let SysvarAccount::Rent(RentInfo {
        burn_percent,
        exemption_threshold,
        lamports_per_byte,
        lamports_per_byte_year,
    }) = sysvar(decode("sysvar", RENT_MODERN).unwrap())
    else {
        panic!("rent expected")
    };
    assert_eq!(burn_percent, None);
    assert_eq!(exemption_threshold, None);
    assert_eq!(lamports_per_byte, Some(u64::MAX));
    assert_eq!(lamports_per_byte_year, None);

    let SysvarAccount::Rent(info) = sysvar(decode("sysvar", RENT_LEGACY).unwrap()) else {
        panic!("rent expected")
    };
    assert_eq!(info.burn_percent, Some(50));
    assert_eq!(info.exemption_threshold, Some(2.0));
    assert_eq!(info.lamports_per_byte, None);
    assert_eq!(info.lamports_per_byte_year, Some(3480));
}

#[test]
fn slot_and_stake_history_project_ordered_entries() {
    let SysvarAccount::SlotHashes(entries) = sysvar(decode("sysvar", SLOT_HASHES).unwrap()) else {
        panic!("slot hashes expected")
    };
    assert_eq!(
        entries,
        vec![
            SlotHashEntry {
                hash: "BAwX3h9EtGqBGnvXqgBoTZL19iHY8PKeCS9AVWFsVLQ8".to_owned(),
                slot: 149_694,
            },
            SlotHashEntry {
                hash: "7HdyQAb5kaZ9RjTuX8uejYXj86J3P2foNfDkqLAFNYFF".to_owned(),
                slot: 149_693,
            },
        ]
    );

    let SysvarAccount::SlotHistory(SlotHistoryInfo { bits, next_slot }) =
        sysvar(decode("sysvar", SLOT_HISTORY).unwrap())
    else {
        panic!("slot history expected")
    };
    assert_eq!(bits, "1111100000");
    assert_eq!(next_slot, 150_104);

    let SysvarAccount::StakeHistory(entries) = sysvar(decode("sysvar", STAKE_HISTORY).unwrap())
    else {
        panic!("stake history expected")
    };
    assert_eq!(
        entries,
        vec![StakeHistoryEntry {
            epoch: 683,
            stake_history: StakeHistoryData {
                activating: 0,
                deactivating: 1,
                effective: u64::MAX,
            },
        }]
    );
}

#[test]
fn last_restart_slot_projects_the_full_slot_width() {
    let SysvarAccount::LastRestartSlot(LastRestartSlotInfo { last_restart_slot }) =
        sysvar(decode("sysvar", LAST_RESTART_SLOT).unwrap())
    else {
        panic!("last restart slot expected")
    };
    assert_eq!(last_restart_slot, u64::MAX);
}

#[test]
fn sysvar_variant_variants_are_distinct() {
    for (literal, expected) in [
        (CLOCK, "clock"),
        (EPOCH_SCHEDULE, "epochSchedule"),
        (FEES, "fees"),
        (RENT_MODERN, "rent"),
        (SLOT_HISTORY, "slotHistory"),
    ] {
        let account = sysvar(decode("sysvar", literal).unwrap());
        let actual = match account {
            SysvarAccount::Clock(_) => "clock",
            SysvarAccount::EpochRewards(_) => "epochRewards",
            SysvarAccount::EpochSchedule(_) => "epochSchedule",
            SysvarAccount::Fees(_) => "fees",
            SysvarAccount::LastRestartSlot(_) => "lastRestartSlot",
            SysvarAccount::RecentBlockhashes(_) => "recentBlockhashes",
            SysvarAccount::Rent(_) => "rent",
            SysvarAccount::SlotHashes(_) => "slotHashes",
            SysvarAccount::SlotHistory(_) => "slotHistory",
            SysvarAccount::StakeHistory(_) => "stakeHistory",
        };
        assert_eq!(actual, expected);
    }
}
