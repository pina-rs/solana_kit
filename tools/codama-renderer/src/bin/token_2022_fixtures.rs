//! Generates caller-only Dart probes from the exact pinned Token-2022 schema.
#[path = "../token_2022_samples.rs"]
mod samples;

use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{error::Error, fs, path::Path};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const PIN: &str = "13cbda7213599f3312e1b7ffd1c7f176f36ebd74dba9feabe6e9a6874dc1d5d2";

/// Every declaration receives four caller cases before all extension variants.
fn main() -> Result<()> {
    let arguments: Vec<_> = std::env::args().collect();
    let output = Path::new(arguments.get(1).ok_or("output directory required")?);
    let bytes = include_bytes!("../../fixtures/token-2022-original.json");

    if format!("{:x}", Sha256::digest(bytes)) != PIN {
        return Err("Token-2022 source pin mismatch".into());
    }

    let root: Value = serde_json::from_slice(bytes)?;
    let program = &root["program"];
    let mut dart = "import 'dart:typed_data';\nimport 'package:solana_kit_token_2022/solana_kit_token_2022.dart' as t;\nimport 'package:solana_kit_addresses/solana_kit_addresses.dart';\nimport 'package:solana_kit_instructions/solana_kit_instructions.dart';\nString hex(List<int> bytes)=>bytes.map((b)=>b.toRadixString(16).padLeft(2,'0')).join();\nvoid ix(String key,Instruction value){print('$key\\t${hex(value.data!)}\\t${value.programAddress}\\t${value.accounts!.map((a)=>'${a.address}:${a.role.value}').join(',')}');}\nvoid main(){\n".to_owned();

    for case in 0..4 {
        for item in program["instructions"]
            .as_array()
            .ok_or("missing instructions")?
        {
            instruction(item, program, case, &mut dart)?;
        }

        for item in program["accounts"].as_array().ok_or("missing accounts")? {
            account(item, program, case, &mut dart)?;
        }
    }

    extensions(program, &mut dart)?;
    dart.push_str("}\n");
    fs::write(output.join("token-2022-oracle.dart"), dart)?;
    Ok(())
}

fn instruction(item: &Value, program: &Value, case: usize, dart: &mut String) -> Result<()> {
    let name = item["name"].as_str().ok_or("missing instruction name")?;
    let mut args = vec![format!("programAddress:{}", samples::address(case))];
    roles(item, case, &mut args)?;
    arguments(item, program, case, &mut args)?;
    dart.push_str(&format!(
        "ix('{name}-{case}',t.get{}Instruction({}));\n",
        samples::title(name),
        args.join(",")
    ));
    Ok(())
}

/// Exercise optional omission and the source's explicit nonsigning-authority flag.
fn roles(item: &Value, case: usize, args: &mut Vec<String>) -> Result<()> {
    for (index, role) in item["accounts"]
        .as_array()
        .ok_or("missing accounts")?
        .iter()
        .enumerate()
    {
        if role["isOptional"] == true && case == 0 {
            continue;
        }

        let name = role["name"].as_str().ok_or("missing account name")?;
        args.push(format!("{name}:{}", samples::address((case + index) % 4)));

        if role["isSigner"] == "either" && case == 3 {
            args.push(format!("{name}IsSigner:false"));
        }
    }
    Ok(())
}

fn arguments(item: &Value, program: &Value, case: usize, args: &mut Vec<String>) -> Result<()> {
    for field in item["arguments"].as_array().ok_or("missing arguments")? {
        if field["defaultValueStrategy"] == "omitted" {
            continue;
        }

        let name = field["name"].as_str().ok_or("missing argument name")?;
        args.push(format!(
            "{name}:{}",
            samples::value(&field["type"], program, case)?
        ));
    }

    Ok(())
}

fn account(item: &Value, program: &Value, case: usize, dart: &mut String) -> Result<()> {
    let name = item["name"].as_str().ok_or("missing account name")?;
    let title = samples::title(name);
    let mut arguments = Vec::new();

    for field in item["data"]["fields"].as_array().ok_or("missing fields")? {
        let name = field["name"].as_str().ok_or("missing field name")?;
        let value = if name == "extensions" {
            "null".to_owned()
        } else {
            samples::value(&field["type"], program, case)?
        };

        arguments.push(format!("{name}:{value}"));
    }

    let encoded = format!(
        "t.get{title}Encoder().encode(t.{title}({}))",
        arguments.join(",")
    );
    dart.push_str(&format!(
        "print('account-{name}-{case}\\t${{hex({encoded})}}\\t-\\t-');\n"
    ));
    Ok(())
}

fn extensions(program: &Value, dart: &mut String) -> Result<()> {
    let extension = program["definedTypes"]
        .as_array()
        .ok_or("missing definitions")?
        .iter()
        .find(|item| item["name"] == "extension")
        .ok_or("missing extension")?;

    for variant in extension["type"]["variants"]
        .as_array()
        .ok_or("missing variants")?
    {
        let name = variant["name"].as_str().ok_or("missing variant name")?;

        for case in 0..4 {
            let value = samples::variant("extension", variant, program, case)?;
            dart.push_str(&format!("print('extension-{name}-{case}\\t${{hex(t.getExtensionEncoder().encode({value}))}}\\t-\\t-');\n"));
        }
    }

    Ok(())
}
