//! Supply, account, fee and voting fixtures execute every operation via Client.
mod economics_common;
use economics_common::{ADDRESS, MAX, address, call};
use solana_kit_client::methods::{Commitment, economics::*};

#[test]
fn largest_accounts_preserve_partition_sorting_context_and_exact_balances() {
    let body = format!(
        r#"{{"context":{{"slot":{MAX}}},"value":[{{"address":"{ADDRESS}","lamports":{MAX}}}]}}"#
    );
    let config = LargestAccountsConfig {
        commitment: Some(Commitment::Confirmed),
        filter: Some(LargestAccountsFilter::NonCirculating),
        sort_results: Some(false),
    };
    let params = r#"[{"commitment":"confirmed","filter":"nonCirculating","sortResults":false}]"#;
    let value = call(
        LargestAccounts(Some(config)),
        "getLargestAccounts",
        params,
        &body,
    )
    .unwrap();
    assert_eq!(value.context.slot, u64::MAX);
    assert_eq!(
        (value.value[0].address, value.value[0].lamports),
        (address(), u64::MAX)
    );

    largest_config_variants(&body);
    largest_invalid_responses(&body);
}

fn largest_config_variants(body: &str) {
    call(LargestAccounts(None), "getLargestAccounts", "[]", body).unwrap();
    call(
        LargestAccounts(Some(LargestAccountsConfig::default())),
        "getLargestAccounts",
        "[{}]",
        body,
    )
    .unwrap();
    call(
        LargestAccounts(Some(LargestAccountsConfig {
            filter: Some(LargestAccountsFilter::Circulating),
            ..Default::default()
        })),
        "getLargestAccounts",
        r#"[{"filter":"circulating"}]"#,
        body,
    )
    .unwrap();
}

fn largest_invalid_responses(body: &str) {
    for malformed in [
        body.replace(ADDRESS, "bad"),
        body.replace("\"lamports\":18446744073709551615", "\"lamports\":1.5"),
        body.replace("\"value\":[", "\"future\":["),
    ] {
        assert!(
            call(
                LargestAccounts(None),
                "getLargestAccounts",
                "[]",
                &malformed
            )
            .is_err()
        );
    }
}

