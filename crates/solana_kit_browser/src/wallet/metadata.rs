use super::{BrowserFeature, WalletFailure, interop::*};
use js_sys::Object;
use solana_kit_wallet::{WalletAccount, WalletFeatures, WalletIcon, is_identifier};
use wasm_bindgen::{JsCast, JsValue};

#[derive(Clone)]
pub(super) struct Account {
    pub metadata: WalletAccount,
    pub raw: JsValue,
}

#[derive(Clone)]
pub(super) struct Snapshot {
    pub chains: Vec<String>,
    pub features: WalletFeatures,
    pub accounts: Vec<Account>,
}

impl Snapshot {
    pub fn read(raw: &JsValue) -> Result<Self, WalletFailure> {
        Ok(Self {
            chains: chains(raw)?,
            features: features(get(raw, "features")?)?,
            accounts: accounts(get(raw, "accounts")?)?,
        })
    }

    pub fn changed(&self, raw: &JsValue) -> Result<Self, WalletFailure> {
        let mut result = self.clone();

        if has(raw, "chains")? {
            result.chains = chains(raw)?;
        }

        if has(raw, "features")? {
            result.features = features(get(raw, "features")?)?;
        }

        if has(raw, "accounts")? {
            result.accounts = accounts(get(raw, "accounts")?)?;
        }

        Ok(result)
    }
}

fn chains(raw: &JsValue) -> Result<Vec<String>, WalletFailure> {
    let chains = strings(raw, "chains")?;

    if chains.iter().any(|chain| !is_identifier(chain)) {
        return Err(WalletFailure::invalid());
    }

    Ok(chains)
}

fn features(raw: JsValue) -> Result<WalletFeatures, WalletFailure> {
    let raw = object(raw)?
        .dyn_into::<Object>()
        .map_err(|_| WalletFailure::invalid())?;
    let mut features = WalletFeatures::new();

    for key in Object::keys(&raw).iter() {
        let key = key.as_string().ok_or_else(WalletFailure::invalid)?;
        let value = object(get(&raw, &key)?)?;
        let version = string(&value, "version")?;
        features
            .insert(
                key,
                BrowserFeature {
                    raw: value,
                    version,
                },
            )
            .map_err(|error| WalletFailure::caused("invalid wallet feature", error))?;
    }

    Ok(features)
}

pub(super) fn accounts(raw: JsValue) -> Result<Vec<Account>, WalletFailure> {
    let mut output: Vec<Account> = Vec::new();

    for raw in array(raw)?.iter() {
        let raw = object(raw)?;
        let metadata = account(&raw)?;

        if output
            .iter()
            .any(|entry| entry.metadata.address() == metadata.address())
        {
            return Err(WalletFailure::invalid());
        }

        output.push(Account { metadata, raw });
    }

    Ok(output)
}

fn account(raw: &JsValue) -> Result<WalletAccount, WalletFailure> {
    let public_key = bytes(raw, "publicKey")?
        .try_into()
        .map_err(|_| WalletFailure::invalid())?;
    let icon = optional_string(raw, "icon")?
        .map(WalletIcon::parse)
        .transpose()
        .map_err(|error| WalletFailure::caused("invalid account icon", error))?;
    WalletAccount::new(
        string(raw, "address")?,
        public_key,
        strings(raw, "chains")?,
        strings(raw, "features")?,
        optional_string(raw, "label")?,
        icon,
    )
    .map_err(|error| WalletFailure::caused("invalid wallet account", error))
}
