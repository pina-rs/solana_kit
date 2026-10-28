//! Account and confidential-key cheatcodes preserve exact positional shapes.

use crate::{Object, hex::HexBytes, types::*};
use serde::{Serialize, Serializer};
use serde_json::Value;
use solana_kit_client::{Error, methods::Method};
use solana_kit_core::Address;

/// Marks one account as offline, with optional provider-defined configuration.
#[derive(Clone, Debug, PartialEq)]
pub struct OfflineAccount {
    /// Account to mark offline.
    pub address: Address,
    /// Optional trailing configuration.
    pub config: Option<Object>,
}

impl Serialize for OfflineAccount {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        crate::wire::trailing(self.address.to_string(), self.config.as_ref(), serializer)
    }
}

method!(OfflineAccount, "surfnet_offlineAccount", Value);

/// Streams account descriptors in one positional array.
#[derive(Clone, Debug, PartialEq)]
pub struct StreamAccounts(#[doc = "Account descriptor objects."] pub Vec<Object>);

impl Serialize for StreamAccounts {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        [&self.0].serialize(serializer)
    }
}

method!(StreamAccounts, "surfnet_streamAccounts", Value);

/// Decrypts only the confidential balance halves whose keys are present.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GetConfidentialBalance {
    /// Token account holding encrypted balances.
    pub token_account: Address,
    /// At least one decryption key.
    pub keys: ConfidentialBalanceKeys,
}

impl Serialize for GetConfidentialBalance {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        (self.token_account.to_string(), &self.keys).serialize(serializer)
    }
}

method!(
    GetConfidentialBalance,
    "surfnet_getConfidentialBalance",
    ConfidentialBalance
);

/// Derives confidential keys from an encoded owner signature.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeriveConfidentialKeys(#[doc = "Owner's encoded signature."] pub String);

impl Serialize for DeriveConfidentialKeys {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        [&self.0].serialize(serializer)
    }
}

method!(
    DeriveConfidentialKeys,
    "surfnet_deriveConfidentialKeys",
    ConfidentialKeys
);

/// Clones one program account into another address.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CloneProgramAccount {
    /// Source program.
    pub source: Address,
    /// Destination program.
    pub destination: Address,
}

impl Serialize for CloneProgramAccount {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        (self.source.to_string(), self.destination.to_string()).serialize(serializer)
    }
}

method!(CloneProgramAccount, "surfnet_cloneProgramAccount", Value);

/// Replaces a program authority, or omits the authority to clear it upstream.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SetProgramAuthority {
    /// Program account.
    pub program_id: Address,
    /// New authority; None omits this positional argument.
    pub new_authority: Option<Address>,
}

impl Serialize for SetProgramAuthority {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let authority = self.new_authority.map(|address| address.to_string());
        crate::wire::trailing(self.program_id.to_string(), authority.as_ref(), serializer)
    }
}

method!(SetProgramAuthority, "surfnet_setProgramAuthority", Value);

/// Writes a byte range to a program account.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WriteProgram {
    /// Program account.
    pub program_id: Address,
    /// Raw bytes; serialization emits lowercase hex.
    pub data: HexBytes,
    /// Exact byte offset.
    pub offset: u64,
    /// Optional trailing authority.
    pub authority: Option<Address>,
}

impl Serialize for WriteProgram {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        BorrowedWriteProgram {
            program_id: self.program_id,
            data: &self.data.0,
            offset: self.offset,
            authority: self.authority,
        }
        .serialize(serializer)
    }
}

method!(WriteProgram, "surfnet_writeProgram", Value);

/// Keeps deployment's borrowed chunks on the public request's exact wire path.
pub(crate) struct BorrowedWriteProgram<'a> {
    pub program_id: Address,
    pub data: &'a [u8],
    pub offset: u64,
    pub authority: Option<Address>,
}

impl Serialize for BorrowedWriteProgram<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut sequence =
            serializer.serialize_seq(Some(3 + usize::from(self.authority.is_some())))?;
        sequence.serialize_element(&self.program_id.to_string())?;
        sequence.serialize_element(&crate::hex::HexSlice(self.data))?;
        sequence.serialize_element(&self.offset)?;

        if let Some(authority) = self.authority {
            sequence.serialize_element(&authority.to_string())?;
        }

        sequence.end()
    }
}

impl Method for BorrowedWriteProgram<'_> {
    type Response = <WriteProgram as Method>::Response;
    const NAME: &'static str = <WriteProgram as Method>::NAME;

    fn decode(&self, result: &str) -> Result<Self::Response, Error> {
        crate::response::decode(result).map_err(Error::Schema)
    }
}
