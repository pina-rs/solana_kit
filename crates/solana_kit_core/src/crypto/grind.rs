//! Finite key grinding with caller-owned entropy and explicit budgets.
//!
//! Pinned Dart exposes one generator with a dynamic `RegExp` or predicate and an
//! unbounded retry loop. This Rust adaptation keeps entropy, execution and
//! cancellation in the caller's hands. [`GrindPattern`] validates literal prefix
//! and suffix patterns before any key is generated. Arbitrary matching,
//! including a flattened source expression, is caller predicate composition
//! through [`grind_keypairs_with`]; no regular-expression engine is included.
//!
//! Every generated key owns a zeroizing signer. Rejected candidates are dropped
//! immediately, and retained candidates are dropped when the budget is
//! exhausted, the entropy host fails, or the caller cancels the returned future.

use alloc::{string::ToString, vec::Vec};
use core::{fmt, future::poll_fn, task::Poll};

use super::{Entropy, ExportableKeypair};
use crate::Address;

/// Longest canonical base58 text for a 32-byte address.
const MAX_LITERAL_LENGTH: usize = 44;
const CANDIDATES_PER_BATCH: usize = 32;

/// Every base58 symbol in canonical ASCII order.
const BASE58_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// The side of the address text a literal pattern compares.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Position {
    Prefix,
    Suffix,
}

/// One validated literal base58 address match.
///
/// Literals are case-sensitive and compare against the canonical address text
/// produced by [`Address`]. An empty literal matches every address.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GrindPattern<'a> {
    literal: &'a str,
    position: Position,
}

impl<'a> GrindPattern<'a> {
    /// Validates a literal address prefix.
    ///
    /// # Errors
    /// Rejects characters outside the base58 alphabet, which no address can
    /// contain, and literals longer than the 44-character address maximum.
    pub fn prefix(literal: &'a str) -> Result<Self, GrindPatternError> {
        Self::new(literal, Position::Prefix)
    }

    /// Validates a literal address suffix.
    ///
    /// # Errors
    /// Returns the same validation failures as [`Self::prefix`].
    pub fn suffix(literal: &'a str) -> Result<Self, GrindPatternError> {
        Self::new(literal, Position::Suffix)
    }

    /// The validated literal compared by this pattern.
    #[must_use]
    pub const fn literal(self) -> &'a str {
        self.literal
    }

    /// Tests one address without generating a key.
    #[must_use]
    pub fn matches(self, address: Address) -> bool {
        let text = address.to_string();

        match self.position {
            Position::Prefix => text.starts_with(self.literal),
            Position::Suffix => text.ends_with(self.literal),
        }
    }

    fn new(literal: &'a str, position: Position) -> Result<Self, GrindPatternError> {
        if literal.len() > MAX_LITERAL_LENGTH {
            return Err(GrindPatternError::TooLong {
                length: literal.len(),
            });
        }

        if let Some(character) = literal.chars().find(|character| !is_base58(*character)) {
            return Err(GrindPatternError::InvalidCharacter { character });
        }

        Ok(Self { literal, position })
    }
}

fn is_base58(character: char) -> bool {
    BASE58_ALPHABET.contains(character)
}

/// A literal pattern that no base58 address can satisfy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GrindPatternError {
    /// The literal contains a symbol outside the base58 alphabet.
    InvalidCharacter {
        /// The first rejected character, without any key material.
        character: char,
    },
    /// The literal is longer than the longest canonical address text.
    TooLong {
        /// Rejected literal length in bytes.
        length: usize,
    },
}

impl fmt::Display for GrindPatternError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCharacter { character } => {
                write!(formatter, "'{character}' is not a base58 address character")
            }
            Self::TooLong { length } => {
                write!(
                    formatter,
                    "literal length {length} exceeds 44 base58 characters"
                )
            }
        }
    }
}

impl core::error::Error for GrindPatternError {}

/// A finite request for matching keys.
///
/// `amount` is the exact number of matches returned on success. `attempts` is
/// the maximum number of candidates generated, including rejected ones. A budget
/// with fewer attempts than matches can only report exhaustion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GrindBudget {
    amount: usize,
    attempts: usize,
}

