//! Pinned canonical program, sysvar and mint literals for off-chain clients.
//!
//! These values identify accounts. Their presence does not promise a particular
//! program is deployed or a mint exists on the caller's selected cluster.

mod metaplex;
mod program;
mod spl;
mod well_known;

pub use metaplex::*;
pub use program::*;
pub use spl::*;
pub use well_known::*;

pub use crate::sysvars::{
    CLOCK_ADDRESS as SYSVAR_CLOCK, EPOCH_REWARDS_ADDRESS as SYSVAR_EPOCH_REWARDS,
    EPOCH_SCHEDULE_ADDRESS as SYSVAR_EPOCH_SCHEDULE, FEES_ADDRESS as SYSVAR_FEES,
    INSTRUCTIONS_ADDRESS as SYSVAR_INSTRUCTIONS,
    LAST_RESTART_SLOT_ADDRESS as SYSVAR_LAST_RESTART_SLOT, OWNER_ADDRESS as SYSVAR_OWNER,
    RECENT_BLOCKHASHES_ADDRESS as SYSVAR_RECENT_BLOCKHASHES, RENT_ADDRESS as SYSVAR_RENT,
    REWARDS_ADDRESS as SYSVAR_REWARDS, SLOT_HASHES_ADDRESS as SYSVAR_SLOT_HASHES,
    SLOT_HISTORY_ADDRESS as SYSVAR_SLOT_HISTORY, STAKE_HISTORY_ADDRESS as SYSVAR_STAKE_HISTORY,
};
