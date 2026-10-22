//! Inflation fixtures are literal protocol data, exercised through the public client.
mod economics_common;
use economics_common::{ADDRESS, MAX, address, call, envelope};
use solana_kit_client::{
    Error,
    methods::{Commitment, ReadConfig, economics::*},
};

const GOVERNOR: &str =
    r#"{"initial":0.15,"terminal":0.015,"taper":0.15,"foundation":0.05,"foundationTerm":7}"#;
const RATE: &str =
    r#"{"total":0.149,"validator":0.148,"foundation":0.001,"epoch":18446744073709551615}"#;

const EXPECTED_GOVERNOR: (f64, f64, f64, f64, f64) = (0.15, 0.015, 0.15, 0.05, 7.0);
const EXPECTED_RATE: (f64, f64, f64, u64) = (0.149, 0.148, 0.001, u64::MAX);

#[test]
fn governor_and_rate_have_complete_finite_fields_and_exact_optional_params() {
    let value = call(
        InflationGovernor(None),
        "getInflationGovernor",
        "[]",
        GOVERNOR,
    )
    .unwrap();
    assert_eq!(
        (
            value.initial,
            value.terminal,
            value.taper,
            value.foundation,
            value.foundation_term
        ),
        EXPECTED_GOVERNOR
    );
    call(
        InflationGovernor(Some(CommitmentConfig::default())),
        "getInflationGovernor",
        "[{}]",
        GOVERNOR,
    )
    .unwrap();
    call(
        InflationGovernor(Some(CommitmentConfig {
            commitment: Some(Commitment::Confirmed),
        })),
        "getInflationGovernor",
        r#"[{"commitment":"confirmed"}]"#,
        GOVERNOR,
    )
    .unwrap();
    let value = call(InflationRate, "getInflationRate", "[]", RATE).unwrap();
    assert_eq!(
        (value.total, value.validator, value.foundation, value.epoch),
        EXPECTED_RATE
    );
}

#[test]
fn inflation_rejects_nonfinite_negative_wrong_types_and_missing_fields() {
    for (field, original) in [
        ("initial", "0.15"),
        ("terminal", "0.015"),
        ("taper", "0.15"),
        ("foundation", "0.05"),
        ("foundationTerm", "7"),
    ] {
        invalid_governor_field(field, original);
    }

    let missing = GOVERNOR.replace("\"foundationTerm\":7", "\"future\":7");
    assert!(
        call(
            InflationGovernor(None),
            "getInflationGovernor",
            "[]",
            &missing
        )
        .is_err()
    );

    for body in [
        RATE.replace(MAX, "18446744073709551616"),
        RATE.replace(MAX, "-1"),
        RATE.replace(MAX, "1.0"),
    ] {
        assert!(call(InflationRate, "getInflationRate", "[]", &body).is_err());
    }

    let future = GOVERNOR.replace("7}", "7,\"future\":{\"flag\":true}}");
    call(
        InflationGovernor(None),
        "getInflationGovernor",
        "[]",
        &future,
    )
    .unwrap();
}

fn invalid_governor_field(field: &str, original: &str) {
    for invalid in ["-0.1", "1e999", "null", r#""0.15""#, "true"] {
        let body = GOVERNOR.replace(
            &format!("\"{field}\":{original}"),
            &format!("\"{field}\":{invalid}"),
        );
        assert!(
            call(InflationGovernor(None), "getInflationGovernor", "[]", &body).is_err(),
            "{body}"
        );
    }
}

