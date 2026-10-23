//! Pure instruction planning for off-chain applications.
//!
//! Trees retain ordering and atomic bundle boundaries. Compilation is always
//! delegated to the Kit compiler, including signature reservation and lifetime
//! validation. No network, executor, signer, or ambient blockhash is selected.
mod async_builder;
mod packers;
mod planner;
mod results;
pub use results::{LeafResult, ResultSummary, Status, TransactionPlanResult};

pub use async_builder::{AsyncMessageBuilder, MessageBuilder};
pub use packers::{
    InstructionBatch, InstructionBatchPacker, Linear, LinearPacker, MessagePacker, PackerFactory,
};
pub use planner::{Planner, append_instruction_plan};

use crate::transaction::{self, Lifetime, LookupTable, Message, V1Config, Version};
use crate::{Address, Instruction};
use alloc::{vec, vec::Vec};
use core::fmt;

/// A recursive ordered plan, shared by instruction, message, and result trees.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Plan<T> {
    /// One leaf operation.
    Single(T),
    /// Children execute in order. An indivisible group requires a bundle.
    Sequential {
        /// Child plans in execution order.
        plans: Vec<Self>,
        /// Whether children may execute in separate batches.
        divisible: bool,
    },
    /// Independent children may execute concurrently.
    Parallel(Vec<Self>),
}

impl<T> Plan<T> {
    /// Creates a divisible sequence.
    #[must_use]
    pub fn sequential(plans: Vec<Self>) -> Self {
        Self::Sequential {
            plans,
            divisible: true,
        }
    }

    /// Creates a sequence requiring atomic execution as a transaction or bundle.
    #[must_use]
    pub fn indivisible(plans: Vec<Self>) -> Self {
        Self::Sequential {
            plans,
            divisible: false,
        }
    }

    /// Direct child plans, empty for a leaf.
    #[must_use]
    pub fn children(&self) -> &[Self] {
        match self {
            Self::Single(_) => &[],
            Self::Sequential { plans, .. } | Self::Parallel(plans) => plans,
        }
    }

    /// First matching node in preorder depth-first traversal.
    pub fn find(&self, predicate: &impl Fn(&Self) -> bool) -> Option<&Self> {
        if predicate(self) {
            return Some(self);
        }

        self.children()
            .iter()
            .find_map(|child| child.find(predicate))
    }

    /// Whether every node matches, short-circuiting before children.
    pub fn every(&self, predicate: &impl Fn(&Self) -> bool) -> bool {
        predicate(self) && self.children().iter().all(|child| child.every(predicate))
    }

    /// Leaf values in depth-first order.
    #[must_use]
    pub fn flatten(&self) -> Vec<&T> {
        match self {
            Self::Single(value) => vec![value],
            _ => self.children().iter().flat_map(Self::flatten).collect(),
        }
    }

    /// Maps leaves while preserving every ordering and bundle boundary.
    pub fn map<U>(self, function: &mut impl FnMut(T) -> U) -> Plan<U> {
        match self {
            Self::Single(value) => Plan::Single(function(value)),
            Self::Sequential { plans, divisible } => Plan::Sequential {
                plans: plans.into_iter().map(|child| child.map(function)).collect(),
                divisible,
            },
            Self::Parallel(plans) => {
                Plan::Parallel(plans.into_iter().map(|child| child.map(function)).collect())
            }
        }
    }

    /// Transforms children before their parent.
    pub fn transform(self, function: &mut impl FnMut(Self) -> Self) -> Self {
        let transformed = match self {
            Self::Single(_) => self,
            Self::Sequential { plans, divisible } => Self::Sequential {
                plans: plans
                    .into_iter()
                    .map(|child| child.transform(function))
                    .collect(),
                divisible,
            },
            Self::Parallel(plans) => Self::Parallel(
                plans
                    .into_iter()
                    .map(|child| child.transform(function))
                    .collect(),
            ),
        };

        function(transformed)
    }
}

/// Typed flexible input; singleton lists unwrap recursively and empty lists remain sequences.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanInput<T> {
    /// A bare leaf.
    Value(T),
    /// An already structured tree.
    Plan(Plan<T>),
    /// Nested input list.
    List(Vec<Self>),
}

impl<T> PlanInput<T> {
    /// Resolves a typed input without runtime casts or mixed-domain lists.
    #[must_use]
    pub fn parse(self) -> Plan<T> {
        match self {
            Self::Value(value) => Plan::Single(value),
            Self::Plan(plan) => plan,

            Self::List(mut inputs) if inputs.len() == 1 => inputs.remove(0).parse(),
            Self::List(inputs) => Plan::sequential(inputs.into_iter().map(Self::parse).collect()),
        }
    }
}

/// An instruction or a repeatable factory for a dynamic message packer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InstructionLeaf<F = InstructionBatch> {
    /// Exactly one owned instruction.
    Instruction(Instruction),
    /// A fresh independent packer is created for each planning attempt.
    Packer(F),
}

