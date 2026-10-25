//! Generated manifests use the same ordinary Git source as the SDK adapter.
use super::Result;
use std::{fs, path::Path};

pub(super) const REVISION: &str = "25b1861ddc729a8f3b70117cd0c48e06659c9697";
const SOURCE: &str = "https://github.com/ifiokjr/leptoskit";
const VERSION: &str = "{ version = \"=0.1.0\"";

pub(super) fn verify_sdk(sdk: &Path) -> Result<()> {
    let manifest = fs::read_to_string(sdk.join("crates/solana_kit_leptoskit/Cargo.toml"))?;
    let dependencies: Vec<_> = manifest
        .lines()
        .filter(|line| line.starts_with("leptoskit_") && line.contains(SOURCE))
        .collect();
    let pin = format!("rev = {REVISION:?}");

    if dependencies.len() != 5 || dependencies.iter().any(|line| !line.contains(&pin)) {
        return Err("SDK framework dependencies must select the verified revision".into());
    }

    Ok(())
}

pub(super) fn pin(contents: &str) -> Result<String> {
    if contents.contains("[patch.") {
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
    let source_keys = [
        "git =",
        "path =",
        "rev =",
        "branch =",
        "tag =",
        "package =",
        "registry =",
    ];

    if !suffix.starts_with(',') || source_keys.iter().any(|key| suffix.contains(key)) {
        return Err("review the generated framework dependency before changing its source".into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
