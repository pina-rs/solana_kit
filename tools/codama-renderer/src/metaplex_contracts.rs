//! Reuses each public Compression and Metaplex behavior owner without copying assertions.
use crate::{generate, program_contracts::entrypoints, schema::Error};
use std::{fs, path::Path};

const INPUTS: &[(&str, &str, &str)] = &[
    (
        "compression_ix",
        "solana_kit_account_compression",
        "instructions",
    ),
    (
        "compression_state",
        "solana_kit_account_compression",
        "state",
    ),
    ("compression_tree", "solana_kit_account_compression", "tree"),
    ("core_acc", "solana_kit_mpl_core", "accounts"),
    ("core_errors", "solana_kit_mpl_core", "errors"),
    ("core_ext", "solana_kit_mpl_core", "external"),
    ("core_ix", "solana_kit_mpl_core", "instructions"),
    ("core_pda", "solana_kit_mpl_core", "pda"),
    ("core_plugins", "solana_kit_mpl_core", "plugins"),
    (
        "metadata_accounts",
        "solana_kit_mpl_token_metadata",
        "accounts",
    ),
    ("metadata_errors", "solana_kit_mpl_token_metadata", "errors"),
    (
        "metadata_helpers",
        "solana_kit_mpl_token_metadata",
        "helpers",
    ),
    (
        "metadata_instructions",
        "solana_kit_mpl_token_metadata",
        "instructions",
    ),
    ("metadata_pdas", "solana_kit_mpl_token_metadata", "pdas"),
    (
        "metadata_variants",
        "solana_kit_mpl_token_metadata",
        "variants",
    ),
];
const BUBBLEGUM: &[&str] = &[
    "das", "enums", "hashing", "helpers", "options", "pdas", "plans", "state", "wire",
];

/// Uses the shared atomic Rust formatter boundary and leaves all original bodies intact.
pub(crate) fn run() -> Result<(), Error> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let (source, count) = render(&root)?;
    let folder = root.join("verification/metaplex-portable");
    generate::replace_source(&source, &folder.join("src/registry.rs"))?;
    fs::write(
        folder.join("report.txt"),
        format!("metaplex-clients: {count}/{count}"),
    )?;
    println!("PASS exact {count}-body Metaplex registry");

    Ok(())
}

/// Keeps Bubblegum's sibling modules together so original fixture imports retain their meaning.
fn render(root: &Path) -> Result<(String, usize), Error> {
    let mut modules = String::from(
        "#[path = \"../../../crates/solana_kit_mpl_bubblegum/tests/contracts.rs\"]\nmod bubblegum;\n",
    );
    let mut cases = Vec::new();

    for (module, package, file) in INPUTS {
        let path = format!("crates/{package}/tests/{file}.rs");
        modules.push_str(&format!("#[path = \"../../../{path}\"]\nmod {module};\n"));
        add_cases(root, &path, module, &mut cases)?;
    }

    for file in BUBBLEGUM {
        let path = format!("crates/solana_kit_mpl_bubblegum/tests/contracts/{file}.rs");
        add_cases(root, &path, &format!("bubblegum::{file}"), &mut cases)?;
    }

    let count = cases.len();
    let source = format!(
        "//! Exact contract entrypoints; maintained by metaplex-contracts.\n{modules}\n\
         pub const COUNT: usize = {count};\n\
         pub const CASES: [(&str, fn()); COUNT] = [{}];\n",
        cases.concat(),
    );

    Ok((source, count))
}

/// A compiled function reference ties every generated case to the unchanged source owner.
fn add_cases(root: &Path, path: &str, module: &str, cases: &mut Vec<String>) -> Result<(), Error> {
    for name in entrypoints(&fs::read_to_string(root.join(path))?)? {
        cases.push(format!(
            "(\"{module}::{name}\", {module}::{name} as fn()),\n"
        ));
    }

    Ok(())
}

#[cfg(all(test, feature = "verification"))]
mod tests {
    use super::*;

    #[test]
    fn registry_includes_literal_macro_cases_and_original_provider_cancellation() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let (source, count) = render(&root).unwrap();
        assert_eq!(count, 158);
        assert!(source.contains("core_ix::create_v1"));
        assert!(source.contains("metadata_instructions::create_metadata_account"));
        assert!(source.contains("bubblegum::das::request_cancellation"));
    }
}
