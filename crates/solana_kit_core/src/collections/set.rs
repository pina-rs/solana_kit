//! Unique values retain their first appearance, including the original object.

use alloc::{collections::TryReserveError, vec::Vec};
use core::borrow::Borrow;

/// An insertion-ordered set with private, distinct values.
///
/// Equality compares membership independently of iteration order. Inspect
/// [`Self::as_slice`] when order affects the encoded wire representation.
#[derive(Clone, Debug)]
pub struct OrderedSet<T> {
    values: Vec<T>,
}

impl<T> OrderedSet<T> {
    /// Creates an empty set without allocating storage.
    #[must_use]
    pub const fn new() -> Self {
        Self { values: Vec::new() }
    }

    /// Borrows distinct values in their insertion order.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        &self.values
    }

    /// Returns the number of distinct values.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Reports whether the set has no values.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Returns the original owned storage in iteration order.
    #[must_use]
    pub fn into_values(self) -> Vec<T> {
        self.values
    }
}

impl<T: Eq> OrderedSet<T> {
    /// Removes later equal values in place, without cloning or reallocating.
    #[must_use]
    pub fn from_values(mut values: Vec<T>) -> Self {
        let mut cursor = 0;

        while cursor < values.len() {
            if values[..cursor].contains(&values[cursor]) {
                values.remove(cursor);

                continue;
            }

            cursor += 1;
        }

        Self { values }
    }

    /// Reports whether an equal borrowed value is present, without allocation.
    #[must_use]
    pub fn contains<Q: Eq + ?Sized>(&self, value: &Q) -> bool
    where
        T: Borrow<Q>,
    {
        self.values.iter().any(|item| item.borrow() == value)
    }

    /// Appends an absent value, returning whether the set changed.
    ///
    /// # Errors
    /// Returns the allocator's reservation failure before appending a new value.
    pub fn try_insert(&mut self, value: T) -> Result<bool, TryReserveError> {
        if self.contains(&value) {
            return Ok(false);
        }

        self.values.try_reserve(1)?;
        self.values.push(value);

        Ok(true)
    }

    /// Removes an equal value without changing the order of remaining values.
    pub fn remove<Q: Eq + ?Sized>(&mut self, value: &Q) -> bool
    where
        T: Borrow<Q>,
    {
        let Some(position) = self.values.iter().position(|item| item.borrow() == value) else {
            return false;
        };
        self.values.remove(position);

        true
    }
}

impl<T> Default for OrderedSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Eq> PartialEq for OrderedSet<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.values.iter().all(|value| other.contains(value))
    }
}

impl<T: Eq> Eq for OrderedSet<T> {}

impl<T> IntoIterator for OrderedSet<T> {
    type Item = T;
    type IntoIter = alloc::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a OrderedSet<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}
