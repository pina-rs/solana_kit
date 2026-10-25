//! Pinned preparation contracts; runtime client generation remains a separate step.
use crate::{generic, schema::Error};
use serde_json::Value;
use sha2::{Digest, Sha256};

fn pinned(bytes: &[u8], digest: &str) -> Result<Value, Error> {
    if format!("{:x}", Sha256::digest(bytes)) != digest {
        return Err("program source pin mismatch".into());
    }

    Ok(serde_json::from_slice(bytes)?)
}

pub(crate) fn compression() -> Result<Value, Error> {
    let root = pinned(
        include_bytes!("../fixtures/compression-original.json"),
        "587f3b4d7a1f4384181f1836c09870e3fcc7ee747a69c9cf0c2164eff57bec74",
    )?;
    generic::prepare_anchor(
        &root,
        &generic::AnchorPolicy {
            program: generic::Policy {
                name: "splAccountCompression",
                address: "cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK".parse()?,
            },
            source_name: "spl_account_compression",
            remaining_proofs: &["replaceLeaf", "verifyLeaf", "insertOrAppend"],
        },
    )
}

pub(crate) fn subscriptions() -> Result<Value, Error> {
    pinned(
        include_bytes!("../fixtures/subscriptions-original.json"),
        "461e1c41da256906644d3e20ab185e97de9d9c141661e1521bf5ef42e255ac40",
    )
}

/// Exposes a source-normalization artifact without claiming generated runtime support.
pub(crate) fn run(output: &std::path::Path) -> Result<(), Error> {
    let prepared = compression()?;
    let bytes = serde_json::to_vec_pretty(&prepared)?;
    let temporary = output.with_extension("json.pending");
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)?;
    file.write_all(&bytes)?;
    file.sync_all()?;
    std::fs::rename(temporary, output)?;
    Ok(())
}

/// Projects the complete pinned Compression model through the shared renderer.
pub(crate) fn generate_compression(output: &std::path::Path) -> Result<(), Error> {
    let source = generic::render_public_program(
        &compression()?,
        &generic::Policy {
            name: "splAccountCompression",
            address: "cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK".parse()?,
        },
    )?;
    crate::generate::replace_source(&source, output)
}

/// Complete account/event/default projection through the common typed renderer.
pub(crate) fn generate_subscriptions(output: &std::path::Path) -> Result<(), Error> {
    let source = generic::render_complete_program(
        &subscriptions()?,
        &generic::Policy {
            name: "subscriptions",
            address: "De1egAFMkMWZSN5rYXRj9CAdheBamobVNubTsi9avR44".parse()?,
        },
    )?;
    crate::generate::replace_source(&source, output)
}

#[cfg(all(test, feature = "verification"))]
mod tests;

pub(crate) fn audit() -> Result<(), Error> {
    let compression = compression()?;
    let subscriptions = subscriptions()?;
    let program = &subscriptions["program"];
    let recipes = generic::pda_recipes(program["pdas"].as_array().ok_or("missing PDA recipes")?)?;
    let defaults = generic::pda_defaults(
        program["instructions"]
            .as_array()
            .ok_or("missing instructions")?,
        &recipes,
    )?;
    let constants = recipes
        .iter()
        .flat_map(|recipe| &recipe.seeds)
        .filter(|seed| matches!(seed, generic::PdaSeed::Constant(_)))
        .count();
    let addresses = defaults
        .iter()
        .filter(|default| matches!(default.value, generic::PdaDefault::Address(_)))
        .count();
    println!(
        "validated {} Compression instruction layouts and {} errors; {} Subscriptions PDA recipes ({constants} constant seeds), {} role defaults ({addresses} fixed addresses)",
        compression["program"]["instructions"]
            .as_array()
            .ok_or("missing prepared instructions")?
            .len(),
        compression["program"]["errors"]
            .as_array()
            .ok_or("missing prepared errors")?
            .len(),
        recipes.len(),
        defaults.len()
    );

    Ok(())
}
