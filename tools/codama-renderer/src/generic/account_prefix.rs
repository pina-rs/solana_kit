//! Explicit framing adapter for private source tables whose account headers are separate.
use super::{Policy, codama_program};
use crate::schema::Error;
use serde_json::{Value, json};

/// Declares each separate header before using the ordinary Codama frontend.
pub(super) fn render(root: &Value, policy: &Policy<'_>) -> Result<String, Error> {
    let mut lowered = root.clone();
    let accounts = lowered["program"]["accounts"]
        .as_array_mut()
        .ok_or("account table required")?;

    for account in accounts {
        prefix(account)?;
    }

    codama_program::render(&lowered, policy)
}

/// Hidden bytes preserve the public value and participate in both codec directions.
fn prefix(account: &mut Value) -> Result<(), Error> {
    let bytes = codama_program::event_header(account)?;
    let field = account["data"]["fields"]
        .as_array_mut()
        .and_then(|fields| fields.first_mut())
        .ok_or("framed account needs a first field")?;
    let wire = field.get_mut("type").ok_or("account field type required")?;

    if wire["kind"] == "hiddenPrefixTypeNode" {
        return Err("account header is already framed".into());
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
}
