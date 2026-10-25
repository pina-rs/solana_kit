//! Address default caps, strict versions and simulations execute through Client.
extern crate std;

use crate::{
    require,
    rpc_fixture::{call, call_with_limits},
    rpc_history::{KEY, MAX, entry},
    rpc_results::binary_entry,
};
use solana_kit_client::{
    Limits, Presence,
    methods::{
        address_history::*, simulation::*, submission::WireEncoding,
        transaction_results::TransactionEncoding,
    },
};
use solana_kit_core::Address;
use std::{format, vec};

const PAGE_BYTES: usize = 400_000;

pub(crate) fn operations() -> Result<(), &'static str> {
    address_pages()?;
    strict_versions()?;
    simulation()
}

fn address_pages() -> Result<(), &'static str> {
    for (config, params) in [
        (None, format!(r#"["{KEY}"]"#)),
        (
            Some(AddressHistoryConfig::default()),
            format!(r#"["{KEY}",{{}}]"#),
        ),
        (
            Some(AddressHistoryConfig {
                limit: Some(1000),
                ..Default::default()
            }),
            format!(r#"["{KEY}",{{"limit":1000}}]"#),
        ),
    ] {
        for count in [0, 1, 100, 101, 1000, 1001] {
            address_page(config.clone(), &params, count)?;
        }
    }

    Ok(())
}

/// Every default spelling preserves pagination and the same strict signature cap.
fn address_page(
    config: Option<AddressHistoryConfig>,
    params: &str,
    count: usize,
) -> Result<(), &'static str> {
    let entries = vec![entry(); count].join(",");
    let result =
        format!(r#"{{"data":[{entries}],"paginationToken":"18446744073709551615:4294967295"}}"#);
    let value = call_with_limits(
        TransactionsForAddress(Address::from_bytes([0; 32]), config),
        "getTransactionsForAddress",
        params,
        &result,
        Limits {
            bytes: PAGE_BYTES,
            depth: 32,
        },
    );

    if count > 1000 {
        return require(value.is_err(), "default signature cap exceeded");
    }

    let page = value.map_err(|_| "default signature page failed")?;
    let AddressTransactions::Signatures(values) = page.data else {
        return Err("default selected full transactions");
    };
    require(
        values.len() == count && page.pagination_token.is_some(),
        "signature page truncated",
    )?;

    if let Some(first) = values.first() {
        require(
            first.slot == u64::MAX && first.transaction_index == Presence::Present(u32::MAX),
            "signature position narrowed",
        )?;
    }

    Ok(())
}

fn strict_versions() -> Result<(), &'static str> {
    let config = AddressHistoryConfig {
        details: Some(AddressDetails::Full(FullOptions {
            encoding: Some(TransactionEncoding::Base64),
            max_supported_transaction_version: Some(1),
        })),
        ..Default::default()
    };
    let params = format!(
        r#"["{KEY}",{{"transactionDetails":"full","encoding":"base64","maxSupportedTransactionVersion":1}}]"#
    );

    for (version, valid) in [
        ("", false),
        (r#","version":1"#, true),
        (r#","version":2"#, false),
    ] {
        let result = format!(
            r#"{{"data":[{}],"paginationToken":null}}"#,
            binary_entry(version)
        );
        let value = call(
            TransactionsForAddress(Address::from_bytes([0; 32]), Some(config.clone())),
            "getTransactionsForAddress",
            &params,
            &result,
        );
        require(value.is_ok() == valid, "address strict version changed")?;
    }

    for count in [100, 101] {
        let entries = vec![binary_entry(r#","version":1"#); count].join(",");
        let result = format!(r#"{{"data":[{entries}],"paginationToken":null}}"#);
        let value = call_with_limits(
            TransactionsForAddress(Address::from_bytes([0; 32]), Some(config.clone())),
            "getTransactionsForAddress",
            &params,
            &result,
            Limits {
                bytes: PAGE_BYTES,
                depth: 32,
            },
        );
        require(value.is_ok() == (count <= 100), "explicit full cap changed")?;
    }

    Ok(())
}

fn simulation() -> Result<(), &'static str> {
    let result = format!(
        concat!(
            r#"{{"context":{{"slot":{0}}},"value":{{"err":null,"fee":{0},"#,
            r#""accounts":[null],"loadedAddresses":null,"logs":["fixture"],"preBalances":[{0}],"#,
            r#""postBalances":[0],"preTokenBalances":null,"postTokenBalances":null,"#,
            r#""returnData":null,"replacementBlockhash":null,"unitsConsumed":{0},"loadedAccountsDataSize":4294967295}}}}"#
        ),
        MAX
    );
    let params = format!(
        r#"["AAEC",{{"encoding":"base64","sigVerify":false,"accounts":{{"addresses":["{KEY}"]}}}}]"#
    );
    let config = SimulationConfig {
        sig_verify: Some(false),
        accounts: Some(SimulationAccounts {
            addresses: vec![Address::from_bytes([0; 32])],
            encoding: None,
        }),
        ..Default::default()
    };
    let value = call(
        SimulateTransaction {
            wire: &[0, 1, 2],
            encoding: WireEncoding::Base64,
            config: config.clone(),
        },
        "simulateTransaction",
        &params,
        &result,
    )
    .map_err(|_| "simulation failed")?;
    require(
        value.context.slot == u64::MAX
            && value.value.execution.fee == Some(u64::MAX)
            && value.value.execution.units_consumed == Presence::Present(Some(u64::MAX)),
        "simulation exact values changed",
    )?;
    require(
        value
            .value
            .accounts
            .as_ref()
            .is_some_and(|values| values.len() == 1),
        "simulation accounts changed",
    )?;

    simulation_failures(&result, &params, &config)
}

/// Incoming scalar rejection and conflicting outgoing options exercise separate boundaries.
fn simulation_failures(
    result: &str,
    params: &str,
    config: &SimulationConfig,
) -> Result<(), &'static str> {
    for invalid in [
        r#"{"$serde_json::private::Number":"18446744073709551615"}"#,
        "\"1\"",
        "-1",
        "1.5",
    ] {
        let bad = result.replace(&format!(r#""fee":{MAX}"#), &format!(r#""fee":{invalid}"#));
        require(
            call(
                SimulateTransaction {
                    wire: &[0, 1, 2],
                    encoding: WireEncoding::Base64,
                    config: config.clone(),
                },
                "simulateTransaction",
                params,
                &bad,
            )
            .is_err(),
            "invalid simulation scalar accepted",
        )?;
    }
    let conflict = SimulationConfig {
        sig_verify: Some(true),
        replace_recent_blockhash: Some(true),
        ..Default::default()
    };
    require(
        call(
            SimulateTransaction {
                wire: &[],
                encoding: WireEncoding::Base58,
                config: conflict,
            },
            "simulateTransaction",
            "[]",
            "null",
        )
        .is_err(),
        "conflicting simulation options accepted",
    )
}
