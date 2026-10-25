//! Unique keys retain their original position when their values are replaced.

use alloc::{collections::TryReserveError, vec::Vec};
use core::borrow::Borrow;

/// An insertion-ordered map with private, unique keys.
///
/// Equal keys retain the first key object and insertion position. A later entry
/// replaces only the value. Equality compares key/value associations, independent
/// of iteration order; inspect [`Self::as_slice`] when wire order matters.
#[derive(Clone, Debug)]
pub struct OrderedMap<K, V> {
    entries: Vec<(K, V)>,
}

impl<K, V> OrderedMap<K, V> {
    /// Creates an empty map without allocating storage.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Borrows entries in their insertion order, without permitting key mutation.
    #[must_use]
    pub fn as_slice(&self) -> &[(K, V)] {
        &self.entries
    }

    /// Returns the number of distinct keys.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Reports whether the map has no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the original owned storage in iteration order.
    #[must_use]
    pub fn into_entries(self) -> Vec<(K, V)> {
        self.entries
    }
}

impl<K: Eq, V> OrderedMap<K, V> {
    /// Normalizes owned entries in place, preserving first keys and last values.
    #[must_use]
    pub fn from_entries(mut entries: Vec<(K, V)>) -> Self {
        let mut cursor = 0;

        while cursor < entries.len() {
            let previous = entries[..cursor]
                .iter()
                .position(|entry| entry.0 == entries[cursor].0);

            if let Some(previous) = previous {
                let (_, value) = entries.remove(cursor);
                entries[previous].1 = value;

                continue;
            }

            cursor += 1;
        }

        Self { entries }
    }

    /// Borrows the current value for an equal borrowed key, without allocation.
    #[must_use]
    pub fn get<Q: Eq + ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
    {
        self.entries
            .iter()
            .find(|entry| entry.0.borrow() == key)
            .map(|entry| &entry.1)
    }

    /// Borrows a value mutably without exposing mutable keys.
    #[must_use]
    pub fn get_mut<Q: Eq + ?Sized>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
    {
        self.entries
            .iter_mut()
            .find(|entry| entry.0.borrow() == key)
            .map(|entry| &mut entry.1)
    }

    /// Replaces a value in place or appends a previously absent key.
    ///
    /// # Errors
    /// Returns the allocator's reservation failure before appending a new key.
    pub fn try_insert(&mut self, key: K, value: V) -> Result<Option<V>, TryReserveError> {
        if let Some(current) = self.get_mut(&key) {
            return Ok(Some(core::mem::replace(current, value)));
        }

        self.entries.try_reserve(1)?;
        self.entries.push((key, value));

        Ok(None)
    }

    /// Removes an equal key while preserving every remaining entry's order.
    pub fn remove<Q: Eq + ?Sized>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
    {
        let position = self
            .entries
            .iter()
            .position(|entry| entry.0.borrow() == key)?;

        Some(self.entries.remove(position).1)
    }
}

impl<K, V> Default for OrderedMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Eq, V: PartialEq> PartialEq for OrderedMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len()
            && self
                .entries
                .iter()
                .all(|(key, value)| other.get(key) == Some(value))
    }
}

impl<K: Eq, V: Eq> Eq for OrderedMap<K, V> {}

impl<K, V> IntoIterator for OrderedMap<K, V> {
    type Item = (K, V);
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.into_iter()
    }
}

impl<'a, K, V> IntoIterator for &'a OrderedMap<K, V> {
    type Item = &'a (K, V);
    type IntoIter = core::slice::Iter<'a, (K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.iter()
    }
}
