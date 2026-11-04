//! Deterministic normalized read models projected from typed portal records.
//!
//! The projection preserves the pinned publisher's display, checkpoint and
//! source precedence while keeping raw provider text as explicit data. These
//! records are not transaction authorization, publication approval or proof of
//! remote state. No model samples a clock, performs I/O, validates a URL or
//! derives `Debug` over application, contact, URL or provider fields.

mod model;
mod progress;
mod project;

pub use model::*;
pub use progress::*;
pub use project::*;
