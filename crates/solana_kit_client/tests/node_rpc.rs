//! Literal protocol fixtures executed by a downstream consumer of Client.
use solana_kit_client::{
    Client, Error, Limits, Presence, Reply, Request, Transport, TransportError,
    methods::{Commitment, Method, ReadConfig, node::*},
};
use solana_kit_core::{Address, Hash};
use std::future::{Ready, ready};

const ZERO: &str = "11111111111111111111111111111111";
const MAX: &str = "18446744073709551615";

struct Fixture {
    name: &'static str,
    params: String,
    reply: String,
}
impl Transport for Fixture {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;
    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        assert_eq!(
            String::from_utf8(request.body).unwrap(),
            format!(
                "{{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"{}\",\"params\":{}}}",
                self.name, self.params
            )
        );
        ready(Ok(Reply {
            status: 200,
            body: self.reply.as_bytes().to_vec(),
        }))
    }
}
fn execute<M: Method>(method: M, params: &str, result: &str) -> Result<M::Response, Error> {
    framed(
        method,
        params,
        &format!("{{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{result}}}"),
    )
}
fn framed<M: Method>(method: M, params: &str, reply: &str) -> Result<M::Response, Error> {
    let client = Client::new(
        Fixture {
            name: M::NAME,
            params: params.to_owned(),
            reply: reply.to_owned(),
        },
        Limits::default(),
    );
    futures::executor::block_on(client.call(method))
}
fn read() -> ReadConfig {
    ReadConfig {
        commitment: Some(Commitment::Finalized),
        min_context_slot: Some(u64::MAX),
    }
}

#[test]
fn all_simple_node_operations_execute_exact_public_requests_and_owned_results() {
    assert_eq!(execute(BlockHeight(None), "[]", MAX).unwrap(), u64::MAX);
    assert_eq!(execute(Slot(None), "[]", MAX).unwrap(), u64::MAX);
    assert_eq!(
        execute(TransactionCount(None), "[]", MAX).unwrap(),
        u64::MAX
    );
    assert_eq!(execute(MaxRetransmitSlot, "[]", MAX).unwrap(), u64::MAX);
    assert_eq!(execute(MaxShredInsertSlot, "[]", MAX).unwrap(), u64::MAX);
    assert_eq!(
        execute(GenesisHash, "[]", &format!("\"{ZERO}\"")).unwrap(),
        Hash::from_bytes([0; 32])
    );
    assert_eq!(
        execute(SlotLeader(None), "[]", &format!("\"{ZERO}\"")).unwrap(),
        Address::from_bytes([0; 32])
    );
    assert_eq!(execute(GetHealth, "[]", "\"ok\"").unwrap(), Health::Ok);
    assert_eq!(
        execute(Identity, "[]", &format!("{{\"identity\":\"{ZERO}\"}}"))
            .unwrap()
            .identity,
        Address::from_bytes([0; 32])
    );
    let version = execute(
        Version,
        "[]",
        r#"{"solana-core":"4.3.0","feature-set":4294967295}"#,
    )
    .unwrap();
    assert_eq!(version.solana_core, "4.3.0");
    assert_eq!(version.feature_set, Presence::Present(Some(u32::MAX)));
    let snapshots = execute(
        HighestSnapshotSlot,
        "[]",
        &format!("{{\"full\":{MAX},\"incremental\":null}}"),
    )
    .unwrap();
    assert_eq!(snapshots.full, u64::MAX);
    assert_eq!(snapshots.incremental, None);
}

#[test]
fn all_optional_read_configs_preserve_omission_empty_and_explicit_fields() {
    let full = r#"[{"commitment":"finalized","minContextSlot":18446744073709551615}]"#;
    assert_eq!(execute(BlockHeight(Some(read())), full, "1").unwrap(), 1);
    assert_eq!(execute(Slot(Some(read())), full, "1").unwrap(), 1);
    assert_eq!(
        execute(TransactionCount(Some(read())), full, "1").unwrap(),
        1
    );
    execute(SlotLeader(Some(read())), full, &format!("\"{ZERO}\"")).unwrap();
    assert_eq!(
        execute(Slot(Some(ReadConfig::default())), "[{}]", "0").unwrap(),
        0
    );
}

