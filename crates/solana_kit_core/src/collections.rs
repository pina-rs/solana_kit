//! Owned collections whose iteration order is their first insertion order.
//!
//! Uniqueness uses Rust [`Eq`], rather than Dart object identity or a hash table.
//! Lookups and insertion are linear; normalizing an owned vector is quadratic.
//! Decoders bound the original wire item count before normalization and reuse
//! the decoded vector, without cloning items or allocating a second collection.
//! Keys and set values must keep their equality stable while stored, including
//! when they expose interior mutability, as with standard Rust collections.

mod map;
mod set;

pub use map::OrderedMap;
pub use set::OrderedSet;
