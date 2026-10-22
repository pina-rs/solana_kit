//! Inflation request parameters and exact result schemas.

use super::{Commitment, CommitmentConfig, Error, Method, config_method, optional_config};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Serialize, Serializer};
use solana_kit_core::Address;

/// Complete current inflation governor parameters.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InflationGovernorValue {
    /// Initial inflation fraction.
    #[serde(deserialize_with = "nonnegative")]
    pub initial: f64,
    /// Long-term inflation fraction.
    #[serde(deserialize_with = "nonnegative")]
    pub terminal: f64,
    /// Annual reduction fraction.
    #[serde(deserialize_with = "nonnegative")]
    pub taper: f64,
    /// Foundation allocation fraction.
    #[serde(deserialize_with = "nonnegative")]
    pub foundation: f64,
    /// Foundation allocation duration in years.
    #[serde(deserialize_with = "nonnegative")]
    pub foundation_term: f64,
}

config_method!(
    InflationGovernor,
    CommitmentConfig,
    InflationGovernorValue,
    "getInflationGovernor"
);

/// Effective current-epoch inflation rates.
#[derive(Debug, Deserialize, PartialEq)]
pub struct InflationRateValue {
    /// Combined inflation fraction.
    #[serde(deserialize_with = "nonnegative")]
    pub total: f64,
    /// Validator inflation fraction.
    #[serde(deserialize_with = "nonnegative")]
    pub validator: f64,
    /// Foundation inflation fraction.
    #[serde(deserialize_with = "nonnegative")]
    pub foundation: f64,
    /// Exact epoch of evaluation.
    pub epoch: u64,
}

/// getInflationRate has no parameters.
#[derive(Debug)]
pub struct InflationRate;

impl Serialize for InflationRate {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_seq(Some(0))?.end()
    }
}

impl Method for InflationRate {
    type Response = InflationRateValue;
    const NAME: &'static str = "getInflationRate";
    fn decode(&self, result: &str) -> Result<Self::Response, Error> {
        serde_json::from_str(result).map_err(Error::Schema)
    }
}

/// Optional epoch, confirmation and stale-read prevention constraints.
#[derive(Debug, Default, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InflationRewardConfig {
    /// Node default when absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Previous epoch when absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<u64>,
    /// Minimum acceptable node slot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_context_slot: Option<u64>,
}

/// getInflationReward preserves input order and nullable reward positions.
#[derive(Debug)]
pub struct InflationReward(pub Vec<Address>, pub Option<InflationRewardConfig>);

impl Serialize for InflationReward {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(1 + usize::from(self.1.is_some())))?;
        let addresses: Vec<_> = self.0.iter().map(ToString::to_string).collect();
        seq.serialize_element(&addresses)?;

        if let Some(config) = &self.1 {
            seq.serialize_element(config)?;
        }

        seq.end()
    }
}

impl Method for InflationReward {
    type Response = Vec<Option<InflationRewardValue>>;
    const NAME: &'static str = "getInflationReward";
    fn decode(&self, result: &str) -> Result<Self::Response, Error> {
        let values: Self::Response = serde_json::from_str(result).map_err(Error::Schema)?;

        if values.len() != self.0.len() {
            return Err(Error::Schema(serde::de::Error::custom(
                "reward result length must match input addresses",
            )));
        }

        Ok(values)
    }
}

/// A reward credited for one address; absence is represented by the outer Option.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InflationRewardValue {
    /// Reward epoch.
    pub epoch: u64,
    /// Slot at which the reward was credited.
    pub effective_slot: u64,
    /// Reward in lamports.
    pub amount: u64,
    /// Account balance after credit, in lamports.
    pub post_balance: u64,
    /// Nullable historical commission; null is different from zero.
    #[serde(deserialize_with = "nullable_commission")]
    pub commission: Option<u8>,
    /// Newer nodes' optional basis-point commission, retained when available.
    pub commission_bps: Option<u16>,
}

fn nullable_commission<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<u8>, D::Error> {
    Option::<u8>::deserialize(d)
}

fn nonnegative<'de, D: serde::Deserializer<'de>>(d: D) -> Result<f64, D::Error> {
    let value = f64::deserialize(d)?;

    if !value.is_finite() || value < 0.0 {
        return Err(serde::de::Error::custom(
            "expected finite nonnegative quantity",
        ));
    }

    Ok(value)
}
