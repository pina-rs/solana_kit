//! Deterministic translation from typed portal records into normalized models.
//!
//! The only absent/null/blank fallbacks are the pinned source's own. No URL is
//! approved, no clock is sampled and no record is fetched while projecting.

mod bundle;
mod ingestion;
mod session;
mod support;

pub use bundle::map_backend_bundle_to_publication_bundle;
pub use ingestion::{
    infer_publication_source_kind, translate_backend_ingestion_session,
    translate_backend_ingestion_session_with_attachments,
};
pub use session::{
    SessionMarkers, normalize_publication_checkpoint, normalize_publication_status,
    translate_backend_publication_session,
};
