//! V2 account queries use positional arguments and typed account filters.

use crate::enums::CommitmentLevel;
use serde::{Deserialize, Serialize, Serializer, ser::SerializeSeq};

address_request!(
    "Read a program-account page with [programId, config?] parameters.",
    GetProgramAccountsV2Request,
    program_address,
    GetProgramAccountsV2Config
);

request!(
    #[derive(Default)]
    "Program-account filters, context and cursor settings.",
    GetProgramAccountsV2Config {} optional {
        commitment: CommitmentLevel => "commitment",
        min_context_slot: u64 => "minContextSlot",
        with_context: bool => "withContext",
        encoding: AccountEncoding => "encoding",
        data_slice: DataSlice => "dataSlice",
        limit: u64 => "limit",
        pagination_key: String => "paginationKey",
        changed_since_slot: u64 => "changedSinceSlot",
        filters: Vec<AccountFilter> => "filters",
    }
);

request!(
    #[derive(Default)]
    "Token-account context and cursor settings, separate from the filter argument.",
    GetTokenAccountsByOwnerV2Config {} optional {
        commitment: CommitmentLevel => "commitment",
        min_context_slot: u64 => "minContextSlot",
        encoding: AccountEncoding => "encoding",
        data_slice: DataSlice => "dataSlice",
        limit: u64 => "limit",
        pagination_key: String => "paginationKey",
        changed_since_slot: u64 => "changedSinceSlot",
    }
);

request!(
    #[derive(Default)]
    "Token-account filters occupy the second positional argument.",
    TokenAccountFilter {} optional {
        mint: String => "mint",
        program_id: String => "programId",
    }
);

/// A token-account query with distinct owner, filter and configuration positions.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(from = "OwnerArguments")]
pub struct GetTokenAccountsByOwnerV2Request {
    /// Wallet whose token accounts are requested.
    pub owner_address: String,
    /// Optional mint or token-program filter.
    pub filter: Option<TokenAccountFilter>,
    /// Optional page and context settings, always in the third argument.
    pub config: Option<GetTokenAccountsByOwnerV2Config>,
}

#[derive(Deserialize)]
struct OwnerArguments(
    String,
    #[serde(default)] Option<TokenAccountFilter>,
    #[serde(default)] Option<GetTokenAccountsByOwnerV2Config>,
);

impl From<OwnerArguments> for GetTokenAccountsByOwnerV2Request {
    fn from(OwnerArguments(owner_address, filter, config): OwnerArguments) -> Self {
        Self {
            owner_address,
            filter,
            config,
        }
    }
}

impl Serialize for GetTokenAccountsByOwnerV2Request {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let has_filter = self.filter.is_some() || self.config.is_some();
        let length = 1 + usize::from(has_filter) + usize::from(self.config.is_some());
        let mut values = serializer.serialize_seq(Some(length))?;
        values.serialize_element(&self.owner_address)?;

        if has_filter {
            // An empty filter keeps configuration in position three.
            values.serialize_element(
                self.filter
                    .as_ref()
                    .unwrap_or(&TokenAccountFilter::default()),
            )?;
        }

        if let Some(config) = &self.config {
            values.serialize_element(config)?;
        }

        values.end()
    }
}

/// Account encodings accepted by the provider's V2 account methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccountEncoding {
    /// Program-defined JSON, retaining the provider's parsed data.
    #[serde(rename = "jsonParsed")]
    JsonParsed,
    /// Base58-encoded account bytes.
    #[serde(rename = "base58")]
    Base58,
    /// Base64-encoded account bytes.
    #[serde(rename = "base64")]
    Base64,
    /// Base64-wrapped compressed data; no decompression is implied.
    #[serde(rename = "base64+zstd")]
    Base64Zstd,
}

request!(
    "A byte range within an account's allocation.",
    DataSlice { offset: u64 => "offset", length: u64 => "length" } optional {}
);

/// A size or byte-comparison account filter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AccountFilter {
    /// Match the account's allocation size in bytes.
    DataSize(u64),
    /// Match the base58 bytes at the specified offset.
    Memcmp(MemcmpFilter),
}

request!(
    #[derive(Eq)]
    "A base58 byte comparison at an offset in account data.",
    MemcmpFilter { offset: u64 => "offset", bytes: String => "bytes" } optional {}
);
