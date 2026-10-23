//! Executed ordinary consumers for newly admitted closed wire and tag shapes.
use super::*;
use serde_json::json;
use std::{fs, process::Command};
const CONFIDENTIAL_TRANSFER_TAG: u8 = 27;
const METADATA_TAG_BYTES: usize = 8;
const ELGAMAL_BYTES: usize = 64;

#[test]
fn signed_float_fixed_bytes_zeroable_and_multiple_header_bytes_execute() {
    let mut root: Value =
        serde_json::from_str(include_str!("../../fixtures/alt-original.json")).unwrap();
    root["program"]["accounts"] = json!([]);
    root["program"]["definedTypes"] = json!([]);
    root["program"]["pdas"] = json!([]);
    root["program"]["instructions"] = json!([{
        "kind":"instructionNode", "name":"sample", "accounts":[], "optionalAccountStrategy":"programId", "docs":[],
        "discriminators":[{"kind":"fieldDiscriminatorNode","name":"discriminator","offset":0},{"kind":"fieldDiscriminatorNode","name":"secondary","offset":1}],
        "arguments":[
            {"kind":"instructionArgumentNode","name":"discriminator","type":{"kind":"numberTypeNode","format":"u8","endian":"le"},"defaultValueStrategy":"omitted","defaultValue":{"kind":"numberValueNode","number":CONFIDENTIAL_TRANSFER_TAG}},
            {"kind":"instructionArgumentNode","name":"secondary","type":{"kind":"fixedSizeTypeNode","size":METADATA_TAG_BYTES,"type":{"kind":"bytesTypeNode"}},"defaultValueStrategy":"omitted","defaultValue":{"kind":"bytesValueNode","data":"0001020304050607","encoding":"base16"}},
            {"kind":"instructionArgumentNode","name":"signedByte","type":{"kind":"numberTypeNode","format":"i8","endian":"le"}},
            {"kind":"instructionArgumentNode","name":"rate","type":{"kind":"numberTypeNode","format":"i16","endian":"le"}},
            {"kind":"instructionArgumentNode","name":"timestamp","type":{"kind":"numberTypeNode","format":"i64","endian":"le"}},
            {"kind":"instructionArgumentNode","name":"multiplier","type":{"kind":"numberTypeNode","format":"f64","endian":"le"}},
            {"kind":"instructionArgumentNode","name":"ciphertext","type":{"kind":"fixedSizeTypeNode","size":ELGAMAL_BYTES,"type":{"kind":"bytesTypeNode"}}},
            {"kind":"instructionArgumentNode","name":"authority","type":{"kind":"zeroableOptionTypeNode","item":{"kind":"publicKeyTypeNode"}}}
        ]
    }]);
    let policy = Policy {
        name: "addressLookupTable",
        address: "AddressLookupTab1e1111111111111111111111111"
            .parse()
            .unwrap(),
    };
    let emitted = data::render(&root, &policy).unwrap();

    let body = "let limits=solana_kit_core::codecs::Limits::new(1024,1024); let value=Sample {signed_byte:i8::MIN,rate:i16::MIN,timestamp:i64::MIN,multiplier:1.5,ciphertext:[255;64],authority:None}; let bytes=SampleCodec.encode(&value,1024).unwrap(); let mut expected=vec![27,0,1,2,3,4,5,6,7,128,0,128,0,0,0,0,0,0,0,128,0,0,0,0,0,0,248,63]; expected.extend_from_slice(&[255;64]); expected.extend_from_slice(&[0;32]); assert_eq!(bytes,expected); assert_eq!(SampleCodec.decode(&bytes,limits).unwrap(),value); for end in 0..bytes.len() { assert!(SampleCodec.decode(&bytes[..end],limits).is_err()); } let mut bad=bytes.clone(); bad[8]=8; assert!(SampleCodec.decode(&bad,limits).is_err()); let mut trailing=bytes.clone(); trailing.push(0); assert!(SampleCodec.decode(&trailing,limits).is_err()); assert!(SampleCodec.encode(&value,bytes.len()-1).is_err()); let value=Sample {authority:Some(Address::from_bytes([7;32])),..value}; let bytes=SampleCodec.encode(&value,1024).unwrap(); assert_eq!(&bytes[bytes.len()-32..],&[7;32]); assert_eq!(SampleCodec.decode(&bytes,limits).unwrap(),value);";
    execute(&emitted, body);
}

fn execute(emitted: &str, body: &str) {
    let body = body.replace("; ", ";\n\n").replace("} let", "}\n\nlet");
    let directory = std::env::temp_dir().join(format!("kit217-expanded-{}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    crate::generate::replace_source(emitted, &directory.join("generated.rs")).unwrap();
    crate::generate::replace_source(
        &format!("extern crate alloc; include!(\"generated.rs\"); fn main() {{ {body} }}"),
        &directory.join("main.rs"),
    )
    .unwrap();
    let core = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/solana_kit_core");
    fs::write(directory.join("Cargo.toml"),format!("[package]\nname=\"expanded_consumer\"\nversion=\"0.0.0\"\nedition=\"2024\"\n[workspace]\n[[bin]]\nname=\"consumer\"\npath=\"main.rs\"\n[dependencies]\nsolana_kit_core={{path={core:?},default-features=false}}\n")).unwrap();
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
