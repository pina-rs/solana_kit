//! Generates the exact external consumer manifest and source-replacement config.

use super::super::model;
use anyhow::{Context, Result};
use std::{fs, path::Path};

pub(super) const DEPENDENCIES: [&str; 10] = [
    "solana_kit_core",
    "solana_kit_client",
    "solana_kit_signers",
    "solana_kit_wallet",
    "solana_kit_native",
    "solana_kit_token",
    "solana_kit_system",
    "solana_kit_memo",
    "solana_kit_associated_token_account",
    "solana_kit_anchor",
];

const SELECTED: [&str; 15] = [
    "solana_kit_core/crypto",
    "solana_kit_core/fixed-points",
    "solana_kit_core/text-codecs",
    "solana_kit_core/bigint-codecs",
    "solana_kit_core/keccak",
    "solana_kit_client/confirmation-factories",
    "solana_kit_client/resource-estimation",
    "solana_kit_client/parsed-accounts",
    "solana_kit_client/reactive",
    "solana_kit_signers/crypto",
    "solana_kit_wallet/mwa",
    "solana_kit_native/mwa",
    "solana_kit_native/surfpool",
    "solana_kit_token/plans",
    "solana_kit_associated_token_account/crypto",
];

/// Writes the manifest, test body and local-registry source replacement.
pub(super) fn write(plan: &model::Plan, directory: &Path, registry: &Path) -> Result<()> {
    fs::create_dir_all(directory.join("src"))?;
    fs::create_dir_all(directory.join("tests"))?;
    fs::create_dir_all(directory.join(".cargo"))?;
    fs::write(directory.join("Cargo.toml"), manifest(plan)?)?;
    fs::write(
        directory.join("build.rs"),
        include_str!("../../../rehearsal/build.rs"),
    )?;
    fs::write(
        directory.join("src/lib.rs"),
        "//! Compile surface for the external registry consumer.\n#![forbid(unsafe_code)]\n",
    )?;
    fs::write(
        directory.join("tests/default.rs"),
        include_str!("../../../rehearsal/default.rs"),
    )?;
    fs::write(
        directory.join("tests/selected.rs"),
        include_str!("../../../rehearsal/selected.rs"),
    )?;
    fs::write(directory.join(".cargo/config.toml"), source(registry)?)?;

    Ok(())
}

/// Exact dependency and feature contract shared by the default and selected runs.
fn manifest(plan: &model::Plan) -> Result<String> {
    let mut text = String::from(
        "[package]\nname = \"solana-kit-registry-consumer\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\n\n[workspace]\n\n[features]\ndefault = []\nselected = [\n",
    );

    for feature in SELECTED {
        text.push_str(&format!("  \"{feature}\",\n"));
    }

    text.push_str("]\n\n[dependencies]\n");

    for name in DEPENDENCIES {
        let target = plan.require(name)?;
        let default = !matches!(
            name,
            "solana_kit_core" | "solana_kit_wallet" | "solana_kit_native"
        );
        text.push_str(&format!(
            "{name} = {{ version = \"={}\", default-features = {default} }}\n",
            target.version
        ));
    }

    text.push_str(
        "tokio = { version = \"=1.54.1\", features = [\"rt\", \"macros\", \"net\", \"time\", \"io-util\"] }\n",
    );

    Ok(text)
}

/// Replaces crates.io with the exact fixture directory, never a path patch.
pub(super) fn source(registry: &Path) -> Result<String> {
    let registry =
        serde_json::to_string(registry.to_str().context("registry path must be UTF-8")?)?;

    Ok(format!(
        "[source.crates-io]\nreplace-with = \"sdk-local-registry\"\n\n[source.sdk-local-registry]\nlocal-registry = {registry}\n\n[net]\noffline = true\n"
    ))
}

#[cfg(test)]
mod tests {
    use super::source;
    use std::path::Path;

    #[test]
    fn source_replacement_never_uses_a_path_patch() {
        let text = source(Path::new("/owned/registry")).unwrap();
        assert!(text.contains("local-registry = \"/owned/registry\""));
        assert!(text.contains("replace-with = \"sdk-local-registry\""));
        assert!(!text.contains("[patch"));
        assert!(!text.contains("path ="));
    }
}
