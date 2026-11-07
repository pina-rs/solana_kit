//! Generated manifests use the same published framework release as the SDK adapter.
use super::Result;
use std::{fs, path::Path};

/// The reviewed LeptosKit release. The generator writes the release it was
/// built from, so the SDK adapter and every generated manifest must agree on it.
pub(super) const VERSION: &str = "=0.3.0";
const SDK_FRAMEWORK_DECLARATIONS: usize = 5;
const SOURCE_KEYS: [&str; 8] = [
    "git",
    "path",
    "rev",
    "branch",
    "tag",
    "registry",
    "registry-index",
    "workspace",
];

pub(super) fn verify_sdk(sdk: &Path) -> Result<()> {
    let manifest = fs::read_to_string(sdk.join("crates/solana_kit_leptoskit/Cargo.toml"))?;
    verify_manifest(&manifest)
}

fn verify_manifest(manifest: &str) -> Result<()> {
    let manifest: toml::Table = manifest.parse()?;
    let dependencies = framework_dependencies(&manifest)?;

    if dependencies.len() != SDK_FRAMEWORK_DECLARATIONS || !dependencies.iter().all(same_source) {
        return Err("SDK framework dependencies must select the verified registry release".into());
    }

    Ok(())
}

/// Refuse a generated manifest unless every framework declaration already
/// selects the reviewed registry release. A different version or source means
/// the generator checkout and the SDK disagree, so it is reviewed rather than
/// silently rewritten.
pub(super) fn verify_generated(contents: &str) -> Result<()> {
    let manifest: toml::Table = contents.parse()?;

    if manifest.contains_key("patch") || manifest.contains_key("replace") {
        return Err("generated consumer must not use Cargo source overrides".into());
    }

    if !framework_dependencies(&manifest)?.iter().all(same_source) {
        return Err("review the generated framework dependency before changing its source".into());
    }

    Ok(())
}

/// Every framework declaration in a manifest: its own dependency sections,
/// `[workspace.dependencies]` and each `[target.*]` table.
fn framework_dependencies(manifest: &toml::Table) -> Result<Vec<&toml::Table>> {
    let mut selected = Vec::new();
    collect_dependencies(manifest, &mut selected)?;

    if let Some(workspace) = manifest.get("workspace") {
        let workspace = workspace
            .as_table()
            .ok_or("expected a Cargo workspace table")?;
        collect_dependencies(workspace, &mut selected)?;
    }

    if let Some(targets) = manifest.get("target") {
        for target in targets
            .as_table()
            .ok_or("expected Cargo target tables")?
            .values()
        {
            let target = target.as_table().ok_or("expected a Cargo target table")?;
            collect_dependencies(target, &mut selected)?;
        }
    }

    Ok(selected)
}

/// Inspect Cargo's dependency sections, including renamed packages. Metadata
/// must never satisfy a required declaration or hide a different source.
fn collect_dependencies<'a>(
    manifest: &'a toml::Table,
    selected: &mut Vec<&'a toml::Table>,
) -> Result<()> {
    for section in ["dependencies", "dev-dependencies", "build-dependencies"] {
        let Some(dependencies) = manifest.get(section) else {
            continue;
        };

        for (name, dependency) in dependencies
            .as_table()
            .ok_or("expected Cargo dependencies")?
        {
            let package = dependency
                .get("package")
                .and_then(toml::Value::as_str)
                .unwrap_or(name);

            if package == "leptoskit" || package.starts_with("leptoskit_") {
                selected.push(
                    dependency
                        .as_table()
                        .ok_or("framework requires an explicit registry source")?,
                );
            }
        }
    }

    Ok(())
}

fn same_source(dependency: &&toml::Table) -> bool {
    dependency.get("version").and_then(toml::Value::as_str) == Some(VERSION)
        && SOURCE_KEYS.iter().all(|key| !dependency.contains_key(*key))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_metadata_masquerading_as_sdk_dependencies() {
        let manifest = include_str!("../../../crates/solana_kit_leptoskit/Cargo.toml");
        assert!(verify_manifest(manifest).is_ok());
        let metadata = manifest
            .replace("[dependencies]", "[package.metadata.dependencies]")
            .replace("[dev-dependencies]", "[package.metadata.dev-dependencies]");
        assert!(verify_manifest(&metadata).is_err());
    }

    #[test]
    fn rejects_patch_tables_with_equivalent_toml_spellings() {
        for source in [
            "[ patch . \"https://github.com/leptoskit/leptoskit\" ]\nexample = { path = \"other\" }\n",
            "[\"patch\".crates-io]\nexample = { path = \"other\" }\n",
            "patch = { crates-io = { example = { path = \"other\" } } }\n",
        ] {
            assert!(
                verify_generated(source).is_err(),
                "accepted an alternate patch table"
            );
        }
    }

    #[test]
    fn rejects_legacy_replace_tables() {
        for source in [
            "[replace]\n\"leptoskit_bridge:0.1.0\" = { path = \"other\" }\n",
            "\"replace\" = { \"leptoskit_bridge:0.1.0\" = { path = \"other\" } }\n",
        ] {
            assert!(
                verify_generated(source).is_err(),
                "accepted a Cargo replacement"
            );
        }
    }

    #[test]
    fn accepts_generated_dependencies_on_the_reviewed_release() {
        let manifest = concat!(
            "[dependencies]\n",
            "leptoskit = { version = \"=0.3.0\", default-features = false, ",
            "features = [\"guest\"] }\nsolana_kit_leptoskit = { path = \"sdk\" }\n",
            "renamed = { package = \"leptoskit_bridge\", version = \"=0.3.0\" }\n",
            "[build-dependencies]\n",
            "leptoskit_build = { version = \"=0.3.0\", default-features = false }\n",
            "[target.'cfg(target_os = \"ios\")'.dependencies]\n",
            "leptoskit_bridge_ffi = { version = \"=0.3.0\", features = [\"app\"] }\n",
        );
        assert!(verify_generated(manifest).is_ok());
    }

    #[test]
    fn refuses_other_releases_and_unreviewed_framework_sources() {
        for dependency in [
            "leptoskit = { version = \"=0.2.0\", default-features = false }",
            "leptoskit = { version = \"0.3.0\", default-features = false }",
            "leptoskit = \"=0.3.0\"",
            "leptoskit = { version = \"=0.3.0\", path = \"framework\" }",
            "leptoskit = { version = \"=0.3.0\", git = \"other\" }",
            "leptoskit = { path = \"framework\" }",
            "leptoskit = { workspace = true }",
            "renamed = { package = \"leptoskit\", version = \"=0.2.0\" }",
        ] {
            for section in [
                "[dependencies]",
                "[workspace.dependencies]",
                "[target.'cfg(unix)'.build-dependencies]",
            ] {
                let manifest = format!("{section}\n{dependency}\n");
                assert!(verify_generated(&manifest).is_err(), "accepted {manifest}");
            }
        }
    }
}
