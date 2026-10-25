//! Bounded second-program generator with explicit input trust and finite types.
mod account_output;
#[cfg(feature = "verification")]
mod account_prefix;
#[cfg(feature = "verification")]
mod anchor;
#[cfg(feature = "verification")]
mod anchor_types;
#[cfg(all(test, feature = "verification"))]
mod boundary;
mod codama_program;
#[cfg(all(test, feature = "verification"))]
mod codama_tests;
#[cfg(all(test, feature = "verification"))]
mod complete_tests;
mod constants;
mod counted_output;
pub(crate) mod data;
mod data_nodes;
mod data_shapes;
mod dispatch_output;
mod pda_output;
mod pda_policy;
#[cfg(feature = "verification")]
mod prepared_layouts;

mod emit;

mod enum_output;
mod equality;

mod errors;
#[cfg(all(test, feature = "verification"))]
mod expanded_tests;
mod expressions;
mod extent;
mod imports;

mod invocations;

mod layout;

#[cfg(all(test, feature = "verification"))]
mod header_tests;
mod instruction_header;
#[cfg(feature = "verification")]
mod normalize;
#[cfg(all(test, feature = "verification"))]
mod oracle;
#[cfg(all(test, feature = "verification"))]
mod owned_tests;
mod parse;
#[cfg(feature = "verification")]
mod provided;
#[cfg(feature = "verification")]
mod receipt;
mod record_checks;
mod roles;
mod tags;

#[cfg(feature = "verification")]
mod tlv_output;
mod traits_output;

mod wire;

#[cfg(all(test, feature = "verification"))]
mod tests;

#[cfg(feature = "verification")]
use crate::generate;
use crate::schema::Error;
use serde_json::Value;
use solana_kit_core::Address;
#[cfg(feature = "verification")]
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
    pub(crate) discriminator: Option<Vec<u8>>,
}

pub(crate) enum Shape {
    Struct(Vec<Field>),
    #[cfg(feature = "verification")]
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
    FixedText(usize),
    ZeroableAddress,
    #[cfg(feature = "verification")]
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
    pub(crate) privileges: RemainingPrivileges,
}

/// Dynamic invocation accounts preserve the caller's individual privileges.
#[derive(Clone, Copy)]
pub(crate) enum RemainingPrivileges {
    Uniform { signer: bool, writable: bool },
    CallerProvided,
}

pub(crate) struct Instruction {
    pub(crate) name: String,
    pub(crate) tag: instruction_header::Header,
    pub(crate) fields: Vec<Field>,
    pub(crate) roles: Vec<Role>,
    pub(crate) remaining: Option<Remaining>,
    pub(crate) omit_optional: bool,
}

#[cfg(feature = "verification")]
pub(crate) fn render(root: &Value, policy: &Policy<'_>) -> Result<String, Error> {
    emit::program(&parse::program(&normalize_input(root)?, policy)?)
}

#[cfg(feature = "verification")]
pub(crate) fn render_complete_program(root: &Value, policy: &Policy<'_>) -> Result<String, Error> {
    codama_program::render(root, policy)
}

/// Private source tables declare separate account headers through an explicit adapter.
#[cfg(feature = "verification")]
pub(crate) fn render_prefixed_program(root: &Value, policy: &Policy<'_>) -> Result<String, Error> {
    account_prefix::render(root, policy)
}

#[cfg(feature = "verification")]
pub(crate) fn render_public_program(root: &Value, policy: &Policy<'_>) -> Result<String, Error> {
    let program = parse::program(&normalize_input(root)?, policy)?;
    let mut output = emit::program(&program)?;
    output.push_str(&traits_output::emit(&program)?);
    Ok(output)
}

/// Exposes the same structural normalization for frozen source-backed input snapshots.
#[cfg(feature = "verification")]
pub(crate) fn normalize_input(root: &Value) -> Result<Value, Error> {
    normalize::records(root)
}

#[cfg(feature = "verification")]
pub(crate) fn replace(root: &Value, policy: &Policy<'_>, output: &Path) -> Result<(), Error> {
    generate::replace_source(&render(root, policy)?, output)
}

#[cfg(feature = "verification")]
pub(crate) fn receipt() -> Result<(), Error> {
    receipt::run()
}

#[cfg(feature = "verification")]
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

#[cfg(feature = "verification")]
pub(crate) use anchor::{AnchorPolicy, prepare as prepare_anchor};
#[cfg(feature = "verification")]
pub(crate) use pda_policy::{Default as PdaDefault, Seed as PdaSeed};

#[cfg(feature = "verification")]
pub(crate) fn pda_recipes(nodes: &[Value]) -> Result<Vec<pda_policy::Pda>, Error> {
    pda_policy::recipes(nodes)
}

#[cfg(feature = "verification")]
pub(crate) fn pda_defaults(
    nodes: &[Value],
    recipes: &[pda_policy::Pda],
) -> Result<Vec<pda_policy::RoleDefault>, Error> {
    pda_policy::defaults(nodes, recipes)
}

/// Uses the public Codama schema contract with the shared lowering and emitter.
pub(crate) fn render_codama_program(root: &Value, policy: &Policy<'_>) -> Result<String, Error> {
    codama_program::render_codama(root, policy)
}
