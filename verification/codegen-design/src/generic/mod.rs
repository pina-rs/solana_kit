//! Bounded second-program generator with explicit input trust and finite types.
#[cfg(test)]
mod boundary;
mod emit;
mod layout;
#[cfg(test)]
mod oracle;
mod parse;
mod receipt;

#[cfg(test)]
mod tests;

use crate::{generate, schema::Error};
use serde_json::Value;
use solana_kit_core::Address;
use std::path::Path;

pub(crate) struct Policy<'a> {
    pub(crate) name: &'a str,
    pub(crate) address: Address,
}

pub(crate) struct Program {
    pub(crate) name: String,
    pub(crate) address: Address,
    pub(crate) definitions: Vec<Definition>,
    pub(crate) instructions: Vec<Instruction>,
}

pub(crate) struct Definition {
    pub(crate) name: String,
    pub(crate) shape: Shape,
}

pub(crate) enum Shape {
    Struct(Vec<Field>),
    Enum(Vec<String>),
}

pub(crate) struct Field {
    pub(crate) name: String,
    pub(crate) wire: Wire,
}

pub(crate) enum Wire {
    Number(&'static str),
    Link(String),
}

pub(crate) struct Instruction {
    pub(crate) name: String,
    pub(crate) tag: u8,
    pub(crate) fields: Vec<Field>,
}

pub(crate) fn render(root: &Value, policy: &Policy<'_>) -> Result<String, Error> {
    emit::program(&parse::program(root, policy)?)
}

pub(crate) fn replace(root: &Value, policy: &Policy<'_>, output: &Path) -> Result<(), Error> {
    generate::replace_source(&render(root, policy)?, output)
}

pub(crate) fn receipt() -> Result<(), Error> {
    receipt::run()
}

pub(crate) fn run() -> Result<(), Error> {
    let root = serde_json::from_str(include_str!("../../fixtures/compute-budget.json"))?;
    let policy = Policy {
        name: "computeBudget",
        address: "ComputeBudget111111111111111111111111111111".parse()?,
    };
    let destination = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/solana_kit_compute_budget/src/generated.rs");
    replace(&root, &policy, &destination)?;
    println!("generated five pinned Compute Budget instruction layouts");
    Ok(())
}
