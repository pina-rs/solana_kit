//! Stake program payloads.
//!
//! Source: `packages/solana_kit_rpc_parsed_types/lib/src/stake_accounts.dart`.

use super::{ParsedAccountData, scalars};
use crate::Error;
use serde::Deserialize;
use serde_json::value::RawValue;
use solana_kit_core::Address;

pub(super) fn decode(program: &str, parsed: &RawValue) -> Result<ParsedAccountData, Error> {
    let envelope = super::envelope(parsed)?;

    let account = match envelope.type_name {
        "delegated" => StakeProgramAccount::Delegated(super::info(&envelope)?),
        "initialized" => StakeProgramAccount::Initialized(super::info(&envelope)?),
        other => return super::unknown_type(program, other, parsed),
    };

    Ok(ParsedAccountData::Stake(account))
}

/// Reference `JsonParsedStakeProgramAccount`, discriminated by the node type.
#[derive(Debug, PartialEq)]
pub enum StakeProgramAccount {
    /// `type: "delegated"`, with stake delegation data.
    Delegated(StakeAccountInfo),
    /// `type: "initialized"`, without delegation when `stake` is null.
    Initialized(StakeAccountInfo),
}

/// Reference `JsonParsedStakeAccountInfo`.
#[derive(Debug, Deserialize, PartialEq)]
pub struct StakeAccountInfo {
    /// Stake account metadata, authorization and lockup.
    pub meta: StakeMeta,
    /// Delegation data when the stake is delegated.
    #[serde(default)]
    pub stake: Option<StakeData>,
}

/// Reference `JsonParsedStakeMeta`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct StakeMeta {
    /// Staker and withdrawer authorization.
    pub authorized: StakeAuthorized,
    /// Lockup configuration.
    pub lockup: StakeLockup,
    /// Rent-exempt reserve reported as unsigned decimal text.
    #[serde(deserialize_with = "scalars::unsigned_decimal")]
    pub rent_exempt_reserve: u64,
}

/// Reference `JsonParsedStakeAuthorized`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct StakeAuthorized {
    /// Address authorized to manage the stake.
    #[serde(deserialize_with = "scalars::required_address")]
    pub staker: Address,
    /// Address authorized to withdraw lamports.
    #[serde(deserialize_with = "scalars::required_address")]
    pub withdrawer: Address,
}

/// Reference `JsonParsedStakeLockup`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct StakeLockup {
    /// Address allowed to change the lockup.
    #[serde(deserialize_with = "scalars::required_address")]
    pub custodian: Address,
    /// Epoch at which the lockup expires.
    pub epoch: u64,
    /// Unix timestamp at which the lockup expires.
    pub unix_timestamp: i64,
}

/// Reference `JsonParsedStakeData`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StakeData {
    /// Exact credits observed at delegation.
    pub credits_observed: u64,
    /// Delegation destination and activation state.
    pub delegation: StakeDelegation,
}

/// Reference `JsonParsedStakeDelegation`.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StakeDelegation {
    /// Epoch in which the stake activated, reported as decimal text.
    #[serde(deserialize_with = "scalars::unsigned_decimal")]
    pub activation_epoch: u64,
    /// Epoch in which the stake deactivated, reported as decimal text.
    #[serde(deserialize_with = "scalars::unsigned_decimal")]
    pub deactivation_epoch: u64,
    /// Delegated quantity reported as decimal text.
    #[serde(deserialize_with = "scalars::unsigned_decimal")]
    pub stake: u64,
    /// Vote account receiving the delegation.
    #[serde(deserialize_with = "scalars::required_address")]
    pub voter: Address,
    /// Legacy warmup/cooldown rate reported before Agave 4.1.0.
    #[serde(default)]
    pub warmup_cooldown_rate: Option<f64>,
}
