//! Complete normalized catalog and fail-closed malformed-model controls.
use super::*;

#[test]
fn every_instruction_account_tag_and_error_is_present() {
    let root = source().unwrap();
    let program = &root["program"];
    let tags = [0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 228];
    assert_eq!(program["instructions"].as_array().unwrap().len(), 12);
    assert_eq!(program["accounts"].as_array().unwrap().len(), 3);
    assert_eq!(program["errors"].as_array().unwrap().len(), 12);
    assert_eq!(
        program["definedTypes"][0]["type"]["variants"]
            .as_array()
            .unwrap()
            .len(),
        26
    );

    for (node, tag) in program["instructions"].as_array().unwrap().iter().zip(tags) {
        assert_eq!(node["arguments"][0]["defaultValue"]["number"], tag);
        assert!(
            node["accounts"]
                .as_array()
                .unwrap()
                .iter()
                .all(|role| role["isOptional"] == false)
        );
    }

    let rendered = render(&root).unwrap();

    for name in [
        "CredentialCodec",
        "SchemaCodec",
        "AttestationCodec",
        "CloseAttestationEventCodec",
        "CreateSchemaCodec",
        "CreateTokenizedAttestationCodec",
        "AttestationServiceError",
    ] {
        assert!(rendered.contains(name), "{name}");
    }
}

#[test]
fn unknown_roles_fields_types_headers_and_programs_are_rejected() {
    let source = source().unwrap();
    let mut root = source.clone();
    root["program"]["instructions"][0]["accounts"][0]["isSigner"] = json!("guessed");
    assert!(render(&root).is_err());
    root = source.clone();
    root["program"]["instructions"][0]["unexpectedConstraint"] = json!(true);
    assert!(render(&root).is_err());
    root = source.clone();

    root["program"]["accounts"][0]["data"]["fields"][0]["type"] = json!({"kind":"unknownWire"});
    assert!(render(&root).is_err());
    root = source.clone();
    root["program"]["accounts"][1]["discriminators"] =
        root["program"]["accounts"][0]["discriminators"].clone();
    assert!(render(&root).is_err());
    root = source.clone();
    root["program"]["instructions"][1]["arguments"][0]["defaultValue"]["number"] = json!(0);

    assert!(render(&root).is_err());
    root = source.clone();
    root["program"]["publicKey"] = json!("11111111111111111111111111111111");
    assert!(render(&root).is_err());
}

#[test]
fn invalid_model_does_not_replace_existing_output() {
    let destination =
        std::env::temp_dir().join(format!("attestation-invalid-{}.rs", std::process::id()));
    std::fs::write(&destination, "preserved-output").unwrap();
    let mut source = source().unwrap();
    source["program"]["instructions"][0]["arguments"][1]["type"] = json!({"kind":"unknown"});
    let result =
        render(&source).and_then(|output| crate::generate::replace_source(&output, &destination));
    assert!(result.is_err());

    assert_eq!(
        std::fs::read_to_string(&destination).unwrap(),
        "preserved-output"
    );
    assert!(!destination.with_extension("rs.pending").exists());
    std::fs::remove_file(destination).unwrap();
}

#[test]
fn counted_strings_use_the_existing_owned_string_adapter_and_reject_unframed_text() {
    let root = source().unwrap();
    let rendered = render(&root).unwrap();
    assert!(rendered.contains("OwnedString(ByteSizePrefix"));
    let mut unframed = root;
    unframed["program"]["instructions"][1]["arguments"][4]["type"]["item"] =
        json!({"kind":"stringTypeNode","encoding":"utf8"});
    assert!(render(&unframed).is_err());
}
