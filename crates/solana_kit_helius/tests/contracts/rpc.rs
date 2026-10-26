//! Literal vectors call real provider Method implementations, not a shadow SDK.

use serde::{Serialize, de::DeserializeOwned};
use solana_kit_client::methods::Method;
use solana_kit_helius::{das::*, priority_fee::*, rpc_v2::*, zk::*};

pub(super) fn vector<M: Method + DeserializeOwned>(name: &str, params: &str, response: &str)
where
    M::Response: Serialize,
{
    let method: M = serde_json::from_str(params).expect("literal request schema");
    let expected: serde_json::Value = serde_json::from_str(params).expect("literal request JSON");
    assert_eq!(M::NAME, name);
    assert_eq!(
        serde_json::to_value(&method).expect("request serialization"),
        expected
    );
    let result = method.decode(response).expect("literal response schema");
    let expected: serde_json::Value =
        serde_json::from_str(response).expect("literal response JSON");
    assert_eq!(
        serde_json::to_value(result).expect("response serialization"),
        expected
    );
}

/// Priority fees use object parameters; enhanced V2 operations use provider parameter tuples.
pub fn enhanced_rpc_vectors() {
    serde_json::from_str::<PriorityFeeOptions>(r#"{"lookbackSlots":1}"#)
        .expect("provider priority-fee lookback is a slot count");
    serde_json::from_str::<GetProgramAccountsV2Request>(r#"["p",{}]"#)
        .expect("provider V2 program-account parameters are positional");
    vector::<GetPriorityFeeEstimateRequest>(
        "getPriorityFeeEstimate",
        r#"{"accountKeys":["a"],"options":{"priorityLevel":"VeryHigh","lookbackSlots":1,"transactionEncoding":"base64","evaluateEmptySlotAsZero":true}}"#,
        r#"{"priorityFeeEstimate":9007199254740993.125,"priorityFeeLevels":{"min":0,"unsafeMax":null}}"#,
    );

    for invalid in ["0", "151", "true", "-1", "1.5", "\"1\""] {
        let json = format!(r#"{{"lookbackSlots":{invalid}}}"#);
        assert!(serde_json::from_str::<PriorityFeeOptions>(&json).is_err());
    }

    let maximum: PriorityFeeOptions = serde_json::from_str(r#"{"lookbackSlots":150}"#)
        .expect("provider accepts its maximum slot lookback");
    assert_eq!(u8::from(maximum.lookback_slots.expect("lookback")), 150);

    super::v2::account_pages();
    super::v2::transaction_history();
    let request = GetTransfersByAddressRequest {
        address: "a".into(),
        config: None,
    };
    assert_eq!(
        serde_json::to_string(&request).expect("positional transfer query"),
        r#"["a"]"#
    );
    assert_eq!(
        <GetTransfersByAddressRequest as Method>::NAME,
        "getTransfersByAddress"
    );
    assert!(
        request
            .decode(r#"{"data":[],"paginationToken":null}"#)
            .is_ok()
    );
}

/// Known malformed fields never become missing, zero, an empty list or success.
pub fn known_fields_reject_malformed_values() {
    let request = GetAssetRequest {
        id: "a".into(),
        options: None,
    };

    for malformed in [
        r#"{}"#,
        r#"{"id":null}"#,
        r#"{"id":1}"#,
        r#"{"id":"a","burnt":1}"#,
        r#"{"id":"a","content":{"files":false}}"#,
        r#"{"id":"a","token_info":{"supply":-1}}"#,
        r#"{"id":"a","id":"b"}"#,
    ] {
        assert!(request.decode(malformed).is_err(), "accepted {malformed}");
    }

    assert!(
        serde_json::from_str::<GetAssetsByOwnerRequest>(
            r#"{"ownerAddress":"a","sortBy":"unknown"}"#
        )
        .is_err()
    );
    let balance = GetCompressedBalanceRequest(AccountLocator::Hash("h".into()));
    assert!(
        balance
            .decode(r#"{"context":{"slot":1},"value":18446744073709551616}"#)
            .is_err()
    );
    assert!(
        balance
            .decode(r#"{"context":{"slot":1},"value":1.5}"#)
            .is_err()
    );
}
