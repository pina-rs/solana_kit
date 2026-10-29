//! Ingestion and media hydration stop before transaction signing or store submission.

mod engine;
mod error;
mod hydrate;
mod io;
mod media;
mod settings;
mod snapshot;
mod source;
mod state;
mod timestamp;

pub use engine::Workflow;
pub use error::Error;
pub use hydrate::HydratedPublication;
pub use io::{SourceReader, Storage};
pub use media::{MediaDimensions, read_image_dimensions};
pub use settings::{Limits, Options};
pub use snapshot::{Effect, Snapshot, UploadIntent};
pub use source::{LocalApk, prepare_local_apk};
pub use state::{ResumeTarget, Stage};

use solana_kit_client::Presence;

pub(super) fn value<T>(field: &crate::portal::Optional<T>) -> Option<&T> {
    match field {
        Presence::Present(Some(value)) => Some(value),
        _ => None,
    }
}

pub(super) fn text(field: &crate::portal::Optional<String>) -> Option<&str> {
    value(field)
        .map(String::as_str)
        .filter(|text| !text.trim().is_empty())
}

pub(super) fn take<T>(field: crate::portal::Optional<T>) -> Option<T> {
    match field {
        Presence::Present(Some(value)) => Some(value),
        _ => None,
    }
}
