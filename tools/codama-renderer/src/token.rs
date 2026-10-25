//! Pinned Token input selection; every wire operation is emitted by generic.
use crate::{
    generic::{self, Policy},
    schema::Error,
};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::path::Path;

pub(crate) fn input() -> Result<Value, Error> {
    let bytes = include_bytes!("../fixtures/token-original.json");

    if format!("{:x}", Sha256::digest(bytes))
        != "33add106ed36fead9222bfd882f1b364f3c4714346becefd5fa6068652b0cc33"
    {
        return Err("Token source pin mismatch".into());
    }
    let mut root: Value = serde_json::from_slice(bytes)?;
    let additions = root["additionalPrograms"]
        .as_array()
        .ok_or("additional program list required")?;

    if additions.len() != 1
        || additions[0]["name"] != "associatedToken"
        || additions[0]["publicKey"] != "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
    {
        return Err("unexpected excluded program owner".into());
    }
    // This exact hashed owner is covered by the separate accepted ATA client.
    root["additionalPrograms"] = serde_json::json!([]);
    Ok(root)
}

pub(crate) fn policy() -> Result<Policy<'static>, Error> {
    Ok(Policy {
        name: "token",
        address: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".parse()?,
    })
}

pub(crate) fn run() -> Result<(), Error> {
    let root = input()?;
    let frozen: Value = serde_json::from_str(include_str!("../fixtures/token-normalized.json"))?;

    if generic::normalize_input(&root)? != frozen {
        return Err("source-backed normalized Token snapshot drift".into());
    }

    let destination = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/solana_kit_token/src/generated.rs");
    generic::replace(&root, &policy()?, &destination)
}

/// Refreshes only the reviewed normalized input from the exact original source pin.
pub(crate) fn freeze() -> Result<(), Error> {
    let root = generic::normalize_input(&input()?)?;
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/token-normalized.json");
    std::fs::write(path, format!("{}\n", serde_json::to_string_pretty(&root)?))?;
    Ok(())
}
