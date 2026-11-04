#![cfg(feature = "portal")]
//! Independent literals exercise the public normalized projection without network or host I/O.
#[path = "publication/bundle.rs"]
mod bundle;

#[path = "publication/diagnostics.rs"]
mod diagnostics;

#[path = "publication/ingestion.rs"]
mod ingestion;

#[path = "publication/session.rs"]
mod session;

#[path = "publication/support.rs"]
mod support;
