//! Runtime selection belongs to the native table, separate from compile-time plugins.
use super::Result;

pub(super) fn select(contents: &str, plugin: &str) -> Result<String> {
    let config: toml::Table = contents.parse()?;
    let native = config
        .get("native")
        .and_then(toml::Value::as_table)
        .ok_or("expected a generated native table")?;

    if let Some(current) = native.get("plugins") {
        let selected = toml::Value::Array(vec![plugin.into()]);

        if current == &selected {
            return Ok(contents.to_owned());
        }

        return Err("review the existing native runtime selection before preparation".into());
    }

    let header = "[native]\n";

    if contents.matches(header).count() != 1 {
        return Err("public native table shape changed; inspect the generated template".into());
    }

    Ok(contents.replacen(header, &format!("{header}plugins = [{plugin:?}]\n"), 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    const GENERATED: &str = concat!(
        "schema_version = 0\nplugins = []\n\n[app]\nname = \"fixture\"\n\n",
        "[native]\npackage_id = \"com.leptoskit.fixture\"\n\n[codepush]\nenabled = false\n",
    );

    #[test]
    fn native_runtime_does_not_replace_compile_time_plugin_specs() {
        for runtime in ["solana-wallet", "solana-journey"] {
            let selected = select(GENERATED, runtime).unwrap();
            let mut parsed: toml::Table = selected.parse().unwrap();
            let native = parsed.get_mut("native").unwrap().as_table_mut().unwrap();
            assert_eq!(
                native.remove("plugins"),
                Some(toml::Value::Array(vec![runtime.into()]))
            );
            let original: toml::Table = GENERATED.parse().unwrap();
            assert_eq!(parsed, original);
            assert_eq!(select(&selected, runtime).unwrap(), selected);
        }
    }

    #[test]
    fn does_not_replace_another_runtime_or_an_unrecognized_native_shape() {
        let selected = select(GENERATED, "solana-wallet").unwrap();
        assert!(select(&selected, "solana-journey").is_err());
        assert!(select("plugins = []\n", "solana-wallet").is_err());
        assert!(
            select(
                &GENERATED.replace("[native]", "[ native ]"),
                "solana-wallet"
            )
            .is_err()
        );
    }
}
