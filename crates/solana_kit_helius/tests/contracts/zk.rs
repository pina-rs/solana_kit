//! Compression contracts use the Photon server schema, including SDK schema disagreements.

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, value::RawValue};
use solana_kit_client::methods::{Contextual, Method};
use solana_kit_helius::zk::*;

#[derive(Deserialize)]
struct Fixture {
    method: String,
    params: Box<RawValue>,
    result: Box<RawValue>,
}

trait ContextPayload {
    fn slot(&self) -> u64;
    fn value_json(&self) -> Value;
}

impl<Payload: Serialize> ContextPayload for Contextual<Payload> {
    fn slot(&self) -> u64 {
        self.context.slot
    }

    fn value_json(&self) -> Value {
        serde_json::to_value(&self.value).expect("typed provider payload")
    }
}

impl ContextPayload for GetCompressedAccountResponse {
    fn slot(&self) -> u64 {
        self.context.slot
    }

    fn value_json(&self) -> Value {
        serde_json::to_value(&self.value).expect("nullable account payload")
    }
}

fn parameters<Operation: Method + DeserializeOwned>(fixture: &Fixture) -> Operation {
    assert_eq!(Operation::NAME, fixture.method);
    let operation: Operation = serde_json::from_str(fixture.params.get()).expect("literal params");
    assert_eq!(
        serde_json::to_value(&operation).expect("typed params"),
        serde_json::from_str::<Value>(fixture.params.get()).expect("independent params"),
    );
    operation
}

fn contextual<Operation: Method + DeserializeOwned>(fixture: &Fixture)
where
    Operation::Response: ContextPayload,
{
    let operation = parameters::<Operation>(fixture);
    let response = operation
        .decode(fixture.result.get())
        .expect("provider response");
    let expected: Value = serde_json::from_str(fixture.result.get()).expect("literal response");
    assert_eq!(
        response.slot(),
        expected["context"]["slot"].as_u64().expect("slot")
    );
    assert_eq!(response.value_json(), expected["value"]);
}

fn direct<Operation: Method + DeserializeOwned>(fixture: &Fixture)
where
    Operation::Response: Serialize,
{
    let operation = parameters::<Operation>(fixture);
    let response = operation
        .decode(fixture.result.get())
        .expect("direct provider response");
    assert_eq!(
        serde_json::to_value(response).expect("typed response"),
        serde_json::from_str::<Value>(fixture.result.get()).expect("independent response"),
    );
}

