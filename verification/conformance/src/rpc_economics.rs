//! Typed economic operations shared by every actual portability host.
extern crate std;

use crate::{require, rpc_fixture::call};
use solana_kit_client::{
    Error,
    methods::{Commitment, ReadConfig, economics::*},
};
use solana_kit_core::Address;
use std::{format, string::String, vec};

const KEY: &str = "11111111111111111111111111111111";
const MAX: &str = "18446744073709551615";
const GOVERNOR: &str =
    r#"{"initial":0.15,"terminal":0.015,"taper":0.15,"foundation":0.05,"foundationTerm":7}"#;
const RATE: &str =
    r#"{"total":0.149,"validator":0.148,"foundation":0.001,"epoch":18446744073709551615}"#;
const GOVERNOR_VALUES: (f64, f64, f64, f64, f64) = (0.15, 0.015, 0.15, 0.05, 7.0);
const RATE_VALUES: (f64, f64, f64, u64) = (0.149, 0.148, 0.001, u64::MAX);

/// Stable appended economics groups retain all preceding case lines.
pub(crate) fn cases() -> [crate::CaseResult; 3] {
    [
        crate::case("rpc-economics-accounts-votes", accounts_and_votes()),
        crate::case("rpc-economics-inflation-rewards", inflation_and_rewards()),
        crate::case("rpc-economics-malformed-config", malformed_and_config()),
    ]
}

fn address() -> Address {
    Address::from_bytes([0; 32])
}

fn largest() -> String {
    format!(r#"{{"context":{{"slot":{MAX}}},"value":[{{"address":"{KEY}","lamports":{MAX}}}]}}"#)
}

fn supply() -> String {
    format!(
        concat!(
            r#"{{"context":{{"slot":{0}}},"value":{{"total":{0},"circulating":{0},"#,
            r#""nonCirculating":{0},"nonCirculatingAccounts":["{1}"]}}}}"#
        ),
        MAX, KEY
    )
}

fn votes() -> String {
    let record = format!(
        concat!(
            r#"{{"votePubkey":"{1}","nodePubkey":"{1}","activatedStake":{0},"#,
            r#""commission":255,"inflationRewardsCommissionBps":65535,"epochVoteAccount":true,"#,
            r#""epochCredits":[[{0},{0},{0}]],"lastVote":{0},"rootSlot":{0}}}"#
        ),
        MAX, KEY
    );
    format!(r#"{{"current":[{record}],"delinquent":[{record}]}}"#)
}

fn reward() -> String {
    format!(
        concat!(
            r#"[null,{{"epoch":{0},"effectiveSlot":{0},"amount":{0},"postBalance":{0},"#,
            r#""commission":null,"commissionBps":65535}}]"#
        ),
        MAX
    )
}

/// Executes account supply, largest accounts, fee samples and both vote partitions.
pub fn accounts_and_votes() -> Result<(), &'static str> {
    let largest = call(
        LargestAccounts(Some(LargestAccountsConfig {
            commitment: Some(Commitment::Confirmed),
            filter: Some(LargestAccountsFilter::NonCirculating),
            sort_results: Some(false),
        })),
        "getLargestAccounts",
        r#"[{"commitment":"confirmed","filter":"nonCirculating","sortResults":false}]"#,
        &largest(),
    )
    .map_err(|_| "largest accounts failed")?;
    let supply = call(
        Supply(Some(SupplyConfig {
            commitment: Some(Commitment::Finalized),
            exclude_non_circulating_accounts_list: Some(false),
        })),
        "getSupply",
        r#"[{"commitment":"finalized","excludeNonCirculatingAccountsList":false}]"#,
        &supply(),
    )
    .map_err(|_| "supply failed")?;
    require(
        largest.context.slot == u64::MAX
            && largest.value.len() == 1
            && largest.value[0].address == address()
            && largest.value[0].lamports == u64::MAX
            && supply.context.slot == u64::MAX
            && supply.value.total == u64::MAX
            && supply.value.circulating == u64::MAX
            && supply.value.non_circulating == u64::MAX
            && supply.value.non_circulating_accounts == [address()],
        "account quantities changed",
    )?;
    fees_and_votes()
}

fn fees_and_votes() -> Result<(), &'static str> {
    let fees = call(
        RecentPrioritizationFees(None),
        "getRecentPrioritizationFees",
        "[]",
        &format!(r#"[{{"slot":{MAX},"prioritizationFee":{MAX}}}]"#),
    )
    .map_err(|_| "fees failed")?;
    let vote = call(
        VoteAccounts(Some(VoteAccountsConfig {
            commitment: Some(Commitment::Processed),
            delinquent_slot_distance: Some(u64::MAX),
            keep_unstaked_delinquents: Some(false),
            vote_pubkey: Some(address()),
        })),
        "getVoteAccounts",
        &format!(
            concat!(
                r#"[{{"commitment":"processed","delinquentSlotDistance":{0},"#,
                r#""keepUnstakedDelinquents":false,"votePubkey":"{1}"}}]"#
            ),
            MAX, KEY
        ),
        &votes(),
    )
    .map_err(|_| "votes failed")?;
    require(
        fees.len() == 1
            && fees[0].slot == u64::MAX
            && fees[0].prioritization_fee == u64::MAX
            && vote.current.len() == 1
            && vote.delinquent.len() == 1,
        "fees/vote partitions changed",
    )?;

    vote_partitions(&vote)
}

/// Both partitions retain every exact stake, commission and epoch-credit field.
fn vote_partitions(vote: &VoteAccountsValue) -> Result<(), &'static str> {
    for value in [&vote.current[0], &vote.delinquent[0]] {
        require(
            value.vote_pubkey == address()
                && value.node_pubkey == address()
                && value.activated_stake == u64::MAX
                && value.last_vote == u64::MAX
                && value.root_slot == u64::MAX
                && value.commission == u8::MAX
                && value.inflation_rewards_commission_bps == Some(u16::MAX)
                && value.epoch_vote_account
                && value.epoch_credits == [(u64::MAX, u64::MAX, u64::MAX)],
            "vote values changed",
        )?;
    }

    Ok(())
}

