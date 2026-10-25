//! Actual pinned preparation tests include independent primary discriminator literals.
use super::*;
use serde_json::json;

#[test]
fn compression_preserves_all_shapes_and_canonical_anchor_headers() {
    let prepared = compression().unwrap();
    let instructions = prepared["program"]["instructions"].as_array().unwrap();
    let expected = [
        ("initEmptyMerkleTree", "bf0b7707b46bdc6e"),
        ("replaceLeaf", "cca54c6449930080"),
        ("transferAuthority", "30a94c48e5b437a1"),
        ("verifyLeaf", "7cdc16df680afae0"),
        ("append", "957812deece158cb"),
        ("insertOrAppend", "062a32be336db2a8"),
        ("closeEmptyTree", "320edb6b4e671067"),
    ];

    for (instruction, (name, header)) in instructions.iter().zip(expected) {
        assert_eq!(instruction["name"], name);
        assert_eq!(instruction["arguments"][0]["defaultValue"]["data"], header);
    }

    assert_eq!(instructions.len(), 7);
    assert_eq!(
        prepared["program"]["definedTypes"]
            .as_array()
            .unwrap()
            .len(),
        10
    );
    assert_eq!(prepared["program"]["errors"].as_array().unwrap().len(), 9);
    assert_eq!(
        instructions
            .iter()
            .filter(|item| item.get("remainingAccounts").is_some())
            .count(),
        3
    );
    assert_eq!(prepared, compression().unwrap());
}

fn original_compression() -> Value {
    serde_json::from_slice(include_bytes!("../../fixtures/compression-original.json")).unwrap()
}

fn prepare(root: &Value) -> Result<Value, Error> {
    generic::prepare_anchor(
        root,
        &generic::AnchorPolicy {
            program: generic::Policy {
                name: "splAccountCompression",
                address: "cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK".parse()?,
            },
            source_name: "spl_account_compression",
            remaining_proofs: &["replaceLeaf", "verifyLeaf", "insertOrAppend"],
        },
    )
}

#[test]
fn malformed_anchor_nodes_and_names_fail_before_any_output() {
    for (pointer, value) in [
        ("/instructions/0/accounts/0/isSigner", json!("false")),
        ("/instructions/0/args/0/type", json!({"array":["u8", -1]})),
        ("/types/0/type/fields/0/type", json!({"defined":"Missing"})),
        (
            "/types/0/type/fields/0/type",
            json!({"defined":"ApplicationDataEventV1"}),
        ),
        (
            "/metadata/address",
            json!("11111111111111111111111111111111"),
        ),
        ("/instructions/1/name", json!("initEmptyMerkleTree")),
        ("/types/0/name", json!("Reader")),
        ("/errors/1/code", json!(6000)),
    ] {
        let mut root = original_compression();
        *root.pointer_mut(pointer).unwrap() = value;
        assert!(prepare(&root).is_err(), "accepted mutation at {pointer}");
    }

    let mut root = original_compression();
    root["types"][0]["type"]["fields"][0]["maxLength"] = json!(32);
    assert!(prepare(&root).is_err());
}

#[test]
fn subscriptions_metadata_preserves_all_six_recipes_and_exact_default_links() {
    let source = subscriptions().unwrap();
    let program = &source["program"];
    let recipes = generic::pda_recipes(program["pdas"].as_array().unwrap()).unwrap();
    assert_eq!(recipes.len(), 6);
    assert_eq!(
        recipes[0].seeds[0],
        generic::PdaSeed::Constant(b"delegation".to_vec())
    );
    assert_eq!(
        recipes[0].seeds[4],
        generic::PdaSeed::U64("nonce".to_owned())
    );
    assert_eq!(
        recipes[3].seeds[0],
        generic::PdaSeed::Constant(b"SubscriptionAuthority".to_vec())
    );
    let defaults =
        generic::pda_defaults(program["instructions"].as_array().unwrap(), &recipes).unwrap();
    assert_eq!(defaults.len(), 28);
    assert_eq!(program["events"].as_array().unwrap().len(), 7);
    assert_eq!(program["accounts"].as_array().unwrap().len(), 6);
    assert_eq!(program["instructions"].as_array().unwrap().len(), 18);
    assert_eq!(program["errors"].as_array().unwrap().len(), 78);
}

#[test]
fn pda_default_unknown_links_missing_seeds_order_and_cycles_are_rejected() {
    let source = subscriptions().unwrap();
    let program = &source["program"];
    let recipes = generic::pda_recipes(program["pdas"].as_array().unwrap()).unwrap();

    for (pointer, value) in [
        ("/0/accounts/1/defaultValue/pda/name", json!("missing")),
        ("/0/accounts/1/defaultValue/seeds", json!([])),
        (
            "/0/accounts/1/defaultValue/seeds/0/name",
            json!("tokenMint"),
        ),
        (
            "/0/accounts/1/defaultValue/seeds/0/value/name",
            json!("missing"),
        ),
        (
            "/0/accounts/1/defaultValue/seeds/0/value/name",
            json!("subscriptionAuthority"),
        ),
    ] {
        let mut instructions = program["instructions"].clone();
        *instructions.pointer_mut(pointer).unwrap() = value;
        assert!(generic::pda_defaults(instructions.as_array().unwrap(), &recipes).is_err());
    }

    let mut pdas = program["pdas"].clone();
    pdas[0]["seeds"][0]["value"]["string"] = json!("x".repeat(33));
    assert!(generic::pda_recipes(pdas.as_array().unwrap()).is_err());
}
