//! Token reads retain the exact raw amount independently of presentation values.
pub use super::accounts::CommitmentConfig;
use super::{
    Contextual, Method,
    accounts::{
        AccountConfig, AccountData,
        requests::{keyed, params},
    },
};
use crate::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer, ser::SerializeSeq};
use solana_kit_core::{Address, accounts::Account};

/// Exact raw token quantity, serialized by the node as an unsigned decimal string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TokenAmount(pub u64);
impl<'de> Deserialize<'de> for TokenAmount {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let text = String::deserialize(d)?;

        if text.is_empty() || !text.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(serde::de::Error::custom(
                "unsigned decimal token amount required",
            ));
        }

        text.parse().map(Self).map_err(serde::de::Error::custom)
    }
}

/// Exact amount and node-supplied display fields; display never determines amount.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalance {
    /// Exact raw integer quantity, independent of decimals.
    pub amount: TokenAmount,
    /// Mint scale as an unsigned byte.
    pub decimals: u8,
    /// Finite approximate presentation value, or explicit null.
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub ui_amount: Option<f64>,
    /// Node-supplied exact presentation spelling, including scaled UI semantics.
    pub ui_amount_string: String,
}

/// Token account and its node-supplied exact balance.
#[derive(Debug, PartialEq)]
pub struct LargestTokenAccount {
    /// Validated account address.
    pub address: Address,
    /// Exact amount and presentation metadata.
    pub balance: TokenBalance,
}

/// getTokenAccountBalance account and optional confirmation.
#[derive(Debug)]
pub struct TokenAccountBalance(pub Address, pub Option<CommitmentConfig>);
/// getTokenSupply mint and optional confirmation.
#[derive(Debug)]
pub struct TokenSupply(pub Address, pub Option<CommitmentConfig>);
/// getTokenLargestAccounts mint and optional confirmation.
#[derive(Debug)]
pub struct TokenLargestAccounts(pub Address, pub Option<CommitmentConfig>);
macro_rules! balance_method {
    ($type:ident, $name:literal, $response:ty) => {
        impl Serialize for $type {
            fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                params(s, &self.0.to_string(), &self.1)
            }
        }
        impl Method for $type {
            type Response = $response;
            const NAME: &'static str = $name;
            fn decode(&self, result: &str) -> Result<Self::Response, Error> {
                serde_json::from_str(result).map_err(Error::Schema)
            }
        }
    };
}
balance_method!(
    TokenAccountBalance,
    "getTokenAccountBalance",
    Contextual<TokenBalance>
);
balance_method!(TokenSupply, "getTokenSupply", Contextual<TokenBalance>);
balance_method!(
    TokenLargestAccounts,
    "getTokenLargestAccounts",
    Contextual<Vec<LargestTokenAccount>>
);

/// Exactly one token account selector; mint and programId cannot conflict.
#[derive(Debug, Clone, Copy)]
pub enum TokenAccountFilter {
    /// Select one mint.
    Mint(Address),
    /// Select accounts belonging to one token program.
    ProgramId(Address),
}
impl Serialize for TokenAccountFilter {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        enum Wire {
            Mint(String),
            ProgramId(String),
        }

        match self {
            Self::Mint(address) => Wire::Mint(address.to_string()),
            Self::ProgramId(address) => Wire::ProgramId(address.to_string()),
        }
        .serialize(s)
    }
}
/// getTokenAccountsByOwner exact owner, selector and account representation.
#[derive(Debug)]
pub struct TokenAccountsByOwner(
    pub Address,
    pub TokenAccountFilter,
    pub Option<AccountConfig>,
);
/// getTokenAccountsByDelegate exact delegate, selector and account representation.
#[derive(Debug)]
pub struct TokenAccountsByDelegate(
    pub Address,
    pub TokenAccountFilter,
    pub Option<AccountConfig>,
);
macro_rules! accounts_method {
    ($type:ident, $name:literal) => {
        impl Serialize for $type {
            fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                accounts_params(s, self.0, self.1, self.2)
            }
        }
        impl Method for $type {
            type Response = Contextual<Vec<Account<AccountData>>>;
            const NAME: &'static str = $name;
            fn decode(&self, result: &str) -> Result<Self::Response, Error> {
                accounts_decode(result, self.1, self.2)
            }
        }
    };
}
accounts_method!(TokenAccountsByOwner, "getTokenAccountsByOwner");
accounts_method!(TokenAccountsByDelegate, "getTokenAccountsByDelegate");

impl<'de> Deserialize<'de> for LargestTokenAccount {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Identity {
            #[serde(deserialize_with = "super::accounts::data::address")]
            address: Address,
        }
        // RawValue avoids serde's flatten buffer normalizing arbitrary-precision numbers.
        let raw = Box::<serde_json::value::RawValue>::deserialize(d)?;
        let identity: Identity =
            serde_json::from_str(raw.get()).map_err(serde::de::Error::custom)?;
        let balance = serde_json::from_str(raw.get()).map_err(serde::de::Error::custom)?;
        Ok(Self {
            address: identity.address,
            balance,
        })
    }
}

fn accounts_params<S: Serializer>(
    s: S,
    address: Address,
    filter: TokenAccountFilter,
    config: Option<AccountConfig>,
) -> Result<S::Ok, S::Error> {
    let mut seq = s.serialize_seq(Some(2 + usize::from(config.is_some())))?;
    seq.serialize_element(&address.to_string())?;
    seq.serialize_element(&filter)?;

    if let Some(config) = config {
        seq.serialize_element(&config)?;
    }

    seq.end()
}
fn accounts_decode(
    result: &str,
    filter: TokenAccountFilter,
    config: Option<AccountConfig>,
) -> Result<Contextual<Vec<Account<AccountData>>>, Error> {
    let wire: Contextual<Box<serde_json::value::RawValue>> =
        serde_json::from_str(result).map_err(Error::Schema)?;

    let owner = match filter {
        TokenAccountFilter::ProgramId(owner) => Some(owner),
        _ => None,
    };

    Ok(Contextual {
        context: wire.context,
        value: keyed(wire.value.get(), config.unwrap_or_default(), owner)?,
    })
}