/// Executes governor, current rates, nullable rewards and minimum delegation.
pub fn inflation_and_rewards() -> Result<(), &'static str> {
    inflation_values()?;
    reward_values()?;
    minimum_delegation()
}

/// Governor parameters and current rates are independent floating-point facts.
fn inflation_values() -> Result<(), &'static str> {
    let governor = call(
        InflationGovernor(Some(CommitmentConfig {
            commitment: Some(Commitment::Confirmed),
        })),
        "getInflationGovernor",
        r#"[{"commitment":"confirmed"}]"#,
        GOVERNOR,
    )
    .map_err(|_| "governor failed")?;
    let rate = call(InflationRate, "getInflationRate", "[]", RATE).map_err(|_| "rate failed")?;
    require(
        (
            governor.initial,
            governor.terminal,
            governor.taper,
            governor.foundation,
            governor.foundation_term,
        ) == GOVERNOR_VALUES
            && (rate.total, rate.validator, rate.foundation, rate.epoch) == RATE_VALUES,
        "inflation values changed",
    )
}

/// Nullable reward positions retain full-width integers and optional commissions.
fn reward_values() -> Result<(), &'static str> {
    let rewards = call(
        InflationReward(
            vec![address(); 2],
            Some(InflationRewardConfig {
                commitment: Some(Commitment::Finalized),
                epoch: Some(u64::MAX),
                min_context_slot: Some(u64::MAX),
            }),
        ),
        "getInflationReward",
        &format!(
            concat!(
                r#"[["{1}","{1}"],{{"commitment":"finalized","#,
                r#""epoch":{0},"minContextSlot":{0}}}]"#
            ),
            MAX, KEY
        ),
        &reward(),
    )
    .map_err(|_| "rewards failed")?;
    require(
        rewards.len() == 2 && rewards[0].is_none(),
        "reward positions changed",
    )?;
    let value = rewards[1].as_ref().ok_or("reward missing")?;
    require(
        (
            value.epoch,
            value.effective_slot,
            value.amount,
            value.post_balance,
        ) == (u64::MAX, u64::MAX, u64::MAX, u64::MAX)
            && value.commission.is_none()
            && value.commission_bps == Some(u16::MAX),
        "reward values changed",
    )
}

/// Minimum delegation carries the same exact context/value contract as account reads.
fn minimum_delegation() -> Result<(), &'static str> {
    let delegation = call(
        StakeMinimumDelegation(Some(ReadConfig {
            commitment: Some(Commitment::Processed),
            min_context_slot: Some(u64::MAX),
        })),
        "getStakeMinimumDelegation",
        &format!(r#"[{{"commitment":"processed","minContextSlot":{MAX}}}]"#),
        &format!(r#"{{"context":{{"slot":{MAX}}},"value":{MAX}}}"#),
    )
    .map_err(|_| "delegation failed")?;
    require(
        delegation.context.slot == u64::MAX && delegation.value == u64::MAX,
        "delegation changed",
    )
}

/// Rejects representative malformed fields and preserves empty and bounded configs.
pub fn malformed_and_config() -> Result<(), &'static str> {
    account_failures()?;
    inflation_failures()?;
    config_boundaries()
}

fn account_failures() -> Result<(), &'static str> {
    require(
        call(
            LargestAccounts(None),
            "getLargestAccounts",
            "[]",
            &largest().replace(KEY, "bad"),
        )
        .is_err(),
        "invalid largest address accepted",
    )?;
    require(
        call(
            Supply(None),
            "getSupply",
            "[]",
            &supply().replace("\"nonCirculatingAccounts\"", "\"missing\""),
        )
        .is_err(),
        "missing supply accounts accepted",
    )?;
    require(
        call(
            RecentPrioritizationFees(None),
            "getRecentPrioritizationFees",
            "[]",
            r#"[{"slot":0,"prioritizationFee":-1}]"#,
        )
        .is_err(),
        "negative fee accepted",
    )?;

    for bad in [
        votes().replace("\"commission\":255", "\"commission\":256"),
        votes().replace("65535", "65536"),
        votes().replace(&format!("[{MAX},{MAX},{MAX}]"), "[1,2]"),
        votes().replace("\"rootSlot\":18446744073709551615", "\"rootSlot\":null"),
    ] {
        require(
            call(VoteAccounts(None), "getVoteAccounts", "[]", &bad).is_err(),
            "invalid vote accepted",
        )?;
    }
    Ok(())
}

