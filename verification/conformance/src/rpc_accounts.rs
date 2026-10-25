//! Typed account requests and representation boundaries shared by actual hosts.
extern crate std;

mod malformed;
mod tokens;

use crate::{require, rpc_fixture::call};
use solana_kit_client::methods::{Commitment, ReadConfig, accounts::*};
use solana_kit_core::{Address, Hash, accounts::MaybeAccount};
use std::{format, string::String, vec};

const KEY: &str = "11111111111111111111111111111111";
const OTHER: &str = "4vJ9JU1bJJE96FWSJKvHsmmFADCg4gpZQff4P3bkLKi";
const MAX: &str = "18446744073709551615";

pub(crate) fn cases() -> [crate::CaseResult; 3] {
    [
        crate::case("rpc-account-representations", accounts()),
        crate::case("rpc-token-exact-quantities", tokens::quantities()),
        crate::case("rpc-account-token-malformed", malformed::boundaries()),
    ]
}

fn address() -> Address {
    Address::from_bytes([0; 32])
}

fn config(encoding: AccountEncoding) -> AccountConfig {
    AccountConfig {
        encoding: Some(encoding),
        ..Default::default()
    }
}

fn context(value: &str) -> String {
    format!(r#"{{"context":{{"slot":{MAX}}},"value":{value}}}"#)
}

fn account(data: &str) -> String {
    format!(
        concat!(
            r#"{{"data":{0},"lamports":{1},"owner":"{2}","executable":true,"#,
            r#""rentEpoch":{1},"space":3}}"#
        ),
        data, MAX, KEY
    )
}

fn keyed(data: &str) -> String {
    format!(r#"[{{"pubkey":"{OTHER}","account":{}}}]"#, account(data))
}

fn snapshot(
    value: MaybeAccount<AccountData>,
    expected: &[u8],
    slice: Option<DataSlice>,
) -> Result<(), &'static str> {
    let MaybeAccount::Present(value) = value else {
        return Err("present account missing");
    };
    require(
        value.address == address()
            && value.owner == address()
            && value.lamports == u64::MAX
            && value.rent_epoch == Some(u64::MAX)
            && value.executable
            && value.space == 3,
        "account metadata changed",
    )?;
    require(
        matches!(value.data, AccountData::Bytes { bytes, slice: actual }
            if bytes == expected && actual == slice),
        "account bytes or requested slice changed",
    )
}

fn accounts() -> Result<(), &'static str> {
    representations()?;
    slices_and_opaque()?;
    multiple_and_program()?;
    rent_and_hash()
}

