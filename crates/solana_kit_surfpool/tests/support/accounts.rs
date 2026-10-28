//! Account mutation oracles cover all eight account/key/program wire methods.

use super::{call, require};
use solana_kit_core::Address;
use solana_kit_surfpool::{builders::*, cheatcodes::*, hex::HexBytes, types::*};
use std::{format, vec};

const ADDRESS: Address = Address::from_bytes([0; 32]);
const TEXT: &str = "11111111111111111111111111111111";

pub fn run() -> Result<(), &'static str> {
    let update = SetAccount::new(ADDRESS)
        .with_lamports(u64::MAX)
        .with_data(vec![0, 127, 255])
        .with_owner(ADDRESS)
        .with_rent_epoch(0)
        .with_executable(false);
    call(
        update,
        "surfnet_setAccount",
        &format!(
            r#"["{TEXT}",{{"lamports":18446744073709551615,"data":"007fff","owner":"{TEXT}","rentEpoch":0,"executable":false}}]"#
        ),
        "null",
    )?;
    token()?;
    call(
        ResetAccount::new(ADDRESS),
        "surfnet_resetAccount",
        &format!(r#"["{TEXT}"]"#),
        "null",
    )?;
    call(
        StreamAccount::new(ADDRESS).with_include_owned_accounts(false),
        "surfnet_streamAccount",
        &format!(r#"["{TEXT}",{{"includeOwnedAccounts":false}}]"#),
        "null",
    )?;
    call(
        OfflineAccount {
            address: ADDRESS,
            config: Some(serde_json::Map::new()),
        },
        "surfnet_offlineAccount",
        &format!(r#"["{TEXT}",{{}}]"#),
        "null",
    )?;
    call(
        StreamAccounts(vec![serde_json::Map::new()]),
        "surfnet_streamAccounts",
        "[[{}]]",
        "null",
    )?;
    confidential()?;

    program()
}

fn token() -> Result<(), &'static str> {
    let update = SetTokenAccount::new(ADDRESS, ADDRESS)
        .with_amount(9_007_199_254_740_993)
        .with_delegate(ADDRESS)
        .without_delegate()
        .with_close_authority(ADDRESS)
        .without_close_authority()
        .with_delegated_amount(0)
        .with_state("initialized")
        .with_token_program(ADDRESS);
    call(
        update,
        "surfnet_setTokenAccount",
        &format!(
            r#"["{TEXT}","{TEXT}",{{"amount":9007199254740993,"delegate":"null","state":"initialized","delegatedAmount":0,"closeAuthority":"null"}},"{TEXT}"]"#
        ),
        "null",
    )?;
    let without_program = SetTokenAccount::new(ADDRESS, ADDRESS)
        .with_delegate(ADDRESS)
        .with_close_authority(ADDRESS);
    call(
        without_program,
        "surfnet_setTokenAccount",
        &format!(r#"["{TEXT}","{TEXT}",{{"delegate":"{TEXT}","closeAuthority":"{TEXT}"}}]"#),
        "null",
    )?;
    Ok(())
}

fn confidential() -> Result<(), &'static str> {
    let response = call(
        GetConfidentialBalance {
            token_account: ADDRESS,
            keys: ConfidentialBalanceKeys::Aes {
                aes_key: SecretString::new("aes"),
            },
        },
        "surfnet_getConfidentialBalance",
        &format!(r#"["{TEXT}",{{"aesKey":"aes"}}]"#),
        r#"{"available":18446744073709551615,"pending":null,"pendingBalanceCreditCounter":2}"#,
    )?;
    require(
        response.value.available == Some(i128::from(u64::MAX)) && response.value.pending.is_none(),
        "confidential balance lost integer precision or null",
    )?;
    let keys = call(
        DeriveConfidentialKeys("signature".into()),
        "surfnet_deriveConfidentialKeys",
        r#"["signature"]"#,
        r#"{"elgamalPubkey":"public","elgamalSecretKey":"secret","aesKey":"aes"}"#,
    )?;
    require(
        keys.value.aes_key.expose() == "aes",
        "confidential key source data was altered",
    )
}

fn program() -> Result<(), &'static str> {
    call(
        CloneProgramAccount {
            source: ADDRESS,
            destination: ADDRESS,
        },
        "surfnet_cloneProgramAccount",
        &format!(r#"["{TEXT}","{TEXT}"]"#),
        "null",
    )?;
    call(
        SetProgramAuthority {
            program_id: ADDRESS,
            new_authority: None,
        },
        "surfnet_setProgramAuthority",
        &format!(r#"["{TEXT}"]"#),
        "null",
    )?;
    call(
        WriteProgram {
            program_id: ADDRESS,
            data: HexBytes(vec![1, 2, 255]),
            offset: u64::MAX,
            authority: Some(ADDRESS),
        },
        "surfnet_writeProgram",
        &format!(r#"["{TEXT}","0102ff",18446744073709551615,"{TEXT}"]"#),
        "null",
    )?;
    Ok(())
}
