//! Account/error schema controls execute real atomic replacement paths.
#[path = "program_binding_tests.rs"]
mod program_binding_tests;

use crate::{builders, system_errors};
use serde_json::{Value, json};
use std::fs;

const INVALID_FLAG: u64 = 7;

fn root() -> Value {
    serde_json::from_str(include_str!("../fixtures/system.json")).unwrap()
}

#[test]
fn selected_account_properties_reject_before_output_or_pending_mutation() {
    let directory =
        std::env::temp_dir().join(format!("system-client-schema-{}", std::process::id()));
    fs::create_dir(&directory).unwrap();
    let output = directory.join("builders.rs");
    let pending = output.with_extension("rs.pending");
    fs::write(&output, "output sentinel").unwrap();
    fs::write(&pending, "pending sentinel").unwrap();

    for instruction in 0..14 {
        reject_accounts(instruction, &output, &pending);
    }

    fs::remove_file(&pending).unwrap();
    builders::replace(&root(), &output).unwrap();
    let first = fs::read(&output).unwrap();
    builders::replace(&root(), &output).unwrap();
    assert_eq!(fs::read(&output).unwrap(), first);
    assert_eq!(
        first,
        include_bytes!("../../../crates/solana_kit_system/src/builders.rs")
    );

    for index in 0..9 {
        for (property, value) in [
            ("code", json!(-1)),
            ("kind", json!("wrong")),
            ("name", json!("unknown")),
            ("message", json!(null)),
            ("constraint", json!(true)),
        ] {
            let mut malformed = root();
            malformed["program"]["errors"][index][property] = value;
            assert!(system_errors::render(&malformed).is_err());
        }
    }

    fs::remove_dir_all(directory).unwrap();
}

/// Runs property mutations for one layout while retaining both sentinels.
fn reject_accounts(instruction: usize, output: &std::path::Path, pending: &std::path::Path) {
    let count = root()["program"]["instructions"][instruction]["accounts"]
        .as_array()
        .unwrap()
        .len();

    for account in 0..count {
        for (property, value) in [
            ("kind", json!("wrong")),
            ("isSigner", json!("either")),
            ("isWritable", json!(null)),
            ("isOptional", json!(INVALID_FLAG)),
            ("unsupported", json!(true)),
            ("name", json!("../escape")),
        ] {
            let mut malformed = root();
            malformed["program"]["instructions"][instruction]["accounts"][account][property] =
                value;
            assert!(builders::replace(&malformed, output).is_err());
            assert_eq!(fs::read_to_string(output).unwrap(), "output sentinel");
            assert_eq!(fs::read_to_string(pending).unwrap(), "pending sentinel");
        }
    }
}

#[test]
/// A generally supported wire kind must not change a manually bound System field.
fn builder_policy_rejects_otherwise_supported_field_kind_changes() {
    let reference = root();

    for instruction in 0..14 {
        let arguments = reference["program"]["instructions"][instruction]["arguments"]
            .as_array()
            .unwrap();

        for (index, argument) in arguments.iter().enumerate().skip(1) {
            let mut malformed = root();
            let alternate = alternate_wire(argument);
            malformed["program"]["instructions"][instruction]["arguments"][index]["type"] =
                alternate;
            assert!(builders::render(&malformed).is_err());
        }
    }
}

fn alternate_wire(argument: &Value) -> Value {
    if argument["type"]["kind"] == "publicKeyTypeNode" {
        return json!({"kind":"numberTypeNode","format":"u64","endian":"le"});
    }

    json!({"kind":"publicKeyTypeNode"})
}
