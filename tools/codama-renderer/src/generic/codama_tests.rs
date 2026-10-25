//! Complete original source admission preserves fields, metadata and old outputs on rejection.
use super::*;
use serde_json::json;
const INVALID_VALUE: u8 = 7;

fn source() -> Value {
    crate::program_preparation::subscriptions().unwrap()
}

fn policy() -> Policy<'static> {
    Policy {
        name: "subscriptions",
        address: "De1egAFMkMWZSN5rYXRj9CAdheBamobVNubTsi9avR44"
            .parse()
            .unwrap(),
    }
}

#[test]
fn complete_program_preserves_all_account_event_error_pda_and_default_contracts() {
    let rendered = render_complete_program(&source(), &policy()).unwrap();
    assert_eq!(
        rendered,
        render_complete_program(&source(), &policy()).unwrap()
    );
    assert_eq!(rendered.matches("pub fn find_").count(), 12);

    for collection in ["definedTypes", "accounts", "events"] {
        for node in source()["program"][collection].as_array().unwrap() {
            assert!(rendered.contains(&format!("pub struct {}Codec", parse::name(node).unwrap())));
        }
    }

    assert!(rendered.contains("pub subscription_authority:Option<Address>"));
    assert!(rendered.contains("pub system_program:Option<Address>"));
    assert!(rendered.contains("find_subscription_authority_with_program(program,_role_0,_role_2)"));
    assert!(rendered.contains("let _role_0=self.owner;"));
    assert!(rendered.contains("let _role_2=self.token_mint;"));
    assert!(rendered.contains("Utf8Config::default()"));
    assert!(rendered.contains("[228, 69, 165, 46, 81, 203, 154, 29, 3]"));
    assert!(rendered.contains("SubscriptionsError"));
    let mut unicode = source();
    unicode["program"]["pdas"][0]["seeds"][0]["value"]["string"] = json!("é");

    assert!(
        render_complete_program(&unicode, &policy())
            .unwrap()
            .contains("&[195, 169]")
    );
}

fn reject(root: &Value, label: &str) {
    let directory =
        std::env::temp_dir().join(format!("kit-complete-{label}-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    let output = directory.join("generated.rs");
    let pending = output.with_extension("rs.pending");
    std::fs::write(&output, b"original output").unwrap();
    std::fs::write(&pending, b"original pending").unwrap();
    let result = render_complete_program(root, &policy())
        .and_then(|source| crate::generate::replace_source(&source, &output));

    assert!(result.is_err(), "accepted {label}");
    assert_eq!(std::fs::read(output).unwrap(), b"original output");
    assert_eq!(std::fs::read(pending).unwrap(), b"original pending");
}

#[test]
fn complete_program_malformed_fields_events_defaults_counts_and_collisions_preserve_output() {
    scalar_mutations();
    collection_boundaries();
}

fn scalar_mutations() {
    for (index, (pointer, value)) in [
        (
            "/program/definedTypes/3/type/fields/6/type/type/encoding",
            json!("ascii"),
        ),
        (
            "/program/definedTypes/9/type/variants/1/discriminator",
            json!(INVALID_VALUE),
        ),
        (
            "/program/events/0/discriminators/1/offset",
            json!(INVALID_VALUE),
        ),
        (
            "/program/events/0/discriminators/0/constant/value/data",
            json!("gg"),
        ),
        ("/program/accounts/0/pda/name", json!("missing")),
        (
            "/program/instructions/0/accounts/1/defaultValue/seeds/0/value/name",
            json!("missing"),
        ),
        (
            "/program/instructions/0/accounts/1/defaultValue/seeds/0/value/name",
            json!("payer"),
        ),
        (
            "/program/instructions/0/optionalAccountStrategy",
            json!(false),
        ),
        (
            "/program/definedTypes/0/name",
            json!("InitSubscriptionAuthorityAccounts"),
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let mut root = source();
        *root.pointer_mut(pointer).unwrap() = value;
        reject(&root, &index.to_string());
    }
}

fn collection_boundaries() {
    let mut root = source();
    root["program"]["events"][1]["discriminators"] =
        root["program"]["events"][0]["discriminators"].clone();
    reject(&root, "event-duplicate");
    let mut root = source();
    let template = root["program"]["instructions"][0].clone();
    root["program"]["instructions"] = Value::Array(vec![template; 257]);
    assert_eq!(
        render_complete_program(&root, &policy())
            .unwrap_err()
            .to_string(),
        "instruction budget exceeded"
    );

    reject(&root, "instruction-budget");
}
