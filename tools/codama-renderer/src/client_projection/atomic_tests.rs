use super::*;

fn assert_preserved(label: &str, item: Value) {
    let destination = std::env::temp_dir().join(format!(
        "token-2022-projection-{label}-{}",
        std::process::id()
    ));
    fs::create_dir_all(&destination).unwrap();
    let existing = destination.join("mod.rs");
    let sentinel = b"existing output must survive malformed input";
    fs::write(&existing, sentinel).unwrap();

    assert!(project(&destination, &[item]).is_err());
    assert_eq!(fs::read(&existing).unwrap(), sentinel);
    assert_eq!(fs::read_dir(&destination).unwrap().count(), 1);
}

#[test]
fn malformed_idl_preserves_existing_output() {
    assert_preserved("malformed", serde_json::json!({"name":"broken"}));
}

#[test]
fn unsupported_account_policy_preserves_existing_output() {
    let root: Value =
        serde_json::from_slice(include_bytes!("../../fixtures/token-2022-original.json")).unwrap();
    let mut item = root["program"]["instructions"][0].clone();
    item["optionalAccountStrategy"] = serde_json::json!("ambient");
    assert_preserved("unsupported", item);
}
