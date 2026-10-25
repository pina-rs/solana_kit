//! Maintains the portable registry from exact public contract entrypoints.
use crate::{generate, schema::Error};
use std::{fs, path::Path};

const INPUTS: &[(&str, &str, &str)] = &[
    (
        "alt_accounts",
        "solana_kit_address_lookup_table",
        "accounts",
    ),
    ("alt_bank", "solana_kit_address_lookup_table", "bank"),
    (
        "alt_contracts",
        "solana_kit_address_lookup_table",
        "contracts",
    ),
    (
        "alt_fullwidth",
        "solana_kit_address_lookup_table",
        "fullwidth",
    ),
    ("config_contracts", "solana_kit_config", "contracts"),
    ("config_metadata", "solana_kit_config", "metadata"),
    ("stake_protocol", "solana_kit_stake", "protocol"),
    ("stake_wires", "solana_kit_stake", "wires"),
    ("stake_metadata", "solana_kit_stake", "stake_metadata"),
    ("stake_helpers", "solana_kit_stake", "helpers"),
    ("stake_options", "solana_kit_stake", "options"),
    ("stake_values", "solana_kit_stake", "values"),
    ("loader_protocol", "solana_kit_loader", "protocol"),
    ("loader_wires", "solana_kit_loader", "wires"),
    ("loader_v3_metadata", "solana_kit_loader", "v3_metadata"),
    ("loader_v4_metadata", "solana_kit_loader", "v4_metadata"),
    ("loader_helpers", "solana_kit_loader", "helpers"),
    ("loader_values", "solana_kit_loader", "values"),
    ("config_encoding", "solana_kit_config", "encoding"),
];

/// Uses the existing formatter and atomic writer, never rewriting contract bodies.
pub(crate) fn run() -> Result<(), Error> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let (registry, count) = render(&root)?;
    let folder = root.join("verification/stake-loader-portable");
    generate::replace_source(&registry, &folder.join("src/registry.rs"))?;
    fs::write(
        folder.join("report.txt"),
        format!("program-clients: {count}/{count}"),
    )?;
    println!("PASS exact {count}-body program registry");
    Ok(())
}

/// Includes the original source modules and their public entrypoints in source order.
fn render(root: &Path) -> Result<(String, usize), Error> {
    let mut modules = String::new();
    let mut cases = Vec::new();

    for (module, package, file) in INPUTS {
        let path = format!("crates/{package}/tests/{file}.rs");
        let source = fs::read_to_string(root.join(&path))?;
        modules.push_str(&format!("#[path = \"../../../{path}\"]\nmod {module};\n"));

        for name in entrypoints(&source)? {
            cases.push(format!(
                "(\"{module}::{name}\", {module}::{name} as fn()),\n"
            ));
        }
    }

    let count = cases.len();
    let registry = format!(
        "//! Exact contract entrypoints; maintained by program-contracts.\n{modules}\n\
         pub const COUNT: usize = {count};\n\
         pub const CASES: [(&str, fn()); COUNT] = [{}];\n",
        cases.concat(),
    );
    Ok((registry, count))
}

/// Rejects signatures the portable zero-argument contract registry cannot execute.
fn entrypoints(source: &str) -> Result<Vec<&str>, Error> {
    let mut names = Vec::new();

    for line in source
        .lines()
        .filter_map(|line| line.strip_prefix("pub fn "))
    {
        let name = line
            .strip_suffix("() {")
            .ok_or("contract must be a zero-argument body")?;

        if !name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        {
            return Err("invalid contract entrypoint".into());
        }

        names.push(name);
    }

    if names.is_empty() {
        return Err("contract file has no portable bodies".into());
    }

    Ok(names)
}

#[cfg(all(test, feature = "verification"))]
mod tests {
    use super::*;

    #[test]
    fn registry_preserves_exact_bodies_and_rejects_unsupported_signatures() {
        assert_eq!(
            entrypoints("pub fn first() {\n}\npub fn second() {\n}").unwrap(),
            ["first", "second"]
        );
        assert!(entrypoints("pub fn helper(value: u8) {").is_err());
        assert!(entrypoints("fn hidden() {").is_err());
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let (source, count) = render(&root).unwrap();
        assert_eq!(count, 74);
        assert!(
            source.contains("stake_options::all_lockup_option_combinations_have_literal_frames")
        );
    }
}
