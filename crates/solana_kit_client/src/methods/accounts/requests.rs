//! Standard request schemas select their own response shape.
use super::{
    AccountConfig, AccountData, Filter,
    data::{self, WireAccount},
};
use crate::{
    Error,
    methods::{Commitment, Contextual, Method, ReadConfig},
};
use serde::{Deserialize, Serialize, Serializer, ser::SerializeSeq};
use solana_kit_core::{
    Address, Hash,
    accounts::{Account, MaybeAccount},
};

pub(crate) fn params<S: Serializer, C: Serialize>(
    serializer: S,
    address: &str,
    config: &Option<C>,
) -> Result<S::Ok, S::Error> {
    let mut seq = serializer.serialize_seq(Some(1 + usize::from(config.is_some())))?;
    seq.serialize_element(address)?;

    if let Some(config) = config {
        seq.serialize_element(config)?;
    }

    seq.end()
}

/// getAccountInfo with alternate encodings and optional slicing.
#[derive(Debug)]
pub struct AccountInfoWithConfig(pub Address, pub Option<AccountConfig>);
impl Serialize for AccountInfoWithConfig {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        params(s, &self.0.to_string(), &self.1)
    }
}
impl Method for AccountInfoWithConfig {
    type Response = Contextual<MaybeAccount<AccountData>>;
    const NAME: &'static str = "getAccountInfo";
    fn decode(&self, result: &str) -> Result<Self::Response, Error> {
        let wire: crate::wire::NullableContext<WireAccount> =
            serde_json::from_str(result).map_err(Error::Schema)?;
        Ok(Contextual {
            context: wire.context,
            value: data::maybe(wire.value, self.0, self.1.unwrap_or_default())?,
        })
    }
}

/// getMultipleAccounts, preserving requested order and absent account identity.
#[derive(Debug)]
pub struct MultipleAccounts(pub Vec<Address>, pub Option<AccountConfig>);
impl Serialize for MultipleAccounts {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        if self.0.len() > 100 {
            return Err(serde::ser::Error::custom("at most100 account addresses"));
        }

        let mut seq = s.serialize_seq(Some(1 + usize::from(self.1.is_some())))?;
        seq.serialize_element(&self.0.iter().map(ToString::to_string).collect::<Vec<_>>())?;

        if let Some(config) = self.1 {
            seq.serialize_element(&config)?;
        }

        seq.end()
    }
}
impl Method for MultipleAccounts {
    type Response = Contextual<Vec<MaybeAccount<AccountData>>>;
    const NAME: &'static str = "getMultipleAccounts";
    fn decode(&self, result: &str) -> Result<Self::Response, Error> {
        let wire: Contextual<Vec<Option<WireAccount>>> =
            serde_json::from_str(result).map_err(Error::Schema)?;

        if wire.value.len() != self.0.len() {
            return Err(Error::AccountData);
        }

        let mut config = self.1.unwrap_or_default();
        config
            .encoding
            .get_or_insert(super::AccountEncoding::Base64);
        let value = wire
            .value
            .into_iter()
            .zip(&self.0)
            .map(|(wire, address)| data::maybe(wire, *address, config))
            .collect::<Result<_, _>>()?;
        Ok(Contextual {
            context: wire.context,
            value,
        })
    }
}

/// getProgramAccounts options, including explicit contextual response choice.
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramAccountsConfig {
    /// Encoding, slicing and evaluation constraints.
    #[serde(flatten)]
    pub account: AccountConfig,
    /// Conjunctive filters; omission differs from an empty list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// Request a context wrapper when true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_context: Option<bool>,
}
/// getProgramAccounts program identity and optional configuration.
#[derive(Debug)]
pub struct ProgramAccounts(pub Address, pub Option<ProgramAccountsConfig>);
impl Serialize for ProgramAccounts {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        const MAX_FILTERS: usize = 4;

