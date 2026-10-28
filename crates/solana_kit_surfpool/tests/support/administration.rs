//! Independent method-name and trailing-argument fixtures for provider administration.

use super::{call, require};
use alloc::{format, vec};
use solana_kit_core::Address;
use solana_kit_surfpool::cheatcodes::*;

pub fn run() -> Result<(), &'static str> {
    call(
        EnableCheatcode(CheatcodeFilter::All(AllCheatcodes::All)),
        "surfnet_enableCheatcode",
        r#"["all"]"#,
        "null",
    )?;
    call(
        DisableCheatcode {
            filter: CheatcodeFilter::Methods(vec!["surfnet_setAccount".into()]),
            lockout: Some(serde_json::Map::new()),
        },
        "surfnet_disableCheatcode",
        r#"[["surfnet_setAccount"],{}]"#,
        "null",
    )?;
    call(ExportSnapshot(None), "surfnet_exportSnapshot", "[]", "null")?;
    call(
        ExportSnapshot(Some(serde_json::Map::new())),
        "surfnet_exportSnapshot",
        "[{}]",
        "null",
    )?;
    call(
        GetLocalSignatures(Some(u64::MAX)),
        "surfnet_getLocalSignatures",
        "[18446744073709551615]",
        "[]",
    )?;
    call(
        GetLocalSignatures(None),
        "surfnet_getLocalSignatures",
        "[]",
        "[]",
    )?;
    call(
        GetActiveIdl {
            program_id: Address::from_bytes([0; 32]),
            slot: Some(0),
        },
        "surfnet_getActiveIdl",
        r#"["11111111111111111111111111111111",0]"#,
        "null",
    )?;
    objects()?;

    call(
        GetStreamedAccounts,
        "surfnet_getStreamedAccounts",
        "[]",
        "[]",
    )?;
    call(GetSurfnetInfo, "surfnet_getSurfnetInfo", "[]", "{}")?;
    call(ResetNetwork, "surfnet_resetNetwork", "[]", "null")?;
    Ok(())
}

fn objects() -> Result<(), &'static str> {
    call(
        RegisterIdl {
            idl: serde_json::Map::new(),
            slot: None,
        },
        "surfnet_registerIdl",
        "[{}]",
        "null",
    )?;
    call(
        RegisterScenario {
            scenario: serde_json::Map::new(),
            slot: Some(u64::MAX),
        },
        "surfnet_registerScenario",
        "[{},18446744073709551615]",
        "null",
    )?;
    call(
        SetSupply(serde_json::Map::new()),
        "surfnet_setSupply",
        "[{}]",
        "null",
    )?;
    Ok(())
}

pub fn manifest() -> Result<(), &'static str> {
    let suffixes = [
        "cloneProgramAccount",
        "deriveConfidentialKeys",
        "disableCheatcode",
        "enableCheatcode",
        "exportSnapshot",
        "getActiveIdl",
        "getConfidentialBalance",
        "getLocalSignatures",
        "getProfileResultsByTag",
        "getStreamedAccounts",
        "getSurfnetInfo",
        "getTransactionProfile",
        "offlineAccount",
        "pauseClock",
        "profileTransaction",
        "registerIdl",
        "registerScenario",
        "resetAccount",
        "resetNetwork",
        "resumeClock",
        "setAccount",
        "setProgramAuthority",
        "setSupply",
        "setTokenAccount",
        "streamAccount",
        "streamAccounts",
        "timeTravel",
        "writeProgram",
    ];
    require(
        SURFNET_CHEATCODE_METHODS
            .iter()
            .zip(suffixes)
            .all(|(actual, suffix)| *actual == format!("surfnet_{suffix}")),
        "Surfpool registered method manifest changed",
    )
}
