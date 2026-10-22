//! Explicit ordered account privileges and remaining-list policies.
use super::{Remaining, Role, Signer, parse::keys};
use crate::schema::{Error, identifier};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn parse(nodes: &[Value]) -> Result<Vec<Role>, Error> {
    let mut names = BTreeSet::new();
    nodes
        .iter()
        .map(|node| {
            let role = role(node)?;
            if !names.insert(role.name.clone()) {
                return Err("duplicate role name".into());
            }

            Ok(role)
        })
        .collect()
}
fn role(node: &Value) -> Result<Role, Error> {
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
        return Err("unexpected account role kind".into());
    }

    if let Some(default) = node.get("defaultValue") {
        default_value(default)?;
    }

    Ok(Role {
        name: identifier(node["name"].as_str().ok_or("missing role name")?)?,
        signer: signer(&node["isSigner"])?,
        link: node.get("accountLink").map(account_link).transpose()?,
        writable: node["isWritable"]
            .as_bool()
            .ok_or("missing writable policy")?,
        optional: node["isOptional"]
            .as_bool()
            .ok_or("missing optional policy")?,
    })
}
fn signer(node: &Value) -> Result<Signer, Error> {
    match node {
        Value::Bool(false) => Ok(Signer::Never),
        Value::Bool(true) => Ok(Signer::Required),

        Value::String(value) if value == "either" => Ok(Signer::Either),
        _ => Err("unsupported signer policy".into()),
    }
}
fn account_link(node: &Value) -> Result<String, Error> {
    keys(node, &["kind", "name"])?;

    if node["kind"] != "accountLinkNode" {
        return Err("unsupported account link kind".into());
    }

    super::parse::name(node)
}
fn default_value(node: &Value) -> Result<(), Error> {
    match node["kind"].as_str() {
        Some("publicKeyValueNode") => {
            keys(node, &["kind", "publicKey"])?;
            let _: solana_kit_core::Address = node["publicKey"]
                .as_str()
                .ok_or("missing default address")?
                .parse()?;
        }
        Some("identityValueNode") => keys(node, &["kind"])?,
        _ => {
            return Err(
                "unsupported account default; caller must supply explicit addresses".into(),
            );
        }
    }

    Ok(())
}
pub(super) fn remaining(node: &Value) -> Result<Option<Remaining>, Error> {
    let Some(nodes) = node.get("remainingAccounts") else {
        return Ok(None);
    };
    let nodes = nodes.as_array().ok_or("remaining account list required")?;

    if nodes.is_empty() {
        return Ok(None);
    }

    if nodes.len() != 1 {
        return Err("only one remaining list is supported".into());
    }

    remaining_policy(&nodes[0]).map(Some)
}
/// Resolve one caller list with explicit privilege defaults.
fn remaining_policy(node: &Value) -> Result<Remaining, Error> {
    keys(
        node,
        &[
            "kind",
            "value",
            "display",
            "isOptional",
            "isSigner",
            "isWritable",
        ],
    )?;
    keys(&node["value"], &["kind", "name"])?;

    if node["kind"] != "instructionRemainingAccountsNode"
        || node["value"]["kind"] != "argumentValueNode"
    {
        return Err("unsupported remaining account binding".into());
    }
    identifier(
        node["value"]["name"]
            .as_str()
            .ok_or("missing remaining input name")?,
    )?;

    for key in ["isOptional", "isSigner", "isWritable"] {
        if node.get(key).is_some_and(|value| !value.is_boolean()) {
            return Err("invalid remaining account policy".into());
        }
    }

    Ok(Remaining {
        signer: node["isSigner"].as_bool().unwrap_or(false),
        writable: node["isWritable"].as_bool().unwrap_or(false),
    })
}
