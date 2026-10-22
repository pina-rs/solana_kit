//! Resolves the selected account contracts before rendering any builders.

use crate::schema::{Error, identifier};
use serde_json::Value;
use std::collections::BTreeSet;

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Account {
    pub(crate) name: String,
    pub(crate) role: &'static str,
    pub(crate) optional: bool,
    pub(crate) fixed: Option<&'static str>,
}

fn keys(node: &Value, allowed: &[&str]) -> Result<(), Error> {
    let object = node.as_object().ok_or("expected account schema object")?;

    if object.keys().any(|key| !allowed.contains(&key.as_str())) {
        return Err("unsupported account schema property".into());
    }

    Ok(())
}

fn account(node: &Value) -> Result<Account, Error> {
    keys(
        node,
        &[
            "kind",
            "name",
            "isWritable",
            "isSigner",
            "isOptional",
            "docs",
            "display",
            "defaultValue",
        ],
    )?;

    if node["kind"] != "instructionAccountNode" {
        return Err("unsupported account kind".into());
    }

    let name = identifier(node["name"].as_str().ok_or("account name missing")?)?;
    let signer = node["isSigner"].as_bool().ok_or("signer must be boolean")?;
    let writable = node["isWritable"]
        .as_bool()
        .ok_or("writable must be boolean")?;
    let optional = node["isOptional"]
        .as_bool()
        .ok_or("optional must be boolean")?;

    let role = match (signer, writable) {
        (false, false) => "Readonly",
        (false, true) => "Writable",
        (true, false) => "ReadonlySigner",
        (true, true) => "WritableSigner",
    };

    let fixed = default_account(node, &name, role, optional)?;

    Ok(Account {
        name,
        role,
        optional,
        fixed,
    })
}

pub(crate) fn parse(node: &Value) -> Result<Vec<Account>, Error> {
    let output = node["accounts"]
        .as_array()
        .ok_or("missing accounts")?
        .iter()
        .map(account)
        .collect::<Result<Vec<_>, _>>()?;
    let names: BTreeSet<_> = output.iter().map(|account| &account.name).collect();

    if names.len() != output.len() || output.is_empty() {
        return Err("empty or duplicate accounts".into());
    }

    let optional: Vec<_> = output
        .iter()
        .enumerate()
        .filter(|(_, account)| account.optional)
        .collect();

    for (index, account) in optional {
        if index + 1 != output.len()
            || !matches!(
                (node["name"].as_str(), account.name.as_str()),
                (Some("createAccountWithSeed"), "base_account")
                    | (Some("createAccountAllowPrefund"), "payer")
            )
        {
            return Err("unsupported optional account contract".into());
        }
    }

    Ok(output)
}

/// Resolves only the two canonical sysvars and explicit payer context.
fn default_account(
    node: &Value,
    name: &str,
    role: &str,
    optional: bool,
) -> Result<Option<&'static str>, Error> {
    Ok(match node.get("defaultValue") {
        None => None,
        Some(value)
            if value == &serde_json::json!({"kind":"payerValueNode"}) && name == "payer" =>
        {
            None
        }
        Some(value) => {
            keys(value, &["kind", "publicKey"])?;

            match (
                value["kind"].as_str(),
                value["publicKey"].as_str(),
                name,
                role,
                optional,
            ) {
                (
                    Some("publicKeyValueNode"),
                    Some("SysvarRecentB1ockHashes11111111111111111111"),
                    "recent_blockhashes_sysvar",
                    "Readonly",
                    false,
                ) => Some("RECENT_BLOCKHASHES"),
                (
                    Some("publicKeyValueNode"),
                    Some("SysvarRent111111111111111111111111111111111"),
                    "rent_sysvar",
                    "Readonly",
                    false,
                ) => Some("RENT"),

                _ => return Err("unsupported account default".into()),
            }
        }
    })
}

/// Fixed program identity and manual policy bindings require the pinned metadata.
pub(crate) fn validate_bindings(root: &Value) -> Result<(), Error> {
    let pinned: Value = serde_json::from_str(include_str!("../fixtures/system.json"))?;

    if root["program"].get("publicKey") != pinned["program"].get("publicKey") {
        return Err("unsupported pinned builder program identity".into());
    }

    let expected = crate::schema::parse(&pinned)?;
    let actual = crate::schema::parse(root)?;
    let nodes = root["program"]["instructions"]
        .as_array()
        .ok_or("missing instructions")?;

    for (item, node) in actual.iter().zip(nodes) {
        let index = usize::try_from(item.tag)?;
        let reference = &expected[index];
        let expected_accounts = parse(&pinned["program"]["instructions"][index])?;

        if item.name != reference.name
            || item
                .fields
                .iter()
                .map(|field| (&field.name, field.wire))
                .ne(reference
                    .fields
                    .iter()
                    .map(|field| (&field.name, field.wire)))
            || parse(node)? != expected_accounts
        {
            return Err("unsupported changes to pinned builder policy bindings".into());
        }
    }

    Ok(())
}
