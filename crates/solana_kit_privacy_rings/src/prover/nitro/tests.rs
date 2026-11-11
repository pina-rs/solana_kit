use coset::{CoseSign1Builder, HeaderBuilder, TaggedCborSerializable, iana};
use p384::ecdsa::SigningKey;
use p384::ecdsa::signature::Signer;
use serde_json::Value;
use sha2::{Digest, Sha256};

use super::*;

const ROOT_SHA256: &str = "641a0321a3e244efe456463195d606317ed7cdcc3c1756e09893f3c68f79bb5b";

#[test]
fn the_embedded_root_is_the_aws_nitro_g1_certificate() {
    let digest = Sha256::digest(AWS_NITRO_ROOT);
    assert_eq!(crate::prover::hex_encode(&digest), ROOT_SHA256);
}

#[test]
fn the_live_nitro_quote_matches_its_policy() {
    let body: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/nitro-live-attestation.json"
    ))
    .unwrap();
    let policy_body: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/nitro-live-policy.json"
    ))
    .unwrap();
    let policy = policy_from(&policy_body["deployment"]);
    let nonce = hex_array::<32>(body["nonce"].as_str().unwrap());
    let now = body["captured_at"].as_u64().unwrap();
    let verified = verify_attestation(&body, &nonce, &policy, now).expect("live quote");
    assert_eq!(verified.image_id, policy.measurements[0].pcr0);

    let mut wrong = nonce;
    wrong[0] ^= 1;
    assert!(verify_attestation(&body, &wrong, &policy, now).is_err());

    let mut pinned = policy.clone();
    pinned.hpke_public_key = Some([9; 32]);
    assert!(verify_attestation(&body, &nonce, &pinned, now).is_err());
    let mut gpu = body.clone();
    gpu["attestation"]["gpu"] = Value::from("token");
    assert!(verify_attestation(&gpu, &nonce, &policy, now).is_err());
    let mut platform = body.clone();
    platform["attestation"]["platform"] = Value::from("other");
    assert!(verify_attestation(&platform, &nonce, &policy, now).is_err());
    assert!(verify_attestation(&Value::from("nope"), &nonce, &policy, now).is_err());
}

#[test]
fn upstream_cbor_cases_match_the_test_root() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/nitro-cbor-cases.json"
    ))
    .unwrap();
    let now = fixture["now"].as_u64().unwrap();
    let root = hex_bytes(fixture["root"].as_str().unwrap());
    let key = signing_key(fixture["leaf_key"].as_str().unwrap());
    let nonce = hex_array::<32>(fixture["nonce"].as_str().unwrap());
    let hpke = hex_array::<32>(fixture["hpke_public_key"].as_str().unwrap());
    let policy = policy_from(&fixture["policy"]);

    for case in fixture["cases"].as_array().unwrap() {
        let document = case_document(case, &key);
        let result = verify_nitro_with(&document, &hpke, &nonce, &policy, now, &root);

        if case["expect"] == "accept" {
            assert!(result.is_ok(), "{}", case["name"]);
        } else {
            assert!(result.is_err(), "{}", case["name"]);
        }
    }

    let first = &fixture["cases"][0];
    let document = case_document(first, &key);
    assert!(verify_nitro(&document, &hpke, &nonce, &policy, now).is_err());

    let mut empty = policy.clone();
    empty.measurements.clear();
    assert!(verify_nitro_with(&document, &hpke, &nonce, &empty, now, &root).is_err());
    assert!(verify_nitro_with(&document, &[0; 32], &nonce, &policy, now, &root).is_err());
}

fn policy_from(value: &Value) -> NitroPolicy {
    let measurements = value["measurements"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| NitroMeasurement {
            pcr0: hex_array(item["pcr0"].as_str().unwrap()),
            pcr1: hex_array(item["pcr1"].as_str().unwrap()),
            pcr2: hex_array(item["pcr2"].as_str().unwrap()),
        })
        .collect();
    NitroPolicy {
        measurements,
        hpke_public_key: None,
    }
}

