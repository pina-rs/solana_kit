//! Clock targets and profiling placeholder positions are separate wire contracts.

use super::{call, require};
use solana_kit_surfpool::{cheatcodes::*, types::TimeTarget};

const EPOCH: &str = r#"{"absoluteSlot":9007199254740993,"slotIndex":0,"slotsInEpoch":432000,"epoch":0,"blockHeight":1}"#;

pub fn run() -> Result<(), &'static str> {
    let response = call(
        TimeTravel(TimeTarget::AbsoluteSlot(9_007_199_254_740_993)),
        "surfnet_timeTravel",
        r#"[{"absoluteSlot":9007199254740993}]"#,
        EPOCH,
    )?;
    require(
        response.value.absolute_slot == 9_007_199_254_740_993,
        "epoch slot lost exact integer precision",
    )?;
    call(
        TimeTravel(TimeTarget::AbsoluteEpoch(2)),
        "surfnet_timeTravel",
        r#"[{"absoluteEpoch":2}]"#,
        EPOCH,
    )?;
    call(
        TimeTravel(TimeTarget::AbsoluteTimestamp(3)),
        "surfnet_timeTravel",
        r#"[{"absoluteTimestamp":3}]"#,
        EPOCH,
    )?;
    call(PauseClock, "surfnet_pauseClock", "[]", EPOCH)?;
    call(ResumeClock, "surfnet_resumeClock", "[]", EPOCH)?;
    profiles()
}

fn profiles() -> Result<(), &'static str> {
    call(
        ProfileTransaction {
            transaction_data: "tx".into(),
            tag: None,
            config: Some(serde_json::Map::new()),
        },
        "surfnet_profileTransaction",
        r#"["tx",null,{}]"#,
        "{}",
    )?;
    call(
        ProfileTransaction {
            transaction_data: "tx".into(),
            tag: None,
            config: None,
        },
        "surfnet_profileTransaction",
        r#"["tx"]"#,
        "{}",
    )?;

    call(
        GetTransactionProfile {
            signature_or_uuid: "signature".into(),
            config: None,
        },
        "surfnet_getTransactionProfile",
        r#"["signature"]"#,
        "{}",
    )?;
    call(
        GetProfileResultsByTag {
            tag: "tag".into(),
            config: Some(serde_json::Map::new()),
        },
        "surfnet_getProfileResultsByTag",
        r#"["tag",{}]"#,
        "[]",
    )?;
    Ok(())
}
