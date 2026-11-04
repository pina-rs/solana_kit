#![allow(dead_code)]
//! Shared literal helpers for the typed parsed-account contracts.
//!
//! Every fixture is an independently authored spelling of the node shape; the
//! field names and example values follow the pinned Dart parsed-type model.

use solana_kit_client::{
    Error, Limits,
    accounts::{assert_account_decoded, parse_json_rpc_account},
    methods::accounts::AccountData,
    parsed::ParsedAccountData,
};
use solana_kit_core::{Address, accounts::Account};

/// The account address used by every standalone fixture.
pub const ADDRESS: Address = Address::from_bytes([7; 32]);

/// The owner field of a standalone fixture; ownership is never authenticated.
pub const OWNER: &str = "11111111111111111111111111111111";

/// Room for every literal in these files with the default nesting budget.
pub const LIMITS: Limits = Limits {
    bytes: 8192,
    depth: 32,
};

/// Wraps a `data.parsed` literal in the complete standalone account shape.
pub fn account_json(program: &str, parsed: &str) -> String {
    format!(
        r#"{{"executable":false,"lamports":1,"owner":"{OWNER}","rentEpoch":0,"space":165,"data":{{"parsed":{parsed},"program":"{program}","space":165}}}}"#
    )
}

/// Parses one standalone `jsonParsed` account without an RPC envelope.
pub fn parsed_account(program: &str, parsed: &str) -> Account<AccountData> {
    parse_json_rpc_account(ADDRESS, &account_json(program, parsed))
        .unwrap()
        .require()
        .unwrap()
}

/// Projects one standalone literal through the public typed entry point.
pub fn decode(program: &str, parsed: &str) -> Result<ParsedAccountData, Error> {
    decode_with(program, parsed, LIMITS)
}

/// Projects one standalone literal with explicit byte and nesting budgets.
pub fn decode_with(
    program: &str,
    parsed: &str,
    limits: Limits,
) -> Result<ParsedAccountData, Error> {
    let account = parsed_account(program, parsed);

    assert_account_decoded(&account)
        .unwrap()
        .decode_typed(limits)
}

/// Reads the exact node payload back out of a parsed account.
pub fn exact(account: &Account<AccountData>) -> &str {
    match &account.data {
        AccountData::Parsed(parsed) => parsed.parsed.get(),
        _ => panic!("fixture is not parsed data"),
    }
}
