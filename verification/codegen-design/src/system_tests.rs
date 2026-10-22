//! All-schema lifecycle controls for the production data emitter.

use crate::system;
use serde_json::{Value, json};
use std::fs;

const WRONG_NUMERIC_KIND: u64 = 7;
const FRACTIONAL_TAG: f64 = 1.5;
type Mutation = (&'static str, Value);

fn root() -> Value {
    serde_json::from_str(include_str!("../fixtures/system.json")).expect("pinned schema")
}

fn rejected(value: &Value, output: &std::path::Path) {
    let pending = output.with_extension("rs.pending");
    let before = fs::read(output).expect("sentinel");
    let staged = pending
        .exists()
        .then(|| fs::read(&pending).expect("pending"));
    assert!(system::replace(value, output).is_err());
    assert_eq!(fs::read(output).expect("unchanged"), before);
    assert_eq!(
        pending
            .exists()
            .then(|| fs::read(&pending).expect("unchanged pending")),
        staged
    );
}

#[test]
fn all_variants_require_real_discriminator_kinds_before_writes() {
    let directory = std::env::temp_dir().join(format!("system-kinds-{}", std::process::id()));
    fs::create_dir(&directory).expect("isolated output");
    let output = directory.join("generated.rs");
    let pending = output.with_extension("rs.pending");
    fs::write(&output, "original").expect("sentinel");

    for instruction in 0..14 {
        for node in ["type", "defaultValue"] {
            for kind in [
                Value::Null,
                json!("wrongKind"),
                json!(WRONG_NUMERIC_KIND),
                json!(["numberTypeNode"]),
            ] {
                let mut malformed = root();
                malformed["program"]["instructions"][instruction]["arguments"][0][node]["kind"] =
                    kind;
                rejected(&malformed, &output);
                fs::write(&pending, "pending").expect("sentinel");
                rejected(&malformed, &output);
                fs::remove_file(&pending).expect("remove pending");
            }
        }
    }

    fs::remove_dir_all(directory).expect("cleanup");
}

fn layout_mutations() -> Vec<Mutation> {
    vec![
        (
            "/program/instructions/0/arguments/0/type/format",
            json!("u64"),
        ),
        (
            "/program/instructions/0/arguments/0/type/endian",
            json!("be"),
        ),
        (
            "/program/instructions/0/arguments/0/defaultValue/number",
            json!(-1),
        ),
        (
            "/program/instructions/0/arguments/0/defaultValue/number",
            json!(4294967296u64),
        ),
        (
            "/program/instructions/0/arguments/0/defaultValue/number",
            json!(FRACTIONAL_TAG),
        ),
        ("/program/instructions/0/discriminators/0/offset", json!(1)),
    ]
}

fn field_mutations() -> Vec<Mutation> {
    vec![
        (
            "/program/instructions/3/arguments/2/type/prefix/kind",
            json!("stringTypeNode"),
        ),
        (
            "/program/instructions/3/arguments/2/type/prefix/format",
            json!("u32"),
        ),
        (
            "/program/instructions/3/arguments/2/type/prefix/endian",
            json!("be"),
        ),
        (
            "/program/instructions/3/arguments/2/type/type/encoding",
            json!("hex"),
        ),
        (
            "/program/instructions/13/arguments/1/defaultValue/kind",
            json!("stringValueNode"),
        ),
        (
            "/program/instructions/13/arguments/1/defaultValue/number",
            json!(-1),
        ),
        (
            "/program/instructions/13/arguments/1/defaultValueStrategy",
            json!("omitted"),
        ),
        ("/program/instructions/1/name", json!("createAccount")),
        (
            "/program/instructions/1/arguments/0/defaultValue/number",
            json!(0),
        ),
        (
            "/program/instructions/0/arguments/2/name",
            json!("lamports"),
        ),
        (
            "/program/instructions/0/arguments/2/name",
            json!("discriminator"),
        ),
        (
            "/program/instructions/0/arguments/1/type/kind",
            json!("definedTypeLinkNode"),
        ),
    ]
}

#[test]
fn layouts_names_defaults_and_atomic_replacement_are_validated() {
    let directory = std::env::temp_dir().join(format!("system-layouts-{}", std::process::id()));
    fs::create_dir(&directory).expect("isolated output");
    let output = directory.join("generated.rs");
    fs::write(&output, "original").expect("sentinel");

    for (pointer, replacement) in layout_mutations().into_iter().chain(field_mutations()) {
        let mut malformed = root();
        *malformed.pointer_mut(pointer).expect("actual node") = replacement;
        rejected(&malformed, &output);
    }

    for pointer in [
        "/program/instructions/0/arguments/1/type",
        "/program/instructions/3/arguments/2/type/prefix",
    ] {
        let mut malformed = root();
        malformed.pointer_mut(pointer).expect("numeric node")["min"] = json!(1);
        rejected(&malformed, &output);
    }

    for name in [
        "../escape",
        "path/name",
        "path\\name",
        "café",
        "0start",
        "fn",
        "x\";panic!()",
        "systemInstructionData",
        "address",
        "allocateCodec",
    ] {
        let mut malformed = root();
        malformed["program"]["instructions"][0]["name"] = name.into();
        rejected(&malformed, &output);
    }

    system::replace(&root(), &output).expect("valid generation");
    let first = fs::read(&output).expect("generated");
    system::replace(&root(), &output).expect("repeat generation");
    assert_eq!(fs::read(&output).expect("repeat"), first);
    assert_eq!(
        first,
        include_bytes!("../../../crates/solana_kit_system/src/generated.rs")
    );
    let pending = output.with_extension("rs.pending");
    fs::write(&pending, "pending").expect("sentinel");
    rejected(&root(), &output);

    fs::remove_dir_all(directory).expect("cleanup");
}
