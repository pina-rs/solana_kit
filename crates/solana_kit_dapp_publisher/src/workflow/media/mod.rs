//! Typed metadata generation and bounded image-header inspection perform no ambient I/O.

mod dimensions;
mod document;
pub(super) mod mime;
mod spec;
pub(super) mod urls;

pub use dimensions::{MediaDimensions, read_image_dimensions};
pub(super) use document::{Document, ResolvedMedia};
pub(super) use spec::{MediaInput, Spec};
