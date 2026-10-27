//! Maintains the portable registry from exact public contract entrypoints.
use crate::{generate, schema::Error};
use std::{fs, path::Path};
use syn::parse::Parser;

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
    ("stake_activation", "solana_kit_stake", "activation"),
    ("stake_withdrawal", "solana_kit_stake", "withdrawal"),
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
pub(crate) fn entrypoints(source: &str) -> Result<Vec<String>, Error> {
    let names = item_entrypoints(&syn::parse_file(source)?.items)?;

    if names.is_empty() {
        return Err("contract file has no portable bodies".into());
    }

    Ok(names)
}

/// Reads module-level owners, including literal case macros and reexported DAS bodies.
fn item_entrypoints(items: &[syn::Item]) -> Result<Vec<String>, Error> {
    let mut names = Vec::new();

    for item in items {
        match item {
            syn::Item::Fn(function) => {
                names.extend(function_name(function)?);
            }

            syn::Item::Macro(item) if item.mac.path.is_ident("case") => {
                names.push(case_name(&item.mac)?);
            }

            syn::Item::Macro(item) if item.mac.path.is_ident("cases") => {
                names.extend(assigned_cases(&item.mac)?);
            }

            syn::Item::Mod(syn::ItemMod {
                content: Some((_, items)),
                ..
            }) => {
                names.extend(item_entrypoints(items)?);
            }

            _ => {}
        }
    }

    Ok(names)
}

/// Private helpers are excluded; public behavior owners must be directly callable.
fn function_name(function: &syn::ItemFn) -> Result<Option<String>, Error> {
    if !matches!(function.vis, syn::Visibility::Public(_)) {
        return Ok(None);
    }

    if !function.sig.inputs.is_empty()
        || !matches!(function.sig.output, syn::ReturnType::Default)
        || !function.sig.generics.params.is_empty()
        || function.sig.asyncness.is_some()
        || !matches!(function.sig.safety, syn::Safety::Default)
    {
        return Err("contract must be a synchronous zero-argument unit body".into());
    }

    Ok(Some(function.sig.ident.to_string()))
}

/// The first case argument names the actual function emitted by the source macro.
fn case_name(item: &syn::Macro) -> Result<String, Error> {
    let mut tokens = item.tokens.clone().into_iter();
    let Some(proc_macro2::TokenTree::Ident(name)) = tokens.next() else {
        return Err("contract case name required".into());
    };

    let delimiter = tokens.next();

    if !matches!(delimiter, Some(proc_macro2::TokenTree::Punct(comma)) if comma.as_char() == ',') {
        return Err("contract case arguments required".into());
    }

    Ok(name.to_string())
}

/// Squads names every emitted behavior owner with a literal fixture index.
fn assigned_cases(item: &syn::Macro) -> Result<Vec<String>, Error> {
    let parser = |input: syn::parse::ParseStream<'_>| {
        syn::punctuated::Punctuated::<String, syn::Token![,]>::parse_terminated_with(
            input,
            assigned_case,
        )
    };
    let names = parser.parse2(item.tokens.clone())?;

    if names.is_empty() {
        return Err("contract cases must name at least one body".into());
    }

    Ok(names.into_iter().collect())
}

/// Expressions and noninteger indices cannot silently reduce portable coverage.
fn assigned_case(input: syn::parse::ParseStream<'_>) -> syn::Result<String> {
    let name = input.parse::<syn::Ident>()?;
    input.parse::<syn::Token![=]>()?;
    input.parse::<syn::LitInt>()?.base10_parse::<usize>()?;

    Ok(name.to_string())
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
        assert_eq!(count, 88);
        assert!(
            source.contains("stake_options::all_lockup_option_combinations_have_literal_frames")
        );
    }

    #[test]
    fn macro_cases_and_reexported_bodies_preserve_source_order() {
        assert_eq!(
            entrypoints(
                "case!(wire, Data, Codec, value, bytes);\n\
                 mod enabled { pub fn provider() {} fn helper() {} }"
            )
            .unwrap(),
            ["wire", "provider"]
        );
        assert!(entrypoints("case!(123, Data);").is_err());
        assert!(entrypoints("pub fn value() -> usize { 1 }").is_err());
        assert!(entrypoints("pub async fn pending() {}").is_err());
    }

    #[test]
    fn indexed_macro_owners_preserve_names_and_reject_unexecutable_cases() {
        assert_eq!(
            entrypoints("cases!(first = 0, second = 1,);").unwrap(),
            ["first", "second"]
        );

        for source in [
            "cases!();",
            "cases!(first = compute());",
            "cases!(first = -1);",
            "cases!(first = 0 second = 1);",
            "cases!(first = 0, 1 = second);",
        ] {
            assert!(entrypoints(source).is_err(), "{source}");
        }
    }
}
