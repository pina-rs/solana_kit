#![cfg(feature = "workflow")]
//! Synthetic caller capabilities exercise workflow behavior without host or network I/O.
#[path = "support/checks.rs"]
mod checks;

#[path = "workflow/failures.rs"]
mod failures;
#[path = "workflow/fixtures.rs"]
mod fixtures;
#[path = "workflow/flows.rs"]
mod flows;
#[path = "workflow/media.rs"]
mod media;

#[path = "workflow/public_urls.rs"]
mod public_urls;

#[path = "workflow/support.rs"]
mod support;