fn case_document(case: &Value, key: &SigningKey) -> Vec<u8> {
    if let Some(cose) = case["cose"].as_str() {
        return hex_bytes(cose);
    }

    let payload = hex_bytes(case["payload"].as_str().unwrap());
    let protected = HeaderBuilder::new()
        .algorithm(iana::Algorithm::ES384)
        .build();
    CoseSign1Builder::new()
        .protected(protected)
        .payload(payload)
        .create_signature(&[], |tbs| {
            let signature: p384::ecdsa::Signature = key.sign(tbs);
            signature.to_bytes().to_vec()
        })
        .build()
        .to_tagged_vec()
        .expect("cose")
}

fn signing_key(hex: &str) -> SigningKey {
    let bytes = hex_bytes(hex);
    let field: p384::FieldBytes = bytes.as_slice().try_into().expect("leaf key");
    SigningKey::from_bytes(&field).expect("leaf key")
}

fn hex_array<const N: usize>(hex: &str) -> [u8; N] {
    hex_bytes(hex).try_into().expect("width")
}

fn hex_bytes(hex: &str) -> Vec<u8> {
    crate::prover::hex_decode(hex).expect("hex")
}

fn fixture_parts() -> (Value, Vec<(ciborium::Value, ciborium::Value)>, SigningKey) {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/nitro-cbor-cases.json"
    ))
    .unwrap();
    let payload = hex_bytes(fixture["cases"][0]["payload"].as_str().unwrap());
    let ciborium::Value::Map(entries) = ciborium::from_reader(payload.as_slice()).unwrap() else {
        panic!("payload map");
    };
    let key = signing_key(fixture["leaf_key"].as_str().unwrap());
    (fixture, entries, key)
}

fn set_field(
    entries: &mut Vec<(ciborium::Value, ciborium::Value)>,
    name: &str,
    value: ciborium::Value,
) {
    let key = ciborium::Value::Text(name.to_owned());

    for (field, current) in entries.iter_mut() {
        if *field == key {
            *current = value;

            return;
        }
    }

    entries.push((key, value));
}

fn sign_entries(entries: Vec<(ciborium::Value, ciborium::Value)>, key: &SigningKey) -> Vec<u8> {
    let mut payload = Vec::new();
    ciborium::into_writer(&ciborium::Value::Map(entries), &mut payload).unwrap();
    case_signed(&payload, key, iana::Algorithm::ES384)
}

fn case_signed(payload: &[u8], key: &SigningKey, algorithm: iana::Algorithm) -> Vec<u8> {
    let protected = HeaderBuilder::new().algorithm(algorithm).build();
    CoseSign1Builder::new()
        .protected(protected)
        .payload(payload.to_vec())
        .create_signature(&[], |tbs| {
            let signature: p384::ecdsa::Signature = key.sign(tbs);
            signature.to_bytes().to_vec()
        })
        .build()
        .to_tagged_vec()
        .expect("cose")
}

fn rejected(document: &[u8], fixture: &Value) {
    assert_ne!(
        reason(document, fixture, fixture["now"].as_u64().unwrap()),
        "accepted"
    );
}

fn reason(document: &[u8], fixture: &Value, now: u64) -> &'static str {
    let root = hex_bytes(fixture["root"].as_str().unwrap());
    let nonce = hex_array::<32>(fixture["nonce"].as_str().unwrap());
    let hpke = hex_array::<32>(fixture["hpke_public_key"].as_str().unwrap());
    let policy = policy_from(&fixture["policy"]);

    match verify_nitro_with(document, &hpke, &nonce, &policy, now, &root) {
        Err(Error::Attestation(reason)) => reason,
        Err(error) => panic!("{error}"),
        Ok(_) => "accepted",
    }
}

