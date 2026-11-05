//! A clean external consumer tests the normalized packaged core without repo flags.

use crate::process::{cargo, success, target_dir};
use anyhow::{Context, Result, ensure};
use serde::Deserialize;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

/// The public library this external consumer packages and extracts.
const PACKAGE: &str = "solana_kit_core";
/// Replaced with the exact extracted archive directory before Cargo reads the manifest.
const PACKAGE_PATH: &str = "__SOLANA_KIT_CORE_PACKAGE__";
const CONSUMER_MANIFEST: &str = include_str!("../consumer/Cargo.toml.txt");

/// One workspace package identity read from Cargo's actual metadata.
struct PackageIdentity {
    name: String,
    version: String,
}

impl PackageIdentity {
    /// The directory Cargo creates when it extracts this exact crate archive.
    fn directory(&self) -> String {
        format!("{}-{}", self.name, self.version)
    }

    /// The archive Cargo writes for this exact package version.
    fn archive(&self) -> String {
        format!("{}.crate", self.directory())
    }
}

/// Packages the public core, extracts it outside the repository, and runs a consumer.
pub(crate) fn run() -> Result<()> {
    let directory: PathBuf =
        std::env::temp_dir().join(format!("solana-kit-consumer-{}", std::process::id()));
    std::fs::create_dir_all(directory.join("src"))?;
    let identity = workspace_package()?;
    success(
        cargo().args([
            "package",
            "--locked",
            "-p",
            "solana_kit_core",
            "--all-features",
        ]),
        "consumer-package",
    )?;
    let archive = archive_path(&identity)?;
    success(
        Command::new("tar")
            .arg("-xzf")
            .arg(&archive)
            .arg("-C")
            .arg(&directory),
        "consumer-extract",
    )?;
    let extracted = verify_extracted(&directory, &identity)?;
    fs::write(directory.join("Cargo.toml"), consumer_manifest(&identity))?;
    fs::write(
        directory.join("src/lib.rs"),
        include_str!("../consumer/lib.rs"),
    )?;
    fs::write(
        directory.join("src/codec_overrides.rs"),
        include_str!("../../crates/solana_kit_core/tests/codec_size/overrides.rs"),
    )?;
    success(
        cargo()
            .args(["test", "--manifest-path", "Cargo.toml"])
            .current_dir(&directory)
            .env_remove("RUSTFLAGS")
            .env_remove("CARGO_ENCODED_RUSTFLAGS")
            .env_remove("CARGO_BUILD_TARGET")
            .env_remove("RUSTC_WRAPPER")
            .env_remove("RUSTC_WORKSPACE_WRAPPER"),
        "consumer-test",
    )?;
    println!(
        "clean consumer retained at {} using {}",
        directory.display(),
        extracted.display()
    );

    Ok(())
}

/// Reads the packaged library's name and version from Cargo's workspace metadata.
fn workspace_package() -> Result<PackageIdentity> {
    let output = success(
        cargo().args([
            "metadata",
            "--locked",
            "--offline",
            "--no-deps",
            "--format-version",
            "1",
        ]),
        "consumer-metadata",
    )?;
    let metadata: Metadata = serde_json::from_slice(&output.stdout)?;
    let package = metadata
        .packages
        .into_iter()
        .find(|package| package.name == PACKAGE)
        .with_context(|| format!("Cargo metadata has no {PACKAGE} package"))?;

    Ok(PackageIdentity {
        name: package.name,
        version: package.version,
    })
}

/// Requires the exact archive named by the metadata-derived package identity.
fn archive_path(identity: &PackageIdentity) -> Result<PathBuf> {
    let archive = target_dir().join("package").join(identity.archive());
    ensure!(
        archive.is_file(),
        "Cargo did not generate the metadata-derived archive {}; found: {}",
        archive.display(),
        archive
            .parent()
            .map_or_else(|| "no package directory".to_owned(), describe)
    );

    Ok(archive)
}

/// Rejects an extracted manifest whose package identity differs from Cargo metadata.
fn verify_extracted(root: &Path, identity: &PackageIdentity) -> Result<PathBuf> {
    let directory = root.join(identity.directory());
    let manifest = directory.join("Cargo.toml");
    let text = fs::read_to_string(&manifest)
        .with_context(|| format!("read extracted manifest {}", manifest.display()))?;
    let manifest: toml::Value = toml::from_str(&text)
        .with_context(|| format!("parse extracted manifest {}", manifest.display()))?;
    let package = manifest
        .get("package")
        .context("extracted manifest has no [package] table")?;
    let name = package
        .get("name")
        .and_then(toml::Value::as_str)
        .context("extracted manifest has no package name")?;
    let version = package
        .get("version")
        .and_then(toml::Value::as_str)
        .context("extracted manifest has no package version")?;

    ensure!(
        name == identity.name,
        "extracted package name {name} disagrees with Cargo metadata {}",
        identity.name
    );
    ensure!(
        version == identity.version,
        "extracted package version {version} disagrees with Cargo metadata {}",
        identity.version
    );

    Ok(directory)
}

