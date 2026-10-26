//! V2 pages preserve the provider cursor and optional RPC evaluation context.

use serde::{Deserialize, Deserializer};
use solana_kit_client::methods::Contextual;
use std::collections::BTreeMap;

/// A provider page returned directly or together with RPC evaluation context.
#[derive(Debug, PartialEq)]
pub enum AccountPage {
    /// The method returned its page without a context wrapper.
    Plain(AccountPageData),
    /// The method included the evaluation context requested by the caller.
    WithContext(Contextual<AccountPageData>),
}

impl<'de> Deserialize<'de> for AccountPage {
    fn deserialize<D: Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
        let raw = Box::<serde_json::value::RawValue>::deserialize(decoder)?;
        let fields: BTreeMap<String, Box<serde_json::value::RawValue>> =
            serde_json::from_str(raw.get()).map_err(serde::de::Error::custom)?;

        // A malformed contextual response must not fall through to an unrelated plain page.
        if fields.contains_key("value") || fields.contains_key("context") {
            serde_json::from_str(raw.get()).map(Self::WithContext)
        } else {
            serde_json::from_str(raw.get()).map(Self::Plain)
        }
        .map_err(serde::de::Error::custom)
    }
}

impl AccountPage {
    /// Borrows the page while leaving evaluation context available to callers.
    pub const fn page(&self) -> &AccountPageData {
        match self {
            Self::Plain(page) => page,
            Self::WithContext(contextual) => &contextual.value,
        }
    }

    /// Consumes the response when a collector needs only the records and cursor.
    pub fn into_page(self) -> AccountPageData {
        match self {
            Self::Plain(page) => page,
            Self::WithContext(contextual) => contextual.value,
        }
    }
}

model!(
    "A V2 account page. The cursor must be present, with null marking the last page.",
    AccountPageData {
        accounts: Vec<ProgramAccountV2> => "accounts",
        #[serde(deserialize_with = "serde::Deserialize::deserialize")]
        pagination_key: Option<String> => "paginationKey",
    } optional { total_results: u64 => "totalResults" }
);

model!(
    "An account record with typed metadata and its program-defined wire data.",
    ProgramAccountV2 {
        pubkey: String => "pubkey",
        account: AccountInfoV2 => "account",
    } optional {}
);

model!(
    "V2 account metadata; encoded or parsed data remains in its provider wire form.",
    AccountInfoV2 {
        lamports: u64 => "lamports",
        owner: String => "owner",
        data: serde_json::Value => "data",
        executable: bool => "executable",
        rent_epoch: u64 => "rentEpoch",
    } optional { space: u64 => "space" }
);

/// Token-account records have the same metadata contract as program-account records.
pub type TokenAccountV2 = ProgramAccountV2;
/// A program-account page, with or without evaluation context.
pub type GetProgramAccountsV2Response = AccountPage;
/// A token-account page, with or without evaluation context.
pub type GetTokenAccountsByOwnerV2Response = AccountPage;
