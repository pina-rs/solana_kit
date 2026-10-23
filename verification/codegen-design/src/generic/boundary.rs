use super::*;
use serde_json::json;
use std::{fs, path::Path, process::Command};

fn execute(root: &Value, name: &str, body: &str) {
    let directory =
        std::env::temp_dir().join(format!("kit-boundary-{name}-{}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    let policy = Policy {
        name: "computeBudget",
        address: "ComputeBudget111111111111111111111111111111"
            .parse()
            .unwrap(),
    };
    replace(root, &policy, &directory.join("generated.rs")).unwrap();
    execute_source(
        &directory,
        &format!("extern crate alloc; include!(\"generated.rs\"); fn main() {{ {body} }}"),
    );
}

pub(super) fn execute_source(directory: &Path, source: &str) {
    let core = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/solana_kit_core");
    fs::write(directory.join("main.rs"), source).unwrap();
    fs::write(
        directory.join("Cargo.toml"),
        format!(
            "[package]\nname = \"generated_consumer\"\nversion = \"0.0.0\"\nedition = \"2024\"\n[workspace]\n[[bin]]\nname = \"consumer\"\npath = \"main.rs\"\n[dependencies]\nsolana_kit_core = {{ path = {:?}, default-features = false }}\n",
            core
        ),
    )
    .unwrap();
    let output = Command::new(env!("CARGO"))
        .args([
            "build",
            "--offline",
            "--message-format=json",
            "--verbose",
            "-j2",
            "--manifest-path",
        ])
        .arg(directory.join("Cargo.toml"))
        .arg("--target-dir")
        .arg(directory.join("target"))
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env("RUSTFLAGS", "-Dwarnings")
        .output()
        .unwrap();
    fs::write(directory.join("build.jsonl"), &output.stdout).unwrap();
    fs::write(directory.join("build.stderr"), &output.stderr).unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let executable = String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .find_map(|item| item["executable"].as_str().map(str::to_owned))
        .unwrap();

    let preserved = directory.join("executed-consumer");
    fs::copy(executable, &preserved).unwrap();
    assert!(Command::new(preserved).status().unwrap().success());
}

#[test]
fn all_256_enum_variants_compile_and_execute_without_warnings() {
    let mut root: Value =
        serde_json::from_str(include_str!("../../fixtures/compute-budget.json")).unwrap();
    let variants: Vec<_> = (0..256)
        .map(|index| json!({"kind":"enumEmptyVariantTypeNode","name":format!("v{index}")}))
        .collect();
    root["program"]["definedTypes"] = json!([{"kind":"definedTypeNode","name":"phase","type":{
        "kind":"enumTypeNode","size":{"kind":"numberTypeNode","format":"u8","endian":"le"},
        "variants": variants
    }}]);
    let mut body = String::new();

    for tag in 0..256 {
        body.push_str(&format!("let value = Phase::V{tag}; assert_eq!(PhaseCodec.encode(&value, 1).unwrap(), [{tag}]); assert_eq!(PhaseCodec.decode(&[{tag}], Limits::new(1,0)).unwrap(), value);"));
    }

    body.push_str("assert!(PhaseCodec.decode(&[], Limits::new(1,0)).is_err()); assert!(PhaseCodec.decode(&[0,1], Limits::new(2,0)).is_err()); assert!(PhaseCodec.encode(&Phase::V0,0).is_err());");
    execute(&root, "enum", &body);
}

#[test]
fn zero_field_instruction_compiles_and_executes_without_warnings() {
    let mut root: Value =
        serde_json::from_str(include_str!("../../fixtures/compute-budget.json")).unwrap();
    root["program"]["instructions"]
        .as_array_mut()
        .unwrap()
        .truncate(1);
    root["program"]["instructions"][0]["arguments"]
        .as_array_mut()
        .unwrap()
        .truncate(1);
    execute(
        &root,
        "empty",
        "let value = RequestUnits {}; let limits = Limits::new(2,0); let ix = value.instruction(limits).unwrap(); assert_eq!(ix.data,[0]); assert_eq!(RequestUnits::parse(&ix,limits).unwrap(),value); assert_eq!(parse_instruction(&ix,limits).unwrap(),ComputeBudgetInstructionData::RequestUnits(value)); assert!(RequestUnitsCodec.decode(&[],limits).is_err()); assert!(RequestUnitsCodec.decode(&[1],limits).is_err()); assert!(RequestUnitsCodec.decode(&[0,0],limits).is_err()); assert!(RequestUnitsCodec.encode(&RequestUnits {},0).is_err()); let mut bad=ix.clone(); bad.program_address=Address::from_bytes([93;32]); assert!(RequestUnits::parse(&bad,limits).is_err());",
    );
}

#[test]
fn complete_and_sparse_instruction_domains_execute_without_warnings() {
    let mut root: Value =
        serde_json::from_str(include_str!("../../fixtures/compute-budget.json")).unwrap();
    let mut prototype = root["program"]["instructions"][0].clone();
    prototype["arguments"].as_array_mut().unwrap().truncate(1);
    let mut body = String::from("let limits = Limits::new(1,0);");
    let mut instructions = Vec::new();

    for tag in 0..256 {
        let mut instruction = prototype.clone();
        instruction["name"] = json!(format!("operation{tag}"));
        instruction["arguments"][0]["defaultValue"]["number"] = json!(tag);
        instructions.push(instruction);
        body.push_str(&format!("let value = Operation{tag} {{}}; let ix = value.instruction(limits).unwrap(); assert_eq!(ix.data,[{tag}]); assert_eq!(identify_instruction(&ix.data).unwrap(),{tag}); assert_eq!(Operation{tag}::parse(&ix,limits).unwrap(),value); let sum = ComputeBudgetInstructionData::Operation{tag}(value); assert_eq!(parse_instruction(&ix,limits).unwrap(),sum); assert_eq!(ComputeBudgetInstructionDataCodec.encode(&sum,1).unwrap(),[{tag}]); assert_eq!(ComputeBudgetInstructionDataCodec.decode(&[{tag}],limits).unwrap(),sum); assert_eq!(Operation{tag}Codec.decode(&[{tag},0],Limits::new(2,0)),Err(Error::TrailingBytes)); assert!(Operation{tag}Codec.encode(&Operation{tag} {{}},0).is_err());"));
    }

    root["program"]["instructions"] = json!(instructions);
    body.push_str("assert_eq!(identify_instruction(&[]),Err(Error::Truncated)); assert_eq!(ComputeBudgetInstructionDataCodec.decode(&[],limits),Err(Error::Truncated)); let mut ix=Operation0 {}.instruction(limits).unwrap(); ix.program_address=Address::from_bytes([93;32]); assert_eq!(parse_instruction(&ix,limits),Err(Error::InvalidValue)); ix.program_address=PROGRAM; ix.accounts.push(solana_kit_core::AccountMeta {address: PROGRAM, role: solana_kit_core::AccountRole::Readonly}); assert_eq!(parse_instruction(&ix,limits),Err(Error::InvalidValue)); assert!(ComputeBudgetInstructionDataCodec.decode(&[0],Limits::new(0,0)).is_err());");
    execute(&root, "dispatch-full", &body);

    root["program"]["instructions"] = json!([instructions[0], instructions[93], instructions[255]]);
    execute(
        &root,
        "dispatch-sparse",
        "let limits=Limits::new(2,0); for tag in 0..=255u8 { if [0,93,255].contains(&tag) { assert_eq!(identify_instruction(&[tag]),Ok(tag)); assert!(ComputeBudgetInstructionDataCodec.decode(&[tag],limits).is_ok()); } else { assert_eq!(identify_instruction(&[tag]),Err(Error::InvalidDiscriminator)); assert_eq!(ComputeBudgetInstructionDataCodec.decode(&[tag],limits),Err(Error::InvalidDiscriminator)); } } assert_eq!(ComputeBudgetInstructionDataCodec.decode(&[93,0],limits),Err(Error::TrailingBytes));",
    );
}
