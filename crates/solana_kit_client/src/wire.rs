//! Wire-only account representation becomes owned core metadata at the boundary.
use crate::{
    Error,
    methods::{Context, Contextual},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use serde::Deserialize;
use solana_kit_core::{
    Address, Hash,
    accounts::{Account as OwnedAccount, MaybeAccount},
};

pub(crate) fn hash<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Hash, D::Error> {
    String::deserialize(d)?
        .parse()
        .map_err(serde::de::Error::custom)
}

fn address<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Address, D::Error> {
    String::deserialize(d)?
        .parse()
        .map_err(serde::de::Error::custom)
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Account {
    pub executable: bool,
    pub lamports: u64,
    #[serde(deserialize_with = "address")]
    pub owner: Address,
    pub rent_epoch: u64,
    pub space: u64,
    pub data: (String, Base64),
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub(crate) enum Base64 {
    #[serde(rename = "base64")]
    Base64,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
pub(crate) struct NullableContext<T> {
    pub context: Context,
    #[serde(deserialize_with = "crate::json::required_nullable")]
    pub value: Option<T>,
}

pub(crate) fn account(
    address: Address,
    result: &str,
) -> Result<Contextual<MaybeAccount<Vec<u8>>>, Error> {
    let wire: NullableContext<Account> = serde_json::from_str(result).map_err(Error::Schema)?;
    let value = match wire.value {
        None => MaybeAccount::Missing(address),
        Some(wire) => {
            let data = STANDARD
                .decode(wire.data.0)
                .map_err(|_| Error::AccountData)?;

            if u64::try_from(data.len()).ok() != Some(wire.space) {
                return Err(Error::AccountData);
            }

            MaybeAccount::Present(OwnedAccount {
                address,
                owner: wire.owner,
                lamports: wire.lamports,
                executable: wire.executable,
                space: wire.space,
                rent_epoch: Some(wire.rent_epoch),
                data,
            })
        }
    };
    Ok(Contextual {
        context: wire.context,
        value,
    })
}
