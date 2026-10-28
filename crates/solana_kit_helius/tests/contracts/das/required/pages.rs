//! Real provider Method results can be explicitly promoted to complete pages.

use serde_json::Value;
use solana_kit_client::methods::Method;
use solana_kit_helius::Field;
use solana_kit_helius::das::{
    CompleteTokenAccountList, GetTokenAccountsRequest, IncompleteDasRecord,
};

pub(super) fn conversion() {
    let request: GetTokenAccountsRequest =
        serde_json::from_str(r#"{"owner":"wallet"}"#).expect("real typed provider method");
    let literal = concat!(
        r#"{"total":9007199254740993,"limit":18446744073709551615,"last_indexed_slot":null,"#,
        r#""token_accounts":[{"address":"a","mint":"m","owner":"o","amount":0}]}"#,
    );
    let page = request.decode(literal).expect("provider response");
    let page = CompleteTokenAccountList::try_from(page).expect("complete provider page");

    assert_eq!(page.total, 9_007_199_254_740_993);
    assert_eq!(page.limit, u64::MAX);
    assert_eq!(page.last_indexed_slot, Field::Null);
    assert_eq!(page.cursor, Field::Missing);
    assert_eq!(page.token_accounts.len(), 1);
    assert_eq!(page.token_accounts[0].amount, 0);

    let mut value: Value = serde_json::from_str(literal).expect("independent complete page");
    let mut account = value["token_accounts"][0].clone();
    account
        .as_object_mut()
        .expect("token object")
        .remove("owner");
    value["token_accounts"]
        .as_array_mut()
        .expect("account list")
        .push(account);
    incomplete(&request, &value, false);
    value["token_accounts"][1]["owner"] = Value::Null;
    incomplete(&request, &value, true);
}

fn incomplete(request: &GetTokenAccountsRequest, value: &Value, explicit_null: bool) {
    let response = serde_json::to_string(value).expect("literal partial page");
    let wire = request
        .decode(&response)
        .expect("provider permits partial accounts");
    assert!(serde_json::from_str::<CompleteTokenAccountList>(&response).is_err());
    let error = CompleteTokenAccountList::try_from(wire).expect_err("reject entire partial page");

    let expected = if explicit_null {
        IncompleteDasRecord::Null {
            field: "owner",
            token_account_index: Some(1),
        }
    } else {
        IncompleteDasRecord::Missing {
            field: "owner",
            token_account_index: Some(1),
        }
    };

    assert_eq!(error, expected);
    let presence = if explicit_null { "null" } else { "missing" };
    assert_eq!(
        error.to_string(),
        format!("token account 1: {presence} required field `owner`")
    );
}
