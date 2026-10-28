//! Provider administration calls keep unspecified object schemas extensible.

use crate::Object;
use serde::{Serialize, Serializer};
use serde_json::Value;
use solana_kit_core::Address;

/// Cheatcode selection is either the literal `all` or an explicit method list.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum CheatcodeFilter {
    /// Select every cheatcode.
    All(AllCheatcodes),
    /// Select these exact method names.
    Methods(Vec<String>),
}

/// The only supported scalar cheatcode filter.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub enum AllCheatcodes {
    /// Every cheatcode.
    #[default]
    #[serde(rename = "all")]
    All,
}

/// Enables cheatcodes matching one explicit filter.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnableCheatcode(#[doc = "Cheatcodes to enable."] pub CheatcodeFilter);

impl Serialize for EnableCheatcode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        [&self.0].serialize(serializer)
    }
}

method!(EnableCheatcode, "surfnet_enableCheatcode", Value);

/// Disables selected cheatcodes with an optional lockout policy.
#[derive(Clone, Debug, PartialEq)]
pub struct DisableCheatcode {
    /// Exact selection.
    pub filter: CheatcodeFilter,
    /// Optional trailing lockout object.
    pub lockout: Option<Object>,
}

impl Serialize for DisableCheatcode {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        crate::wire::trailing(&self.filter, self.lockout.as_ref(), serializer)
    }
}

method!(DisableCheatcode, "surfnet_disableCheatcode", Value);

/// Exports a snapshot using an optional provider-defined configuration.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExportSnapshot(#[doc = "Optional snapshot configuration."] pub Option<Object>);

impl Serialize for ExportSnapshot {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        crate::wire::optional(self.0.as_ref(), serializer)
    }
}

method!(ExportSnapshot, "surfnet_exportSnapshot", Value);

/// Reads recent local signatures with an optional exact limit.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GetLocalSignatures(#[doc = "Optional signature count limit."] pub Option<u64>);

impl Serialize for GetLocalSignatures {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        crate::wire::optional(self.0.as_ref(), serializer)
    }
}

method!(GetLocalSignatures, "surfnet_getLocalSignatures", Value);

/// Reads the active IDL at an optional slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetActiveIdl {
    /// Program whose IDL is requested.
    pub program_id: Address,
    /// Optional historical slot.
    pub slot: Option<u64>,
}

impl Serialize for GetActiveIdl {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        crate::wire::trailing(self.program_id.to_string(), self.slot.as_ref(), serializer)
    }
}

method!(GetActiveIdl, "surfnet_getActiveIdl", Value);

macro_rules! object_slot {
    ($name:ident, $wire:literal, $field:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Debug, PartialEq)]
        pub struct $name {
            /// Provider-defined object; fields and exact JSON numbers are retained.
            pub $field: Object,
            /// Optional trailing slot.
            pub slot: Option<u64>,
        }

        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                crate::wire::trailing(&self.$field, self.slot.as_ref(), serializer)
            }
        }

        method!($name, $wire, Value);
    };
}

object_slot!(
    RegisterIdl,
    "surfnet_registerIdl",
    idl,
    "Register an Anchor IDL at an optional slot."
);
object_slot!(
    RegisterScenario,
    "surfnet_registerScenario",
    scenario,
    "Register a replay scenario."
);

/// Sets total SOL supply using the upstream's open update object.
#[derive(Clone, Debug, PartialEq)]
pub struct SetSupply(#[doc = "Provider supply update object."] pub Object);

impl Serialize for SetSupply {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        [&self.0].serialize(serializer)
    }
}

method!(SetSupply, "surfnet_setSupply", Value);
