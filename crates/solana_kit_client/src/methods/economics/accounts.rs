//! Account economics, prioritization fees and validator voting state.

use super::{
    Address, Commitment, Contextual, Error, Method, address, addresses, config_method,
    optional_address, optional_config,
};
use serde::ser::{Error as _, SerializeSeq};
use serde::{Deserialize, Serialize, Serializer};

/// Largest-account supply partition.
#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum LargestAccountsFilter {
    /// Accounts counted as circulating supply.
    Circulating,
    /// Accounts outside circulating supply.
    NonCirculating,
}

/// Largest-account selection and optional ordering.
#[derive(Debug, Default, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LargestAccountsConfig {
    /// Node default when absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Both partitions when absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<LargestAccountsFilter>,
    /// Node sorting default when absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_results: Option<bool>,
}

/// One validated largest-account address and exact lamport balance.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct LargestAccount {
    /// Account address.
    #[serde(deserialize_with = "address")]
    pub address: Address,
    /// Exact balance in lamports.
    pub lamports: u64,
}

config_method!(
    LargestAccounts,
    LargestAccountsConfig,
    Contextual<Vec<LargestAccount>>,
    "getLargestAccounts"
);

/// getRecentPrioritizationFees with optional writable-account lock filtering.
///
/// At most128 addresses are accepted; None omits filtering and Some(empty)
/// sends an explicitly empty address array.
#[derive(Debug)]
pub struct RecentPrioritizationFees(pub Option<Vec<Address>>);

impl Serialize for RecentPrioritizationFees {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(usize::from(self.0.is_some())))?;

        if let Some(addresses) = &self.0 {
            if addresses.len() > 128 {
                return Err(S::Error::custom("at most 128 writable account addresses"));
            }

            let texts: Vec<_> = addresses.iter().map(ToString::to_string).collect();
            seq.serialize_element(&texts)?;
        }

        seq.end()
    }
}

impl Method for RecentPrioritizationFees {
    type Response = Vec<PrioritizationFee>;
    const NAME: &'static str = "getRecentPrioritizationFees";
    fn decode(&self, result: &str) -> Result<Self::Response, Error> {
        serde_json::from_str(result).map_err(Error::Schema)
    }
}

/// One slot's observed prioritization fee sample.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PrioritizationFee {
    /// Observation slot.
    pub slot: u64,
    /// Observed prioritization fee in micro-lamports per compute unit.
    pub prioritization_fee: u64,
}

/// Supply confirmation and account-list inclusion policy.
#[derive(Debug, Default, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SupplyConfig {
    /// Node default when absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Omitted uses the node default; explicit false remains on the wire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_non_circulating_accounts_list: Option<bool>,
}

/// Exact supply quantities and validated noncirculating account addresses.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SupplyValue {
    /// Total lamports.
    pub total: u64,
    /// Circulating lamports.
    pub circulating: u64,
    /// Noncirculating lamports.
    pub non_circulating: u64,
    /// Empty when the node excludes this list; a missing member is invalid.
    #[serde(deserialize_with = "addresses")]
    pub non_circulating_accounts: Vec<Address>,
}

config_method!(Supply, SupplyConfig, Contextual<SupplyValue>, "getSupply");

/// Vote-account confirmation, delinquency and address selection.
#[derive(Debug, Default, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VoteAccountsConfig {
    /// Node default when absent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<Commitment>,
    /// Slots behind the tip required for delinquency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delinquent_slot_distance: Option<u64>,
    /// Include unstaked delinquent accounts when true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_unstaked_delinquents: Option<bool>,
    /// Select one validated vote-account address.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "optional_address"
    )]
    pub vote_pubkey: Option<Address>,
}

/// Vote accounts partitioned by their node-reported delinquency state.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct VoteAccountsValue {
    /// Currently voting accounts.
    pub current: Vec<VoteAccount>,
    /// Delinquent accounts, including unstaked accounts only when requested.
    pub delinquent: Vec<VoteAccount>,
}

/// Complete vote-account record, retaining exact epoch credits.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VoteAccount {
    /// Validated vote account address.
    #[serde(deserialize_with = "address")]
    pub vote_pubkey: Address,
    /// Validated validator identity address.
    #[serde(deserialize_with = "address")]
    pub node_pubkey: Address,
    /// Delegated stake in lamports.
    pub activated_stake: u64,
    /// Legacy fraction numerator with denominator100; full wire u8 is preserved.
    pub commission: u8,
    /// Optional newer-node basis-point commission.
    pub inflation_rewards_commission_bps: Option<u16>,
    /// Whether this account is staked in the current epoch.
    pub epoch_vote_account: bool,
    /// Ordered (epoch, credits, previous credits) tuples.
    pub epoch_credits: Vec<(u64, u64, u64)>,
    /// Latest voted slot, or zero when no vote exists.
    pub last_vote: u64,
    /// Latest root slot, or zero when no root exists.
    pub root_slot: u64,
}

config_method!(
    VoteAccounts,
    VoteAccountsConfig,
    VoteAccountsValue,
    "getVoteAccounts"
);
