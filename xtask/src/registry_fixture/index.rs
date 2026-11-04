//! Converts exact cached sparse records into an ordinary Cargo local-registry index.

use super::{Config, digest, index_path, json, string};
use anyhow::{Context, Result, ensure};
use serde_json::Value;
use std::{collections::BTreeMap, fs, path::Path};

/// Installs one lockfile-selected package, using a reviewed archive when configured.
pub(super) fn install(
    config: &Config,
    package: BTreeMap<String, String>,
    archive: Option<&Path>,
) -> Result<Value> {
    let name = package
        .get("name")
        .context("lockfile package has no name")?;
    let version = package
        .get("version")
        .context("lockfile package has no version")?;

    match archive {
        Some(archive) => install_archive(config, name, version, archive, Some(&package)),
        None => install_cached(config, &package),
    }
}

/// Installs one reviewed archive that is absent from the derived lock input.
pub(super) fn install_archive(
    config: &Config,
    name: &str,
    version: &str,
    archive: &Path,
    package: Option<&BTreeMap<String, String>>,
) -> Result<Value> {
    let checksum = digest(archive)?;
    let record = metadata_record(config, name, version, &checksum)?;
    let index = config.destination.join("index").join(index_path(name)?);
    append_record(&index, &record)?;
    let destination = config.destination.join(format!("{name}-{version}.crate"));

    if config.omit_archive.as_deref() != Some(name) {
        install_archive_bytes(
            archive,
            &destination,
            config.corrupt_archive.as_deref() == Some(name),
        )?;
    }

    Ok(json!({"name":name, "version":version, "sha256":checksum,
        "archive_present":destination.exists(), "source":archive, "record":record,
        "locked":package.is_some()}))
}

/// Installs one checksum-verified cached dependency selected by the lock input.
fn install_cached(config: &Config, package: &BTreeMap<String, String>) -> Result<Value> {
    let name = package.get("name").context("missing locked name")?;
    let version = package.get("version").context("missing locked version")?;
    let source = config.cache.join(format!("{name}-{version}.crate"));
    let checksum = digest(&source)?;
    let record = cached_record(config, package, &checksum)?;
    let index = config.destination.join("index").join(index_path(name)?);
    append_record(&index, &record)?;
    let destination = config.destination.join(format!("{name}-{version}.crate"));

    if config.omit_archive.as_deref() != Some(name) {
        install_archive_bytes(
            &source,
            &destination,
            config.corrupt_archive.as_deref() == Some(name),
        )?;
    }

    Ok(json!({"name":name, "version":version, "sha256":checksum,
        "archive_present":destination.exists(), "source":source, "record":record,
        "locked":true}))
}

/// Preserves the original sparse JSON only after both lockfile and archive checksums agree.
fn cached_record(
    config: &Config,
    package: &BTreeMap<String, String>,
    checksum: &str,
) -> Result<Value> {
    let name = package.get("name").context("missing locked name")?;
    let version = package.get("version").context("missing locked version")?;
    ensure!(
        package.get("checksum").map(String::as_str) == Some(checksum),
        "cached archive checksum differs: {name}"
    );
    ensure!(
        package.get("source").map(String::as_str)
            == Some("registry+https://github.com/rust-lang/crates.io-index"),
        "unexpected locked source"
    );
    let bytes = fs::read(config.sparse_index.join(".cache").join(index_path(name)?))?;

    for part in bytes
        .split(|byte| *byte == 0)
        .filter(|part| part.starts_with(b"{"))
    {
        let record: Value = serde_json::from_slice(part)?;

        if record.get("vers").and_then(Value::as_str) == Some(version.as_str()) {
            ensure!(
                record.get("cksum").and_then(Value::as_str) == Some(checksum),
                "sparse checksum differs"
            );

            return Ok(record);
        }
    }

    anyhow::bail!("cached index lacks exact {name} {version}")
}

/// Converts actual Cargo metadata, dropping local source paths from the registry contract.
fn metadata_record(config: &Config, name: &str, version: &str, checksum: &str) -> Result<Value> {
    let metadata: Value = serde_json::from_slice(&fs::read(&config.metadata)?)?;
    let packages = metadata
        .get("packages")
        .and_then(Value::as_array)
        .context("missing metadata packages")?;
    let selected: Vec<_> = packages
        .iter()
        .filter(|package| {
            package.get("name").and_then(Value::as_str) == Some(name)
                && package.get("version").and_then(Value::as_str) == Some(version)
        })
        .collect();
    ensure!(
        selected.len() == 1,
        "metadata must identify one {name} {version} package"
    );
    let package = selected[0];
    let dependencies = package
        .get("dependencies")
        .and_then(Value::as_array)
        .context("missing dependencies")?;
    let dependencies: Vec<_> = dependencies.iter().map(dependency).collect::<Result<_>>()?;
    let features = package
        .get("features")
        .and_then(Value::as_object)
        .context("missing package features")?;

    Ok(json!({"name":name, "vers":version, "deps":dependencies,
        "cksum":checksum, "features":{}, "features2":features, "yanked":false,
        "rust_version":package["rust_version"], "v":2}))
}

/// Registry records preserve dependency feature/target/kind semantics explicitly.
fn dependency(value: &Value) -> Result<Value> {
    let name = string(value, "name")?;
    let rename = value.get("rename").and_then(Value::as_str);
    let kind = value
        .get("kind")
        .filter(|value| !value.is_null())
        .cloned()
        .unwrap_or(json!("normal"));
    let features = value
        .get("features")
        .and_then(Value::as_array)
        .context("missing dependency features")?;
    let optional = value
        .get("optional")
        .and_then(Value::as_bool)
        .context("missing optional flag")?;
    let defaults = value
        .get("uses_default_features")
        .and_then(Value::as_bool)
        .context("missing default-feature flag")?;

    Ok(
        json!({"name":rename.unwrap_or(name), "package":rename.map(|_| name),
        "req":string(value,"req")?, "features":features,
        "optional":optional, "default_features":defaults,
        "target":value["target"], "kind":kind,
        "registry":null}),
    )
}

/// A local registry uses newline JSON records without invented Git repository metadata.
fn append_record(path: &Path, record: &Value) -> Result<()> {
    use std::io::Write;
    fs::create_dir_all(path.parent().context("index has no parent")?)?;
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    serde_json::to_writer(&mut file, record)?;
    file.write_all(b"\n")?;
    Ok(())
}

/// Negative mutation never changes a hardlinked source archive.
fn install_archive_bytes(source: &Path, destination: &Path, corrupt: bool) -> Result<()> {
    if corrupt {
        let mut bytes = fs::read(source)?;
        ensure!(
            bytes.len() >= 10 && bytes[..2] == [31, 139],
            "expected gzip archive"
        );
        // A timestamp change keeps decompression valid but invalidates the archive checksum.
        bytes[4] ^= 1;
        fs::write(destination, bytes)?;

        return Ok(());
    }

    fs::hard_link(source, destination).context("hardlink selected archive")?;
    Ok(())
}
