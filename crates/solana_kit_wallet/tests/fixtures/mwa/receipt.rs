//! Reproduces the bounded declaration receipt from actual immutable source bytes.

use std::{collections::BTreeSet, env, fs, path::Path, process::Command};

const PACKAGE: &str = "solana_kit_mobile_wallet_adapter_protocol";
const EXPECTED_IDS: usize = 179;
const SOURCE_COLUMN: usize = 2;
const HASH_COLUMN: usize = 12;

fn digest(path: &Path) -> String {
    let output = Command::new("shasum")
        .args(["-a", "256"])
        .arg(path)
        .output()
        .unwrap();
    assert!(output.status.success(), "source digest failed");
    String::from_utf8(output.stdout)
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .into()
}

fn contract(family: &str) -> &'static str {
    match family {
        "association_keypair" => "mwa::AssociationKey;PublicPoint",
        "association_port" => "mwa::AssociationPort",
        "association_uri" => "mwa::association_uri;parse_association_uri",
        "constants" => "mwa;typed fixed-size arrays and integers",
        "crypto" => "mwa::AssociationKey;AgreementKey;PublicPoint;encrypt_message;decrypt_message",
        "ecdh_keypair" => "mwa::AgreementKey",
        "encrypted_message" => "mwa::encrypt_message;decrypt_message;Session",
        "hello_req" => "mwa::AgreementKey::hello_request;verify_hello_request",
        "hello_rsp" => "mwa::AgreementKey::accept_hello",
        "json_rpc_message" => "mwa::Request;Response;Session",
        "jws" => "mwa::AssociationKey::jws;PublicPoint::verify_jws",
        "protocol_error" => "mwa::ProtocolError",
        "reflector_id" => "mwa::ReflectorId",
        "sequence_number" => "mwa::Session;u32::to_be_bytes",
        "session_properties" => "mwa::SessionProperties::from_json;Session::properties",
        "siws_message" => "mwa::SignInPayload::message;SignInResult::verify",
        "types" => "mwa public application types",
        "wallet_proxy" => "mwa::Request::new;map_capabilities;MobileWallet",
        _ => panic!("unmapped defining source family"),
    }
}

fn main() {
    let arguments: Vec<_> = env::args().skip(1).collect();
    assert_eq!(
        arguments.len(),
        3,
        "expected catalog, immutable root, and retained license"
    );
    let catalog = fs::read_to_string(&arguments[0]).unwrap();
    let root = Path::new(&arguments[1]);
    let license = root.join("packages").join(PACKAGE).join("LICENSE");
    assert_eq!(
        fs::read(license).unwrap(),
        fs::read(&arguments[2]).unwrap(),
        "MIT license changed"
    );
    let mut ids = BTreeSet::new();
    let mut sources = BTreeSet::new();

    println!("id\tstatus\trust_contract\tevidence\tsha256\tprovenance");

    for line in catalog.lines().skip(1) {
        let columns: Vec<_> = line.split('\t').collect();

        if columns.get(1) != Some(&PACKAGE) {
            continue;
        }

        assert_eq!(columns.len(), 14, "catalog shape changed");
        assert!(ids.insert(columns[0]), "duplicate canonical ID");
        let source = columns[SOURCE_COLUMN];
        assert_eq!(
            digest(&root.join(source)),
            columns[HASH_COLUMN],
            "source hash changed"
        );
        sources.insert(source);
        let family = source.rsplit('/').next().unwrap().trim_end_matches(".dart");
        let contract = contract(family);
        println!(
            "{}\tpartial\t{contract}\ttests/mwa.rs;fixtures/mwa/oracle.dart\t{}\t{}",
            columns[0], columns[HASH_COLUMN], columns[13]
        );
    }

    assert_eq!(ids.len(), EXPECTED_IDS, "canonical ID count changed");
    eprintln!(
        "verified {} unique canonical IDs and {} defining source files; every row remains partial",
        ids.len(),
        sources.len()
    );
}
