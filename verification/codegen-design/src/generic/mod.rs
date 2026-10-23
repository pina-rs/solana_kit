//! Bounded second-program generator with explicit input trust and finite types.
mod account_output;
#[cfg(test)]
mod boundary;
mod constants;
mod counted_output;
pub(crate) mod data;
mod data_nodes;
mod data_shapes;
mod dispatch_output;

mod emit;

mod enum_output;
mod equality;

mod errors;
#[cfg(test)]
mod expanded_tests;
mod expressions;
mod extent;
mod imports;

mod invocations;

mod layout;

mod normalize;
#[cfg(test)]
mod oracle;
mod parse;
mod provided;
mod receipt;
mod roles;
mod tags;

mod tlv_output;

mod wire;

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
    pub(crate) accounts: Vec<Account>,
    pub(crate) errors: Vec<ProgramError>,
}

pub(crate) struct Account {
    pub(crate) definition: Definition,
    pub(crate) size: usize,
}
pub(crate) struct ProgramError {
    pub(crate) name: String,
    pub(crate) code: u32,
    pub(crate) message: String,
}

pub(crate) struct Definition {
    pub(crate) name: String,
    pub(crate) shape: Shape,
}

pub(crate) enum Shape {
    Struct(Vec<Field>),
    Enum(Vec<String>),
    Tagged {
        prefix: &'static str,
        variants: Vec<Variant>,
    },
}

pub(crate) struct Variant {
    pub(crate) name: String,
    pub(crate) payload: Option<Wire>,
}

#[derive(Clone)]
pub(crate) struct Field {
    pub(crate) name: String,
    pub(crate) wire: Wire,
}

#[derive(Clone)]
pub(crate) enum Wire {
    Number(&'static str),
    Link(String),
    Address,
    Bool,
    Text,
    Bytes,
    FixedBytes(usize),
    ZeroableAddress,
    TlvSequence(Box<Wire>),
    RemainderOption(Box<Wire>),
    HiddenPrefix {
        item: Box<Wire>,
        bytes: Vec<u8>,
    },
    Option {
        item: Box<Wire>,
        prefix: &'static str,
        fixed: bool,
    },
    Array {
        item: Box<Wire>,
        count: Option<usize>,
    },
    Prefix {
        item: Box<Wire>,
        prefix: &'static str,
    },
    Counted {
        item: Box<Wire>,
        prefix: &'static str,
    },
    Tuple(Box<Wire>, Box<Wire>),
}

pub(crate) enum Signer {
    Never,
    Required,
    Either,
}
pub(crate) struct Role {
    pub(crate) name: String,
    pub(crate) writable: bool,
    pub(crate) signer: Signer,
    pub(crate) optional: bool,
    pub(crate) link: Option<String>,
}
pub(crate) struct Remaining {
    pub(crate) signer: bool,
    pub(crate) writable: bool,
}

pub(crate) struct Instruction {
    pub(crate) name: String,
    pub(crate) tag: u8,
    pub(crate) fields: Vec<Field>,
    pub(crate) roles: Vec<Role>,
    pub(crate) remaining: Option<Remaining>,
    pub(crate) omit_optional: bool,
}

pub(crate) fn render(root: &Value, policy: &Policy<'_>) -> Result<String, Error> {
    emit::program(&parse::program(&normalize_input(root)?, policy)?)
}

/// Exposes the same structural normalization for frozen source-backed input snapshots.
pub(crate) fn normalize_input(root: &Value) -> Result<Value, Error> {
    normalize::records(root)
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
