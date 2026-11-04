//! Restricts Cargo metadata to the actual SDK crate directory.

use super::{Metadata, Package};
use anyhow::{Result, ensure};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

/// Restricts the metadata to workspace members under the actual `crates` root.
pub(super) fn packages<'a>(
    metadata: &'a Metadata,
    crates: &Path,
) -> Result<BTreeMap<String, &'a Package>> {
    let members: BTreeSet<&str> = metadata
        .workspace_members
        .iter()
        .map(String::as_str)
        .collect();
    let mut packages = BTreeMap::new();

    for package in &metadata.packages {
        let manifest = package.manifest_path.canonicalize()?;

        if !members.contains(package.id.as_str()) || !manifest.starts_with(crates) {
            continue;
        }

        ensure!(
            packages.insert(package.name.clone(), package).is_none(),
            "duplicate SDK package: {}",
            package.name
        );
    }

    ensure!(!packages.is_empty(), "no crates/* workspace packages found");
    Ok(packages)
}
