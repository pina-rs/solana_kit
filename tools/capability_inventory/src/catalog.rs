//! Reconciles operation catalogs from their literal source lists.

use crate::{
    model::Result,
    parse::{descendants, text},
};
use std::{collections::BTreeSet, fs, path::Path};
use tree_sitter::{Node, Parser};

pub(crate) struct Catalogs {
    pub rpc: BTreeSet<String>,
    pub subscriptions: BTreeSet<String>,
    pub programs: BTreeSet<String>,
}

impl Catalogs {
    /// Source lists retain spread semantics by unioning both named lists.
    pub fn load(reference: &Path, packages: &[crate::model::Package]) -> Result<Self> {
        let rpc = lists(
            reference,
            "solana_kit_rpc_api",
            "solana_rpc_api.dart",
            &[
                "solanaRpcMethodsForAllClusters",
                "solanaRpcMethodsForTestClusters",
            ],
        )?;
        let subscriptions = lists(
            reference,
            "solana_kit_rpc_subscriptions_api",
            "solana_rpc_subscriptions_api.dart",
            &[
                "solanaRpcSubscriptionsMethodsStable",
                "solanaRpcSubscriptionsMethodsUnstable",
            ],
        )?;
        let programs = packages
            .iter()
            .filter(|package| package.phase == "6" && package.name != "codama-renderers-dart")
            .map(|package| package.name.clone())
            .collect::<BTreeSet<_>>();

        if (rpc.len(), subscriptions.len(), programs.len()) != (54, 9, 17) {
            return Err(format!(
                "catalog mismatch: RPC {}, subscriptions {}, programs {}",
                rpc.len(),
                subscriptions.len(),
                programs.len()
            )
            .into());
        }

        Ok(Self {
            rpc,
            subscriptions,
            programs,
        })
    }
}

/// Reads literal values only from the named top-level variable declarations.
fn lists(reference: &Path, package: &str, file: &str, names: &[&str]) -> Result<BTreeSet<String>> {
    let source = fs::read_to_string(reference.join(format!("packages/{package}/lib/src/{file}")))?;
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_dart::LANGUAGE.into())?;
    let tree = parser.parse(&source, None).ok_or("catalog parse failed")?;
    let mut values = BTreeSet::new();
    let mut found = BTreeSet::new();

    for declaration in descendants(tree.root_node(), "top_level_variable_declaration") {
        let Some(identifier) = descendants(declaration, "static_final_declaration")
            .into_iter()
            .next()
        else {
            continue;
        };
        let Some(name) = identifier.child_by_field_name("name") else {
            continue;
        };
        let name = text(name, &source);

        if names.contains(&name) {
            found.insert(name.to_owned());
            literals(declaration, &source, &mut values);
        }
    }

    if found.len() != names.len() {
        return Err("missing source catalog declaration".into());
    }

    Ok(values)
}

/// Avoids documentation strings and identifier guesses when counting operations.
fn literals(node: Node<'_>, source: &str, values: &mut BTreeSet<String>) {
    for literal in descendants(node, "string_literal") {
        values.insert(text(literal, source).trim_matches(['\'', '"']).to_owned());
    }
}
