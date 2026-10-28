//! Every cheatcode name registered by the pinned Surfpool client.

mod account;
mod administration;
mod profiling;

pub use crate::builders::{ResetAccount, SetAccount, SetTokenAccount, StreamAccount};
pub use account::*;
pub use administration::*;
pub use profiling::*;

use crate::types::{EpochInfoValue, TimeTarget};
use serde::{Serialize, Serializer};
use serde_json::Value;

/// All 28 upstream registered cheatcode method names, in source order.
pub const SURFNET_CHEATCODE_METHODS: [&str; 28] = [
    "surfnet_cloneProgramAccount",
    "surfnet_deriveConfidentialKeys",
    "surfnet_disableCheatcode",
    "surfnet_enableCheatcode",
    "surfnet_exportSnapshot",
    "surfnet_getActiveIdl",
    "surfnet_getConfidentialBalance",
    "surfnet_getLocalSignatures",
    "surfnet_getProfileResultsByTag",
    "surfnet_getStreamedAccounts",
    "surfnet_getSurfnetInfo",
    "surfnet_getTransactionProfile",
    "surfnet_offlineAccount",
    "surfnet_pauseClock",
    "surfnet_profileTransaction",
    "surfnet_registerIdl",
    "surfnet_registerScenario",
    "surfnet_resetAccount",
    "surfnet_resetNetwork",
    "surfnet_resumeClock",
    "surfnet_setAccount",
    "surfnet_setProgramAuthority",
    "surfnet_setSupply",
    "surfnet_setTokenAccount",
    "surfnet_streamAccount",
    "surfnet_streamAccounts",
    "surfnet_timeTravel",
    "surfnet_writeProgram",
];

/// Moves the local clock to exactly one absolute target.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeTravel(#[doc = "The single absolute clock target."] pub TimeTarget);

impl Serialize for TimeTravel {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        [self.0].serialize(serializer)
    }
}

method!(TimeTravel, "surfnet_timeTravel", EpochInfoValue);
empty!(
    PauseClock,
    "surfnet_pauseClock",
    EpochInfoValue,
    "Pause the local clock."
);
empty!(
    ResumeClock,
    "surfnet_resumeClock",
    EpochInfoValue,
    "Resume the local clock."
);
empty!(
    GetStreamedAccounts,
    "surfnet_getStreamedAccounts",
    Value,
    "Read the streamed-account listing."
);
empty!(
    GetSurfnetInfo,
    "surfnet_getSurfnetInfo",
    Value,
    "Read Surfnet information."
);
empty!(
    ResetNetwork,
    "surfnet_resetNetwork",
    Value,
    "Reset the local network to its initial state."
);
