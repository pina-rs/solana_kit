//! Value-free format declarations, independent of operation budgets and cursors.

use super::Error;
use alloc::vec::Vec;

/// Keep the complete declaration list so unknown classes cannot hide a known
/// subtotal's overflow. Metadata collection reserves one entry per schema field.
pub(super) fn sum_sizes(
    sizes: impl ExactSizeIterator<Item = Result<Size, Error>>,
) -> Result<Size, Error> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(sizes.len())
        .map_err(|_| Error::AllocationFailed)?;

    for size in sizes {
        values.push(size?);
    }

    Size::checked_sum(&values)
}

/// A format declaration for one encoder or decoder implementation.
///
/// Declarations are stable for unchanged codec configuration. They do not prove
/// actual cursor movement, allocation bounds or inverse behavior. In particular,
/// adapters can retain a declaration while changing cursor positions. Operations
/// continue to use their existing sizing, limits and shared allocation claims.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Size {
    /// The implementation has not declared its format class.
    Unknown,
    /// A fixed format width, including zero.
    Fixed(usize),
    /// A variable format, with or without a declared maximum.
    Variable {
        /// The complete optional maximum. Unknown and zero are distinct.
        max: Option<usize>,
    },
}

impl Size {
    /// Returns the width only for a declared fixed format.
    #[must_use]
    pub const fn fixed_size(self) -> Option<usize> {
        match self {
            Self::Fixed(size) => Some(size),
            _ => None,
        }
    }

    /// Returns a fixed width or the declared variable maximum, when known.
    #[must_use]
    pub const fn max_size(self) -> Option<usize> {
        match self {
            Self::Fixed(size) => Some(size),
            Self::Variable { max } => max,
            Self::Unknown => None,
        }
    }

    /// Adds sequential declarations. The empty sum is `Fixed(0)`.
    ///
    /// # Errors
    /// Returns [`Error::Overflow`] if the subtotal of known widths or maxima
    /// overflows, even beside an unknown declaration or maximum, in any order.
    pub fn checked_sum(sizes: &[Self]) -> Result<Self, Error> {
        let total = sizes
            .iter()
            .filter_map(|size| size.max_size())
            .try_fold(0usize, |total, size| {
                total.checked_add(size).ok_or(Error::Overflow)
            })?;

        if sizes.contains(&Self::Unknown) {
            return Ok(Self::Unknown);
        }

        if sizes.iter().all(|size| matches!(size, Self::Fixed(_))) {
            return Ok(Self::Fixed(total));
        }

        let max = sizes
            .iter()
            .all(|size| size.max_size().is_some())
            .then_some(total);
        Ok(Self::Variable { max })
    }

    /// Repeats a declaration. Zero repetitions are `Fixed(0)` for every input.
    ///
    /// # Errors
    /// Returns [`Error::Overflow`] if a known width or maximum product overflows.
    pub fn checked_repeat(self, count: usize) -> Result<Self, Error> {
        if count == 0 {
            return Ok(Self::Fixed(0));
        }

        match self {
            Self::Fixed(size) => size
                .checked_mul(count)
                .map(Self::Fixed)
                .ok_or(Error::Overflow),
            Self::Variable { max: Some(size) } => size
                .checked_mul(count)
                .map(|size| Self::Variable { max: Some(size) })
                .ok_or(Error::Overflow),
            _ => Ok(self),
        }
    }

    /// Classifies alternatives without arithmetic. The empty choice is `Fixed(0)`.
    /// Equal fixed arms stay fixed. Other declared arms use the largest maximum,
    /// with an unknown maximum absorbing known maxima. Unknown class propagates.
    #[must_use]
    pub fn choice(sizes: &[Self]) -> Self {
        let Some(first) = sizes.first() else {
            return Self::Fixed(0);
        };

        if sizes.contains(&Self::Unknown) {
            return Self::Unknown;
        }

        if matches!(first, Self::Fixed(_)) && sizes.iter().all(|size| size == first) {
            return *first;
        }

        let max = sizes
            .iter()
            .try_fold(0usize, |maximum, size| Some(maximum.max(size.max_size()?)));
        Self::Variable { max }
    }
}