fn representations() -> Result<(), &'static str> {
    for (encoding, options, data) in [
        (None, "", r#""Ldp""#),
        (
            Some(AccountEncoding::Base58),
            r#",{"encoding":"base58"}"#,
            r#"["Ldp","base58"]"#,
        ),
        (
            Some(AccountEncoding::Base64),
            r#",{"encoding":"base64"}"#,
            r#"["AQID","base64"]"#,
        ),
        (
            Some(AccountEncoding::JsonParsed),
            r#",{"encoding":"jsonParsed"}"#,
            r#"["AQID","base64"]"#,
        ),
    ] {
        let got = call(
            AccountInfoWithConfig(address(), encoding.map(config)),
            "getAccountInfo",
            &format!(r#"["{KEY}"{options}]"#),
            &context(&account(data)),
        )
        .map_err(|_| "account representation failed")?;
        require(got.context.slot == u64::MAX, "account context changed")?;
        snapshot(got.value, &[1, 2, 3], None)?;
    }
    let missing = call(
        AccountInfoWithConfig(address(), Some(AccountConfig::default())),
        "getAccountInfo",
        &format!(r#"["{KEY}",{{}}]"#),
        &context("null"),
    )
    .map_err(|_| "null account failed")?;
    require(
        matches!(missing.value, MaybeAccount::Missing(key) if key == address()),
        "null identity changed",
    )
}

fn slices_and_opaque() -> Result<(), &'static str> {
    account_slices()?;
    opaque_representations()
}

/// Offset boundaries apply to decoded bytes without altering original metadata.
fn account_slices() -> Result<(), &'static str> {
    const PAST_END: u64 = 9;

    for (offset, length, wire, bytes) in [
        (1, PAST_END, "AgM=", &[2, 3][..]),
        (3, PAST_END, "", &[][..]),
        (u64::MAX, 0, "", &[][..]),
    ] {
        let slice = DataSlice { offset, length };
        let options = AccountConfig {
            data_slice: Some(slice),
            ..config(AccountEncoding::Base64)
        };
        let got = call(
            AccountInfoWithConfig(address(), Some(options)), "getAccountInfo",
            &format!(r#"["{KEY}",{{"encoding":"base64","dataSlice":{{"offset":{offset},"length":{length}}}}}]"#),
            &context(&account(&format!(r#"["{wire}","base64"]"#))),
        ).map_err(|_| "account slice failed")?;
        snapshot(got.value, bytes, Some(slice))?;
    }
    Ok(())
}

/// Parsed JSON stays exact and compressed payloads remain explicitly compressed.
fn opaque_representations() -> Result<(), &'static str> {
    let parsed = call(
        AccountInfoWithConfig(address(), Some(config(AccountEncoding::JsonParsed))),
        "getAccountInfo",
        &format!(r#"["{KEY}",{{"encoding":"jsonParsed"}}]"#),
        &context(&account(
            r#"{"program":"future","space":3,"parsed":{ "number":1e400,"text":"\u0061" }}"#,
        )),
    )
    .map_err(|_| "parsed account failed")?;
    let MaybeAccount::Present(value) = parsed.value else {
        return Err("parsed account missing");
    };
    let AccountData::Parsed(data) = value.data else {
        return Err("parsed representation missing");
    };
    require(
        data.program == "future"
            && data.space == Some(3)
            && data.parsed.get() == r#"{ "number":1e400,"text":"\u0061" }"#,
        "parsed payload changed",
    )?;
    let compressed = call(
        AccountInfoWithConfig(address(), Some(config(AccountEncoding::Base64Zstd))),
        "getAccountInfo",
        &format!(r#"["{KEY}",{{"encoding":"base64+zstd"}}]"#),
        &context(&account(r#"["AQID","base64+zstd"]"#)),
    )
    .map_err(|_| "compressed representation failed")?;

    let MaybeAccount::Present(value) = compressed.value else {
        return Err("compressed account missing");
    };
    require(
        matches!(value.data, AccountData::Zstd { compressed, slice: None }
        if compressed == [1, 2, 3]),
        "compressed bytes changed",
    )
}

fn multiple_and_program() -> Result<(), &'static str> {
    multiple_accounts()?;
    program_accounts()
}

/// Missing entries retain their requested identities rather than losing positions.
fn multiple_accounts() -> Result<(), &'static str> {
    let other = Address::from_bytes([1; 32]);
    let got = call(
        MultipleAccounts(vec![address(), other], None),
        "getMultipleAccounts",
        &format!(r#"[["{KEY}","{OTHER}"]]"#),
        &context(&format!("[{},null]", account(r#"["AQID","base64"]"#))),
    )
    .map_err(|_| "multiple accounts failed")?;
    require(
        got.context.slot == u64::MAX && got.value.len() == 2,
        "multiple context/cardinality changed",
    )?;
    let mut entries = got.value.into_iter();
    snapshot(
        entries.next().ok_or("first account missing")?,
        &[1, 2, 3],
        None,
    )?;
    require(
        matches!(entries.next(), Some(MaybeAccount::Missing(key)) if key == other),
        "multiple null identity changed",
    )
}

/// Explicit filters, context and default encoding retain the program response shape.
fn program_accounts() -> Result<(), &'static str> {
    let other = Address::from_bytes([1; 32]);
    let filters = vec![
        Filter::DataSize(3),
        Filter::Memcmp(
            Memcmp::new(0, vec![1, 2, 3], FilterEncoding::Base64).map_err(|_| "memcmp failed")?,
        ),
        Filter::TokenAccountState,
    ];
    let got = call(
        ProgramAccounts(
            address(),
            Some(ProgramAccountsConfig {
                account: AccountConfig {
                    read: ReadConfig {
                        commitment: Some(Commitment::Finalized),
                        min_context_slot: Some(u64::MAX),
                    },
                    ..config(AccountEncoding::Base64)
                },
                filters: Some(filters),
                with_context: Some(true),
            }),
        ),
        "getProgramAccounts",
        &format!(
            concat!(
                r#"["{0}",{{"commitment":"finalized","minContextSlot":{1},"encoding":"base64","#,
                r#""filters":[{{"dataSize":3}},{{"memcmp":{{"offset":0,"encoding":"base64","#,
                r#""bytes":"AQID"}}}},"tokenAccountState"],"withContext":true}}]"#
            ),
            KEY, MAX
        ),
        &context(&keyed(r#"["AQID","base64"]"#)),
    )
    .map_err(|_| "program filters/context failed")?;

    require(
        matches!(got, ProgramAccountsResponse::Contextual(value)
            if value.context.slot == u64::MAX && value.value.len() == 1
            && value.value[0].address == other && value.value[0].owner == address()),
        "program identity changed",
    )?;
    let plain = call(
        ProgramAccounts(address(), None),
        "getProgramAccounts",
        &format!(r#"["{KEY}"]"#),
        &keyed(r#""Ldp""#),
    )
    .map_err(|_| "default program encoding failed")?;
    require(
        matches!(plain, ProgramAccountsResponse::Accounts(value) if value.len() == 1),
        "program shape changed",
    )
}

fn rent_and_hash() -> Result<(), &'static str> {
    let rent = call(
        MinimumBalanceForRentExemption(
            u64::MAX,
            Some(CommitmentConfig {
                commitment: Some(Commitment::Confirmed),
            }),
        ),
        "getMinimumBalanceForRentExemption",
        &format!(r#"[{MAX},{{"commitment":"confirmed"}}]"#),
        MAX,
    )
    .map_err(|_| "rent failed")?;
    let valid = call(
        BlockhashValid(
            Hash::from_bytes([0; 32]),
            Some(ReadConfig {
                commitment: Some(Commitment::Finalized),
                min_context_slot: Some(u64::MAX),
            }),
        ),
        "isBlockhashValid",
        &format!(r#"["{KEY}",{{"commitment":"finalized","minContextSlot":{MAX}}}]"#),
        &context("false"),
    )
    .map_err(|_| "blockhash validity failed")?;
    require(
        rent == u64::MAX && !valid.value && valid.context.slot == u64::MAX,
        "rent/hash values changed",
    )
}
