//! Vote account payloads.
//!
//! Source: `packages/solana_kit_rpc_parsed_types/lib/src/vote_accounts.dart`.
//! The pinned model exposes the info payload directly, so the node
//! `type: "vote"` discriminator is validated but not repeated in the variant.

use super::{ParsedAccountData, scalars};
use crate::Error;
use serde::Deserialize;
use serde_json::value::RawValue;
use solana_kit_core::Address;

pub(super) fn decode(program: &str, parsed: &RawValue) -> Result<ParsedAccountData, Error> {
    let envelope = super::envelope(parsed)?;

    if envelope.type_name != "vote" {
        return super::unknown_type(program, envelope.type_name, parsed);
    }

    Ok(ParsedAccountData::Vote(super::info(&envelope)?))
}

/// Reference `JsonParsedVoteInfo`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VoteInfo {
    /// Authorized voters and the epoch each authorization began.
    pub authorized_voters: Vec<AuthorizedVoter>,
    /// Address authorized to withdraw from the vote account.
    #[serde(deserialize_with = "scalars::required_address")]
    pub authorized_withdrawer: Address,
    /// Optional block revenue collector reported by Agave v4 vote state.
    #[serde(default, deserialize_with = "scalars::optional_address")]
    pub block_revenue_collector: Option<Address>,
    /// Optional block revenue commission in basis points.
    #[serde(default)]
    pub block_revenue_commission_bps: Option<u16>,
    /// Optional compressed BLS public key text.
    #[serde(default)]
    pub bls_pubkey_compressed: Option<String>,
    /// Inflation rewards commission as a whole percentage.
    pub commission: u8,
    /// Epoch credits in node order.
    pub epoch_credits: Vec<EpochCredit>,
    /// Optional inflation rewards collector reported by Agave v4 vote state.
    #[serde(default, deserialize_with = "scalars::optional_address")]
    pub inflation_rewards_collector: Option<Address>,
    /// Optional inflation rewards commission in basis points.
    #[serde(default)]
    pub inflation_rewards_commission_bps: Option<u16>,
    /// Last vote timestamp recorded by the account.
    pub last_timestamp: LastTimestamp,
    /// Node identity that produced the votes.
    #[serde(deserialize_with = "scalars::required_address")]
    pub node_pubkey: Address,
    /// Optional pending delegator rewards reported as decimal text.
    #[serde(default, deserialize_with = "scalars::optional_unsigned_decimal")]
    pub pending_delegator_rewards: Option<u64>,
    /// Prior authorized voters retained by the node.
    pub prior_voters: Vec<PriorVoter>,
    /// Optional rooted slot.
    #[serde(default)]
    pub root_slot: Option<u64>,
    /// Landed votes in node order.
    pub votes: Vec<Vote>,
}

/// Reference `JsonParsedAuthorizedVoter`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizedVoter {
    /// Voter address authorized for this epoch.
    #[serde(deserialize_with = "scalars::required_address")]
    pub authorized_voter: Address,
    /// Epoch at which the authorization began.
    pub epoch: u64,
}

/// Reference `JsonParsedEpochCredit`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EpochCredit {
    /// Credits earned through this epoch, reported as decimal text.
    #[serde(deserialize_with = "scalars::unsigned_decimal")]
    pub credits: u64,
    /// Epoch number.
    pub epoch: u64,
    /// Credits reported for the previous epoch, reported as decimal text.
    #[serde(deserialize_with = "scalars::unsigned_decimal")]
    pub previous_credits: u64,
}

/// Reference `JsonParsedLastTimestamp`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct LastTimestamp {
    /// Slot at which the timestamp was recorded.
    pub slot: u64,
    /// Recorded Unix timestamp.
    pub timestamp: i64,
}

/// Reference `JsonParsedPriorVoter`.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PriorVoter {
    /// Address of the prior authorized voter.
    #[serde(deserialize_with = "scalars::required_address")]
    pub authorized_pubkey: Address,
    /// Epoch of the last authorized switch.
    pub epoch_of_last_authorized_switch: u64,
    /// Target epoch of the switch.
    pub target_epoch: u64,
}

/// Reference `JsonParsedVote`, with the node's flattened lockout members.
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Vote {
    /// Number of confirmations recorded for the vote.
    pub confirmation_count: u32,
    /// Optional vote latency in slots.
    #[serde(default)]
    pub latency: Option<u8>,
    /// Voted slot.
    pub slot: u64,
}
