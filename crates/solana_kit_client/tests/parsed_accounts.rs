//! Typed `jsonParsed` projection for token, stake, vote, nonce, lookup table,
//! loader and config families. Literals are independently authored from the
//! pinned Dart declarations and their published example values.
#![cfg(feature = "parsed-accounts")]

mod parsed_accounts_support;

use parsed_accounts_support::{ADDRESS, decode, exact, parsed_account};
use serde_json::json;
use solana_kit_client::{
    accounts::assert_account_decoded,
    methods::accounts::AccountData,
    parsed::{
        AddressLookupTableInfo, Base64Encoding, BpfProgramDataInfo, BpfProgramInfo,
        BpfUpgradeableLoaderProgramAccount, ConfigProgramAccount, NonceInfo, ParsedAccountData,
        StakeAccountInfo, StakeProgramAccount, TokenAccountState, TokenProgramAccount,
        ValidatorInfoData,
    },
};
use solana_kit_core::{Address, Hash};

const TOKEN_ACCOUNT: &str = r#"{
  "type": "account",
  "info": {
    "isNative": false,
    "mint": "Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr",
    "owner": "6UsGbaMgchgj4wiwKKuE1v5URHdcDfEiMSM25QpesKir",
    "state": "initialized",
    "tokenAmount": {
      "amount": "9999999779500000",
      "decimals": 6,
      "uiAmount": 9999999779.5,
      "uiAmountString": "9999999779.5"
    },
    "delegate": "Fkc4FN7PPhyGsAcHPW3dBBJ4BvtYkDr2rBFBgFpvy3nB",
    "delegatedAmount": {
      "amount": "18446744073709551615",
      "decimals": 6,
      "uiAmount": 1.8446744073709552e19,
      "uiAmountString": "18446744073709.551615"
    },
    "closeAuthority": "5scSndUhfZJ8j8wZz5UNHhvuPBhvN1RboTdkKSvFHLtW",
    "rentExemptReserve": {
      "amount": "2039280",
      "decimals": 9,
      "uiAmount": 0.00203928,
      "uiAmountString": "0.00203928"
    },
    "extensions": [
      {"extension": "futureExtension", "state": {"nested": [1, 18446744073709551615]}}
    ]
  }
}"#;

const TOKEN_MINT: &str = r#"{
  "type": "mint",
  "info": {
    "decimals": 6,
    "isInitialized": true,
    "supply": "1792635195340523528",
    "mintAuthority": "Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr",
    "freezeAuthority": null,
    "extensions": [{"extension": "transferFeeConfig", "state": {"withheldAmount": 42}}]
  }
}"#;

const TOKEN_MULTISIG: &str = r#"{
  "type": "multisig",
  "info": {
    "isInitialized": true,
    "numRequiredSigners": 2,
    "numValidSigners": 2,
    "signers": [
      "Fkc4FN7PPhyGsAcHPW3dBBJ4BvtYkDr2rBFBgFpvy3nB",
      "5scSndUhfZJ8j8wZz5UNHhvuPBhvN1RboTdkKSvFHLtW"
    ]
  }
}"#;

const STAKE_META: &str = r#""meta": {
    "authorized": {
      "staker": "3HRNKNXafhr3wE9NSXRpNVdFYt6EVygdqFwqf6WpG57V",
      "withdrawer": "3HRNKNXafhr3wE9NSXRpNVdFYt6EVygdqFwqf6WpG57V"
    },
    "lockup": {
      "custodian": "11111111111111111111111111111111",
      "epoch": 0,
      "unixTimestamp": -9223372036854775808
    },
    "rentExemptReserve": "2282880"
  }"#;

fn stake_delegated() -> String {
    format!(
        r#"{{
  "type": "delegated",
  "info": {{
    {STAKE_META},
    "stake": {{
      "creditsObserved": 169965713,
      "delegation": {{
        "activationEpoch": "386",
        "deactivationEpoch": "18446744073709551615",
        "stake": "8007935",
        "voter": "CertusDeBmqN8ZawdkxK5kFGMwBXdudvWHYwtNgNhvLu",
        "warmupCooldownRate": 0.25
      }}
    }}
  }}
}}"#
    )
}