#[test]
fn reward_preserves_nullable_positions_exact_lamports_and_commission_evolution() {
    let body = r#"[null,{"epoch":18446744073709551615,"effectiveSlot":18446744073709551615,"amount":18446744073709551615,"postBalance":18446744073709551615,"commission":null,"commissionBps":65535}]"#;
    let config = InflationRewardConfig {
        commitment: Some(Commitment::Finalized),
        epoch: Some(u64::MAX),
        min_context_slot: Some(u64::MAX),
    };
    let params = format!(
        r#"[["{ADDRESS}","{ADDRESS}"],{{"commitment":"finalized","epoch":{MAX},"minContextSlot":{MAX}}}]"#
    );
    let values = call(
        InflationReward(vec![address(), address()], Some(config)),
        "getInflationReward",
        &params,
        body,
    )
    .unwrap();
    assert!(values[0].is_none());
    let reward = values[1].as_ref().unwrap();
    assert_eq!(
        (
            reward.epoch,
            reward.effective_slot,
            reward.amount,
            reward.post_balance
        ),
        (u64::MAX, u64::MAX, u64::MAX, u64::MAX)
    );
    assert_eq!(reward.commission, None);
    assert_eq!(reward.commission_bps, Some(u16::MAX));

    let legacy = body.replace("[null,", "[").replace(
        "\"commission\":null,\"commissionBps\":65535",
        "\"commission\":255",
    );
    let params = format!(r#"[["{ADDRESS}"]]"#);
    let value = call(
        InflationReward(vec![address()], None),
        "getInflationReward",
        &params,
        &legacy,
    )
    .unwrap();
    assert_eq!(value[0].as_ref().unwrap().commission, Some(255));
    call(
        InflationReward(vec![], Some(InflationRewardConfig::default())),
        "getInflationReward",
        "[[],{}]",
        "[]",
    )
    .unwrap();
}

#[test]
fn reward_rejects_cardinality_numeric_null_and_schema_errors() {
    let body = r#"[{"epoch":1,"effectiveSlot":2,"amount":3,"postBalance":4,"commission":0}]"#;
    let params = format!(r#"[["{ADDRESS}"]]"#);

    for malformed in [
        "[]".to_owned(),
        "null".to_owned(),
        body.replace("\"commission\":0", "\"commission\":256"),
        body.replace("\"commission\":0", "\"future\":0"),
        body.replace("\"amount\":3", "\"amount\":-1"),
        body.replace("\"amount\":3", "\"amount\":3.0"),
        body.replace("\"postBalance\":4", "\"postBalance\":null"),
    ] {
        assert!(
            call(
                InflationReward(vec![address()], None),
                "getInflationReward",
                &params,
                &malformed
            )
            .is_err(),
            "{malformed}"
        );
    }
}

#[test]
fn minimum_delegation_retains_context_and_maximum_slot_constraints() {
    let config = ReadConfig {
        commitment: Some(Commitment::Processed),
        min_context_slot: Some(u64::MAX),
    };
    let params = format!(r#"[{{"commitment":"processed","minContextSlot":{MAX}}}]"#);
    let body = format!(r#"{{"context":{{"slot":{MAX},"apiVersion":"4.2.2"}},"value":{MAX}}}"#);
    let value = call(
        StakeMinimumDelegation(Some(config)),
        "getStakeMinimumDelegation",
        &params,
        &body,
    )
    .unwrap();
    assert_eq!(value.context.slot, u64::MAX);
    assert_eq!(value.value, u64::MAX);
    call(
        StakeMinimumDelegation(None),
        "getStakeMinimumDelegation",
        "[]",
        &body,
    )
    .unwrap();
    call(
        StakeMinimumDelegation(Some(ReadConfig::default())),
        "getStakeMinimumDelegation",
        "[{}]",
        &body,
    )
    .unwrap();

    for malformed in [
        body.replace("\"value\":18446744073709551615", "\"value\":null"),
        body.replace("\"slot\":18446744073709551615", "\"slot\":-1"),
    ] {
        assert!(
            call(
                StakeMinimumDelegation(None),
                "getStakeMinimumDelegation",
                "[]",
                &malformed
            )
            .is_err()
        );
    }
}

#[test]
fn economics_reuses_central_correlation_remote_and_duplicate_guards() {
    assert!(matches!(
        envelope(
            InflationRate,
            "getInflationRate",
            "[]",
            r#"{"jsonrpc":"2.0","id":2,"result":{}}"#
        ),
        Err(Error::Correlation)
    ));
    assert!(matches!(
        envelope(
            InflationRate,
            "getInflationRate",
            "[]",
            r#"{"jsonrpc":"2.0","id":1,"error":{"code":-32005,"message":"behind","data":null}}"#
        ),
        Err(Error::Remote(_))
    ));
    let duplicate = RATE.replace(
        "\"total\":0.149",
        "\"total\":0.149,\"future\":1,\"future\":2",
    );
    assert!(matches!(
        call(InflationRate, "getInflationRate", "[]", &duplicate),
        Err(Error::Json(_))
    ));
}