#[test]
fn epoch_info_and_schedule_have_required_fields_and_exact_counters() {
    let info = r#"{"absoluteSlot":18446744073709551615,"blockHeight":9,"epoch":8,"slotIndex":7,"slotsInEpoch":6,"transactionCount":null}"#;
    let value = execute(
        GetEpochInfo(Some(read())),
        r#"[{"commitment":"finalized","minContextSlot":18446744073709551615}]"#,
        info,
    )
    .unwrap();
    assert_eq!(value.absolute_slot, u64::MAX);
    assert_eq!(value.transaction_count, None);
    assert_eq!(
        execute(GetEpochInfo(None), "[]", &info.replace("null", MAX))
            .unwrap()
            .transaction_count,
        Some(u64::MAX)
    );
    let schedule = r#"{"firstNormalEpoch":14,"firstNormalSlot":524256,"leaderScheduleSlotOffset":432000,"slotsPerEpoch":432000,"warmup":true}"#;
    assert!(execute(GetEpochSchedule, "[]", schedule).unwrap().warmup);

    for malformed in [
        info.replace(",\"transactionCount\":null", ""),
        info.replace("\"epoch\":8", "\"epoch\":null"),
        info.replace("\"slotIndex\":7", "\"slotIndex\":-1"),
    ] {
        assert!(execute(GetEpochInfo(None), "[]", &malformed).is_err());
    }

    assert!(execute(GetEpochSchedule, "[]", &schedule.replace("true", "1")).is_err());
}

