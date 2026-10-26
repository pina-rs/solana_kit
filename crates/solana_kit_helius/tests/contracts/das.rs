//! DAS requests and results follow independently pinned provider wire schemas.

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::value::RawValue;
use solana_kit_client::methods::Method;
use solana_kit_helius::das::*;

#[derive(Deserialize)]
struct Fixture {
    method: String,
    params: Box<RawValue>,
    result: Box<RawValue>,
}

fn contract<Operation: Method + DeserializeOwned>(fixture: &Fixture)
where
    Operation::Response: Serialize,
{
    super::rpc::vector::<Operation>(&fixture.method, fixture.params.get(), fixture.result.get());
}

type WireContract = (&'static str, fn(&Fixture));

const CONTRACTS: &[WireContract] = &[
    ("getAsset", contract::<GetAssetRequest>),
    ("getAssetBatch", contract::<GetAssetBatchRequest>),
    ("getAssetProof", contract::<GetAssetProofRequest>),
    ("getAssetProofBatch", contract::<GetAssetProofBatchRequest>),
    (
        "getAssetsByAuthority",
        contract::<GetAssetsByAuthorityRequest>,
    ),
    ("getAssetsByCreator", contract::<GetAssetsByCreatorRequest>),
    ("getAssetsByGroup", contract::<GetAssetsByGroupRequest>),
    ("getAssetsByOwner", contract::<GetAssetsByOwnerRequest>),
    ("getNftEditions", contract::<GetNftEditionsRequest>),
    (
        "getSignaturesForAsset",
        contract::<GetSignaturesForAssetRequest>,
    ),
    ("getTokenAccounts", contract::<GetTokenAccountsRequest>),
    ("searchAssets", contract::<SearchAssetsRequest>),
];

/// Every DAS method preserves independent requests and response metadata without rounding.
pub fn vectors() {
    let fixtures: Vec<Fixture> = serde_json::from_str(include_str!("../fixtures/das-methods.json"))
        .expect("literal DAS contracts");
    assert_eq!(fixtures.len(), 12);
    assert_eq!(CONTRACTS.len(), fixtures.len());

    for ((name, execute), fixture) in CONTRACTS.iter().zip(&fixtures) {
        assert_eq!(*name, fixture.method);
        execute(fixture);
    }

    malformed_parameters();
    malformed_results();
}

fn reject<Operation: DeserializeOwned>(inputs: &[&str]) {
    for input in inputs {
        assert!(
            serde_json::from_str::<Operation>(input).is_err(),
            "accepted {input}"
        );
    }
}

fn malformed_parameters() {
    reject::<GetAssetRequest>(&[
        r#"{"id":"a","displayOptions":false}"#,
        r#"{"id":"a","options":false}"#,
        r#"{"id":"a","options":{"showFungible":1}}"#,
        r#"{"id":"a","options":{"showFungibe":true}}"#,
    ]);
    reject::<GetAssetsByOwnerRequest>(&[
        r#"{"ownerAddress":"a","sortDirection":"asc"}"#,
        r#"{"ownerAddress":"a","sortBy":"recent_action"}"#,
        r#"{"ownerAddress":"a","sortBy":{"sortBy":"id"}}"#,
        r#"{"ownerAddress":"a","sortBy":{"sortBy":"id","sortDirection":"up"}}"#,
    ]);
    reject::<SearchAssetsRequest>(&[
        r#"{"grouping":"collection"}"#,
        r#"{"grouping":["collection"]}"#,
        r#"{"grouping":["collection","a","b"]}"#,
        r#"{"ownerType":"other"}"#,
        r#"{"royaltyAmount":-1}"#,
    ]);
    reject::<GetTokenAccountsRequest>(&[
        r#"{"cursor":1}"#,
        r#"{"options":{"showZeroBalance":"false"}}"#,
    ]);
}

fn malformed_results() {
    let editions: GetNftEditionsRequest =
        serde_json::from_str(r#"{"mint":"m"}"#).expect("edition request");

    for result in [
        "[]",
        r#"{"editions":{}}"#,
        r#"{"supply":-1}"#,
        r#"{"editions":[{"edition":1.5}]}"#,
    ] {
        assert!(editions.decode(result).is_err(), "accepted {result}");
    }

    let tokens: GetTokenAccountsRequest =
        serde_json::from_str("{}").expect("unfiltered token request");

    for result in [
        "[]",
        r#"{"token_accounts":false}"#,
        r#"{"token_accounts":[{"amount":18446744073709551616}]}"#,
        r#"{"token_accounts":[{"amount":-1}]}"#,
        r#"{"last_indexed_slot":"1"}"#,
    ] {
        assert!(tokens.decode(result).is_err(), "accepted {result}");
    }

    super::rpc::vector::<GetNftEditionsRequest>("getNftEditions", r#"{"mint":"m"}"#, "{}");
    super::rpc::vector::<GetTokenAccountsRequest>("getTokenAccounts", "{}", "{}");
}
