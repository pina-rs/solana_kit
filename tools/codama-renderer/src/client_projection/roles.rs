//! Closed source role facts, including optional omission and caller authorities.
use super::{Result, text};
use serde_json::Value;

pub(super) struct Role {
    pub name: String,
    pub writable: bool,
    pub signer: &'static str,
    pub optional: bool,

    pub default: Option<String>,
}

pub(super) fn parse(node: &Value) -> Result<Role> {
    keys(
        node,
        &[
            "kind",
            "name",
            "docs",
            "display",
            "isWritable",
            "isSigner",
            "isOptional",
            "defaultValue",
            "accountLink",
        ],
    )?;

    if node["kind"] != "instructionAccountNode" {
        return Err("unknown fixed-account node".into());
    }

    Ok(Role {
        name: super::snake(text(node, "name")?),
        writable: node["isWritable"].as_bool().ok_or("writability required")?,
        signer: signer(&node["isSigner"])?,
        optional: node["isOptional"].as_bool().ok_or("optionality required")?,
        default: default(node)?,
    })
}

fn signer(node: &Value) -> Result<&'static str> {
    Ok(match node {
        Value::Bool(false) => "Never",
        Value::Bool(true) => "Required",

        Value::String(value) if value == "either" => "Either",
        _ => return Err("unknown signer contract".into()),
    })
}

fn default(node: &Value) -> Result<Option<String>> {
    Ok(match node.get("defaultValue") {
        None => None,

        Some(value) if value["kind"] == "identityValueNode" => None,

        Some(value) if value["kind"] == "publicKeyValueNode" => {
            let address = text(value, "publicKey")?;
            let _: solana_kit_core::Address = address.parse()?;
            Some(address.to_owned())
        }
        _ => return Err("unsupported source default".into()),
    })
}

/// Remaining lists default to readonly nonsigning roles exactly as in the source.
pub(super) fn remaining(node: &Value) -> Result<Role> {
    keys(
        node,
        &[
            "kind",
            "value",
            "display",
            "docs",
            "isOptional",
            "isSigner",
            "isWritable",
        ],
    )?;

    if node["kind"] != "instructionRemainingAccountsNode"
        || node["value"]["kind"] != "argumentValueNode"
    {
        return Err("unknown remaining-account contract".into());
    }

    for property in ["isOptional", "isSigner", "isWritable"] {
        if node.get(property).is_some_and(|value| !value.is_boolean()) {
            return Err("invalid remaining-account privilege".into());
        }
    }

    Ok(Role {
        name: super::snake(text(&node["value"], "name")?),
        writable: node["isWritable"].as_bool().unwrap_or(false),
        signer: if node["isSigner"].as_bool().unwrap_or(false) {
            "Required"
        } else {
            "Never"
        },

        optional: node["isOptional"].as_bool().unwrap_or(false),
        default: None,
    })
}

fn keys(node: &Value, allowed: &[&str]) -> Result<()> {
    let object = node.as_object().ok_or("account object required")?;

    if object.keys().any(|key| !allowed.contains(&key.as_str())) {
        return Err("unsupported account property".into());
    }

    Ok(())
}
