//! Nullable build fields and lookup map ordering match the pinned reference.

use crate::require;
use solana_kit_client::rest::Budgets;
use solana_kit_jupiter::{BuildResponse, decode_build};

pub(super) fn responses() -> Result<(), &'static str> {
    let body = br#"{"computeBudgetInstructions":[{}],"setupInstructions":[],"swapInstruction":{"programId":null,"accounts":[{}],"data":null},"cleanupInstruction":null,"otherInstructions":[{}],"tipInstruction":{},"addressesByLookupTableAddress":{"z-table":["third","first","third"],"a-table":[]},"blockhashWithMetadata":{"blockhash":[0,255,1],"lastValidBlockHeight":"18446744073709551616000"},"future":true}"#;
    let build = decode_build(body, Budgets::default()).map_err(|_| "build decoding failed")?;
    require(build.raw_response == body, "build raw response changed")?;
    lookup_order(&build)?;
    instructions(&build)?;
    blockhash(&build)?;
    fallback_optionality()
}

fn lookup_order(build: &BuildResponse) -> Result<(), &'static str> {
    require(
        build.address_lookup_table_addresses.as_deref()
            == Some(&["z-table".into(), "a-table".into()]),
        "lookup fallback sorted JSON map keys",
    )?;
    let tables = build
        .addresses_by_lookup_table_address
        .as_ref()
        .ok_or("lookup map disappeared")?;
    require(
        tables.entries[0].addresses == ["third", "first", "third"]
            && tables.entries[1].addresses.is_empty(),
        "lookup addresses changed index order",
    )
}

fn instructions(build: &BuildResponse) -> Result<(), &'static str> {
    require(
        build
            .compute_budget_instructions
            .as_ref()
            .map(|items| items.len())
            == Some(1)
            && build.setup_instructions.as_ref().map(|items| items.len()) == Some(0),
        "missing and empty instructions were conflated",
    )?;
    require(
        build
            .swap_instruction
            .as_ref()
            .and_then(|value| value.accounts.as_ref())
            .map(|items| items.len())
            == Some(1)
            && build.cleanup_instruction.is_none(),
        "raw nullable instruction fields changed",
    )?;
    require(
        build.other_instructions.as_ref().map(|items| items.len()) == Some(1)
            && build.tip_instruction.is_some(),
        "other or tip instruction omitted",
    )
}

fn blockhash(build: &BuildResponse) -> Result<(), &'static str> {
    let metadata = build
        .blockhash_with_metadata
        .as_ref()
        .ok_or("blockhash metadata lost")?;
    require(
        metadata.blockhash == [0, 255, 1]
            && metadata.last_valid_block_height.as_str() == "18446744073709551616000",
        "raw blockhash or expiry changed",
    )
}

fn fallback_optionality() -> Result<(), &'static str> {
    let absent = decode_build(b"{}", Budgets::default()).map_err(|_| "empty build rejected")?;
    require(
        absent.address_lookup_table_addresses.is_none()
            && absent.addresses_by_lookup_table_address.is_none()
            && absent.swap_instruction.is_none(),
        "missing build fields changed",
    )?;

    for body in [
        br#"{"addressLookupTableAddresses":null,"addressesByLookupTableAddress":{}}"#.as_slice(),
        br#"{"addressLookupTableAddresses":[],"addressesByLookupTableAddress":{"ignored":[]}}"#
            .as_slice(),
    ] {
        let response =
            decode_build(body, Budgets::default()).map_err(|_| "empty lookup fallback failed")?;
        require(
            response
                .address_lookup_table_addresses
                .as_ref()
                .is_some_and(|addresses| addresses.is_empty()),
            "explicit empty list or empty map fallback changed",
        )?;
    }

    let body = br#"{"addressLookupTableAddresses":["explicit"],"addressesByLookupTableAddress":{"other":[]}}"#;
    let explicit =
        decode_build(body, Budgets::default()).map_err(|_| "explicit lookup list rejected")?;
    require(
        explicit.address_lookup_table_addresses.as_deref() == Some(&["explicit".into()]),
        "lookup map replaced explicit list",
    )
}
