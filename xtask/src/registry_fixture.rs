//! Builds a wholly offline local-registry backend from checksum-verified cached inputs.

use anyhow::{Context, Result, ensure};
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

mod index;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    archive: PathBuf,
    lockfile: PathBuf,
    metadata: PathBuf,
    cache: PathBuf,
    sparse_index: PathBuf,
    destination: PathBuf,
    package: String,
    version: String,
    omit_archive: Option<String>,
    corrupt_archive: Option<String>,
}

/// Materializes only packages selected by the implementation archive's own lockfile.
pub(crate) fn run(path: &Path) -> Result<()> {
    let config: Config = serde_json::from_slice(&fs::read(path)?)?;
    let packages = locked_packages(&fs::read_to_string(&config.lockfile)?)?;
    let selected = packages
        .iter()
        .filter(|package| {
            package.get("name") == Some(&config.package)
                && package.get("version") == Some(&config.version)
        })
        .count();
    ensure!(
        selected == 1,
        "lockfile must identify exactly one implementation package"
    );
    fs::create_dir(&config.destination).context("registry destination must be fresh")?;
    fs::create_dir(config.destination.join("index"))?;
    let mut receipts = Vec::new();

    for package in packages {
        receipts.push(index::install(&config, package)?);
    }

    fs::write(
        config.destination.join("inputs.json"),
        serde_json::to_vec_pretty(&receipts)?,
    )?;
    println!(
        "registry-fixture: {} exact package archives",
        receipts.len()
    );
    Ok(())
}

/// Parses only the package identity/checksum cells from a standard Cargo-generated lockfile.
fn locked_packages(text: &str) -> Result<Vec<BTreeMap<String, String>>> {
    let mut packages = Vec::new();
    let mut current = None;

    for line in text.lines() {
        if line == "[[package]]" {
            packages.extend(current.take());
            current = Some(BTreeMap::new());
            continue;
        }

        let Some((key, value)) = line.split_once(" = ") else {
            continue;
        };

        if matches!(key, "name" | "version" | "source" | "checksum") && current.is_some() {
            let package = current
                .as_mut()
                .context("package field outside lockfile package")?;
            let value: String =
                serde_json::from_str(value).context("lockfile identity must be quoted")?;
            ensure!(
                package.insert(key.to_owned(), value).is_none(),
                "duplicate lockfile field"
            );
        }
    }

    packages.extend(current);

    ensure!(!packages.is_empty(), "lockfile contains no packages");
    Ok(packages)
}

/// Index directory names follow Cargo's actual registry layout.
fn index_path(name: &str) -> Result<PathBuf> {
    ensure!(
        !name.is_empty()
            && name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"_-".contains(&byte)),
        "invalid registry package name"
    );
    let name = name.to_ascii_lowercase();

    let path = match name.len() {
        1 => format!("1/{name}"),
        2 => format!("2/{name}"),
        3 => format!("3/{}/{name}", &name[..1]),
        _ => format!("{}/{}/{name}", &name[..2], &name[2..4]),
    };

    Ok(path.into())
}

/// Hashes actual selected archive bytes instead of trusting a cached extraction.
fn digest(path: &Path) -> Result<String> {
    ensure!(
        fs::symlink_metadata(path)?.is_file(),
        "archive is not regular"
    );
    Ok(format!("{:x}", Sha256::digest(fs::read(path)?)))
}

/// Requires an explicit string cell rather than silently inventing metadata.
fn string<'a>(value: &'a Value, key: &str) -> Result<&'a str> {
    value
        .get(key)
        .and_then(Value::as_str)
        .with_context(|| format!("missing metadata {key}"))
}

#[cfg(test)]
mod tests {
    use super::{index_path, locked_packages};

    #[test]
    fn preserves_exact_lockfile_versions_and_checksums() {
        let packages = locked_packages("version = 4\n\n[[package]]\nname = \"core\"\nversion = \"0.1.0\"\n\n[[package]]\nname = \"bs58\"\nversion = \"0.5.1\"\nchecksum = \"abc\"\n").unwrap();
        assert_eq!(packages.len(), 2);
        assert_eq!(packages[1]["checksum"], "abc");
        assert_eq!(index_path("bs58").unwrap().to_str(), Some("bs/58/bs58"));
        assert!(index_path("../escape").is_err());
    }
}
