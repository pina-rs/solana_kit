//! Compilation checks for the Dart-to-Rust migration guide.
//!
//! The maintained guide is included as crate documentation so its Rust blocks
//! are compiled by rustdoc instead of being copied into a second source.
#![doc = include_str!("../../../docs/migration/from-dart.md")]
#![forbid(unsafe_code)]