        if self
            .1
            .as_ref()
            .and_then(|config| config.filters.as_ref())
            .is_some_and(|filters| filters.len() > MAX_FILTERS)
        {
            return Err(serde::ser::Error::custom(
                "at most four program account filters",
            ));
        }

        params(s, &self.0.to_string(), &self.1)
    }
}
/// Shape selected by getProgramAccounts' withContext configuration.
#[derive(Debug)]
pub enum ProgramAccountsResponse {
    /// Ordinary result without an evaluation context.
    Accounts(Vec<Account<AccountData>>),
    /// Explicitly requested context wrapper.
    Contextual(Contextual<Vec<Account<AccountData>>>),
}
#[derive(Deserialize)]
struct KeyedAccount {
    #[serde(deserialize_with = "data::address")]
    pubkey: Address,
    account: WireAccount,
}
pub(crate) fn keyed(
    result: &str,
    config: AccountConfig,
    owner: Option<Address>,
) -> Result<Vec<Account<AccountData>>, Error> {
    let wire: Vec<KeyedAccount> = serde_json::from_str(result).map_err(Error::Schema)?;
    let mut seen = std::collections::BTreeSet::new();
    wire.into_iter()
        .map(|entry| {
            if !seen.insert(entry.pubkey.to_string()) {
                return Err(Error::AccountData);
            }

            let account = entry.account.owned(entry.pubkey, config)?;
            if owner.is_some_and(|owner| owner != account.owner) {
                return Err(Error::AccountData);
            }

            Ok(account)
        })
        .collect()
}
impl Method for ProgramAccounts {
    type Response = ProgramAccountsResponse;
    const NAME: &'static str = "getProgramAccounts";
    fn decode(&self, result: &str) -> Result<Self::Response, Error> {
        let config = self.1.as_ref();
        let account = config.map_or_else(AccountConfig::default, |c| c.account);

        if config.and_then(|c| c.with_context) == Some(true) {
            let wire: Contextual<Box<serde_json::value::RawValue>> =
                serde_json::from_str(result).map_err(Error::Schema)?;
            return Ok(ProgramAccountsResponse::Contextual(Contextual {
                context: wire.context,
                value: keyed(wire.value.get(), account, Some(self.0))?,
            }));
        }

        Ok(ProgramAccountsResponse::Accounts(keyed(
            result,
            account,
            Some(self.0),
        )?))
    }
}

/// Commitment-only configuration for rent and token queries.
#[derive(Debug, Default, Clone, Copy, Serialize)]
pub struct CommitmentConfig {
    /// Required confirmation level, omitted for node default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
}
/// getMinimumBalanceForRentExemption complete allocation size.
#[derive(Debug)]
pub struct MinimumBalanceForRentExemption(pub u64, pub Option<CommitmentConfig>);
impl Serialize for MinimumBalanceForRentExemption {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut seq = s.serialize_seq(Some(1 + usize::from(self.1.is_some())))?;
        seq.serialize_element(&self.0)?;

        if let Some(config) = self.1 {
            seq.serialize_element(&config)?;
        }

        seq.end()
    }
}
impl Method for MinimumBalanceForRentExemption {
    type Response = u64;
    const NAME: &'static str = "getMinimumBalanceForRentExemption";
    fn decode(&self, result: &str) -> Result<u64, Error> {
        serde_json::from_str(result).map_err(Error::Schema)
    }
}
/// isBlockhashValid exact hash and optional evaluation constraints.
#[derive(Debug)]
pub struct BlockhashValid(pub Hash, pub Option<ReadConfig>);
impl Serialize for BlockhashValid {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        params(s, &self.0.to_string(), &self.1)
    }
}
impl Method for BlockhashValid {
    type Response = Contextual<bool>;
    const NAME: &'static str = "isBlockhashValid";
    fn decode(&self, result: &str) -> Result<Self::Response, Error> {
        serde_json::from_str(result).map_err(Error::Schema)
    }
}
