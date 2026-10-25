//! Generated manifests use the same ordinary Git source as the SDK adapter.
use super::Result;
use std::{fs, path::Path};

pub(super) const REVISION: &str = "25b1861ddc729a8f3b70117cd0c48e06659c9697";
const SOURCE: &str = "https://github.com/ifiokjr/leptoskit";
const VERSION: &str = "{ version = \"=0.1.0\"";
const SDK_FRAMEWORK_DECLARATIONS: usize = 5;

pub(super) fn verify_sdk(sdk: &Path) -> Result<()> {
    let manifest = fs::read_to_string(sdk.join("crates/solana_kit_leptoskit/Cargo.toml"))?;
    verify_manifest(&manifest)
}

fn verify_manifest(manifest: &str) -> Result<()> {
    let manifest: toml::Table = manifest.parse()?;
    let mut dependencies = Vec::new();
    collect_dependencies(&manifest, &mut dependencies)?;

    if let Some(targets) = manifest.get("target") {
        for target in targets
            .as_table()
            .ok_or("expected Cargo target tables")?
            .values()
        {
            collect_dependencies(
                target.as_table().ok_or("expected a Cargo target table")?,
                &mut dependencies,
            )?;
        }
    }

    if dependencies.len() != SDK_FRAMEWORK_DECLARATIONS
        || dependencies
            .iter()
            .any(|dependency| !same_source(dependency))
    {
        return Err("SDK framework dependencies must select the verified revision".into());
    }

    Ok(())
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
                        .ok_or("framework requires an explicit Git source")?,
                );
            }
        }
    }

    Ok(())
}

fn same_source(dependency: &toml::Table) -> bool {
    dependency.get("git").and_then(toml::Value::as_str) == Some(SOURCE)
        && dependency.get("rev").and_then(toml::Value::as_str) == Some(REVISION)
        && [
            "path",
            "branch",
            "tag",
            "registry",
            "registry-index",
            "workspace",
        ]
        .iter()
        .all(|key| !dependency.contains_key(*key))
}

pub(super) fn pin(contents: &str) -> Result<String> {
    let manifest: toml::Table = contents.parse()?;

    if manifest.contains_key("patch") {
        return Err("generated consumer must not use Cargo source patches".into());
    }

    let mut output = String::new();

    for line in contents.split_inclusive('\n') {
        let replacement = dependency(line)?;
        output.push_str(replacement.as_deref().unwrap_or(line));
    }

    Ok(output)
}

fn dependency(line: &str) -> Result<Option<String>> {
    let Some((name, value)) = line.split_once('=') else {
        return Ok(None);
    };
    let name = name.trim();

    if name != "leptoskit" && !name.starts_with("leptoskit_") {
        return Ok(None);
    }

    let pinned = format!("{VERSION}, git = {SOURCE:?}, rev = {REVISION:?}");
    let value = value.trim_start();

    if let Some(suffix) = value.strip_prefix(&pinned) {
        validate_suffix(suffix)?;

        return Ok(None);
    }

    let suffix = value
        .strip_prefix(VERSION)
        .ok_or("generated framework dependency shape changed")?;

    validate_suffix(suffix)?;
    Ok(Some(format!("{name} = {pinned}{suffix}")))
}

fn validate_suffix(suffix: &str) -> Result<()> {
    let declaration: toml::Table = format!("dependency = {VERSION}{suffix}").parse()?;
    let dependency = declaration["dependency"]
        .as_table()
        .ok_or("expected an inline framework dependency")?;
    let source_keys = [
        "git",
        "path",
        "rev",
        "branch",
        "tag",
        "package",
        "registry",
        "registry-index",
        "workspace",
    ];

    if !suffix.starts_with(',') || source_keys.iter().any(|key| dependency.contains_key(*key)) {
        return Err("review the generated framework dependency before changing its source".into());
    }

    Ok(())
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
            "[ patch . \"https://github.com/ifiokjr/leptoskit\" ]\nexample = { path = \"other\" }\n",
            "[\"patch\".crates-io]\nexample = { path = \"other\" }\n",
            "patch = { crates-io = { example = { path = \"other\" } } }\n",
        ] {
            assert!(pin(source).is_err(), "accepted an alternate patch table");
        }
    }

    #[test]
    fn pins_generated_dependencies_and_preserves_features_and_local_packages() {
        let original = concat!(
            "[dependencies]\n",
            "leptoskit = { version = \"=0.1.0\", default-features = false, ",
            "features = [\"guest\"] }\nsolana_kit_leptoskit = { path = \"sdk\" }\n"
        );
        let changed = pin(original).unwrap();
        assert!(changed.contains(&format!("git = {SOURCE:?}, rev = {REVISION:?}")));
        assert!(changed.contains("default-features = false, features = [\"guest\"] }\n"));
        assert!(changed.contains("solana_kit_leptoskit = { path = \"sdk\" }\n"));
        assert_eq!(pin(&changed).unwrap(), changed);
    }

    #[test]
    fn refuses_existing_overrides_and_unreviewed_framework_sources() {
        for source in [
            "[patch.crates-io]\n",
            "leptoskit = { version = \"=0.2.0\", default-features = false }\n",
            "leptoskit = { version = \"=0.1.0\", path = \"framework\" }\n",
            "leptoskit = { version = \"=0.1.0\", git = \"other\" }\n",
        ] {
            assert!(pin(source).is_err());
        }
    }

    #[test]
    fn refuses_extra_sources_on_an_already_pinned_dependency() {
        let original = "leptoskit = { version = \"=0.1.0\", default-features = false }\n";
        let changed = pin(original).unwrap().replace(" }", ", path = \"other\" }");
        assert!(pin(&changed).is_err());
    }
}
