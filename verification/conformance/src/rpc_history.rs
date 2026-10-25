//! Public historical operations retain exact slots and required metadata.
extern crate std;

use crate::{require, rpc_fixture::call};
use solana_kit_client::{
    Presence,
    methods::{LatestBlockhash, history::*},
};
use solana_kit_core::{Address, Signature};
use std::{
    format,
    string::{String, ToString},
};

pub(super) const KEY: &str = "11111111111111111111111111111111";
pub(super) const SIG: &str = concat!(
    "11111111111111111111111111111111",
    "1111111111111111111111111111111",
    "2"
);
pub(super) const MAX: &str = "18446744073709551615";

pub(super) fn signature() -> Signature {
    let mut bytes = [0; 64];
    bytes[63] = 1;
    Signature::from_bytes(bytes)
}

pub(super) fn entry() -> String {
    format!(
        concat!(
            r#"{{"signature":"{0}","slot":{1},"err":null,"memo":null,"#,
            r#""blockTime":-9223372036854775808,"confirmationStatus":"finalized","#,
            r#""transactionIndex":4294967295}}"#
        ),
        SIG, MAX
    )
}

pub(crate) fn queries() -> Result<(), &'static str> {
    require(signature().to_string() == SIG, "signature oracle changed")?;
    let latest = call(
        LatestBlockhash(None),
        "getLatestBlockhash",
        "[]",
        &format!(
            concat!(
                r#"{{"context":{{"slot":{0}}},"value":{{"blockhash":"{1}","#,
                r#""lastValidBlockHeight":{0}}}}}"#
            ),
            MAX, KEY
        ),
    )
    .map_err(|_| "latest blockhash failed")?;
    require(
        latest.context.slot == u64::MAX && latest.value.last_valid_block_height == u64::MAX,
        "latest block height narrowed",
    )?;

    block_times()?;
    block_ranges()?;
    signature_history()?;
    malformed()
}

/// Signed minimum timestamps and null responses stay distinct from commitment stake.
fn block_times() -> Result<(), &'static str> {
    let time = call(
        BlockTime(u64::MAX),
        "getBlockTime",
        &format!("[{MAX}]"),
        "-9223372036854775808",
    )
    .map_err(|_| "block time failed")?;
    require(time == Some(i64::MIN), "signed block time changed")?;

    require(
        call(BlockTime(1), "getBlockTime", "[1]", "null")
            .map_err(|_| "null time failed")?
            .is_none(),
        "null time invented",
    )?;
    let commitment = call(
        BlockCommitment(1),
        "getBlockCommitment",
        "[1]",
        &format!(r#"{{"commitment":null,"totalStake":{MAX}}}"#),
    )
    .map_err(|_| "block commitment failed")?;
    require(
        commitment.commitment.is_none() && commitment.total_stake == u64::MAX,
        "stake changed",
    )
}

/// Block-range ordering and exact slot limits survive the same public client path.
fn block_ranges() -> Result<(), &'static str> {
    let slots = call(
        Blocks {
            start: 2,
            end: Some(4),
            config: Some(BlocksConfig {
                commitment: Some(HistoricalCommitment::Finalized),
            }),
        },
        "getBlocks",
        r#"[2,4,{"commitment":"finalized"}]"#,
        "[2,4]",
    )
    .map_err(|_| "block range failed")?;
    require(slots == [2, 4], "block order changed")?;
    let limited = call(
        BlocksWithLimit {
            start: 2,
            limit: 2,
            config: None,
        },
        "getBlocksWithLimit",
        "[2,2]",
        &format!("[2,{MAX}]"),
    )
    .map_err(|_| "block limit failed")?;
    require(limited == [2, u64::MAX], "limited slots changed")?;
    require(
        call(FirstAvailableBlock, "getFirstAvailableBlock", "[]", MAX)
            .map_err(|_| "first block failed")?
            == u64::MAX,
        "first block narrowed",
    )?;
    require(
        call(MinimumLedgerSlot, "minimumLedgerSlot", "[]", "0")
            .map_err(|_| "minimum slot failed")?
            == 0,
        "minimum slot changed",
    )
}

/// Signature pages preserve positions and full-width metadata under explicit filters.
fn signature_history() -> Result<(), &'static str> {
    let config = SignaturesConfig {
        before: Some(signature()),
        until: Some(signature()),
        limit: Some(1),
        commitment: Some(HistoricalCommitment::Confirmed),
        min_context_slot: Some(u64::MAX),
    };
    let params = format!(
        concat!(
            r#"["{0}",{{"before":"{1}","until":"{1}","limit":1,"#,
            r#""commitment":"confirmed","minContextSlot":{2}}}]"#
        ),
        KEY, SIG, MAX
    );

    let result = format!("[{}]", entry());
    let values = call(
        SignaturesForAddress(Address::from_bytes([0; 32]), Some(config)),
        "getSignaturesForAddress",
        &params,
        &result,
    )
    .map_err(|_| "signature history failed")?;
    require(
        values.len() == 1
            && values[0].signature == signature()
            && values[0].slot == u64::MAX
            && values[0].block_time == Some(i64::MIN)
            && values[0].transaction_index == Presence::Present(u32::MAX),
        "signature history changed",
    )
}

fn malformed() -> Result<(), &'static str> {
    for invalid in ["1.0", "1e2", "\"1\"", "9223372036854775808", "{}"] {
        require(
            call(BlockTime(1), "getBlockTime", "[1]", invalid).is_err(),
            "invalid block time accepted",
        )?;
    }

    for result in ["[1]", "[2,2]", "[4,2]", "[2,5]"] {
        require(
            call(
                Blocks {
                    start: 2,
                    end: Some(4),
                    config: None,
                },
                "getBlocks",
                "[2,4]",
                result,
            )
            .is_err(),
            "invalid block range accepted",
        )?;
    }

    required_history_fields()
}

/// Omitting any required member must fail instead of inventing nullable defaults.
fn required_history_fields() -> Result<(), &'static str> {
    let params = format!(r#"["{KEY}"]"#);

    for field in [
        "signature",
        "slot",
        "err",
        "memo",
        "blockTime",
        "confirmationStatus",
    ] {
        let original = entry();

        let needle = match field {
            "signature" => format!(r#""signature":"{SIG}","#),
            "slot" => format!(r#""slot":{MAX},"#),
            "blockTime" => r#""blockTime":-9223372036854775808,"#.into(),
            "confirmationStatus" => r#""confirmationStatus":"finalized","#.into(),
            _ => format!(r#""{field}":null,"#),
        };

        let result = format!("[{}]", original.replace(&needle, ""));
        require(
            call(
                SignaturesForAddress(Address::from_bytes([0; 32]), None),
                "getSignaturesForAddress",
                &params,
                &result,
            )
            .is_err(),
            "missing history member accepted",
        )?;
    }

    Ok(())
}
