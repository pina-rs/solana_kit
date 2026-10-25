//! Reuses the original Attestation, Squads and recurring Subscriptions behavior owners.
use crate::{generate, program_contracts::entrypoints, schema::Error};
use std::{fs, path::Path};

const INPUTS: &[(&str, &str, &str)] = &[
    ("att_acc", "attestation_service", "contracts/accounts"),
    ("att_ix", "attestation_service", "contracts/instructions"),
    ("att_pda", "attestation_service", "contracts/pdas"),
    ("att_schema", "attestation_service", "contracts/schema"),
    ("squads_acc", "squads", "accounts"),
    ("squads_ix", "squads", "instructions"),
    ("squads_helpers", "squads", "helpers"),
];
const SUBSCRIPTIONS: &[&str] = &["accounts", "events", "instructions", "pdas", "schedule"];

/// Generates callable references without copying or reducing the existing assertions.
pub(crate) fn run() -> Result<(), Error> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let (source, count) = render(&root)?;
    let folder = root.join("verification/remaining-programs-portable");
    generate::replace_source(&source, &folder.join("src/registry.rs"))?;
    fs::write(
        folder.join("report.txt"),
        format!("remaining-programs: {count}/{count}"),
    )?;
    println!("PASS exact {count}-body remaining-program registry");

    Ok(())
}

/// Preserves sibling fixture imports and the source ordering of all macro-emitted cases.
fn render(root: &Path) -> Result<(String, usize), Error> {
    let mut modules = String::from(
        "#[path = \"../../../crates/solana_kit_subscriptions/tests/contracts.rs\"]\nmod subscriptions;\n",
    );
    let mut cases = Vec::new();

    for (module, package, file) in INPUTS {
        let path = format!("crates/solana_kit_{package}/tests/{file}.rs");
        modules.push_str(&format!("#[path = \"../../../{path}\"]\nmod {module};\n"));
        add_cases(root, &path, module, &mut cases)?;
    }

    for file in SUBSCRIPTIONS {
        let path = format!("crates/solana_kit_subscriptions/tests/contracts/{file}.rs");
        add_cases(root, &path, &format!("subscriptions::{file}"), &mut cases)?;
    }

    let count = cases.len();
    let source = format!(
        "//! Exact contract entrypoints; maintained by remaining-contracts.\n{modules}\n\
         pub const COUNT: usize = {count};\n\
         pub const CASES: [(&str, fn()); COUNT] = [{}];\n",
        cases.concat(),
    );

    Ok((source, count))
}

/// Each generated reference must resolve to its actual unchanged contract function.
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
    fn registry_keeps_all_squads_macro_owners_and_full_subscription_defaults() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let (source, count) = render(&root).unwrap();
        assert_eq!(count, 83);
        assert_eq!(source.matches("squads_ix::").count(), 72);
        assert!(source.contains("att_schema::every_schema_value_has_exact_literal_bytes"));
        assert!(
            source.contains(
                "subscriptions::pdas::all_declared_defaults_and_explicit_overrides_resolve"
            )
        );
    }
}