#[test]
fn cluster_nodes_keep_all_endpoints_and_missing_distinct_from_null() {
    let node = format!(
        r#"[{{"pubkey":"{ZERO}","clientId":"agave","featureSet":4294967295,"gossip":"127.0.0.1:8001","pubsub":"[::1]:8900","rpc":null,"serveRepair":"127.0.0.1:8002","shredVersion":65535,"tpu":"127.0.0.1:8003","tpuForwards":"127.0.0.1:8004","tpuForwardsQuic":"127.0.0.1:8005","tpuQuic":"127.0.0.1:8006","tpuVote":"127.0.0.1:8007","tvu":"127.0.0.1:8008","version":"4.3.0"}}]"#
    );
    let nodes = execute(ClusterNodes, "[]", &node).unwrap();
    assert_eq!(nodes[0].rpc, Presence::Present(None));
    assert_eq!(nodes[0].shred_version, Presence::Present(Some(u16::MAX)));
    assert_eq!(nodes[0].feature_set, Presence::Present(Some(u32::MAX)));
    assert_eq!(
        nodes[0].gossip,
        Presence::Present(Some("127.0.0.1:8001".parse().unwrap()))
    );
    let old = execute(ClusterNodes, "[]", &format!(r#"[{{"pubkey":"{ZERO}"}}]"#)).unwrap();
    assert_eq!(old[0].rpc, Presence::Missing);

    for malformed in [
        node.replace("65535", "65536"),
        node.replace("4294967295", "4294967296"),
        node.replace("127.0.0.1:8001", "bad endpoint"),
        node.replace(ZERO, "bad key"),
    ] {
        assert!(execute(ClusterNodes, "[]", &malformed).is_err());
    }
}

#[test]
fn production_and_leader_schedules_validate_domain_map_keys_and_tuple_arity() {
    let config = BlockProductionConfig {
        commitment: Some(Commitment::Confirmed),
        identity: Some(Address::from_bytes([0; 32])),
        range: Some(SlotRange {
            first_slot: 0,
            last_slot: Some(u64::MAX),
        }),
    };
    let params = format!(
        r#"[{{"commitment":"confirmed","identity":"{ZERO}","range":{{"firstSlot":0,"lastSlot":{MAX}}}}}]"#
    );
    let result = r#"{"context":{"slot":1},"value":{"byIdentity":{"KEY":[MAX,2]},"range":{"firstSlot":0,"lastSlot":MAX}}}"#.replace("KEY", ZERO).replace("MAX", MAX);
    let value = execute(BlockProduction(Some(config)), &params, &result).unwrap();
    assert_eq!(
        value.value.by_identity[&Address::from_bytes([0; 32])],
        [u64::MAX, 2]
    );
    assert_eq!(value.value.range.last_slot, u64::MAX);
    execute(BlockProduction(None), "[]", &result).unwrap();
    let schedule = format!(r#"{{"{ZERO}":[0,{MAX}]}}"#);
    let value = execute(LeaderSchedule(None, None), "[null]", &schedule)
        .unwrap()
        .unwrap();
    assert_eq!(value.0[&Address::from_bytes([0; 32])], [0, u64::MAX]);
    let config = LeaderScheduleConfig {
        commitment: Some(Commitment::Processed),
        identity: Some(Address::from_bytes([0; 32])),
    };
    let params = format!(r#"[{MAX},{{"commitment":"processed","identity":"{ZERO}"}}]"#);
    assert!(
        execute(
            LeaderSchedule(Some(u64::MAX), Some(config)),
            &params,
            "null"
        )
        .unwrap()
        .is_none()
    );

    for malformed in [
        result.replace(ZERO, "wrong"),
        result.replace("[18446744073709551615,2]", "[1]"),
        result.replace("[18446744073709551615,2]", "[1,2,3]"),
    ] {
        assert!(execute(BlockProduction(None), "[]", &malformed).is_err());
    }

    assert!(
        execute(
            LeaderSchedule(None, None),
            "[null]",
            &schedule.replace(ZERO, "wrong")
        )
        .is_err()
    );
}

#[test]
fn performance_and_slot_leaders_execute_optional_limits_and_exact_results() {
    let sample = r#"[{"slot":18446744073709551615,"numTransactions":18446744073709551615,"numNonVoteTransactions":3,"numSlots":2,"samplePeriodSecs":60}]"#;
    let value = execute(RecentPerformanceSamples(Some(720)), "[720]", sample).unwrap();
    assert_eq!(value[0].num_transactions, u64::MAX);
    assert_eq!(
        value[0].num_non_vote_transactions,
        Presence::Present(Some(3))
    );
    assert!(
        execute(RecentPerformanceSamples(None), "[]", "[]")
            .unwrap()
            .is_empty()
    );
    assert!(
        execute(RecentPerformanceSamples(Some(0)), "[0]", "[]")
            .unwrap()
            .is_empty()
    );
    let keys = format!(r#"["{ZERO}","{ZERO}"]"#);
    assert_eq!(
        execute(SlotLeaders(u64::MAX, 2), &format!("[{MAX},2]"), &keys).unwrap(),
        vec![Address::from_bytes([0; 32]); 2]
    );
    assert!(serde_json::to_string(&RecentPerformanceSamples(Some(721))).is_err());
    assert!(serde_json::to_string(&SlotLeaders(0, 0)).is_err());
    assert!(serde_json::to_string(&SlotLeaders(0, 5001)).is_err());
    assert!(execute(SlotLeaders(0, 0), "must not reach transport", "[]").is_err());
    assert!(
        execute(
            RecentPerformanceSamples(Some(721)),
            "must not reach transport",
            "[]"
        )
        .is_err()
    );
    assert!(execute(SlotLeaders(0, 1), "[0,1]", "[\"bad\"]").is_err());
    assert!(
        execute(
            RecentPerformanceSamples(None),
            "[]",
            &sample.replace("60", "65536")
        )
        .is_err()
    );
}

fn certificate() -> String {
    format!(
        r#"{{"block":{{"blockId":[{}],"slot":{MAX}}},"signature":{{"signature":[{}],"bitmap":[0,255]}}}}"#,
        vec!["17"; 32].join(","),
        vec!["42"; 192].join(",")
    )
}

#[test]
fn genesis_certificate_bytes_have_exact_widths_and_null_is_a_real_result() {
    assert!(execute(AgGenesisCert, "[]", "null").unwrap().is_none());
    let text = certificate();
    let cert = execute(AgGenesisCert, "[]", &text).unwrap().unwrap();
    assert_eq!(cert.block.slot, u64::MAX);
    assert_eq!(cert.block.block_id, [17; 32]);
    assert_eq!(cert.signature.signature, [42; 192]);
    assert_eq!(cert.signature.bitmap, [0, 255]);

    for bad in [
        text.replacen("17,", "", 1),
        text.replacen("42,", "", 1),
        text.replacen("17,", "256,", 1),
        text.replacen("42,", "-1,", 1),
        text.replace("[0,255]", "[null]"),
        text.replace("\"slot\":18446744073709551615", "\"slot\":1.5"),
    ] {
        assert!(execute(AgGenesisCert, "[]", &bad).is_err());
    }
}

#[test]
fn numeric_and_domain_schema_errors_cannot_look_like_success() {
    for invalid in ["null", "-1", "1.0", "\"1\"", "18446744073709551616", "{}"] {
        assert!(execute(BlockHeight(None), "[]", invalid).is_err());
        assert!(execute(Slot(None), "[]", invalid).is_err());
        assert!(execute(TransactionCount(None), "[]", invalid).is_err());
        assert!(execute(MaxRetransmitSlot, "[]", invalid).is_err());
        assert!(execute(MaxShredInsertSlot, "[]", invalid).is_err());
    }

    assert!(execute(GetHealth, "[]", "\"unhealthy\"").is_err());
    assert!(execute(GenesisHash, "[]", "\"bad\"").is_err());
    assert!(execute(SlotLeader(None), "[]", "\"bad\"").is_err());
    assert!(execute(Identity, "[]", r#"{"identity":null}"#).is_err());
    assert!(execute(Version, "[]", r#"{"feature-set":0}"#).is_err());
    assert!(execute(HighestSnapshotSlot, "[]", r#"{"full":1}"#).is_err());
}

#[test]
fn central_correlation_and_remote_errors_precede_node_success_schemas() {
    assert!(matches!(
        framed(GetHealth, "[]", r#"{"jsonrpc":"2.0","id":2,"result":"ok"}"#),
        Err(Error::Correlation)
    ));
    let error = framed(
        AgGenesisCert,
        "[]",
        r#"{"jsonrpc":"2.0","id":1,"error":{"code":-32601,"message":"method unavailable"}}"#,
    );
    let Err(Error::Remote(error)) = error else {
        panic!("expected remote error")
    };
    assert_eq!(error.code, -32601);
    assert!(matches!(
        framed(
            Slot(None),
            "[]",
            r#"{"jsonrpc":"2.0","id":1,"result":1,"result":2}"#
        ),
        Err(Error::Json(_))
    ));
}

#[test]
fn historical_optional_fields_preserve_absent_null_and_present_values() {
    let version = execute(Version, "[]", r#"{"solana-core":"old"}"#).unwrap();
    assert_eq!(version.feature_set, Presence::Missing);
    let version = execute(Version, "[]", r#"{"solana-core":"old","feature-set":null}"#).unwrap();
    assert_eq!(version.feature_set, Presence::Present(None));
    let sample = r#"[{"slot":1,"numTransactions":2,"numSlots":3,"samplePeriodSecs":60}]"#;
    assert_eq!(
        execute(RecentPerformanceSamples(None), "[]", sample).unwrap()[0].num_non_vote_transactions,
        Presence::Missing
    );
    let sample = sample.replace(
        "\"numSlots\":3",
        "\"numNonVoteTransactions\":null,\"numSlots\":3",
    );
    assert_eq!(
        execute(RecentPerformanceSamples(None), "[]", &sample).unwrap()[0]
            .num_non_vote_transactions,
        Presence::Present(None)
    );
    let config = BlockProductionConfig {
        range: Some(SlotRange {
            first_slot: 1,
            last_slot: None,
        }),
        ..Default::default()
    };
    let response =
        r#"{"context":{"slot":1},"value":{"byIdentity":{},"range":{"firstSlot":1,"lastSlot":2}}}"#;
    execute(
        BlockProduction(Some(config)),
        r#"[{"range":{"firstSlot":1}}]"#,
        response,
    )
    .unwrap();
    assert_eq!(
        ReadConfig::default(),
        ReadConfig {
            commitment: None,
            min_context_slot: None
        }
    );
    assert_eq!(
        LeaderScheduleConfig::default(),
        LeaderScheduleConfig {
            commitment: None,
            identity: None
        }
    );
}