const VOTE: &str = r#"{
  "type": "vote",
  "info": {
    "authorizedVoters": [
      {"authorizedVoter": "HMU77m6WSL9Xew9YvVCgz1hLuhzamz74eD9avi4XPdr", "epoch": 529}
    ],
    "authorizedWithdrawer": "HMU77m6WSL9Xew9YvVCgz1hLuhzamz74eD9avi4XPdr",
    "blockRevenueCollector": "Vote111111111111111111111111111111111111111",
    "blockRevenueCommissionBps": 65535,
    "blsPubkeyCompressed": "bls-compressed-key",
    "commission": 50,
    "epochCredits": [
      {"credits": "18446744073709551615", "epoch": 466, "previousCredits": "68325825"}
    ],
    "inflationRewardsCollector": "Vote111111111111111111111111111111111111111",
    "inflationRewardsCommissionBps": 250,
    "lastTimestamp": {"slot": 18446744073709551615, "timestamp": -9223372036854775808},
    "nodePubkey": "HMU77m6WSL9Xew9YvVCgz1hLuhzamz74eD9avi4XPdr",
    "pendingDelegatorRewards": "123456789",
    "priorVoters": [
      {
        "authorizedPubkey": "HMU77m6WSL9Xew9YvVCgz1hLuhzamz74eD9avi4XPdr",
        "epochOfLastAuthorizedSwitch": 1,
        "targetEpoch": 2
      }
    ],
    "rootSlot": 228884499,
    "votes": [
      {"confirmationCount": 31, "latency": 2, "slot": 228884500},
      {"confirmationCount": 30, "slot": 228884501}
    ]
  }
}"#;

const VOTE_REQUIRED_ONLY: &str = r#"{
  "type": "vote",
  "info": {
    "authorizedVoters": [],
    "authorizedWithdrawer": "HMU77m6WSL9Xew9YvVCgz1hLuhzamz74eD9avi4XPdr",
    "commission": 0,
    "epochCredits": [],
    "lastTimestamp": {"slot": 0, "timestamp": 0},
    "nodePubkey": "HMU77m6WSL9Xew9YvVCgz1hLuhzamz74eD9avi4XPdr",
    "priorVoters": [],
    "votes": []
  }
}"#;

const NONCE: &str = r#"{
  "type": "initialized",
  "info": {
    "authority": "3xxDCjN8s6MgNHwdRExRLa6gHmmRTWPnUdzkbKfEgNAe",
    "blockhash": "TcVy2wVcs7WqWVopv8LAJBHQfqVYZrm8UDqjDvBFQt8",
    "feeCalculator": {"lamportsPerSignature": "18446744073709551615"}
  }
}"#;

const LOOKUP_TABLE: &str = r#"{
  "type": "lookupTable",
  "info": {
    "addresses": [
      "F1Vc6AGoxXLwGB7QV8f4So3C5d8SXEk3KKGHxKGEJ8qn",
      "FWscgV4VDSsMxkQg7jZ4HksqjLyadJS5RiCnAVZv2se9"
    ],
    "authority": "4msgK65vdz5ADUAB3eTQGpF388NuQUAoknLxutUQJd5B",
    "deactivationSlot": "18446744073709551615",
    "lastExtendedSlot": "204699234",
    "lastExtendedSlotStartIndex": 255
  }
}"#;

const LOOKUP_TABLE_NO_AUTHORITY: &str = r#"{
  "type": "lookupTable",
  "info": {
    "addresses": [],
    "deactivationSlot": "0",
    "lastExtendedSlot": "0",
    "lastExtendedSlotStartIndex": 0
  }
}"#;

const LOADER_PROGRAM: &str = r#"{
  "type": "program",
  "info": {"programData": "3vnUTQbDuCgfVn7yQcigUwMQNGkLBZ7GfKWb3gYbAY23"}
}"#;

const LOADER_PROGRAM_DATA: &str = r#"{
  "type": "programData",
  "info": {
    "authority": "7g4Los4WMQnpxYiBJpU1HejBiM6xCk5RDFGCABhWE9M6",
    "data": ["f0VMRgIBAAAAAAAAA", "base64"],
    "slot": 259942942
  }
}"#;

