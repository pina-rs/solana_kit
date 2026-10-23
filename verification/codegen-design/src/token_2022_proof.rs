//! Generates an ordinary Rust caller against all independent Dart data rows.
use crate::{
    schema::{Error, identifier},
    token_2022, token_2022_rust_values as values,
};
use serde_json::Value;
use std::{fs, path::Path};

pub(crate) fn run(output: &Path) -> Result<(), Error> {
    fs::create_dir_all(output)?;
    let root = token_2022::root()?;
    let program = &root["program"];
    token_2022::run(&output.join("generated.rs"))?;
    let mut calls = data_calls(program)?;
    calls.push_str(&extension_calls(program)?);
    let main = format!(
        "extern crate alloc;\ninclude!(\"generated.rs\");\n{}\n{}\nfn main() {{ let rows=rows();\n{calls}\ncontrols(&rows);\nassert_eq!(rows.len(),528); println!(\"412 full byte matches; 116 extension payload comparisons preserve the IDL u16 length omitted by Dart\"); }}\n",
        include_str!("token_2022_proof_body.rs.txt"),
        include_str!("token_2022_controls.rs.txt")
    );
    crate::generate::replace_source(&main, &output.join("main.rs"))?;

    fs::write(
        output.join("oracle.tsv"),
        include_bytes!("../fixtures/token-2022-oracle.tsv"),
    )?;
    manifest(output)
}

fn data_calls(program: &Value) -> Result<String, Error> {
    let mut calls = String::new();

    for case in 0..4 {
        for item in program["instructions"]
            .as_array()
            .ok_or("missing instructions")?
        {
            calls.push_str(&instruction(item, program, case)?);
        }

        for item in program["accounts"].as_array().ok_or("missing accounts")? {
            let raw = item["name"].as_str().ok_or("missing account name")?;
            let name = values::title(raw);
            calls.push_str(&format!(
                "check({name}Codec,{},\"account-{raw}-{case}\",&rows);\n",
                values::record(&name, &item["data"], program, case)?
            ));
        }
    }

    Ok(calls)
}

fn instruction(item: &Value, program: &Value, case: usize) -> Result<String, Error> {
    let raw = item["name"].as_str().ok_or("missing instruction name")?;
    let name = values::title(raw);
    let mut arguments = Vec::new();

    for field in item["arguments"].as_array().ok_or("missing arguments")? {
        if field["defaultValueStrategy"] == "omitted" {
            continue;
        }

        let field_name = field["name"].as_str().ok_or("missing argument name")?;
        let path = format!("{name}{}", values::title(field_name));
        let value = values::value(&field["type"], program, case, &path)?;
        arguments.push(format!("{}:{value}", identifier(field_name)?));
    }

    Ok(format!(
        "check({name}Codec,{name}{{{}}},\"{raw}-{case}\",&rows);\n",
        arguments.join(",")
    ))
}

fn extension_calls(program: &Value) -> Result<String, Error> {
    let extension = program["definedTypes"]
        .as_array()
        .ok_or("missing definitions")?
        .iter()
        .find(|item| item["name"] == "extension")
        .ok_or("missing extension")?;
    let mut calls = String::new();

    for variant in extension["type"]["variants"]
        .as_array()
        .ok_or("missing variants")?
    {
        let name = variant["name"].as_str().ok_or("missing extension name")?;

        for case in 0..4 {
            calls.push_str(&format!(
                "check(ExtensionCodec,{},\"extension-{name}-{case}\",&rows);\n",
                values::variant("Extension", variant, program, case)?
            ));
        }
    }

    Ok(calls)
}

fn manifest(output: &Path) -> Result<(), Error> {
    let core = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/solana_kit_core")
        .canonicalize()?;
    fs::write(
        output.join("Cargo.toml"),
        format!(
            "[package]\nname=\"token_2022_data_consumer\"\nversion=\"0.0.0\"\nedition=\"2024\"\n[workspace]\n[[bin]]\nname=\"consumer\"\npath=\"main.rs\"\n[dependencies]\nsolana_kit_core={{path={core:?},default-features=false}}\n"
        ),
    )?;
    Ok(())
}
