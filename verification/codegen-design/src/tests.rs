//! Generator acceptance controls over the real pinned fixture.

use crate::{generate, proof};
use serde_json::Value;
use std::{fs, path::Path};

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

    for name in [
        "../escaped",
        "path/name",
        "path\\name",
        "café",
        "0invalid",
        "fn",
    ] {
        let mut malformed = root();
        malformed["program"]["instructions"][0]["name"] = name.into();
        assert!(generate::replace(&malformed, &output).is_err());
        assert_eq!(
            fs::read_to_string(&output).expect("preserved output"),
            "prior output"
        );
        assert!(!directory.join("generated.rs.pending").exists());
    }

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

/// Mutates only a selected discriminator kind, retaining its numeric fields.
fn discriminator_kind(instruction: &str, node: &str, kind: Option<&str>) -> Value {
    let mut malformed = root();
    let selected = malformed["program"]["instructions"]
        .as_array_mut()
        .expect("instruction list")
        .iter_mut()
        .find(|candidate| candidate["name"] == instruction)
        .expect("selected instruction");
    let value = selected["arguments"][0][node]
        .as_object_mut()
        .expect("discriminator node");
    value.remove("kind");

    if let Some(kind) = kind {
        value.insert("kind".to_owned(), kind.into());
    }

    malformed
}

/// Requires schema rejection before any destination or pending-file mutation.
fn assert_kind_rejection(root: &Value, output: &Path) {
    let pending = output.with_extension("rs.pending");
    let before = fs::read(output).expect("existing output");
    let pending_before = pending
        .exists()
        .then(|| fs::read(&pending).expect("pending"));
    let error = generate::replace(root, output).expect_err("invalid discriminator kind");
    assert_eq!(error.to_string(), "unsupported discriminator contract");
    assert_eq!(fs::read(output).expect("preserved output"), before);
    assert_eq!(
        pending
            .exists()
            .then(|| fs::read(&pending).expect("pending")),
        pending_before
    );
}

#[test]
fn selected_discriminator_kinds_are_required_before_replacement() {
    let directory =
        std::env::temp_dir().join(format!("solana-kit-discriminator-{}", std::process::id()));
    fs::create_dir(&directory).expect("isolated output directory");
    let output = directory.join("generated.rs");
    let pending = output.with_extension("rs.pending");
    generate::replace(&root(), &output).expect("valid numeric discriminator nodes");
    assert_eq!(
        fs::read_to_string(&output).expect("generated output"),
        include_str!("generated.rs")
    );

    for instruction in ["createAccount", "createAccountWithSeed"] {
        for (node, kind) in [
            ("type", None),
            ("type", Some("definedTypeLinkNode")),
            ("defaultValue", None),
            ("defaultValue", Some("stringValueNode")),
        ] {
            let malformed = discriminator_kind(instruction, node, kind);
            assert_kind_rejection(&malformed, &output);
            fs::write(&pending, "pending sentinel").expect("pending sentinel");
            assert_kind_rejection(&malformed, &output);
            fs::remove_file(&pending).expect("remove pending sentinel");
        }
    }

    fs::remove_dir_all(directory).expect("clean temporary output");
}

#[test]
fn existing_pending_file_preserves_valid_generation_output() {
    let directory = std::env::temp_dir().join(format!("solana-kit-pending-{}", std::process::id()));
    fs::create_dir(&directory).expect("isolated output directory");
    let output = directory.join("generated.rs");
    let pending = output.with_extension("rs.pending");
    fs::write(&output, "prior output").expect("output sentinel");
    fs::write(&pending, "pending sentinel").expect("pending sentinel");

    assert!(generate::replace(&root(), &output).is_err());
    assert_eq!(fs::read_to_string(output).expect("output"), "prior output");
    assert_eq!(
        fs::read_to_string(pending).expect("pending"),
        "pending sentinel"
    );
    fs::remove_dir_all(directory).expect("clean temporary output");
}