const LOADER_PROGRAM_DATA_NO_AUTHORITY: &str = r#"{
  "type": "programData",
  "info": {"authority": null, "data": ["", "base64"], "slot": 0}
}"#;

const CONFIG_STAKE: &str = r#"{
  "type": "stakeConfig",
  "info": {"slashPenalty": 12, "warmupCooldownRate": 0.25}
}"#;

const CONFIG_VALIDATOR: &str = r#"{
  "type": "validatorInfo",
  "info": {
    "configData": {"name": "HoldTheNode", "website": "https://holdthenode.com"},
    "keys": [
      {"pubkey": "Va1idator1nfo111111111111111111111111111111", "signer": false},
      {"pubkey": "5hvJ19nRgtzAkosb5bcx9bqeN2QA1Qwxq4M349Q2L6s2", "signer": true}
    ]
  }
}"#;

fn address(text: &str) -> Address {
    text.parse().unwrap()
}

#[test]
fn token_account_projects_every_declared_member_and_keeps_extensions_exact() {
    let account = parsed_account("spl-token", TOKEN_ACCOUNT);
    let parsed = assert_account_decoded(&account).unwrap();
    let before = parsed.parsed.get().to_owned();
    let projected = parsed
        .decode_typed(parsed_accounts_support::LIMITS)
        .unwrap();

    let ParsedAccountData::Token(TokenProgramAccount::Account(info)) = projected else {
        panic!("token account expected")
    };

    assert_eq!(
        info.mint,
        address("Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr")
    );
    assert_eq!(
        info.owner,
        address("6UsGbaMgchgj4wiwKKuE1v5URHdcDfEiMSM25QpesKir")
    );
    assert!(!info.is_native);
    assert_eq!(info.state, TokenAccountState::Initialized);
    assert_eq!(info.token_amount.amount, 9_999_999_779_500_000);
    assert_eq!(info.token_amount.decimals, 6);
    assert_eq!(info.token_amount.ui_amount_string, "9999999779.5");
    assert_eq!(
        info.delegate,
        Some(address("Fkc4FN7PPhyGsAcHPW3dBBJ4BvtYkDr2rBFBgFpvy3nB"))
    );
    assert_eq!(
        info.delegated_amount.unwrap().amount,
        u64::MAX,
        "exact full-width delegated amount"
    );
    assert_eq!(
        info.close_authority,
        Some(address("5scSndUhfZJ8j8wZz5UNHhvuPBhvN1RboTdkKSvFHLtW"))
    );
    assert_eq!(info.rent_exempt_reserve.unwrap().amount, 2_039_280);
    let extensions = info.extensions.unwrap();
    let extension_value: serde_json::Value = serde_json::from_str(extensions.get()).unwrap();
    assert_eq!(
        extension_value,
        json!([{
            "extension": "futureExtension",
            "state": {"nested": [1, 18446744073709551615_u64]}
        }])
    );

    assert_eq!(parsed.parsed.get(), before);
    assert_eq!(parsed.program, "spl-token");
    assert_eq!(parsed.space, Some(165));
}

