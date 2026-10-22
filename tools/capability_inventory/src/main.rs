//! Extracts a pinned, reviewable capability worklist without claiming SDK parity.

mod catalog;
mod inventory;
mod ledger;
mod model;
mod parse;
mod typescript;

#[cfg(test)]
mod tests;

use crate::{
    catalog::Catalogs,
    inventory::Inventory,
    ledger::Tables,
    model::{PIN, Result},
};
use std::{env, path::Path, process::Command};

/// Pins are checked before either reading or replacing the canonical ledgers.
fn main() -> Result<()> {
    let arguments: Vec<_> = env::args().collect();
    let mode = arguments
        .get(1)
        .ok_or("usage: capability_inventory <check|write|controls> <reference> [repository]")?;
    let reference = Path::new(arguments.get(2).ok_or("missing reference path")?);
    let repository = Path::new(arguments.get(3).map_or(".", String::as_str));

    if !matches!(mode.as_str(), "check" | "write" | "controls") {
        return Err("expected check, write or controls".into());
    }

    verify_pin(reference)?;
    ledger::validate_pilot(repository)?;
    let inventory = Inventory::load(reference, repository)?;
    let catalogs = Catalogs::load(reference, &inventory.packages)?;
    let tables = Tables::build(&inventory)?;
    let directory = repository.join("docs/parity");

    match mode.as_str() {
        "write" => tables.write(&directory)?,
        "controls" => tables.controls()?,
        _ => tables.check(&directory)?,
    }

    summarize(&inventory, &catalogs, &tables);
    Ok(())
}

/// Reports diagnostics separately from declaration counts to preserve uncertainty.
fn summarize(inventory: &Inventory, catalogs: &Catalogs, tables: &Tables) {
    let warnings: usize = inventory
        .sources
        .values()
        .map(|source| source.parsed.warnings.len())
        .sum();
    println!(
        "pin {PIN}; {} packages; {} files; {} capabilities; {warnings} unresolved parser/export diagnostics",
        inventory.packages.len(),
        inventory.sources.len(),
        tables.counts.values().sum::<usize>()
    );
    println!(
        "catalogs: {} RPC operations; {} subscription families; {} program clients",
        catalogs.rpc.len(),
        catalogs.subscriptions.len(),
        catalogs.programs.len()
    );
    println!(
        "pending typed RPC catalog values: {}",
        catalogs.rpc.iter().cloned().collect::<Vec<_>>().join(" ")
    );
    println!(
        "pending typed subscription catalog values: {}",
        catalogs
            .subscriptions
            .iter()
            .cloned()
            .collect::<Vec<_>>()
            .join(" ")
    );

    for (phase, count) in &tables.counts {
        println!("phase {phase}: {count}");
    }

    for source in inventory.sources.values() {
        for warning in &source.parsed.warnings {
            println!("{} {warning}", source.path);
        }
    }
}

/// A wrong checkout must fail before generating plausible-looking provenance.
fn verify_pin(reference: &Path) -> Result<()> {
    let output = Command::new("git")
        .arg("-C")
        .arg(reference)
        .args(["rev-parse", "HEAD"])
        .output()?;

    if !output.status.success() || String::from_utf8(output.stdout)?.trim() != PIN {
        return Err("reference HEAD does not match immutable Dart pin".into());
    }

    let dirty = Command::new("git")
        .arg("-C")
        .arg(reference)
        .args(["status", "--porcelain", "--untracked-files=all"])
        .output()?;

    if !dirty.status.success() || !dirty.stdout.is_empty() {
        return Err("reference checkout contains modifications or untracked files".into());
    }

    Ok(())
}
