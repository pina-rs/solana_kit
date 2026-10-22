//! Generator acceptance controls over the real pinned fixture.

use crate::{generate, proof};
use serde_json::Value;
use std::fs;

fn root() -> Value {
    serde_json::from_str(include_str!("../fixtures/system.json")).expect("valid pinned fixture")
}

#[test]
fn generated_encoders_match_independent_bytes() {
    proof::run().expect("independent byte proof");
}

#[test]
fn generation_is_deterministic() {
    assert_eq!(
        generate::render(&root()).expect("first"),
        generate::render(&root()).expect("second")
    );
}

#[test]
fn invalid_schema_preserves_existing_output() {
    let directory =
        std::env::temp_dir().join(format!("solana-kit-generator-{}", std::process::id()));
    fs::create_dir(&directory).expect("isolated output directory");
    let output = directory.join("generated.rs");
    fs::write(&output, "prior output").expect("sentinel");
    let mut malformed = root();
    malformed["program"]["instructions"][0]["name"] = "../escaped".into();
    assert!(generate::replace(&malformed, &output).is_err());
    assert_eq!(
        fs::read_to_string(&output).expect("preserved output"),
        "prior output"
    );
    assert!(!directory.join("generated.rs.pending").exists());
    fs::remove_dir_all(directory).expect("clean temporary output");
}

#[test]
fn duplicate_names_unknown_types_and_discriminators_are_rejected() {
    let mut duplicate = root();
    duplicate["program"]["instructions"][1]["name"] = "createAccount".into();
    assert!(generate::render(&duplicate).is_err());

    let mut field = root();
    field["program"]["instructions"][0]["arguments"][2]["name"] = "lamports".into();
    assert!(generate::render(&field).is_err());

    let mut unknown = root();
    unknown["program"]["instructions"][0]["arguments"][1]["type"]["kind"] =
        "definedTypeLinkNode".into();
    assert!(generate::render(&unknown).is_err());

    let mut discriminator = root();
    discriminator["program"]["instructions"][0]["arguments"][0]["defaultValue"]["number"] =
        (-1).into();
    assert!(generate::render(&discriminator).is_err());
}