fn inflation_failures() -> Result<(), &'static str> {
    governor_failures()?;
    rate_and_reward_failures()
}

/// Every governor field independently rejects each malformed numeric representation.
fn governor_failures() -> Result<(), &'static str> {
    for (field, original) in [
        ("initial", "0.15"),
        ("terminal", "0.015"),
        ("taper", "0.15"),
        ("foundation", "0.05"),
        ("foundationTerm", "7"),
    ] {
        for invalid in ["-0.1", "1e999", "null", "true", "\"7\""] {
            let body = GOVERNOR.replace(
                &format!("\"{field}\":{original}"),
                &format!("\"{field}\":{invalid}"),
            );
            require(
                call(InflationGovernor(None), "getInflationGovernor", "[]", &body).is_err(),
                "invalid governor accepted",
            )?;
        }
    }

    Ok(())
}

/// Rates, rewards and delegation retain their own required numeric and nullable fields.
fn rate_and_reward_failures() -> Result<(), &'static str> {
    for bad in [
        RATE.replace(MAX, "18446744073709551616"),
        RATE.replace("0.149", "null"),
    ] {
        require(
            call(InflationRate, "getInflationRate", "[]", &bad).is_err(),
            "invalid rate accepted",
        )?;
    }

    for bad in [
        String::from("[]"),
        reward().replace("\"amount\":18446744073709551615", "\"amount\":-1"),
        reward().replace("65535", "65536"),
    ] {
        require(
            call(
                InflationReward(vec![address(); 2], None),
                "getInflationReward",
                &format!(r#"[["{KEY}","{KEY}"]]"#),
                &bad,
            )
            .is_err(),
            "invalid reward accepted",
        )?;
    }
    require(
        call(
            StakeMinimumDelegation(None),
            "getStakeMinimumDelegation",
            "[]",
            r#"{"context":{"slot":0},"value":null}"#,
        )
        .is_err(),
        "null delegation accepted",
    )
}

fn config_boundaries() -> Result<(), &'static str> {
    call(
        LargestAccounts(Some(LargestAccountsConfig::default())),
        "getLargestAccounts",
        "[{}]",
        &largest(),
    )
    .map_err(|_| "empty largest config failed")?;
    call(Supply(None), "getSupply", "[]", &supply()).map_err(|_| "absent supply config failed")?;
    call(
        InflationGovernor(Some(CommitmentConfig::default())),
        "getInflationGovernor",
        "[{}]",
        GOVERNOR,
    )
    .map_err(|_| "empty governor config failed")?;
    call(
        InflationReward(vec![], Some(InflationRewardConfig::default())),
        "getInflationReward",
        "[[],{}]",
        "[]",
    )
    .map_err(|_| "empty reward config failed")?;
    call(
        RecentPrioritizationFees(Some(vec![])),
        "getRecentPrioritizationFees",
        "[[]]",
        "[]",
    )
    .map_err(|_| "empty fees filter failed")?;

    fee_filter_limits()?;
    nullable_commission()
}

/// Fee filters accept the exact maximum and reject the immediately larger request.
fn fee_filter_limits() -> Result<(), &'static str> {
    let keys = vec![format!(r#""{KEY}""#); 128].join(",");
    call(
        RecentPrioritizationFees(Some(vec![address(); 128])),
        "getRecentPrioritizationFees",
        &format!("[[{keys}]]"),
        "[]",
    )
    .map_err(|_| "128 fee filters failed")?;
    require(
        matches!(
            call(
                RecentPrioritizationFees(Some(vec![address(); 129])),
                "getRecentPrioritizationFees",
                "unused",
                "[]"
            ),
            Err(Error::Json(_))
        ),
        "129 fee filters accepted",
    )
}

/// Zero commission remains distinct from a null optional basis-point field.
fn nullable_commission() -> Result<(), &'static str> {
    let legacy = reward().replace(
        "\"commission\":null,\"commissionBps\":65535",
        "\"commission\":0,\"commissionBps\":null",
    );
    let values = call(
        InflationReward(vec![address(); 2], None),
        "getInflationReward",
        &format!(r#"[["{KEY}","{KEY}"]]"#),
        &legacy,
    )
    .map_err(|_| "nullable basis points failed")?;
    require(
        values[1]
            .as_ref()
            .is_some_and(|value| value.commission == Some(0) && value.commission_bps.is_none()),
        "reward commission null/zero changed",
    )
}

#[test]
fn all_economics_groups_execute() {
    accounts_and_votes().unwrap();
    inflation_and_rewards().unwrap();
    malformed_and_config().unwrap();
}
