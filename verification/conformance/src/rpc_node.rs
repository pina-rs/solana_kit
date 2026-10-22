//! Typed node operations shared by every actual portability host.
extern crate std;

use crate::{
    require,
    rpc_fixture::{call, framed},
};
use solana_kit_client::{
    Error, Presence,
    methods::{Commitment, ReadConfig, node::*},
};
use solana_kit_core::{Address, Hash};
use std::{format, string::String, vec};

const KEY: &str = "11111111111111111111111111111111";
const MAX: &str = "18446744073709551615";
const SAMPLE_LIMIT: u16 = 720;
const BLOCK_HEIGHT: u64 = 9;
const EPOCH: u64 = 8;
const SLOT_INDEX: u64 = 7;
const EPOCH_SLOTS: u64 = 6;
const FIRST_NORMAL_EPOCH: u64 = 14;
const NORMAL_EPOCH_SLOTS: u64 = 432_000;

/// Executes all eleven scalar, identity, version and snapshot operations.
pub fn scalar_metadata() -> Result<(), &'static str> {
    let config = Some(ReadConfig {
        commitment: Some(Commitment::Finalized),
        min_context_slot: Some(u64::MAX),
    });
    let params = r#"[{"commitment":"finalized","minContextSlot":18446744073709551615}]"#;
    let values = [
        call(BlockHeight(config), "getBlockHeight", params, MAX),
        call(Slot(config), "getSlot", params, MAX),
        call(TransactionCount(config), "getTransactionCount", params, MAX),
        call(MaxRetransmitSlot, "getMaxRetransmitSlot", "[]", MAX),
        call(MaxShredInsertSlot, "getMaxShredInsertSlot", "[]", MAX),
    ];
    require(
        values
            .into_iter()
            .all(|value| matches!(value, Ok(u64::MAX))),
        "node counters/request mismatch",
    )?;
    require(
        call(GenesisHash, "getGenesisHash", "[]", &format!("\"{KEY}\""))
            .map_err(|_| "genesis hash failed")?
            == Hash::from_bytes([0; 32])
            && call(
                SlotLeader(None),
                "getSlotLeader",
                "[]",
                &format!("\"{KEY}\""),
            )
            .map_err(|_| "slot leader failed")?
                == Address::from_bytes([0; 32])
            && matches!(call(GetHealth, "getHealth", "[]", "\"ok\""), Ok(Health::Ok)),
        "node hash/leader/health mismatch",
    )?;

    let identity = call(
        Identity,
        "getIdentity",
        "[]",
        &format!(r#"{{"identity":"{KEY}"}}"#),
    )
    .map_err(|_| "identity call failed")?;
    let version = call(
        Version,
        "getVersion",
        "[]",
        r#"{"solana-core":"4.3.0","feature-set":4294967295}"#,
    )
    .map_err(|_| "version call failed")?;
    let snapshot = call(
        HighestSnapshotSlot,
        "getHighestSnapshotSlot",
        "[]",
        &format!(r#"{{"full":{MAX},"incremental":null}}"#),
    )
    .map_err(|_| "snapshot call failed")?;
    require(
        identity.identity == Address::from_bytes([0; 32])
            && version.solana_core == "4.3.0"
            && version.feature_set == Presence::Present(Some(u32::MAX))
            && snapshot.full == u64::MAX
            && snapshot.incremental.is_none(),
        "node metadata changed",
    )
}

/// Executes all seven epoch, cluster, production and leader collection operations.
pub fn structured_metadata() -> Result<(), &'static str> {
    epochs_and_cluster()?;
    production_and_leaders()?;
    let body = concat!(
        r#"[{"slot":18446744073709551615,"numTransactions":18446744073709551615,"#,
        r#""numSlots":2,"samplePeriodSecs":60}]"#,
    );
    let sample = call(
        RecentPerformanceSamples(Some(SAMPLE_LIMIT)),
        "getRecentPerformanceSamples",
        "[720]",
        body,
    )
    .map_err(|_| "performance call failed")?;
    let leaders = call(
        SlotLeaders(u64::MAX, 2),
        "getSlotLeaders",
        &format!("[{MAX},2]"),
        &format!(r#"["{KEY}","{KEY}"]"#),
    )
    .map_err(|_| "slot leaders call failed")?;
    require(
        sample.len() == 1
            && sample[0].slot == u64::MAX
            && sample[0].num_transactions == u64::MAX
            && sample[0].num_non_vote_transactions == Presence::Missing
            && sample[0].num_slots == 2
            && sample[0].sample_period_secs == 60
            && leaders == vec![Address::from_bytes([0; 32]); 2],
        "performance/leader values changed",
    )
}

fn epochs_and_cluster() -> Result<(), &'static str> {
    let epoch = call(GetEpochInfo(None), "getEpochInfo", "[]", r#"{"absoluteSlot":18446744073709551615,"blockHeight":9,"epoch":8,"slotIndex":7,"slotsInEpoch":6,"transactionCount":null}"#).map_err(|_| "epoch call failed")?;
    let schedule = call(GetEpochSchedule, "getEpochSchedule", "[]", r#"{"firstNormalEpoch":14,"firstNormalSlot":524256,"leaderScheduleSlotOffset":432000,"slotsPerEpoch":432000,"warmup":true}"#).map_err(|_| "epoch schedule failed")?;
    let nodes = call(
        ClusterNodes,
        "getClusterNodes",
        "[]",
        &format!(r#"[{{"pubkey":"{KEY}","gossip":"[::1]:8001","rpc":null,"shredVersion":65535}}]"#),
    )
    .map_err(|_| "cluster call failed")?;
    require(
        epoch.absolute_slot == u64::MAX
            && epoch.transaction_count.is_none()
            && epoch.block_height == BLOCK_HEIGHT
            && epoch.epoch == EPOCH
            && epoch.slot_index == SLOT_INDEX
            && epoch.slots_in_epoch == EPOCH_SLOTS
            && schedule.warmup
            && schedule.first_normal_epoch == FIRST_NORMAL_EPOCH
            && schedule.slots_per_epoch == NORMAL_EPOCH_SLOTS
            && nodes.len() == 1
            && nodes[0].pubkey == Address::from_bytes([0; 32])
            && nodes[0].rpc == Presence::Present(None)
            && nodes[0].version == Presence::Missing
            && nodes[0].shred_version == Presence::Present(Some(u16::MAX)),
        "epoch/cluster fields changed",
    )
}

fn production_and_leaders() -> Result<(), &'static str> {
    let body = r#"{"context":{"slot":MAX},"value":{"byIdentity":{"KEY":[MAX,2]},"range":{"firstSlot":0,"lastSlot":MAX}}}"#.replace("KEY", KEY).replace("MAX", MAX);
    let production = call(BlockProduction(None), "getBlockProduction", "[]", &body)
        .map_err(|_| "production call failed")?;
    let schedule = call(
        LeaderSchedule(None, None),
        "getLeaderSchedule",
        "[null]",
        &format!(r#"{{"{KEY}":[0,{MAX}]}}"#),
    )
    .map_err(|_| "leader schedule failed")?
    .ok_or("leader schedule missing")?;
    let missing = call(
        LeaderSchedule(Some(u64::MAX), None),
        "getLeaderSchedule",
        &format!("[{MAX}]"),
        "null",
    )
    .map_err(|_| "null leader schedule failed")?;
    let key = Address::from_bytes([0; 32]);
    require(
        production.context.slot == u64::MAX
            && production.value.by_identity.get(&key) == Some(&[u64::MAX, 2])
            && production.value.range.last_slot == u64::MAX
            && schedule.0.get(&key) == Some(&vec![0, u64::MAX])
            && missing.is_none(),
        "production/schedule fields changed",
    )
}

fn certificate() -> String {
    format!(
        r#"{{"block":{{"blockId":[{}],"slot":{MAX}}},"signature":{{"signature":[{}],"bitmap":[0,255]}}}}"#,
        vec!["17"; 32].join(","),
        vec!["42"; 192].join(",")
    )
}

/// Validates the nineteenth operation's schema, malformed replies and framing failures.
pub fn certificate_and_failures() -> Result<(), &'static str> {
    let text = certificate();
    let cert = call(AgGenesisCert, "getAgGenesisCert", "[]", &text)
        .map_err(|_| "certificate call failed")?
        .ok_or("certificate missing")?;
    require(
        cert.block.slot == u64::MAX
            && cert.block.block_id == [17; 32]
            && cert.signature.signature == [42; 192]
            && cert.signature.bitmap == [0, 255]
            && call(AgGenesisCert, "getAgGenesisCert", "[]", "null")
                .map_err(|_| "null certificate failed")?
                .is_none(),
        "certificate fields changed",
    )?;
    for bad in [
        text.replacen("17,", "", 1),
        text.replacen("42,", "", 1),
        text.replace("[0,255]", "[256]"),
    ] {
        require(
            matches!(
                call(AgGenesisCert, "getAgGenesisCert", "[]", &bad),
                Err(Error::Schema(_))
            ),
            "malformed certificate accepted",
        )?;
    }
    require(
        matches!(
            call(
                HighestSnapshotSlot,
                "getHighestSnapshotSlot",
                "[]",
                r#"{"full":1}"#
            ),
            Err(Error::Schema(_))
        ),
        "missing nullable snapshot field accepted",
    )?;
    require(
        matches!(
            framed(
                GetHealth,
                "getHealth",
                "[]",
                r#"{"jsonrpc":"2.0","id":2,"result":"ok"}"#
            ),
            Err(Error::Correlation)
        ),
        "node correlation failure accepted",
    )?;
    require(
        matches!(
            framed(
                Slot(None),
                "getSlot",
                "[]",
                r#"{"jsonrpc":"2.0","id":1,"result":1,"result":2}"#
            ),
            Err(Error::Json(_))
        ),
        "node duplicate reply accepted",
    )
}

#[test]
fn all_node_groups_execute() {
    scalar_metadata().unwrap();
    structured_metadata().unwrap();
    certificate_and_failures().unwrap();
}
