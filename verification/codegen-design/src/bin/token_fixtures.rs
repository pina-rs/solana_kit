//! Creates source-only caller matrices; expected bytes come from actual Dart.
#[path = "../token_samples.rs"]
mod samples;
use serde_json::Value;
use std::{error::Error, fs, path::Path};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn title(value: &str) -> String {
    let mut value = value.to_owned();
    value[0..1].make_ascii_uppercase();
    value
}

fn main() -> Result<()> {
    let arguments: Vec<_> = std::env::args().collect();
    let output = Path::new(arguments.get(1).ok_or("output directory required")?);
    let root: Value = serde_json::from_str(include_str!("../../fixtures/token-original.json"))?;
    let program = &root["program"];
    let mut dart="import 'dart:typed_data';\nimport 'package:solana_kit_token/solana_kit_token.dart' as t;\nimport 'package:solana_kit_addresses/solana_kit_addresses.dart';\nimport 'package:solana_kit_instructions/solana_kit_instructions.dart';\nString hex(List<int> bytes)=>bytes.map((b)=>b.toRadixString(16).padLeft(2,'0')).join();\nvoid ix(String key,Instruction value){print('$key\\t${hex(value.data!)}\\t${value.programAddress}\\t${value.accounts!.map((a)=>'${a.address}:${a.role.value}').join(',')}');}\nvoid main(){\n".to_owned();
    let mut rust="//! Executed original Dart constructor and account byte oracles.\nuse solana_kit_token::*;\nuse solana_kit_core::{Address,AccountMeta,AccountRole,codecs::{Encoder,Decoder,Limits}};\nmod support;\nconst LIMITS:Limits=Limits::new(4096,4096);\nconst NONZERO_BYTE:u8=7;\n".to_owned();

    for case in 0..3 {
        for item in program["instructions"]
            .as_array()
            .ok_or("instructions missing")?
        {
            instruction(item, program, case, &mut dart, &mut rust)?;
        }

        for item in program["accounts"].as_array().ok_or("accounts missing")? {
            account(item, program, case, &mut dart, &mut rust)?;
        }
    }

    dart.push_str("}\n");
    fs::write(output.join("token-oracle.dart"), dart)?;
    fs::write(output.join("token-oracle.rs"), rust)?;
    Ok(())
}

/// Emit caller values; actual Dart computes every expected byte.
fn instruction(
    item: &Value,
    program: &Value,
    case: usize,
    dart: &mut String,
    rust: &mut String,
) -> Result<()> {
    let name = item["name"].as_str().ok_or("name missing")?;
    let title = title(name);
    let key = format!("{name}-{case}");
    let mut dargs = Vec::new();
    let mut rargs = Vec::new();
    let address = samples::address(case);
    dargs.push(format!("programAddress:Address('{address}')"));
    let accounts = roles(item, case, &mut dargs)?;

    for field in item["arguments"]
        .as_array()
        .ok_or("arguments missing")?
        .iter()
        .skip(1)
    {
        let field_name = field["name"].as_str().ok_or("field name missing")?;
        let path = format!("{title}{}", title_of(field_name));
        dargs.push(format!(
            "{field_name}:{}",
            samples::value(&field["type"], program, case, true, &path)?
        ));
        rargs.push(format!(
            "{}:{}",
            samples::snake(field_name),
            samples::value(&field["type"], program, case, false, &path)?
        ));
    }
    dart.push_str(&format!(
        "ix('{key}',t.get{title}Instruction({}));\n",
        dargs.join(",")
    ));
    rust.push_str(&format!("#[test]\nfn {}() {{\nlet value={title}{{{}}};\nlet accounts=vec![{}];\nlet program:Address={address:?}.parse().unwrap();\nlet instruction=value.instruction_with_program(program,&accounts,LIMITS).unwrap();\nsupport::instruction({key:?},&instruction);\nassert_eq!({title}::parse_with_program(program,&instruction,LIMITS).unwrap(),value);\nassert!(parse_instruction_with_program(program,&instruction,LIMITS).is_ok());\n}}\n",samples::snake(&key.replace('-',"Case")),rargs.join(","),accounts.join(",")));
    Ok(())
}
fn roles(item: &Value, case: usize, dargs: &mut Vec<String>) -> Result<Vec<String>> {
    let mut accounts = Vec::new();

    for (index, role) in item["accounts"]
        .as_array()
        .ok_or("roles missing")?
        .iter()
        .enumerate()
    {
        let name = role["name"].as_str().ok_or("missing role name")?;

        if role["isOptional"] == true && case == 0 {
            continue;
        }

        let address = samples::address((index + case) % 3);
        dargs.push(format!("{name}:Address('{address}')"));

        let signer = role["isSigner"] == true || role["isSigner"] == "either";
        let writable = role["isWritable"] == true;
        accounts.push(format!("AccountMeta{{address:{address:?}.parse().unwrap(),role:AccountRole::from_privileges({signer},{writable})}}"));
    }
    Ok(accounts)
}
fn title_of(name: &str) -> String {
    title(name)
}

fn account(
    item: &Value,
    program: &Value,
    case: usize,
    dart: &mut String,
    rust: &mut String,
) -> Result<()> {
    let name = item["name"].as_str().ok_or("account name missing")?;
    let title = title(name);
    let key = format!("account-{name}-{case}");
    let mut dargs = Vec::new();
    let mut rargs = Vec::new();

    for field in item["data"]["fields"]
        .as_array()
        .ok_or("account fields missing")?
    {
        let name = field["name"].as_str().ok_or("field name missing")?;
        dargs.push(format!(
            "{name}:{}",
            samples::value(&field["type"], program, case, true, name)?
        ));
        rargs.push(format!(
            "{}:{}",
            samples::snake(name),
            samples::value(&field["type"], program, case, false, name)?
        ));
    }
    dart.push_str(&format!(
        "print('{key}\\t${{hex(t.get{title}Encoder().encode(t.{title}({})))}}\\t-\\t-');\n",
        dargs.join(",")
    ));
    rust.push_str(&format!("#[test]\nfn account_{}() {{\nlet value={title}{{{}}};\nlet bytes={title}Codec.encode(&value,4096).unwrap();\nsupport::bytes({key:?},&bytes);\nassert_eq!({title}Codec.decode(&bytes,LIMITS).unwrap(),value);\nsupport::fixed_negatives(&{title}Codec,&bytes);\n}}\n",samples::snake(&format!("{name}Case{case}")),rargs.join(",")));
    Ok(())
}