#[test]
fn document_fields_outside_the_profile_are_rejected() {
    let (fixture, base, key) = fixture_parts();
    let mut digest = base.clone();
    set_field(
        &mut digest,
        "digest",
        ciborium::Value::Text("SHA256".to_owned()),
    );
    rejected(&sign_entries(digest, &key), &fixture);
    let mut module = base.clone();
    set_field(
        &mut module,
        "module_id",
        ciborium::Value::Text(String::new()),
    );
    rejected(&sign_entries(module, &key), &fixture);
    let mut stamp = base.clone();

    set_field(
        &mut stamp,
        "timestamp",
        ciborium::Value::Integer((-1i64).into()),
    );
    rejected(&sign_entries(stamp, &key), &fixture);
    let mut typed = base.clone();
    set_field(&mut typed, "module_id", ciborium::Value::Integer(1.into()));
    rejected(&sign_entries(typed, &key), &fixture);
    let mut pcrs = base.clone();
    set_field(&mut pcrs, "pcrs", ciborium::Value::Integer(1.into()));
    rejected(&sign_entries(pcrs, &key), &fixture);
    let mut bundle = base.clone();
    set_field(&mut bundle, "cabundle", ciborium::Value::Integer(1.into()));
    rejected(&sign_entries(bundle, &key), &fixture);
    let mut certificate = base.clone();
    set_field(
        &mut certificate,
        "certificate",
        ciborium::Value::Text("der".to_owned()),
    );
    rejected(&sign_entries(certificate, &key), &fixture);
    let mut optional = base.clone();
    optional.retain(|(name, _)| *name != ciborium::Value::Text("public_key".to_owned()));
    rejected(&sign_entries(optional, &key), &fixture);
}

#[test]
fn document_shape_limits_are_rejected() {
    let (fixture, base, key) = fixture_parts();
    let mut wide = base.clone();

    for index in 0..8 {
        wide.push((
            ciborium::Value::Integer(index.into()),
            ciborium::Value::Null,
        ));
    }

    rejected(&sign_entries(wide, &key), &fixture);
    let mut numbered = base.clone();
    numbered.push((ciborium::Value::Integer(1.into()), ciborium::Value::Null));
    numbered.push((
        ciborium::Value::Text("extra".to_owned()),
        ciborium::Value::Map(vec![(
            ciborium::Value::Text("inner".to_owned()),
            ciborium::Value::Null,
        )]),
    ));

    let extra = sign_entries(numbered, &key);
    let now = fixture["now"].as_u64().unwrap();
    let root = hex_bytes(fixture["root"].as_str().unwrap());
    let nonce = hex_array::<32>(fixture["nonce"].as_str().unwrap());
    let hpke = hex_array::<32>(fixture["hpke_public_key"].as_str().unwrap());
    let policy = policy_from(&fixture["policy"]);
    assert!(verify_nitro_with(&extra, &hpke, &nonce, &policy, now, &root).is_ok());
    let mut duplicate = base.clone();
    duplicate.push((
        ciborium::Value::Text("module_id".to_owned()),
        ciborium::Value::Null,
    ));
    rejected(&sign_entries(duplicate, &key), &fixture);
    let mut bytes_key = base.clone();
    bytes_key.push((ciborium::Value::Bytes(vec![1]), ciborium::Value::Null));
    rejected(&sign_entries(bytes_key, &key), &fixture);
    let mut nested = base.clone();
    let repeated = ciborium::Value::Text("again".to_owned());
    nested.push((
        ciborium::Value::Text("extra".to_owned()),
        ciborium::Value::Map(vec![
            (repeated.clone(), ciborium::Value::Null),
            (repeated, ciborium::Value::Null),
        ]),
    ));
    rejected(&sign_entries(nested, &key), &fixture);
}

#[test]
fn pcr_and_bundle_limits_are_rejected() {
    let (fixture, base, key) = fixture_parts();
    let mut many_pcrs = base.clone();
    let pcr = ciborium::Value::Bytes(vec![1; 48]);
    let entries = (0..33)
        .map(|index| (ciborium::Value::Integer(index.into()), pcr.clone()))
        .collect();
    set_field(&mut many_pcrs, "pcrs", ciborium::Value::Map(entries));
    rejected(&sign_entries(many_pcrs, &key), &fixture);
    let mut named_pcr = base.clone();
    set_field(
        &mut named_pcr,
        "pcrs",
        ciborium::Value::Map(vec![(ciborium::Value::Text("0".to_owned()), pcr.clone())]),
    );
    rejected(&sign_entries(named_pcr, &key), &fixture);
    let mut duplicate_pcr = base.clone();
    set_field(
        &mut duplicate_pcr,
        "pcrs",
        ciborium::Value::Map(vec![
            (ciborium::Value::Integer(0.into()), pcr.clone()),
            (ciborium::Value::Integer(0.into()), pcr),
        ]),
    );
    rejected(&sign_entries(duplicate_pcr, &key), &fixture);
    let bundle_root = cabundle_root(&base);
    let mut long_bundle = base.clone();
    let der = ciborium::Value::Bytes(bundle_root);
    set_field(
        &mut long_bundle,
        "cabundle",
        ciborium::Value::Array(vec![der; 9]),
    );
    rejected(&sign_entries(long_bundle, &key), &fixture);
    rejected(&case_signed(&[], &key, iana::Algorithm::ES384), &fixture);
}

