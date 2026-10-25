//! Resolves display-only provided context without executing resolver code.
use super::{
    Account, Shape,
    parse::{array, keys, name},
};
use crate::schema::{Error, identifier};
use serde_json::Value;
use std::collections::BTreeSet;

pub(super) fn validate(node: &Value, accounts: &[Account]) -> Result<(), Error> {
    let Some(provides) = node.get("provides") else {
        return Ok(());
    };
    let mut names = BTreeSet::new();

    for provided in provides.as_array().ok_or("provided list required")? {
        let name = provided_name(provided)?;

        if !names.insert(name) {
            return Err("duplicate provided name".into());
        }

        dependency(node, &provided["node"], accounts)?;
    }

    Ok(())
}
fn provided_name(node: &Value) -> Result<String, Error> {
    keys(node, &["kind", "name", "node"])?;

    if node["kind"] != "providedNode" {
        return Err("unsupported provided metadata".into());
    }

    identifier(node["name"].as_str().ok_or("missing provided name")?)
}
fn dependency(node: &Value, value: &Value, accounts: &[Account]) -> Result<(), Error> {
    match value["kind"].as_str() {
        Some("argumentValueNode") => {
            keys(value, &["kind", "name"])?;
            if !array(node, "arguments")?
                .iter()
                .any(|item| item["name"] == value["name"])
            {
                return Err("unresolved provided argument".into());
            }
        }
        Some("accountFieldValueNode") => account_field(node, value, accounts)?,
        _ => return Err("unsupported provided dependency".into()),
    }

    Ok(())
}
fn account_field(node: &Value, value: &Value, accounts: &[Account]) -> Result<(), Error> {
    keys(value, &["kind", "account", "path"])?;
    let field = identifier(value["path"].as_str().ok_or("missing field path")?)?;
    let role = array(node, "accounts")?
        .iter()
        .find(|role| role["name"] == value["account"])
        .ok_or("unresolved provided account")?;
    let linked = name(&role["accountLink"])?;
    let account = accounts
        .iter()
        .find(|account| account.definition.name == linked)
        .ok_or("unresolved provided account layout")?;
    let Shape::Struct(fields) = &account.definition.shape else {
        return Err("provided path needs a record".into());
    };

    if !fields.iter().any(|item| item.name == field) {
        return Err("unresolved provided field path".into());
    }

    Ok(())
}