/// Substitutes the exact extracted directory into the dependency-free template.
fn consumer_manifest(identity: &PackageIdentity) -> String {
    CONSUMER_MANIFEST.replace(PACKAGE_PATH, &identity.directory())
}

/// Lists a directory for an actionable missing-archive diagnostic.
fn describe(directory: &Path) -> String {
    let mut entries = match fs::read_dir(directory) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect::<Vec<_>>(),
        Err(error) => return error.to_string(),
    };
    entries.sort();

    if entries.is_empty() {
        "empty package directory".to_owned()
    } else {
        entries.join(", ")
    }
}

/// The fields this command needs from Cargo's workspace metadata.
#[derive(Deserialize)]
struct Metadata {
    packages: Vec<Package>,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
}

#[cfg(test)]
mod tests {
    use super::{
        CONSUMER_MANIFEST, PACKAGE_PATH, PackageIdentity, consumer_manifest, verify_extracted,
    };
    use std::{fs, path::Path, process::Command};

    #[test]
    fn consumer_manifest_tracks_the_staged_package_without_a_hard_coded_version() {
        let identity = PackageIdentity {
            name: "solana_kit_core".into(),
            version: "0.1.1".into(),
        };
        let manifest = consumer_manifest(&identity);

        assert!(manifest.contains(r#"path = "solana_kit_core-0.1.1""#));
        assert!(!manifest.contains("solana_kit_core-0.1.0"));
        assert!(!manifest.contains(PACKAGE_PATH));
        assert!(!CONSUMER_MANIFEST.contains("solana_kit_core-"));
    }

    #[test]
    fn cargo_resolves_the_staged_extracted_version_not_the_legacy_hardcode() {
        let root = temporary("resolution");
        let identity = PackageIdentity {
            name: "solana_kit_core".into(),
            version: "0.2.7".into(),
        };
        stage(&root, "solana_kit_core", "0.2.7");

        let legacy = PackageIdentity {
            name: "solana_kit_core".into(),
            version: "0.1.0".into(),
        };
        assert!(verify_extracted(&root, &legacy).is_err());
        assert_eq!(
            verify_extracted(&root, &identity).expect("actual staged package"),
            root.join("solana_kit_core-0.2.7")
        );

        let consumer = root.clone();
        fs::create_dir_all(consumer.join("src")).expect("consumer source directory");
        fs::write(consumer.join("Cargo.toml"), consumer_manifest(&identity))
            .expect("consumer manifest");
        fs::write(consumer.join("src/lib.rs"), "").expect("consumer library");

        let output = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
            .args([
                "metadata",
                "--format-version",
                "1",
                "--no-deps",
                "--offline",
            ])
            .current_dir(&consumer)
            .env("CARGO_TARGET_DIR", root.join("target"))
            .output()
            .expect("run Cargo metadata");
        assert!(
            output.status.success(),
            "Cargo metadata failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let metadata: serde_json::Value =
            serde_json::from_slice(&output.stdout).expect("parse Cargo metadata");
        let package = metadata["packages"]
            .as_array()
            .expect("metadata packages")
            .iter()
            .find(|package| package["name"] == "solana_kit_core")
            .expect("resolved core package");

        assert_eq!(package["version"], "0.2.7");
        assert_eq!(
            package["manifest_path"],
            root.join("solana_kit_core-0.2.7/Cargo.toml")
                .to_string_lossy()
                .as_ref()
        );

        fs::remove_dir_all(root).expect("remove test directory");
    }

    #[test]
    fn extraction_rejects_a_different_package_identity() {
        let root = temporary("mismatch");
        stage(&root, "solana_kit_core", "0.2.7");
        let claimed = PackageIdentity {
            name: "solana_kit_core".into(),
            version: "0.2.8".into(),
        };

        let error = verify_extracted(&root, &claimed).expect_err("version disagreement");
        assert!(error.to_string().contains("0.2.8"), "{error}");

        fs::remove_dir_all(root).expect("remove test directory");
    }

    /// Writes an already-extracted package layout without invoking Cargo.
    fn stage(root: &Path, name: &str, version: &str) {
        let directory = root.join(format!("{name}-{version}"));

        fs::create_dir_all(directory.join("src")).expect("staged source directory");
        fs::write(
            directory.join("Cargo.toml"),
            format!(
                "[package]\nname = \"{name}\"\nversion = \"{version}\"\nedition = \"2024\"\n\n[lib]\npath = \"src/lib.rs\"\n"
            ),
        )
        .expect("staged manifest");
        fs::write(directory.join("src/lib.rs"), "").expect("staged library");
    }

    /// Uses one fresh private directory per test process.
    fn temporary(name: &str) -> std::path::PathBuf {
        let directory = std::env::temp_dir().join(format!(
            "solana-kit-consumer-test-{}-{name}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&directory);
        fs::create_dir_all(&directory).expect("test directory");

        directory.canonicalize().expect("canonical test directory")
    }
}