fn certificate_der(entries: &[(ciborium::Value, ciborium::Value)]) -> Vec<u8> {
    let key = ciborium::Value::Text("certificate".to_owned());
    let Some((_, ciborium::Value::Bytes(bytes))) = entries.iter().find(|(field, _)| *field == key)
    else {
        panic!("certificate");
    };
    bytes.clone()
}

fn cabundle_root(entries: &[(ciborium::Value, ciborium::Value)]) -> Vec<u8> {
    let key = ciborium::Value::Text("cabundle".to_owned());
    let Some((_, ciborium::Value::Array(items))) = entries.iter().find(|(field, _)| *field == key)
    else {
        panic!("cabundle");
    };
    let ciborium::Value::Bytes(bytes) = &items[0] else {
        panic!("root")
    };
    bytes.clone()
}

#[test]
fn certificate_bytes_outside_the_profile_are_rejected() {
    let (fixture, base, key) = fixture_parts();
    let document = sign_entries(base.clone(), &key);
    assert_eq!(reason(&document, &fixture, 0), "validity");

    let mut untrusted = base.clone();
    let leaf = certificate_der(&base);
    let root_der = cabundle_root(&base);
    set_field(
        &mut untrusted,
        "cabundle",
        ciborium::Value::Array(vec![
            ciborium::Value::Bytes(root_der),
            ciborium::Value::Bytes(leaf),
        ]),
    );
    assert_eq!(
        reason(
            &sign_entries(untrusted, &key),
            &fixture,
            fixture["now"].as_u64().unwrap()
        ),
        "ca"
    );
    let signed = |entries| sign_entries(entries, &key);
    let now = fixture["now"].as_u64().unwrap();
    assert_eq!(
        reason(
            &signed(patched_parent(
                &base,
                "551d0f0101ff040403020204",
                "551d0f0101ff040403020200"
            )),
            &fixture,
            now
        ),
        "key usage"
    );
    assert_eq!(
        reason(
            &signed(patched(&base, "certificate", "03020780", "03020700")),
            &fixture,
            now
        ),
        "key usage"
    );
    assert_eq!(
        reason(
            &signed(patched(&base, "certificate", "551d13", "551d14")),
            &fixture,
            now
        ),
        "extension"
    );
    assert_eq!(
        reason(
            &signed(patched(
                &base,
                "certificate",
                "2a8648ce3d040303",
                "2a8648ce3d040302"
            )),
            &fixture,
            now
        ),
        "certificate signature"
    );
    assert_eq!(
        reason(
            &signed(patched_parent(&base, "2b81040022", "2b81040021")),
            &fixture,
            now
        ),
        "p384"
    );
}

#[test]
fn a_broken_issuer_or_report_is_rejected() {
    let (fixture, base, key) = fixture_parts();
    let root = hex_bytes(fixture["root"].as_str().unwrap());
    let nonce = hex_array::<32>(fixture["nonce"].as_str().unwrap());
    let hpke = hex_array::<32>(fixture["hpke_public_key"].as_str().unwrap());
    let policy = policy_from(&fixture["policy"]);
    let mut issued = base.clone();
    let root_der = cabundle_root(&base);
    set_field(
        &mut issued,
        "cabundle",
        ciborium::Value::Array(vec![ciborium::Value::Bytes(root_der); 2]),
    );
    rejected(&sign_entries(issued, &key), &fixture);
    let mut debug = base.clone();
    let zeros = ciborium::Value::Bytes(vec![0; 48]);
    set_field(&mut debug, "pcrs", pcrs_with_zero(&base, zeros));
    rejected(&sign_entries(debug, &key), &fixture);
    let mut report = base.clone();
    set_field(
        &mut report,
        "user_data",
        ciborium::Value::Bytes(vec![0; 64]),
    );
    let signed = sign_entries(report, &key);
    let now = fixture["now"].as_u64().unwrap();
    assert!(verify_nitro_with(&signed, &hpke, &nonce, &policy, now, &root).is_err());
}

