//! A historical balance query selects exactly one provider evaluation point.

use super::GetBalanceAtRequested;
use serde::{Deserialize, Serialize};

/// One historical selector; datetime interpretation belongs to the provider.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BalanceAtPoint {
    /// Unix seconds.
    Time(i64),
    /// A provider-supported datetime string, interpreted as UTC unless it has a timezone.
    Datetime(String),
    /// An exact slot.
    Slot(u64),
}

impl BalanceAtPoint {
    pub(super) fn matches(&self, requested: &GetBalanceAtRequested) -> bool {
        match self {
            Self::Time(time) => {
                requested.time == Some(*time)
                    && requested.slot.is_none()
                    && requested.datetime.is_none()
            }
            Self::Datetime(datetime) => {
                requested.datetime.as_ref() == Some(datetime)
                    && requested.time.is_some()
                    && requested.slot.is_none()
            }
            Self::Slot(slot) => {
                requested.slot == Some(*slot)
                    && requested.time.is_none()
                    && requested.datetime.is_none()
            }
        }
    }
}

/// A historical wallet balance request with one required evaluation point.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "BalanceAtFields")]
pub struct GetBalanceAtRequest {
    /// Wallet address used as one opaque path segment.
    pub wallet: String,
    /// Token mint, including the provider's native SOL pseudo-mint when appropriate.
    pub mint: String,
    /// Exactly one historical selector.
    #[serde(flatten)]
    pub point: BalanceAtPoint,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BalanceAtFields {
    wallet: String,
    mint: String,
    time: Option<i64>,
    datetime: Option<String>,
    slot: Option<u64>,
}

impl TryFrom<BalanceAtFields> for GetBalanceAtRequest {
    type Error = &'static str;

    fn try_from(value: BalanceAtFields) -> Result<Self, Self::Error> {
        let point = match (value.time, value.datetime, value.slot) {
            (Some(time), None, None) => BalanceAtPoint::Time(time),
            (None, Some(datetime), None) => BalanceAtPoint::Datetime(datetime),
            (None, None, Some(slot)) => BalanceAtPoint::Slot(slot),
            _ => return Err("provide exactly one historical time, datetime or slot"),
        };

        Ok(Self {
            wallet: value.wallet,
            mint: value.mint,
            point,
        })
    }
}
