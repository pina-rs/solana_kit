//! Config program payloads.
//!
//! Source: `packages/solana_kit_rpc_parsed_types/lib/src/config_accounts.dart`.
//! Opacity is intentional: the reference model declares `configData` as
//! `Object?`, so its exact JSON value is retained without inventing a schema.

use super::{ParsedAccountData, scalars};
use crate::Error;
use serde::Deserialize;
use serde_json::value::RawValue;
use solana_kit_core::Address;

pub(super) fn decode(program: &str, parsed: &RawValue) -> Result<ParsedAccountData, Error> {
    let envelope = super::envelope(parsed)?;

    let account = match envelope.type_name {
        "stakeConfig" => ConfigProgramAccount::StakeConfig(super::info(&envelope)?),
        "validatorInfo" => ConfigProgramAccount::ValidatorInfo(super::info(&envelope)?),
        other => return super::unknown_type(program, other, parsed),
    };

    Ok(ParsedAccountData::Config(account))
}

/// Reference `JsonParsedConfigProgramAccount`.
#[derive(Debug, PartialEq)]
pub enum ConfigProgramAccount {
    /// `type: "stakeConfig"`, a shape only older validators report.
    StakeConfig(StakeConfigInfo),
    /// `type: "validatorInfo"`.
    ValidatorInfo(ValidatorInfoData),
}

/// Reference `JsonParsedStakeConfigInfo`.
///
/// Agave 4.1.0 and later no longer parse this account and fall back to annotated
/// base64, so these members remain for valid responses from older validators.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StakeConfigInfo {
    /// Slash penalty percentage.
    pub slash_penalty: u8,
    /// Warmup/cooldown rate.
    pub warmup_cooldown_rate: f64,
}

/// Reference `JsonParsedValidatorInfoData`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatorInfoData {
    /// Arbitrary validator configuration retained as its exact JSON value.
    pub config_data: Box<RawValue>,
    /// Keys controlling the configuration account.
    pub keys: Vec<ValidatorInfoKey>,
}

/// Reference `JsonParsedValidatorInfoKey`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ValidatorInfoKey {
    /// Key address.
    #[serde(deserialize_with = "scalars::required_address")]
    pub pubkey: Address,
    /// Whether the key must sign configuration changes.
    pub signer: bool,
}

impl PartialEq for ValidatorInfoData {
    fn eq(&self, other: &Self) -> bool {
        self.keys == other.keys && self.config_data.get() == other.config_data.get()
    }
}
