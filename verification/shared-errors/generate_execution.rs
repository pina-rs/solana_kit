//! Reproduces the typed execution mappings from owned enums and the Dart catalog.
use std::{env, fs, path::Path};

fn main() {
    let arguments: Vec<_> = env::args().collect();
    let root = Path::new(&arguments[1]);
    let output = Path::new(&arguments[2]);
    let source =
        fs::read_to_string(root.join("crates/solana_kit_client/src/methods/submission/failure.rs"))
            .expect("execution owners");
    let catalog = fs::read_to_string("oracle.tsv").expect("Dart catalog");
    let mut generated = String::from(
        "//! Stable mappings for existing execution error owners.\nuse crate::methods::submission::{InstructionFailure, KnownTransactionFailure};\nuse solana_kit_core::errors::ErrorCode;\nuse core::mem::discriminant;\n",
    );

    for (kind, prefix, table) in [
        ("InstructionFailure", "instructionError", "INSTRUCTIONS"),
        (
            "KnownTransactionFailure",
            "transactionError",
            "TRANSACTIONS",
        ),
    ] {
        generated.push_str(&emit_table(&source, &catalog, kind, prefix, table));
    }

    fs::write(output, generated).expect("generated execution mapping");
}

/// Payload variants need representative values; instruction delegation stays in its owner.
fn emit_table(source: &str, catalog: &str, kind: &str, prefix: &str, table: &str) -> String {
    let marker = format!("pub enum {kind} {{");
    let body = source
        .split(&marker)
        .nth(1)
        .expect("owned enum")
        .split("\n}\n")
        .next()
        .expect("enum end");
    let mut output = format!("const {table}: &[({kind},Option<ErrorCode>)]=&[\n");

    for line in body
        .lines()
        .filter(|line| line.starts_with("    ") && !line.starts_with("        "))
    {
        let line = line.trim();

        if !line.chars().next().is_some_and(char::is_uppercase) {
            continue;
        }

        let name = line
            .split(['(', '{', ','])
            .next()
            .expect("variant name")
            .trim();

        if name == "InstructionError" {
            continue;
        }

        let pattern = variant_pattern(kind, name, line);
        let key = format!("{prefix}{name}");
        let code = code_expression(catalog, &key);
        output.push_str(&format!("({pattern},{code}),\n"));
    }

    output.push_str("];\n");
    output.push_str(&format!("impl {kind} {{\n/// Stable pinned code for this execution variant.\npub fn error_code(&self)->Option<ErrorCode> {{\n"));

    if kind == "KnownTransactionFailure" {
        output.push_str(
            "if let Self::InstructionError(_,error)=self{return error.error_code();}\n\n",
        );
    }

    output.push_str(&format!("{table}.iter().find(|(variant,_)|discriminant(variant)==discriminant(self)).and_then(|(_,code)|*code)\n}}\n}}\n"));
    output
}

fn variant_pattern(kind: &str, name: &str, line: &str) -> String {
    if line.contains('(') {
        return format!("{kind}::{name}(0)");
    }

    if line.contains('{') {
        return format!("{kind}::{name} {{account_index:0}}");
    }

    format!("{kind}::{name}")
}

fn code_expression(catalog: &str, name: &str) -> String {
    if !catalog
        .lines()
        .any(|row| row.starts_with(&format!("{name}\t")))
    {
        return "None".into();
    }

    let mut constant = String::new();

    for character in name.chars() {
        if character.is_ascii_uppercase() {
            constant.push('_');
        }

        constant.push(character.to_ascii_uppercase());
    }

    format!("Some(ErrorCode::{constant})")
}
