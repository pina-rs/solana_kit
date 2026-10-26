//! V2 vectors follow the provider's positional schemas, independently of Dart fixtures.

use serde::de::DeserializeOwned;
use solana_kit_client::methods::Method;
use solana_kit_helius::{Field, rpc_v2::*};

const PAGE: &str = r#"{"accounts":[{"pubkey":"a","account":{"lamports":18446744073709551615,"owner":"p","data":["","base64"],"executable":false,"rentEpoch":0}}],"paginationKey":"c","totalResults":null}"#;
const SIGNATURES: &str = r#"{"data":[{"signature":"s","slot":1,"transactionIndex":0,"err":null,"memo":null,"blockTime":-1,"confirmationStatus":"finalized"}],"paginationToken":null}"#;
const FULL: &str = r#"{"data":[{"slot":1,"transactionIndex":0,"transaction":{"signatures":["s"]},"meta":null,"blockTime":null}],"paginationToken":"next"}"#;

fn request<M: Method + DeserializeOwned>(params: &str) -> M {
    let request: M = serde_json::from_str(params).expect("provider parameter tuple");
    assert_eq!(
        serde_json::to_value(&request).expect("typed parameters"),
        serde_json::from_str::<serde_json::Value>(params).expect("literal parameters"),
    );
    request
}

/// Program and token pages retain typed metadata, context and required nullable cursors.
pub fn account_pages() {
    let query = request::<GetProgramAccountsV2Request>(
        r#"["p",{"filters":[{"dataSize":42},{"memcmp":{"offset":1,"bytes":"x"}}],"dataSlice":{"offset":0,"length":8},"limit":1}]"#,
    );
    assert_eq!(GetProgramAccountsV2Request::NAME, "getProgramAccountsV2");
    let plain = query.decode(PAGE).expect("provider page");
    assert_eq!(plain.page().accounts[0].account.lamports, u64::MAX);
    assert_eq!(plain.page().pagination_key.as_deref(), Some("c"));
    assert_eq!(plain.page().total_results, Field::Null);
    let contextual = format!(r#"{{"context":{{"slot":1}},"value":{PAGE}}}"#);
    assert!(matches!(
        query.decode(&contextual),
        Ok(AccountPage::WithContext(_))
    ));

    for malformed in [
        r#"{"accounts":[]}"#,
        r#"{"accounts":[],"cursor":null}"#,
        r#"{"accounts":[],"paginationKey":3}"#,
        r#"{"context":false,"accounts":[],"paginationKey":null}"#,
    ] {
        assert!(query.decode(malformed).is_err());
    }

    token_arguments();
    positional_failures();
}

fn token_arguments() {
    let query = request::<GetTokenAccountsByOwnerV2Request>(
        r#"["o",{"programId":"p"},{"limit":2,"paginationKey":"c"}]"#,
    );
    assert_eq!(
        GetTokenAccountsByOwnerV2Request::NAME,
        "getTokenAccountsByOwnerV2"
    );
    assert_eq!(
        query
            .decode(PAGE)
            .expect("token page")
            .page()
            .accounts
            .len(),
        1
    );
    let query = GetTokenAccountsByOwnerV2Request {
        owner_address: "o".into(),
        filter: None,
        config: Some(GetTokenAccountsByOwnerV2Config {
            limit: Some(1),
            ..Default::default()
        }),
    };
    assert_eq!(
        serde_json::to_string(&query).expect("configuration position"),
        r#"["o",{},{"limit":1}]"#
    );
    request::<GetTokenAccountsByOwnerV2Request>(r#"["o"]"#);
}

fn positional_failures() {
    request::<GetProgramAccountsV2Request>(r#"["p"]"#);

    for malformed in [r#"{"programAddress":"p"}"#, "[]", r#"["p",{},{}]"#] {
        assert!(serde_json::from_str::<GetProgramAccountsV2Request>(malformed).is_err());
    }
}

/// History mode selects signature or full records and validates the whole page cursor.
pub fn transaction_history() {
    let signatures =
        request::<GetTransactionsForAddressRequest>(r#"["a",{"commitment":"finalized"}]"#);
    assert_eq!(
        GetTransactionsForAddressRequest::NAME,
        "getTransactionsForAddress"
    );
    assert!(matches!(
        signatures.decode(SIGNATURES).expect("signature page").data[0],
        TransactionForAddress::Signature(_)
    ));
    let full = request::<GetTransactionsForAddressRequest>(
        r#"["a",{"transactionDetails":"full","maxSupportedTransactionVersion":0,"filters":{"status":"failed","tokenAccounts":"balanceChanged","slot":{"gte":0},"blockTime":{"gt":-1},"signature":{"lt":"s"}}}]"#,
    );
    let page = full.decode(FULL).expect("full page");
    assert_eq!(page.pagination_token.as_deref(), Some("next"));
    assert!(matches!(page.data[0], TransactionForAddress::Full(_)));
    assert!(signatures.decode(FULL).is_err());
    assert!(full.decode(SIGNATURES).is_err());
    assert!(full.decode(r#"{"data":[]}"#).is_err());
    assert!(
        full.decode(r#"{"data":[],"paginationToken":false}"#)
            .is_err()
    );
    binary_history(&full);
}

fn binary_history(json_query: &GetTransactionsForAddressRequest) {
    let full = request::<GetTransactionsForAddressRequest>(
        r#"["a",{"transactionDetails":"full","encoding":"base64"}]"#,
    );
    let encoded = FULL.replace(r#"{"signatures":["s"]}"#, r#"["AA==","base64"]"#);
    assert_ne!(
        encoded, FULL,
        "binary fixture mutation must change actual bytes"
    );
    let page = full.decode(&encoded).expect("binary full history");
    assert!(matches!(&page.data[0], TransactionForAddress::Full(record)
        if matches!(&record.transaction, HistoryTransactionPayload::Binary((text,
            BinaryTransactionEncoding::Base64)) if text == "AA==")));
    assert!(full.decode(FULL).is_err());
    assert!(json_query.decode(&encoded).is_err());
}