#[test]
fn token_2022_uses_the_same_reference_account_mint_and_multisig_shapes() {
    let projected = decode("spl-token-2022", TOKEN_MINT).unwrap();
    let ParsedAccountData::Token(TokenProgramAccount::Mint(info)) = projected else {
        panic!("mint expected")
    };

    assert_eq!(info.decimals, 6);
    assert!(info.is_initialized);
    assert_eq!(info.supply, 1_792_635_195_340_523_528);
    assert_eq!(
        info.mint_authority,
        Some(address("Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr"))
    );
    assert_eq!(info.freeze_authority, None, "explicit null is absent");
    assert!(info.extensions.is_some());

    let ParsedAccountData::Token(TokenProgramAccount::Multisig(info)) =
        decode("spl-token", TOKEN_MULTISIG).unwrap()
    else {
        panic!("multisig expected")
    };

    assert!(info.is_initialized);
    assert_eq!((info.num_required_signers, info.num_valid_signers), (2, 2));
    assert_eq!(
        info.signers,
        vec![
            address("Fkc4FN7PPhyGsAcHPW3dBBJ4BvtYkDr2rBFBgFpvy3nB"),
            address("5scSndUhfZJ8j8wZz5UNHhvuPBhvN1RboTdkKSvFHLtW"),
        ]
    );

    let ParsedAccountData::Token(TokenProgramAccount::Mint(info)) = decode(
        "spl-token",
        r#"{"type":"mint","info":{"decimals":255,"isInitialized":false,"supply":"18446744073709551615"}}"#,
    )
    .unwrap() else {
        panic!("mint expected")
    };

    assert_eq!(info.supply, u64::MAX);
    assert_eq!(info.mint_authority, None);
    assert_eq!(info.freeze_authority, None);
    assert!(info.extensions.is_none());

    let ParsedAccountData::Token(TokenProgramAccount::Mint(info)) = decode(
        "spl-token",
        r#"{"type":"mint","info":{"decimals":0,"extensions":null,"isInitialized":true,"supply":"0"}}"#,
    )
    .unwrap() else {
        panic!("mint expected")
    };
    assert!(info.extensions.is_none(), "explicit null is absent");
}

fn stake_info(projected: ParsedAccountData) -> StakeAccountInfo {
    match projected {
        ParsedAccountData::Stake(StakeProgramAccount::Delegated(info))
        | ParsedAccountData::Stake(StakeProgramAccount::Initialized(info)) => info,
        _ => panic!("stake expected"),
    }
}

