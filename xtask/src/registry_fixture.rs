//! Builds a wholly offline local-registry backend from checksum-verified cached inputs.

use anyhow::{Context, Result, ensure};
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

mod index;

/// One additional reviewed implementation archive in the same local registry.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Additional {
    archive: PathBuf,
    package: String,
    version: String,
}

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
    #[serde(default)]
    additions: Vec<Additional>,
}

/// Materializes only packages selected by the implementation archives' combined lock input.
pub(crate) fn run(path: &Path) -> Result<()> {
    let config: Config = serde_json::from_slice(&fs::read(path)?)?;
    let selected = selected_archives(&config)?;
    let packages = locked_packages(&fs::read_to_string(&config.lockfile)?)?;
    ensure!(
        packages
            .iter()
            .filter(|package| {
                package.get("name") == Some(&config.package)
                    && package.get("version") == Some(&config.version)
            })
            .count()
            == 1,
        "lockfile must identify exactly one primary implementation package"
    );
    fs::create_dir(&config.destination).context("registry destination must be fresh")?;
    fs::create_dir(config.destination.join("index"))?;
    let mut receipts = Vec::new();
    let mut installed = BTreeSet::new();

    for package in &packages {
        let name = package
            .get("name")
            .context("lockfile package has no name")?;
        let version = package
            .get("version")
            .context("lockfile package has no version")?;
        let key = (name.clone(), version.clone());
        let archive = selected.get(&key).map(PathBuf::as_path);

        receipts.push(index::install(&config, package.clone(), archive)?);
        installed.insert(key);
    }

    for ((name, version), archive) in &selected {
        if !installed.contains(&(name.clone(), version.clone())) {
            receipts.push(index::install_archive(
                &config, name, version, archive, None,
            )?);
        }
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

/// Maps every reviewed implementation identity to its actual archive path.
fn selected_archives(config: &Config) -> Result<BTreeMap<(String, String), PathBuf>> {
    let mut selected = BTreeMap::new();
    ensure!(
        !config.package.is_empty() && !config.version.is_empty(),
        "primary implementation identity is empty"
    );
    index_path(&config.package)?;
    ensure!(
        selected
            .insert(
                (config.package.clone(), config.version.clone()),
                config.archive.clone()
            )
            .is_none(),
        "duplicate implementation archive"
    );

    for addition in &config.additions {
        ensure!(
            !addition.package.is_empty() && !addition.version.is_empty(),
            "additional implementation identity is empty"
        );
        index_path(&addition.package)?;
        ensure!(
            selected
                .insert(
                    (addition.package.clone(), addition.version.clone()),
                    addition.archive.clone()
                )
                .is_none(),
            "duplicate implementation archive: {}",
            addition.package
        );
    }

    for archive in selected.values() {
        ensure!(
            fs::symlink_metadata(archive)?.is_file(),
            "implementation archive is not regular: {}",
            archive.display()
        );
    }

    Ok(selected)
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
    use super::{Config, index_path, locked_packages, selected_archives};

    fn config(archive: &str, additions: &str) -> Config {
        serde_json::from_str(&format!(
            r#"{{"archive":"{archive}","lockfile":"lock","metadata":"metadata",
            "cache":"cache","sparse_index":"index","destination":"destination",
            "package":"core","version":"0.1.0","omit_archive":null,
            "corrupt_archive":null,"additions":{additions}}}"#
        ))
        .unwrap()
    }

    #[test]
    fn preserves_exact_lockfile_versions_and_checksums() {
        let packages = locked_packages("version = 4\n\n[[package]]\nname = \"core\"\nversion = \"0.1.0\"\n\n[[package]]\nname = \"bs58\"\nversion = \"0.5.1\"\nchecksum = \"abc\"\n").unwrap();
        assert_eq!(packages.len(), 2);
        assert_eq!(packages[1]["checksum"], "abc");
        assert_eq!(index_path("bs58").unwrap().to_str(), Some("bs/58/bs58"));
        assert!(index_path("../escape").is_err());
    }

    #[test]
    fn rejects_duplicate_and_unsafe_implementation_identities() {
        let directory =
            std::env::temp_dir().join(format!("registry-fixture-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&directory);
        std::fs::create_dir(&directory).unwrap();
        let archive = directory.join("core-0.1.0.crate");
        std::fs::write(&archive, b"archive").unwrap();
        let path = archive.to_str().unwrap();
        let duplicate = format!(r#"[{{"archive":"{path}","package":"core","version":"0.1.0"}}]"#);
        assert!(selected_archives(&config(path, &duplicate)).is_err());

        let escaped = format!(r#"[{{"archive":"{path}","package":"../core","version":"0.1.0"}}]"#);
        assert!(selected_archives(&config(path, &escaped)).is_err());
        std::fs::remove_dir_all(directory).unwrap();
    }
}
