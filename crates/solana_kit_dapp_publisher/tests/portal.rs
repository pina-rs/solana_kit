#![cfg(feature = "portal")]
//! Synthetic protocol tests perform no network, upload, credential lookup or publication.
#[path = "portal/boundaries.rs"]
mod boundaries;
#[path = "support/checks.rs"]
mod checks;
#[path = "portal/retry.rs"]
mod retry;

#[path = "portal/routes.rs"]
mod routes;
#[path = "portal/support.rs"]
mod support;

#[path = "portal/strict_boundaries.rs"]
mod strict_boundaries;
#[path = "portal/upload.rs"]
mod upload;