impl GrindBudget {
    /// Requests exactly `amount` matches from at most `attempts` candidates.
    #[must_use]
    pub const fn new(amount: usize, attempts: usize) -> Self {
        Self { amount, attempts }
    }

    /// Exact number of matches required for success.
    #[must_use]
    pub const fn amount(self) -> usize {
        self.amount
    }

    /// Maximum number of candidates, including rejected ones.
    #[must_use]
    pub const fn attempts(self) -> usize {
        self.attempts
    }
}

/// A grind that stopped without a complete result.
///
/// A successful call always returns the exact requested amount. Partial matches
/// are never reported as success.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GrindError<E> {
    /// The requested result storage could not be allocated, before entropy was used.
    Allocation,
    /// The host entropy operation failed. No further candidates were generated.
    Entropy(E),
    /// The finite attempt budget ended before enough matches were found.
    Exhausted {
        /// Candidates generated before the budget ended.
        attempts: usize,
        /// Matches found before the budget ended.
        found: usize,
    },
}

impl<E: fmt::Display> fmt::Display for GrindError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Allocation => formatter.write_str("key grinding result allocation failed"),
            Self::Entropy(error) => write!(formatter, "key grinding entropy failure: {error}"),
            Self::Exhausted { attempts, found } => write!(
                formatter,
                "key grinding exhausted {attempts} attempts with {found} matches"
            ),
        }
    }
}

impl<E: core::error::Error + 'static> core::error::Error for GrindError<E> {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Entropy(error) => Some(error),
            Self::Allocation | Self::Exhausted { .. } => None,
        }
    }
}

/// Generates exactly `budget.amount()` exportable keys matching a literal pattern.
///
/// Generation is sequential and yields after each batch of 32 candidates, as
/// well as suspending inside the caller's entropy host. Dropping the future
/// cancels the grind; every candidate then owned by the
/// future, rejected or retained, is dropped with its zeroizing signer.
///
/// # Errors
/// Returns [`GrindError::Entropy`] as soon as the caller's entropy host fails,
/// discarding earlier matches, or [`GrindError::Exhausted`] after the finite
/// budget without the requested number of matches. [`GrindError::Allocation`]
/// rejects unavailable result storage before requesting entropy.
pub async fn grind_keypairs<E: Entropy>(
    entropy: &mut E,
    pattern: GrindPattern<'_>,
    budget: GrindBudget,
) -> Result<Vec<ExportableKeypair>, GrindError<E::Error>> {
    grind_keypairs_with(entropy, budget, |address| pattern.matches(address)).await
}

/// Generates exactly `budget.amount()` exportable keys satisfying a predicate.
///
/// The predicate runs once per generated candidate over its canonical address
/// and may retain local state. Callers compose arbitrary matching here, but keep
/// their own predicate work bounded because the candidate budget bounds only key
/// generation.
///
/// # Errors
/// Returns the same failure contract as [`grind_keypairs`].
pub async fn grind_keypairs_with<E: Entropy, F>(
    entropy: &mut E,
    budget: GrindBudget,
    mut matches: F,
) -> Result<Vec<ExportableKeypair>, GrindError<E::Error>>
where
    F: FnMut(Address) -> bool,
{
    if budget.amount == 0 {
        return Ok(Vec::new());
    }

    let mut found = Vec::new();
    found
        .try_reserve_exact(budget.amount.min(budget.attempts))
        .map_err(|_| GrindError::Allocation)?;

    for attempt in 0..budget.attempts {
        let key = ExportableKeypair::generate(entropy)
            .await
            .map_err(GrindError::Entropy)?;

        if matches(key.signer().address()) {
            found.push(key);

            if found.len() == budget.amount {
                return Ok(found);
            }
        } else {
            drop(key);
        }

        if (attempt + 1).is_multiple_of(CANDIDATES_PER_BATCH) {
            yield_batch().await;
        }
    }

    Err(GrindError::Exhausted {
        attempts: budget.attempts,
        found: found.len(),
    })
}

async fn yield_batch() {
    let mut yielded = false;
    poll_fn(|context| {
        if yielded {
            return Poll::Ready(());
        }

        yielded = true;
        context.waker().wake_by_ref();
        Poll::Pending
    })
    .await
}