/// Instruction tree. Pattern matching replaces runtime kind assertions.
pub type InstructionPlan<F = InstructionBatch> = Plan<InstructionLeaf<F>>;
/// Immutable compiled transaction tree with preserved lifetime metadata.
pub type TransactionPlan = Plan<Message>;

impl<F> From<Instruction> for Plan<InstructionLeaf<F>> {
    fn from(value: Instruction) -> Self {
        Self::Single(InstructionLeaf::Instruction(value))
    }
}

/// Validated top-level instruction budget, inclusive of builder-injected instructions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MaxInstructions(u8);

/// Default budget leaving headroom for inner instructions.
pub const DEFAULT_MAX_INSTRUCTIONS: MaxInstructions = MaxInstructions(16);
/// Shared top-level instruction limit across supported wire versions.
pub const TRANSACTION_INSTRUCTION_LIMIT: u8 = 64;

impl Default for MaxInstructions {
    fn default() -> Self {
        DEFAULT_MAX_INSTRUCTIONS
    }
}
impl MaxInstructions {
    /// Validates the reference default's configurable 1..=64 range.
    ///
    /// # Errors
    /// Returns [`Error::InvalidMaxInstructions`] outside that range.
    pub const fn new(value: u8) -> Result<Self, Error> {
        if value == 0 || value > TRANSACTION_INSTRUCTION_LIMIT {
            return Err(Error::InvalidMaxInstructions);
        }

        Ok(Self(value))
    }

    /// Configured top-level count limit.
    #[must_use]
    pub const fn get(self) -> usize {
        self.0 as usize
    }

    /// Checks a count against this validated budget.
    ///
    /// # Errors
    /// Returns [`Error::InstructionLimit`] when the count exceeds the budget.
    pub fn check(self, actual: usize) -> Result<(), Error> {
        if actual > self.get() {
            return Err(Error::InstructionLimit {
                actual,
                maximum: self.get(),
            });
        }

        Ok(())
    }
}

/// Caller-owned source inputs for exact compilation of one prospective message.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MessageDraft {
    /// Wire version.
    pub version: Version,
    /// Explicit fee payer.
    pub payer: Address,
    /// Explicit blockhash expiry or nonce metadata.
    pub lifetime: Lifetime,
    /// Ordered owned instructions.
    pub instructions: Vec<Instruction>,
    /// Immutable lookup snapshots.
    pub lookup_tables: Vec<LookupTable>,
    /// Version-one configuration; rejected on other versions.
    pub config: Option<V1Config>,
}

impl MessageDraft {
    /// Compiles using actual packet, account, signature, nonce, and wire limits.
    ///
    /// # Errors
    /// Preserves compiler errors and rejects a configured instruction overflow.
    pub fn compile(&self, maximum: MaxInstructions) -> Result<Message, Error> {
        maximum.check(self.instructions.len())?;

        let message = match (self.version, self.config) {
            (Version::V1, Some(config)) if self.lookup_tables.is_empty() => {
                Message::compile_v1(self.payer, self.lifetime, &self.instructions, config)
            }

            (Version::V1, None) => Message::compile(
                self.version,
                self.payer,
                self.lifetime,
                &self.instructions,
                &self.lookup_tables,
            ),
            (Version::V1, Some(_)) | (_, Some(_)) => Err(transaction::Error::InvalidConfig),
            (_, None) => Message::compile(
                self.version,
                self.payer,
                self.lifetime,
                &self.instructions,
                &self.lookup_tables,
            ),
        }?;

        Ok(message)
    }
}

/// A planning failure. Unexpected compiler/builder failures never become retries.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// The plan contains no instructions or unfinished packers.
    EmptyPlan,
    /// The configured maximum must lie in 1..=64.
    InvalidMaxInstructions,
    /// A prospective message has too many instructions.
    InstructionLimit {
        /// Actual count.
        actual: usize,
        /// Configured limit.
        maximum: usize,
    },
    /// A custom packer rejected this candidate without consuming input.
    Rejected(alloc::string::String),
    /// A packer was invoked after completion.
    PackerComplete,
    /// A successful packer appended no instructions, even if it completed.
    NoProgress,
    /// A packer changed existing instructions or construction metadata.
    InvalidPackerOutput,
    /// A packer consumed input but returned an invalid compiled message.
    PackedMessage(alloc::boxed::Box<Error>),

    /// Exact compiler validation failed.
    Compile(transaction::Error),
    /// Caller-provided builder or update operation failed.
    Builder(alloc::string::String),
}

impl Error {
    /// Whether a fresh candidate can resolve this capacity/rejection error.
    #[must_use]
    pub const fn requires_new_candidate(&self) -> bool {
        matches!(
            self,
            Self::InstructionLimit { .. }
                | Self::Rejected(_)
                | Self::Compile(
                    transaction::Error::PacketTooLarge
                        | transaction::Error::InvalidIndex
                        | transaction::Error::InvalidV1Limits
                )
        )
    }
}
impl From<transaction::Error> for Error {
    fn from(value: transaction::Error) -> Self {
        Self::Compile(value)
    }
}
impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "instruction planning error: {self:?}")
    }
}
impl core::error::Error for Error {}
