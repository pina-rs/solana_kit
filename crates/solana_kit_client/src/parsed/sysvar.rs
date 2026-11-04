//! Sysvar account payloads.
//!
//! Source: `packages/solana_kit_rpc_parsed_types/lib/src/sysvar_accounts.dart`.
//! The pinned model declares ten sysvar variants. Node variants outside that set,
//! including `rewards`, are retained as unknown payloads.

use super::{FeeCalculator, ParsedAccountData, scalars};
use crate::{Error, Presence};
use serde::Deserialize;
use serde_json::value::RawValue;
use solana_kit_core::Hash;

pub(super) fn decode(program: &str, parsed: &RawValue) -> Result<ParsedAccountData, Error> {
    let envelope = super::envelope(parsed)?;

    let account = match envelope.type_name {
        "clock" => SysvarAccount::Clock(super::info(&envelope)?),
        "epochRewards" => SysvarAccount::EpochRewards(super::info(&envelope)?),
        "epochSchedule" => SysvarAccount::EpochSchedule(super::info(&envelope)?),
        "fees" => SysvarAccount::Fees(super::info(&envelope)?),
        "lastRestartSlot" => SysvarAccount::LastRestartSlot(super::info(&envelope)?),
        "recentBlockhashes" => SysvarAccount::RecentBlockhashes(super::info(&envelope)?),
        "rent" => SysvarAccount::Rent(super::info(&envelope)?),
        "slotHashes" => SysvarAccount::SlotHashes(super::info(&envelope)?),
        "slotHistory" => SysvarAccount::SlotHistory(super::info(&envelope)?),
        "stakeHistory" => SysvarAccount::StakeHistory(super::info(&envelope)?),
        other => return super::unknown_type(program, other, parsed),
    };

    Ok(ParsedAccountData::Sysvar(account))
}

/// Reference `JsonParsedSysvarAccount`.
#[derive(Debug, PartialEq)]
pub enum SysvarAccount {
    /// `type: "clock"`.
    Clock(ClockInfo),
    /// `type: "epochRewards"`.
    EpochRewards(EpochRewardsInfo),
    /// `type: "epochSchedule"`.
    EpochSchedule(EpochScheduleInfo),
    /// `type: "fees"`, a deprecated sysvar.
    Fees(FeesInfo),
    /// `type: "lastRestartSlot"`.
    LastRestartSlot(LastRestartSlotInfo),
    /// `type: "recentBlockhashes"`, a deprecated sysvar.
    RecentBlockhashes(Vec<RecentBlockhashEntry>),
    /// `type: "rent"`.
    Rent(RentInfo),
    /// `type: "slotHashes"`.
    SlotHashes(Vec<SlotHashEntry>),
    /// `type: "slotHistory"`.
    SlotHistory(SlotHistoryInfo),
    /// `type: "stakeHistory"`.
    StakeHistory(Vec<StakeHistoryEntry>),
}

/// Reference `JsonParsedClockInfo`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ClockInfo {
    /// Current epoch.
    pub epoch: u64,
    /// Unix timestamp at the start of the current epoch.
    pub epoch_start_timestamp: i64,
    /// Epoch whose leader schedule is being computed.
    pub leader_schedule_epoch: u64,
    /// Current slot.
    pub slot: u64,
    /// Current Unix timestamp.
    pub unix_timestamp: i64,
}

/// Reference `JsonParsedEpochRewardsInfo`.
///
/// The pinned model requires `distributionCompleteBlockHeight`, but the local
/// Agave 4.3.2 account decoder reports an `EpochRewards` shape without that
/// member. Absence is explicit through [`Presence`] instead of being collapsed
/// into an invented `null`, and the unmodeled Agave members remain in the
/// original raw payload.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EpochRewardsInfo {
    /// Rewards already distributed, reported as decimal text.
    #[serde(deserialize_with = "scalars::unsigned_decimal")]
    pub distributed_rewards: u64,
    /// Block height at which distribution completes, when the node reports it.
    #[serde(default)]
    pub distribution_complete_block_height: Presence<u64>,
    /// Total rewards for the epoch, reported as decimal text.
    #[serde(deserialize_with = "scalars::unsigned_decimal")]
    pub total_rewards: u64,
}

/// Reference `JsonParsedEpochScheduleInfo`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EpochScheduleInfo {
    /// First epoch outside the warmup period.
    pub first_normal_epoch: u64,
    /// First slot outside the warmup period.
    pub first_normal_slot: u64,
    /// Slots before an epoch that its leader schedule is computed.
    pub leader_schedule_slot_offset: u64,
    /// Maximum slots per epoch.
    pub slots_per_epoch: u64,
    /// Whether the cluster is still in the warmup period.
    pub warmup: bool,
}

/// Reference `JsonParsedFeesInfo`, a deprecated sysvar shape.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FeesInfo {
    /// Fee rate recorded by the deprecated sysvar.
    pub fee_calculator: FeeCalculator,
}

/// Reference `JsonParsedLastRestartSlotInfo`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LastRestartSlotInfo {
    /// Last recorded restart slot.
    pub last_restart_slot: u64,
}

/// Reference `JsonParsedRecentBlockhashEntry`, a deprecated sysvar shape.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RecentBlockhashEntry {
    /// Recorded blockhash.
    #[serde(deserialize_with = "crate::wire::hash")]
    pub blockhash: Hash,
    /// Fee rate recorded with the blockhash.
    pub fee_calculator: FeeCalculator,
}

/// Reference `JsonParsedRentInfo`.
///
/// Agave 4.1.0 reshaped the parsed rent sysvar to only `lamportsPerByte`. Both
/// shapes remain representable because the pinned model declares the older
/// members as optional.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RentInfo {
    /// Percentage of collected rent burned, reported before Agave 4.1.0.
    #[serde(default)]
    pub burn_percent: Option<u8>,
    /// Exemption threshold multiplier, reported before Agave 4.1.0.
    #[serde(default)]
    pub exemption_threshold: Option<f64>,
    /// Lamports charged per byte on Agave 4.1.0 and later.
    #[serde(default, deserialize_with = "scalars::optional_unsigned_decimal")]
    pub lamports_per_byte: Option<u64>,
    /// Lamports charged per byte-year on earlier validators.
    #[serde(default, deserialize_with = "scalars::optional_unsigned_decimal")]
    pub lamports_per_byte_year: Option<u64>,
}

/// Reference `JsonParsedSlotHashEntry`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct SlotHashEntry {
    /// Hash of the slot, exactly as the reference model's plain string member.
    pub hash: String,
    /// Slot number.
    pub slot: u64,
}

/// Reference `JsonParsedSlotHistoryInfo`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SlotHistoryInfo {
    /// Slot-history bit vector, exactly as reported.
    pub bits: String,
    /// Next slot to record.
    pub next_slot: u64,
}

/// Reference `JsonParsedStakeHistoryEntry`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct StakeHistoryEntry {
    /// Epoch of this history entry.
    pub epoch: u64,
    /// Stake state for the epoch.
    pub stake_history: StakeHistoryData,
}

/// Reference `JsonParsedStakeHistoryData`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct StakeHistoryData {
    /// Stake activating during the epoch.
    pub activating: u64,
    /// Stake deactivating during the epoch.
    pub deactivating: u64,
    /// Effective stake during the epoch.
    pub effective: u64,
}
