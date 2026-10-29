//! Typed single-exchange portal requests; publication orchestration belongs to the caller.

mod client;
mod error;
mod models;
mod object;
mod operations;
mod requests;

mod response;
mod retry;
mod upload;

pub use client::{Client, PortalService};
pub use error::Error;
pub use models::*;
pub use operations::Operation;
pub use requests::*;
pub use retry::IngestionRetry;
pub use upload::{UploadRequest, Uploader, upload_bytes};

/// An optional backend field retaining absent, explicit null and available values.
pub type Optional<T> = solana_kit_client::Presence<Option<T>>;