#[test]
fn fees_distinguish_omitted_empty_and_full_account_lists_and_exact_samples() {
    let body = format!(r#"[{{"slot":{MAX},"prioritizationFee":{MAX},"future":false}}]"#);
    let value = call(
        RecentPrioritizationFees(None),
        "getRecentPrioritizationFees",
        "[]",
        &body,
    )
    .unwrap();
    assert_eq!(
        (value[0].slot, value[0].prioritization_fee),
        (u64::MAX, u64::MAX)
    );

    fee_list_variants(&body);
    fee_invalid_responses(&body);
}

fn fee_list_variants(body: &str) {
    call(
        RecentPrioritizationFees(Some(vec![])),
        "getRecentPrioritizationFees",
        "[[]]",
        body,
    )
    .unwrap();

    let params = format!(r#"[["{ADDRESS}"]]"#);
    call(
        RecentPrioritizationFees(Some(vec![address()])),
        "getRecentPrioritizationFees",
        &params,
        body,
    )
    .unwrap();
}

fn fee_invalid_responses(body: &str) {
    let full = format!(
        "[{}]",
        (0..128)
            .map(|_| format!(r#""{ADDRESS}""#))
            .collect::<Vec<_>>()
            .join(",")
    );
    call(
        RecentPrioritizationFees(Some(vec![address(); 128])),
        "getRecentPrioritizationFees",
        &format!("[{full}]"),
        body,
    )
    .unwrap();
    assert!(
        call(
            RecentPrioritizationFees(Some(vec![address(); 129])),
            "getRecentPrioritizationFees",
            "unused",
            body
        )
        .is_err()
    );

    fee_invalid_samples(body);
}

fn fee_invalid_samples(body: &str) {
    for malformed in [
        body.replace(
            "\"prioritizationFee\":18446744073709551615",
            "\"prioritizationFee\":-1",
        ),
        body.replace(MAX, "18446744073709551616"),
        "null".to_owned(),
    ] {
        assert!(
            call(
                RecentPrioritizationFees(None),
                "getRecentPrioritizationFees",
                "[]",
                &malformed
            )
            .is_err()
        );
    }
}

#[test]
fn supply_retains_omission_boolean_false_and_all_owned_quantities() {
    let body = format!(
        r#"{{"context":{{"slot":9}},"value":{{"total":{MAX},"circulating":{MAX},"nonCirculating":0,"nonCirculatingAccounts":["{ADDRESS}"]}}}}"#
    );
    let config = SupplyConfig {
        commitment: Some(Commitment::Finalized),
        exclude_non_circulating_accounts_list: Some(false),
    };
    let value = call(
        Supply(Some(config)),
        "getSupply",
        r#"[{"commitment":"finalized","excludeNonCirculatingAccountsList":false}]"#,
        &body,
    )
    .unwrap();
    assert_eq!(
        (
            value.value.total,
            value.value.circulating,
            value.value.non_circulating
        ),
        (u64::MAX, u64::MAX, 0)
    );
    assert_eq!(value.value.non_circulating_accounts, [address()]);

    call(Supply(None), "getSupply", "[]", &body).unwrap();
    call(
        Supply(Some(SupplyConfig::default())),
        "getSupply",
        "[{}]",
        &body,
    )
    .unwrap();
}

#[test]
fn supply_exclusion_and_invalid_schema_are_explicit() {
    let body = format!(
        r#"{{"context":{{"slot":9}},"value":{{"total":{MAX},"circulating":{MAX},"nonCirculating":0,"nonCirculatingAccounts":["{ADDRESS}"]}}}}"#
    );
    let excluded = body.replace(&format!(r#"["{ADDRESS}"]"#), "[]");
    call(
        Supply(Some(SupplyConfig {
            exclude_non_circulating_accounts_list: Some(true),
            ..Default::default()
        })),
        "getSupply",
        r#"[{"excludeNonCirculatingAccountsList":true}]"#,
        &excluded,
    )
    .unwrap();

    for malformed in [
        body.replace(ADDRESS, "bad"),
        body.replace("\"nonCirculating\":0", "\"nonCirculating\":null"),
        body.replace("\"nonCirculatingAccounts\":", "\"future\":"),
        body.replace(MAX, "-1"),
    ] {
        assert!(
            call(Supply(None), "getSupply", "[]", &malformed).is_err(),
            "{malformed}"
        );
    }
}

fn vote_record() -> String {
    format!(
        r#"{{"votePubkey":"{ADDRESS}","nodePubkey":"{ADDRESS}","activatedStake":{MAX},"commission":255,"inflationRewardsCommissionBps":65535,"epochVoteAccount":true,"epochCredits":[[{MAX},{MAX},{MAX}]],"lastVote":{MAX},"rootSlot":{MAX}}}"#
    )
}

#[test]
fn votes_preserve_both_partitions_complete_config_and_typed_optional_evolution() {
    let body = format!(
        r#"{{"current":[{}],"delinquent":[{}]}}"#,
        vote_record(),
        vote_record()
    );
    let config = VoteAccountsConfig {
        commitment: Some(Commitment::Processed),
        delinquent_slot_distance: Some(u64::MAX),
        keep_unstaked_delinquents: Some(false),
        vote_pubkey: Some(address()),
    };
    let params = format!(
        r#"[{{"commitment":"processed","delinquentSlotDistance":{MAX},"keepUnstakedDelinquents":false,"votePubkey":"{ADDRESS}"}}]"#
    );
    let value = call(
        VoteAccounts(Some(config)),
        "getVoteAccounts",
        &params,
        &body,
    )
    .unwrap();

    for account in [&value.current[0], &value.delinquent[0]] {
        assert_eq!(
            (account.vote_pubkey, account.node_pubkey),
            (address(), address())
        );
        assert_eq!(
            (
                account.activated_stake,
                account.last_vote,
                account.root_slot
            ),
            (u64::MAX, u64::MAX, u64::MAX)
        );
        assert_eq!(account.epoch_credits, [(u64::MAX, u64::MAX, u64::MAX)]);
        assert_eq!(account.commission, 255);
        assert_eq!(account.inflation_rewards_commission_bps, Some(u16::MAX));
        assert!(account.epoch_vote_account);
    }

    vote_optional_evolution(&body);
}

fn vote_optional_evolution(body: &str) {
    call(VoteAccounts(None), "getVoteAccounts", "[]", body).unwrap();
    call(
        VoteAccounts(Some(VoteAccountsConfig::default())),
        "getVoteAccounts",
        "[{}]",
        body,
    )
    .unwrap();
    let old = body.replace("\"inflationRewardsCommissionBps\":65535,", "");
    let value = call(VoteAccounts(None), "getVoteAccounts", "[]", &old).unwrap();
    assert_eq!(value.current[0].inflation_rewards_commission_bps, None);
    let future = body.replace(
        "\"commission\":255",
        "\"commission\":255,\"future\":{\"nested\":1}",
    );
    call(VoteAccounts(None), "getVoteAccounts", "[]", &future).unwrap();
}

#[test]
fn votes_reject_invalid_addresses_integer_widths_tuple_lengths_and_missing_fields() {
    let body = format!(r#"{{"current":[{}],"delinquent":[]}}"#, vote_record());

    for malformed in [
        body.replace(ADDRESS, "bad"),
        body.replace("\"commission\":255", "\"commission\":256"),
        body.replace(
            "\"inflationRewardsCommissionBps\":65535",
            "\"inflationRewardsCommissionBps\":65536",
        ),
        body.replace(&format!("[{MAX},{MAX},{MAX}]"), &format!("[{MAX},{MAX}]")),
        body.replace("\"rootSlot\":18446744073709551615", "\"rootSlot\":null"),
        body.replace("\"delinquent\":[]", "\"future\":[]"),
        body.replace("\"epochVoteAccount\":true", "\"epochVoteAccount\":1"),
    ] {
        assert!(
            call(VoteAccounts(None), "getVoteAccounts", "[]", &malformed).is_err(),
            "{malformed}"
        );
    }
}