type WireContract = (&'static str, fn(&Fixture));

const CONTRACTS: &[WireContract] = &[
    (
        "getCompressedAccount",
        contextual::<GetCompressedAccountRequest>,
    ),
    (
        "getCompressedAccountProof",
        contextual::<GetCompressedAccountProofRequest>,
    ),
    (
        "getCompressedAccountsByOwner",
        contextual::<GetCompressedAccountsByOwnerRequest>,
    ),
    (
        "getCompressedBalance",
        contextual::<GetCompressedBalanceRequest>,
    ),
    (
        "getCompressedBalanceByOwner",
        contextual::<GetCompressedBalanceByOwnerRequest>,
    ),
    (
        "getCompressedMintTokenHolders",
        contextual::<GetCompressedMintTokenHoldersRequest>,
    ),
    (
        "getCompressedTokenAccountBalance",
        contextual::<GetCompressedTokenAccountBalanceRequest>,
    ),
    (
        "getCompressedTokenAccountsByDelegate",
        contextual::<GetCompressedTokenAccountsByDelegateRequest>,
    ),
    (
        "getCompressedTokenAccountsByOwner",
        contextual::<GetCompressedTokenAccountsByOwnerRequest>,
    ),
    (
        "getCompressedTokenBalancesByOwner",
        contextual::<GetCompressedTokenBalancesByOwnerRequest>,
    ),
    (
        "getCompressedTokenBalancesByOwnerV2",
        contextual::<GetCompressedTokenBalancesByOwnerV2Request>,
    ),
    (
        "getCompressionSignaturesForAccount",
        contextual::<GetCompressionSignaturesForAccountRequest>,
    ),
    (
        "getCompressionSignaturesForAddress",
        contextual::<GetCompressionSignaturesForAddressRequest>,
    ),
    (
        "getCompressionSignaturesForOwner",
        contextual::<GetCompressionSignaturesForOwnerRequest>,
    ),
    (
        "getCompressionSignaturesForTokenOwner",
        contextual::<GetCompressionSignaturesForTokenOwnerRequest>,
    ),
    (
        "getLatestCompressionSignatures",
        contextual::<GetLatestCompressionSignaturesRequest>,
    ),
    (
        "getLatestNonVotingSignatures",
        contextual::<GetLatestNonVotingSignaturesRequest>,
    ),
    (
        "getMultipleCompressedAccountProofs",
        contextual::<GetMultipleCompressedAccountProofsRequest>,
    ),
    (
        "getMultipleCompressedAccounts",
        contextual::<GetMultipleCompressedAccountsRequest>,
    ),
    (
        "getMultipleNewAddressProofs",
        contextual::<GetMultipleNewAddressProofsRequest>,
    ),
    (
        "getMultipleNewAddressProofsV2",
        contextual::<GetMultipleNewAddressProofsV2Request>,
    ),
    (
        "getSignaturesForAsset",
        direct::<GetZkSignaturesForAssetRequest>,
    ),
    (
        "getTransactionWithCompressionInfo",
        direct::<GetTransactionWithCompressionInfoRequest>,
    ),
    ("getValidityProof", contextual::<GetValidityProofRequest>),
    ("getIndexerHealth", direct::<GetIndexerHealth>),
    ("getIndexerSlot", direct::<GetIndexerSlot>),
];

/// Every declared compression operation has an independent request and result fixture.
pub fn vectors() {
    let fixtures: Vec<Fixture> = serde_json::from_str(include_str!("../fixtures/zk-methods.json"))
        .expect("reviewed Photon wire fixtures");
    assert_eq!(fixtures.len(), CONTRACTS.len());

    for ((name, contract), fixture) in CONTRACTS.iter().zip(&fixtures) {
        assert_eq!(*name, fixture.method, "fixture order and method coverage");
        contract(fixture);
    }

    account_lookup_presence();
    locator_selection();
    malformed_proofs(&fixtures);
    result_associations(&fixtures);
}

fn account_lookup_presence() {
    let request = GetCompressedAccountRequest(AccountLocator::Hash("h".into()));
    let response = request
        .decode(r#"{"context":{"slot":1},"value":null}"#)
        .expect("an absent compressed account is an explicit nullable value");
    assert_eq!(response.context.slot, 1);
    assert!(response.value.is_none());

    for malformed in [
        r#"{"context":{"slot":1}}"#,
        r#"{"value":null}"#,
        r#"{"context":{"slot":null},"value":null}"#,
    ] {
        assert!(request.decode(malformed).is_err());
    }
}

fn locator_selection() {
    for invalid in [
        "{}",
        r#"{"hash":"h","address":"a"}"#,
        r#"{"hash":null,"address":null}"#,
        r#"{"hash":"h","addres":"a"}"#,
    ] {
        assert!(serde_json::from_str::<GetCompressedAccountRequest>(invalid).is_err());
    }

    let hash: GetCompressedAccountRequest =
        serde_json::from_str(r#"{"address":null,"hash":"h"}"#).expect("nullable alternative");
    assert_eq!(
        hash,
        GetCompressedAccountRequest(AccountLocator::Hash("h".into()))
    );

    for invalid in ["{}", r#"{"addresses":["a"],"hashes":["h"]}"#] {
        assert!(serde_json::from_str::<GetMultipleCompressedAccountsRequest>(invalid).is_err());
    }
}

fn malformed_proofs(fixtures: &[Fixture]) {
    let fixture = fixtures
        .iter()
        .find(|fixture| fixture.method == "getValidityProof")
        .expect("required proof fixture");
    let request = parameters::<GetValidityProofRequest>(fixture);
    let valid: Value = serde_json::from_str(fixture.result.get()).expect("literal proof");

    for invalid in [
        serde_json::json!("AA=="),
        serde_json::json!([256]),
        serde_json::json!([-1]),
        serde_json::json!([1.5]),
    ] {
        let mut malformed = valid.clone();
        malformed["value"]["compressedProof"]["a"] = invalid;
        assert!(request.decode(&malformed.to_string()).is_err());
    }

    let mut malformed = valid;
    malformed["value"]["leafIndices"] = serde_json::json!([4294967296_u64]);
    assert!(request.decode(&malformed.to_string()).is_err());
}

fn fixture_result(fixtures: &[Fixture], name: &str) -> Value {
    let fixture = fixtures
        .iter()
        .find(|fixture| fixture.method == name)
        .expect("named fixture");
    serde_json::from_str(fixture.result.get()).expect("literal provider result")
}

fn result_associations(fixtures: &[Fixture]) {
    let proof = GetCompressedAccountProofRequest {
        hash: "other".into(),
    };
    assert!(
        proof
            .decode(&fixture_result(fixtures, "getCompressedAccountProof").to_string())
            .is_err()
    );
    let single = GetCompressedAccountRequest(AccountLocator::Hash("other".into()));
    assert!(
        single
            .decode(&fixture_result(fixtures, "getCompressedAccount").to_string())
            .is_err()
    );
    let batch = GetMultipleCompressedAccountsRequest::Hashes(vec!["missing".into(), "h".into()]);
    let mut accounts = fixture_result(fixtures, "getMultipleCompressedAccounts");
    accounts["value"]["items"]
        .as_array_mut()
        .expect("literal account list")
        .reverse();
    assert!(batch.decode(&accounts.to_string()).is_err());
    assert!(
        batch
            .decode(r#"{"context":{"slot":1},"value":{"items":[]}}"#)
            .is_err()
    );
    let proofs = GetMultipleCompressedAccountProofsRequest(vec!["other".into()]);
    assert!(
        proofs
            .decode(&fixture_result(fixtures, "getMultipleCompressedAccountProofs").to_string())
            .is_err()
    );
    let addresses = GetMultipleNewAddressProofsRequest(vec!["other".into()]);
    assert!(
        addresses
            .decode(&fixture_result(fixtures, "getMultipleNewAddressProofs").to_string())
            .is_err()
    );
    let pairs = GetMultipleNewAddressProofsV2Request(vec![AddressWithTree {
        address: "a".into(),
        tree: "wrong-tree".into(),
    }]);
    assert!(
        pairs
            .decode(&fixture_result(fixtures, "getMultipleNewAddressProofsV2").to_string())
            .is_err()
    );
}
