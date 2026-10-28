//! Portable public-API contract bodies shared with the integrated host suite.

pub mod accounts;
pub mod administration;
pub mod configuration;
pub mod failures;
pub mod lifecycle;
pub mod models;
pub mod profiling;

mod fixture;
mod privacy;

pub use fixture::{call, require};

/// One stable group executes the same body on every host.
pub type Contract = (&'static str, fn() -> Result<(), &'static str>);

/// Every Surfpool contract body uses real public library entry points.
pub fn groups() -> [Contract; 8] {
    [
        ("surfpool-account-wire", accounts::run),
        ("surfpool-administration-wire", administration::run),
        ("surfpool-profile-clock-wire", profiling::run),
        ("surfpool-context-exact-models", models::run),
        ("surfpool-error-budget", failures::run),
        ("surfpool-local-cancellation", lifecycle::run),
        ("surfpool-config-deploy", configuration::run),
        ("surfpool-method-manifest", administration::manifest),
    ]
}
