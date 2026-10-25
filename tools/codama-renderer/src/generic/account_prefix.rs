//! Explicit framing for private source tables with separate account or event headers.
use super::{Policy, codama_program};
use crate::schema::Error;
use serde_json::{Value, json};

/// Declares each separate header before using the ordinary Codama frontend.
pub(super) fn render(root: &Value, policy: &Policy<'_>) -> Result<String, Error> {
    render_table(root, policy, "accounts")
}

/// Events can require separate framing while accounts already declare their own bytes.
pub(super) fn render_events(root: &Value, policy: &Policy<'_>) -> Result<String, Error> {
    render_table(root, policy, "events")
}

/// Only the selected private table receives hidden bytes; public Codama semantics stay intact.
fn render_table(root: &Value, policy: &Policy<'_>, table: &str) -> Result<String, Error> {
    let mut lowered = root.clone();
    let records = lowered["program"][table]
        .as_array_mut()
        .ok_or("framed source table required")?;

    for record in records {
        prefix(record)?;
    }

    codama_program::render(&lowered, policy)
}

/// Hidden bytes preserve the public value and participate in both codec directions.
fn prefix(record: &mut Value) -> Result<(), Error> {
    let bytes = codama_program::event_header(record)?;
    let field = record["data"]["fields"]
        .as_array_mut()
        .and_then(|fields| fields.first_mut())
        .ok_or("framed record needs a first field")?;
    let wire = field.get_mut("type").ok_or("record field type required")?;

    if wire["kind"] == "hiddenPrefixTypeNode" {
        return Err("record header is already framed".into());
    }

    let constants = bytes
        .into_iter()
        .map(|byte| {
            json!({"kind":"constantValueNode",
            "type":{"kind":"numberTypeNode","format":"u8","endian":"le"},
            "value":{"kind":"numberValueNode","number":byte}})
        })
        .collect::<Vec<_>>();
    *wire = json!({"kind":"hiddenPrefixTypeNode","type":wire.take(),"prefix":constants});

    Ok(())
}

#[cfg(all(test, feature = "verification"))]
mod tests {
    use super::*;

    #[test]
    fn source_tables_require_one_explicit_nonempty_header() {
        let root = crate::bubblegum::source().unwrap();
        let original = root["program"]["accounts"][0].clone();
        let mut framed = original.clone();
        prefix(&mut framed).unwrap();

        assert_eq!(
            framed["data"]["fields"][0]["type"]["prefix"]
                .as_array()
                .unwrap()
                .len(),
            8
        );
        assert_eq!(
            original["data"]["fields"][0]["type"]["kind"],
            "publicKeyTypeNode"
        );
        assert!(prefix(&mut framed).is_err());

        let mut missing = original.clone();
        missing["discriminators"] = json!([]);
        assert!(prefix(&mut missing).is_err());

        let mut empty = original;
        empty["data"]["fields"] = json!([]);
        assert!(prefix(&mut empty).is_err());
    }

    #[test]
    fn event_domain_and_variant_tag_form_one_explicit_prefix() {
        const TRANSFER_EVENT_HEADER: [u64; 9] = [228, 69, 165, 46, 81, 203, 154, 29, 3];
        let root = crate::program_preparation::subscriptions().unwrap();
        let mut event = root["program"]["events"][0].clone();
        prefix(&mut event).unwrap();
        let bytes = event["data"]["fields"][0]["type"]["prefix"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value["value"]["number"].as_u64().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(bytes, TRANSFER_EVENT_HEADER);
        assert!(prefix(&mut event).is_err());
    }
}