fn patched_parent(
    entries: &[(ciborium::Value, ciborium::Value)],
    from: &str,
    to: &str,
) -> Vec<(ciborium::Value, ciborium::Value)> {
    let from = hex_bytes(from);
    let to = hex_bytes(to);
    let mut entries = entries.to_vec();
    let key = ciborium::Value::Text("cabundle".to_owned());

    for (name, value) in &mut entries {
        if *name == key
            && let ciborium::Value::Array(items) = value
            && let Some(parent) = items.get_mut(1)
        {
            *parent = replace_bytes(parent.clone(), &from, &to);
        }
    }

    entries
}

fn patched(
    entries: &[(ciborium::Value, ciborium::Value)],
    field: &str,
    from: &str,
    to: &str,
) -> Vec<(ciborium::Value, ciborium::Value)> {
    let from = hex_bytes(from);
    let to = hex_bytes(to);
    let mut entries = entries.to_vec();
    let key = ciborium::Value::Text(field.to_owned());

    for (name, value) in &mut entries {
        if *name == key {
            *value = replace_bytes(value.clone(), &from, &to);
        }
    }

    entries
}

fn replace_bytes(value: ciborium::Value, from: &[u8], to: &[u8]) -> ciborium::Value {
    match value {
        ciborium::Value::Bytes(bytes) => ciborium::Value::Bytes(swap_once(&bytes, from, to)),
        ciborium::Value::Array(items) => ciborium::Value::Array(
            items
                .into_iter()
                .map(|item| replace_bytes(item, from, to))
                .collect(),
        ),
        other => other,
    }
}

fn swap_once(bytes: &[u8], from: &[u8], to: &[u8]) -> Vec<u8> {
    let mut bytes = bytes.to_vec();
    let Some(index) = bytes.windows(from.len()).position(|window| window == from) else {
        panic!("pattern {} missing", hex_of(from));
    };
    bytes[index..index + to.len()].copy_from_slice(to);
    bytes
}

fn hex_of(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn pcrs_with_zero(
    entries: &[(ciborium::Value, ciborium::Value)],
    zeros: ciborium::Value,
) -> ciborium::Value {
    let key = ciborium::Value::Text("pcrs".to_owned());
    let Some((_, ciborium::Value::Map(pcrs))) = entries.iter().find(|(field, _)| *field == key)
    else {
        panic!("pcrs");
    };
    let mut pcrs = pcrs.clone();

    for (index, value) in &mut pcrs {
        if *index == ciborium::Value::Integer(0.into()) {
            *value = zeros;
            break;
        }
    }

    ciborium::Value::Map(pcrs)
}

#[test]
fn cose_framing_outside_the_profile_is_rejected() {
    let (fixture, base, key) = fixture_parts();
    let mut payload = Vec::new();
    ciborium::into_writer(&ciborium::Value::Map(base.clone()), &mut payload).unwrap();
    rejected(
        &case_signed(&payload, &key, iana::Algorithm::ES256),
        &fixture,
    );
    rejected(&case_signed(&[], &key, iana::Algorithm::ES384), &fixture);
    let protected = HeaderBuilder::new().build();
    let empty = CoseSign1Builder::new()
        .protected(protected)
        .payload(payload)
        .create_signature(&[], |tbs| {
            let signature: p384::ecdsa::Signature = key.sign(tbs);
            signature.to_bytes().to_vec()
        })
        .build()
        .to_tagged_vec()
        .expect("cose");
    rejected(&empty, &fixture);

    let mut array = Vec::new();
    ciborium::into_writer(&ciborium::Value::Array(vec![]), &mut array).unwrap();
    rejected(&case_signed(&array, &key, iana::Algorithm::ES384), &fixture);
}
