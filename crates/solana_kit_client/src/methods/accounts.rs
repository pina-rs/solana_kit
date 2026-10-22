//! Account reads preserve encoding, slicing, allocation, identity and context.
pub(crate) mod data;
mod filters;
pub(crate) mod requests;
use super::ReadConfig;
pub use data::{AccountData, AccountEncoding, ParsedAccount};
pub use filters::{Filter, FilterEncoding, Memcmp};
pub use requests::{
    AccountInfoWithConfig, BlockhashValid, CommitmentConfig, MinimumBalanceForRentExemption,
    MultipleAccounts, ProgramAccounts, ProgramAccountsConfig, ProgramAccountsResponse,
};
use serde::{Serialize, Serializer};

/// A byte range in an account's complete allocation.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct DataSlice {
    /// First requested byte.
    pub offset: u64,
    /// Maximum number of requested bytes.
    pub length: u64,
}

/// All account read options. An omitted encoding uses the node's legacy default.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AccountConfig {
    /// Requested confirmation and minimum evaluation slot.
    pub read: ReadConfig,
    /// Explicit representation, or the node default when absent.
    pub encoding: Option<AccountEncoding>,
    /// Byte range; incompatible with parsed JSON requests.
    pub data_slice: Option<DataSlice>,
}
impl Serialize for AccountConfig {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Wire<'a> {
            #[serde(flatten)]
            read: &'a ReadConfig,
            #[serde(skip_serializing_if = "Option::is_none")]
            encoding: Option<AccountEncoding>,
            #[serde(skip_serializing_if = "Option::is_none")]
            data_slice: Option<DataSlice>,
        }
        if self.data_slice.is_some_and(|slice| {
            self.encoding == Some(AccountEncoding::JsonParsed)
                || slice.offset.checked_add(slice.length).is_none()
        }) {
            return Err(serde::ser::Error::custom("invalid account data slice"));
        }

        Wire {
            read: &self.read,
            encoding: self.encoding,
            data_slice: self.data_slice,
        }
        .serialize(serializer)
    }
}
