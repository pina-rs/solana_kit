//! Rich literal fixtures validate nested known fields and exact numeric data.

use solana_kit_client::methods::Method;
use solana_kit_helius::{Field, das::GetAssetRequest, rpc_v2::GetTransfersByAddressRequest};

/// Every nested DAS model retains its pinned field names and explicit nulls.
pub fn complete_asset_model() {
    let json = include_str!("../fixtures/asset.json");
    let request = GetAssetRequest {
        id: "asset".into(),
        options: None,
    };
    let asset = request.decode(json).expect("full literal asset");
    assert_eq!(
        asset.compression.value().expect("compression").seq,
        Field::Value(u64::MAX)
    );
    assert_eq!(
        asset.token_info.value().expect("token info").supply,
        Field::Value(u64::MAX)
    );
    assert_eq!(
        serde_json::to_value(&asset).expect("typed serialization"),
        serde_json::from_str::<serde_json::Value>(json).expect("literal JSON")
    );

    for (old, malformed) in [
        ("\"eligible\": true", "\"eligible\": 1"),
        ("\"share\": 100", "\"share\": -1"),
        (
            "\"price_per_token\": 9007199254740993.125",
            "\"price_per_token\": \"1\"",
        ),
        ("\"files\": [", "\"files\": false, \"unknown\": ["),
    ] {
        let invalid = json.replace(old, malformed);
        assert_ne!(invalid, json, "literal mutation must change the fixture");
        assert!(request.decode(&invalid).is_err());
    }

    object_fields(&request, json);
}

fn object_fields(request: &GetAssetRequest, json: &str) {
    let source: serde_json::Value = serde_json::from_str(json).expect("asset fixture");

    for name in ["token_info", "compression", "content", "ownership"] {
        let mut invalid = source.clone();
        invalid[name] = serde_json::json!([]);
        assert!(
            request.decode(&invalid.to_string()).is_err(),
            "nested {name} object must reject a positional array"
        );
    }
}

/// Transfer amounts and display quantities remain exact strings; negative inner indices are valid.
pub fn transfer_amounts_and_nulls() {
    let request = GetTransfersByAddressRequest {
        address: "a".into(),
        config: None,
    };
    let json = r#"{"data":[{"signature":"s","slot":18446744073709551615,"blockTime":-1,"type":"token","fromUserAccount":null,"toUserAccount":"b","fromTokenAccount":"ta","toTokenAccount":null,"mint":"m","amount":"1844674407370955161500000000000","feeAmount":"1","feeAccount":"f","decimals":9,"uiAmount":"1844674407370955161500.000000000","feeUiAmount":"0.000000001","confirmationStatus":"future-status","transactionIdx":0,"instructionIdx":1,"innerInstructionIdx":-1}],"paginationToken":null}"#;
    let page = request.decode(json).expect("literal transfer");
    assert_eq!(page.data[0].from_user_account, Field::Null);
    assert_eq!(page.data[0].inner_instruction_idx, -1);
    assert_eq!(
        serde_json::to_value(&page).expect("typed serialization"),
        serde_json::from_str::<serde_json::Value>(json).expect("literal JSON")
    );
}
