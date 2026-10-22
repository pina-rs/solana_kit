//! Fixed-size field layouts with signed timestamp and wide reward support.

use super::{HashCodec, SysvarCodec, layout};
use crate::{
    Hash,
    codecs::{Bool, Decoder, Encoder, Error, Number, Reader, Writer},
};

/// Clock encoded size.
pub const CLOCK_SIZE: usize = 40;
/// Rent encoded size.
pub const RENT_SIZE: usize = 17;
/// Epoch schedule encoded size.
pub const EPOCH_SCHEDULE_SIZE: usize = 33;
/// Epoch rewards encoded size.
pub const EPOCH_REWARDS_SIZE: usize = 81;
/// Last restart slot encoded size.
pub const LAST_RESTART_SLOT_SIZE: usize = 8;

/// Network time snapshot. Timestamps are signed Unix seconds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Clock {
    /// Current slot.
    pub slot: u64,
    /// First timestamp in the epoch.
    pub epoch_start_timestamp: i64,
    /// Current epoch.
    pub epoch: u64,
    /// Latest generated leader schedule epoch.
    pub leader_schedule_epoch: u64,
    /// Current Unix timestamp.
    pub unix_timestamp: i64,
}
layout!(Clock, CLOCK_SIZE, slot: Number::<u64>::LE, epoch_start_timestamp: Number::<i64>::LE, epoch: Number::<u64>::LE, leader_schedule_epoch: Number::<u64>::LE, unix_timestamp: Number::<i64>::LE);

/// Rent configuration. Float bits and the full reference u8 burn range are preserved.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rent {
    /// Lamports per data byte per year.
    pub lamports_per_byte_year: u64,
    /// Years used for rent exemption, encoded as IEEE 754 binary64.
    pub exemption_threshold: f64,
    /// Percentage burned; the reference codec accepts all u8 values.
    pub burn_percent: u8,
}
layout!(Rent, RENT_SIZE, lamports_per_byte_year: Number::<u64>::LE, exemption_threshold: Number::<f64>::LE, burn_percent: Number::<u8>::LE);

/// Epoch timing and warmup configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EpochSchedule {
    /// Maximum slots per epoch.
    pub slots_per_epoch: u64,
    /// Leader schedule selection offset.
    pub leader_schedule_slot_offset: u64,
    /// Whether early epochs grow.
    pub warmup: bool,
    /// First normal-length epoch.
    pub first_normal_epoch: u64,
    /// First normal-length slot.
    pub first_normal_slot: u64,
}
layout!(EpochSchedule, EPOCH_SCHEDULE_SIZE, slots_per_epoch: Number::<u64>::LE, leader_schedule_slot_offset: Number::<u64>::LE, warmup: Bool, first_normal_epoch: Number::<u64>::LE, first_normal_slot: Number::<u64>::LE);

/// Epoch reward distribution snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EpochRewards {
    /// First reward distribution block height.
    pub distribution_starting_block_height: u64,
    /// Number of distribution partitions.
    pub num_partitions: u64,
    /// Parent block hash at the start of the epoch.
    pub parent_blockhash: Hash,
    /// Exact total stake-credit points.
    pub total_points: u128,
    /// Total lamports to distribute.
    pub total_rewards: u64,
    /// Lamports already distributed.
    pub distributed_rewards: u64,
    /// Whether distribution is active.
    pub active: bool,
}
layout!(EpochRewards, EPOCH_REWARDS_SIZE, distribution_starting_block_height: Number::<u64>::LE, num_partitions: Number::<u64>::LE, parent_blockhash: HashCodec, total_points: Number::<u128>::LE, total_rewards: Number::<u64>::LE, distributed_rewards: Number::<u64>::LE, active: Bool);

/// Last restart slot, zero when no hard fork was recorded.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LastRestartSlot {
    /// Recorded restart slot.
    pub last_restart_slot: u64,
}
layout!(LastRestartSlot, LAST_RESTART_SLOT_SIZE, last_restart_slot: Number::<u64>::LE);
