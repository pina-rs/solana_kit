//! Complete Dart records coexist with the provider's intentionally partial wire records.

use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use solana_kit_helius::Field;
use solana_kit_helius::das::{
    CompleteNftEdition, CompleteTokenAccount, CompleteTokenAccountList, IncompleteDasRecord,
    NftEdition, TokenAccount, TokenAccountList,
};

mod pages;

const EDITION: &str =
    r#"{"mint":"edition","edition":18446744073709551615,"edition_address":null,"burnt":null}"#;
const TOKEN: &str = concat!(
    r#"{"address":"account","mint":"mint","owner":"owner","amount":18446744073709551615,"#,
    r#""delegated_amount":null,"frozen":false,"burnt":null,"token_extensions":null}"#,
);
const PAGE: &str = concat!(
    r#"{"total":0,"limit":0,"token_accounts":[],"#,
    r#""last_indexed_slot":null,"page":null,"cursor":null}"#,
);

/// The complete API enforces required fields without rejecting valid partial provider results.
pub(super) fn fields() {
    contract::<NftEdition, CompleteNftEdition>(EDITION, &["mint", "edition"]);
    contract::<TokenAccount, CompleteTokenAccount>(TOKEN, &["address", "mint", "owner", "amount"]);
    contract::<TokenAccountList, CompleteTokenAccountList>(
        PAGE,
        &["total", "limit", "token_accounts"],
    );
    numeric_fields();
    metadata();
    pages::conversion();
}

fn contract<Wire, Complete>(literal: &str, required: &[&'static str])
where
    Wire: DeserializeOwned,
    Complete: DeserializeOwned + Serialize + TryFrom<Wire, Error = IncompleteDasRecord>,
{
    let source: Value = serde_json::from_str(literal).expect("independent literal record");
    let wire: Wire = serde_json::from_str(literal).expect("complete wire record");
    let converted = Complete::try_from(wire).expect("all required values present");
    let direct: Complete = serde_json::from_str(literal).expect("complete record JSON");

    assert_eq!(
        serde_json::to_value(converted).expect("converted record"),
        source
    );
    assert_eq!(serde_json::to_value(direct).expect("direct record"), source);

    serde_json::from_str::<Wire>("{}").expect("provider permits partial records");
    assert!(serde_json::from_str::<Complete>("[]").is_err());

    for &name in required {
        let mut missing = source.clone();
        missing
            .as_object_mut()
            .expect("literal object")
            .remove(name);
        reject::<Wire, Complete>(
            missing,
            IncompleteDasRecord::Missing {
                field: name,
                token_account_index: None,
            },
        );

        let mut null = source.clone();
        null[name] = Value::Null;
        reject::<Wire, Complete>(
            null,
            IncompleteDasRecord::Null {
                field: name,
                token_account_index: None,
            },
        );
    }
}

fn reject<Wire, Complete>(value: Value, expected: IncompleteDasRecord)
where
    Wire: DeserializeOwned,
    Complete: DeserializeOwned + TryFrom<Wire, Error = IncompleteDasRecord>,
{
    assert!(serde_json::from_value::<Complete>(value.clone()).is_err());
    let wire: Wire = serde_json::from_value(value).expect("partial provider record remains valid");
    assert_eq!(Complete::try_from(wire).err(), Some(expected));
}

fn numeric_fields() {
    let invalid_integer = &["-1", "1.5", "18446744073709551616", "\"0\""];
    malformed::<CompleteNftEdition>(EDITION, "edition", invalid_integer);
    malformed::<CompleteTokenAccount>(TOKEN, "amount", invalid_integer);

    for field in ["total", "limit"] {
        malformed::<CompleteTokenAccountList>(PAGE, field, invalid_integer);
    }

    let invalid_list = &["false", "0", "{}", "[[]]"];
    let invalid_identity = &["false", "0", "[]", "{}"];
    malformed::<CompleteTokenAccountList>(PAGE, "token_accounts", invalid_list);
    malformed::<CompleteTokenAccount>(TOKEN, "owner", invalid_identity);

    assert!(serde_json::from_str::<CompleteNftEdition>(r#"["edition",0]"#).is_err());
    assert!(serde_json::from_str::<CompleteTokenAccount>(r#"["a","m","o",0]"#).is_err());
    assert!(serde_json::from_str::<CompleteTokenAccountList>("[0,0,[]]").is_err());
}

fn malformed<Model: DeserializeOwned>(literal: &str, name: &str, invalid: &[&str]) {
    for invalid in invalid {
        let mut value: Value = serde_json::from_str(literal).expect("valid literal record");
        value[name] = serde_json::from_str(invalid).expect("literal invalid field JSON");
        assert!(
            serde_json::from_value::<Model>(value).is_err(),
            "accepted {name}: {invalid}"
        );
    }
}

fn metadata() {
    let extensions: Value = serde_json::from_str(
        r#"{"future":{"values":[null,false,18446744073709551615],"rate":9007199254740993.125}}"#,
    )
    .expect("opaque provider extension");
    let mut wire: TokenAccount = serde_json::from_str(TOKEN).expect("token literal");
    wire.token_extensions = Field::Value(extensions.clone());
    let token = CompleteTokenAccount::try_from(wire).expect("complete token");

    assert_eq!(token.amount, u64::MAX);
    assert_eq!(token.frozen, Field::Value(false));
    assert_eq!(token.delegated_amount, Field::Null);
    assert_eq!(token.token_extensions, Field::Value(extensions));

    let edition: NftEdition = serde_json::from_str(r#"{"mint":"edition","edition":0}"#)
        .expect("optional edition metadata may be omitted");
    let edition = CompleteNftEdition::try_from(edition).expect("zero is a present edition");
    assert_eq!(edition.edition, 0);
    assert_eq!(edition.edition_address, Field::Missing);
    assert_eq!(edition.burnt, Field::Missing);
}