#[test]
fn delegated_and_initialized_stake_share_the_reference_meta_shape() {
    let delegated = stake_info(decode("stake", &stake_delegated()).unwrap());
    let stake = delegated.stake.unwrap();
    let delegation = stake.delegation;

    assert_eq!(
        delegated.meta.authorized.staker,
        address("3HRNKNXafhr3wE9NSXRpNVdFYt6EVygdqFwqf6WpG57V")
    );
    assert_eq!(
        delegated.meta.lockup.custodian,
        address("11111111111111111111111111111111")
    );
    assert_eq!(delegated.meta.lockup.epoch, 0);
    assert_eq!(delegated.meta.lockup.unix_timestamp, i64::MIN);
    assert_eq!(delegated.meta.rent_exempt_reserve, 2_282_880);
    assert_eq!(stake.credits_observed, 169_965_713);
    assert_eq!(delegation.activation_epoch, 386);
    assert_eq!(delegation.deactivation_epoch, u64::MAX);
    assert_eq!(delegation.stake, 8_007_935);
    assert_eq!(
        delegation.voter,
        address("CertusDeBmqN8ZawdkxK5kFGMwBXdudvWHYwtNgNhvLu")
    );
    assert_eq!(delegation.warmup_cooldown_rate, Some(0.25));

    let initialized = format!(r#"{{"type":"initialized","info":{{{STAKE_META},"stake":null}}}}"#);
    assert_eq!(
        stake_info(decode("stake", &initialized).unwrap()).stake,
        None
    );

    let absent = format!(r#"{{"type":"initialized","info":{{{STAKE_META}}}}}"#);
    assert_eq!(stake_info(decode("stake", &absent).unwrap()).stake, None);
}

#[test]
fn vote_projects_required_and_optional_members_with_exact_integers() {
    let ParsedAccountData::Vote(info) = decode("vote", VOTE).unwrap() else {
        panic!("vote expected")
    };

    assert_eq!(info.authorized_voters.len(), 1);
    assert_eq!(info.authorized_voters[0].epoch, 529);
    assert_eq!(
        info.authorized_voters[0].authorized_voter,
        address("HMU77m6WSL9Xew9YvVCgz1hLuhzamz74eD9avi4XPdr")
    );
    assert_eq!(
        info.authorized_withdrawer,
        address("HMU77m6WSL9Xew9YvVCgz1hLuhzamz74eD9avi4XPdr")
    );
    assert_eq!(info.commission, 50);
    assert_eq!(info.block_revenue_commission_bps, Some(u16::MAX));
    assert_eq!(info.inflation_rewards_commission_bps, Some(250));
    assert_eq!(
        info.bls_pubkey_compressed.as_deref(),
        Some("bls-compressed-key")
    );
    assert_eq!(info.epoch_credits[0].credits, u64::MAX);
    assert_eq!(info.epoch_credits[0].previous_credits, 68_325_825);
    assert_eq!(info.last_timestamp.slot, u64::MAX);
    assert_eq!(info.last_timestamp.timestamp, i64::MIN);
    assert_eq!(info.pending_delegator_rewards, Some(123_456_789));
    assert_eq!(info.root_slot, Some(228_884_499));
    assert_eq!(info.prior_voters[0].epoch_of_last_authorized_switch, 1);
    assert_eq!(info.prior_voters[0].target_epoch, 2);
    assert_eq!(info.votes.len(), 2);
    assert_eq!(info.votes[0].confirmation_count, 31);
    assert_eq!(info.votes[0].latency, Some(2));
    assert_eq!(info.votes[0].slot, 228_884_500);
    assert_eq!(info.votes[1].latency, None);

    let ParsedAccountData::Vote(info) = decode("vote", VOTE_REQUIRED_ONLY).unwrap() else {
        panic!("vote expected")
    };
    assert_eq!(info.block_revenue_collector, None);
    assert_eq!(info.block_revenue_commission_bps, None);
    assert_eq!(info.bls_pubkey_compressed, None);
    assert_eq!(info.inflation_rewards_collector, None);
    assert_eq!(info.pending_delegator_rewards, None);
    assert_eq!(info.root_slot, None);
    assert!(info.authorized_voters.is_empty());
    assert!(info.votes.is_empty());
}

#[test]
fn nonce_projects_authority_blockhash_and_full_width_fee_rate() {
    let ParsedAccountData::Nonce(NonceInfo {
        authority,
        blockhash,
        fee_calculator,
    }) = decode("nonce", NONCE).unwrap()
    else {
        panic!("nonce expected")
    };

    assert_eq!(
        authority,
        address("3xxDCjN8s6MgNHwdRExRLa6gHmmRTWPnUdzkbKfEgNAe")
    );
    assert_eq!(
        blockhash,
        "TcVy2wVcs7WqWVopv8LAJBHQfqVYZrm8UDqjDvBFQt8"
            .parse::<Hash>()
            .unwrap()
    );
    assert_eq!(fee_calculator.lamports_per_signature, u64::MAX);
}

#[test]
fn lookup_table_projects_order_authority_and_stringified_slots() {
    let ParsedAccountData::AddressLookupTable(AddressLookupTableInfo {
        addresses,
        authority,
        deactivation_slot,
        last_extended_slot,
        last_extended_slot_start_index,
    }) = decode("address-lookup-table", LOOKUP_TABLE).unwrap()
    else {
        panic!("lookup table expected")
    };

    assert_eq!(
        addresses,
        vec![
            address("F1Vc6AGoxXLwGB7QV8f4So3C5d8SXEk3KKGHxKGEJ8qn"),
            address("FWscgV4VDSsMxkQg7jZ4HksqjLyadJS5RiCnAVZv2se9"),
        ]
    );
    assert_eq!(
        authority,
        Some(address("4msgK65vdz5ADUAB3eTQGpF388NuQUAoknLxutUQJd5B"))
    );
    assert_eq!(deactivation_slot, u64::MAX);
    assert_eq!(last_extended_slot, 204_699_234);
    assert_eq!(last_extended_slot_start_index, u8::MAX);

    let ParsedAccountData::AddressLookupTable(info) =
        decode("address-lookup-table", LOOKUP_TABLE_NO_AUTHORITY).unwrap()
    else {
        panic!("lookup table expected")
    };
    assert_eq!(info.authority, None);
    assert!(info.addresses.is_empty());
}

#[test]
fn loader_program_and_program_data_keep_encoded_bytes_exact() {
    let ParsedAccountData::BpfUpgradeableLoader(BpfUpgradeableLoaderProgramAccount::Program(
        BpfProgramInfo { program_data },
    )) = decode("bpf-upgradeable-loader", LOADER_PROGRAM).unwrap()
    else {
        panic!("loader program expected")
    };
    assert_eq!(
        program_data,
        address("3vnUTQbDuCgfVn7yQcigUwMQNGkLBZ7GfKWb3gYbAY23")
    );

    let ParsedAccountData::BpfUpgradeableLoader(BpfUpgradeableLoaderProgramAccount::ProgramData(
        BpfProgramDataInfo {
            authority,
            data,
            slot,
        },
    )) = decode("bpf-upgradeable-loader", LOADER_PROGRAM_DATA).unwrap()
    else {
        panic!("program data expected")
    };

    assert_eq!(
        authority,
        Some(address("7g4Los4WMQnpxYiBJpU1HejBiM6xCk5RDFGCABhWE9M6"))
    );
    assert_eq!(data.data, "f0VMRgIBAAAAAAAAA");
    assert_eq!(data.encoding, Base64Encoding::Base64);
    assert_eq!(slot, 259_942_942);

    let ParsedAccountData::BpfUpgradeableLoader(BpfUpgradeableLoaderProgramAccount::ProgramData(
        info,
    )) = decode("bpf-upgradeable-loader", LOADER_PROGRAM_DATA_NO_AUTHORITY).unwrap()
    else {
        panic!("program data expected")
    };
    assert_eq!(info.authority, None);
    assert_eq!(info.data.data, "");
}

#[test]
fn config_projects_legacy_stake_config_and_opaque_validator_config_data() {
    let ParsedAccountData::Config(ConfigProgramAccount::StakeConfig(info)) =
        decode("config", CONFIG_STAKE).unwrap()
    else {
        panic!("stake config expected")
    };
    assert_eq!(info.slash_penalty, 12);
    assert_eq!(info.warmup_cooldown_rate, 0.25);

    let ParsedAccountData::Config(ConfigProgramAccount::ValidatorInfo(ValidatorInfoData {
        config_data,
        keys,
    })) = decode("config", CONFIG_VALIDATOR).unwrap()
    else {
        panic!("validator info expected")
    };

    let value: serde_json::Value = serde_json::from_str(config_data.get()).unwrap();
    assert_eq!(
        value,
        json!({"name": "HoldTheNode", "website": "https://holdthenode.com"})
    );
    assert_eq!(keys.len(), 2);
    assert_eq!(
        keys[0].pubkey,
        address("Va1idator1nfo111111111111111111111111111111")
    );
    assert!(!keys[0].signer);
    assert_eq!(
        keys[1].pubkey,
        address("5hvJ19nRgtzAkosb5bcx9bqeN2QA1Qwxq4M349Q2L6s2")
    );
    assert!(keys[1].signer);

    let ParsedAccountData::Config(ConfigProgramAccount::ValidatorInfo(info)) = decode(
        "config",
        r#"{"type":"validatorInfo","info":{"configData":null,"keys":[]}}"#,
    )
    .unwrap() else {
        panic!("validator info expected")
    };
    assert_eq!(info.config_data.get(), "null");
    assert!(info.keys.is_empty());
}

#[test]
fn the_original_parsed_account_survives_projection_unchanged() {
    let account = parsed_account("stake", &stake_delegated());
    let raw = exact(&account).to_owned();
    let parsed = assert_account_decoded(&account).unwrap();
    let program = parsed.program.clone();
    let space = parsed.space;

    let _ = parsed
        .decode_typed(parsed_accounts_support::LIMITS)
        .unwrap();

    let parsed = assert_account_decoded(&account).unwrap();
    assert_eq!(parsed.parsed.get(), raw);
    assert_eq!(parsed.program, program);
    assert_eq!(parsed.space, space);
    assert_eq!(exact(&account), raw);
}

#[test]
fn binary_fallback_is_never_reinterpreted_as_parsed_json() {
    let raw = parsed_accounts_support::account_json("spl-token", r#"{"type":"account","info":{}}"#)
        .replace(
            r#"{"parsed":{"type":"account","info":{}},"program":"spl-token","space":165}"#,
            r#"["AQID","base64"]"#,
        );
    let account = solana_kit_client::accounts::parse_json_rpc_account(ADDRESS, &raw)
        .unwrap()
        .require()
        .unwrap();

    assert!(matches!(account.data, AccountData::Bytes { .. }));
    assert!(assert_account_decoded(&account).is_err());
}
