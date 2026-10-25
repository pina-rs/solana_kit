//! Shared Rust engine for off-chain Solana Kit client generation.
//! The private experiment dispatcher and NPM WASI bridge share these modules.

#[cfg(feature = "verification")]
mod accounts_schema;
#[cfg(feature = "verification")]
mod alt_config;
#[cfg(feature = "verification")]
mod alt_config_receipt;
#[cfg(feature = "verification")]
mod attestation;

#[cfg(feature = "verification")]
mod bubblegum;
#[cfg(feature = "verification")]
mod builders;
#[cfg(feature = "verification")]
mod client_receipt;
#[cfg(feature = "verification")]
mod generate;

#[cfg(feature = "verification")]
mod generated;
#[cfg(feature = "verification")]
mod generated_style;
#[cfg(feature = "verification")]
mod program_preparation;

mod generic;
#[cfg(feature = "verification")]
mod inputs;
#[cfg(feature = "verification")]
mod mpl_core;
#[cfg(feature = "verification")]
mod proof;
#[cfg(feature = "verification")]
mod renderer;
mod schema;
#[cfg(feature = "verification")]
mod squads;
#[cfg(feature = "verification")]
mod stake_loader;

#[cfg(feature = "verification")]
mod system;
#[cfg(feature = "verification")]
mod system_errors;
#[cfg(feature = "verification")]
mod system_receipt;
#[cfg(feature = "verification")]
mod token;

#[cfg(feature = "verification")]
mod token_2022;
#[cfg(feature = "verification")]
mod token_2022_proof;
#[cfg(feature = "verification")]
mod token_2022_rust_values;
#[cfg(feature = "verification")]
mod token_metadata;

#[cfg(feature = "verification")]
mod token_receipt;
#[cfg(all(test, feature = "verification"))]
mod token_tests;

#[cfg(all(test, feature = "verification"))]
mod client_oracle;
#[cfg(all(test, feature = "verification"))]
mod client_schema_tests;
#[cfg(all(test, feature = "verification"))]
mod system_oracle;
#[cfg(all(test, feature = "verification"))]
mod system_tests;

#[cfg(all(test, feature = "verification"))]
mod tests;

/// Runs the retained private audit commands. This is not an application API.
#[doc(hidden)]
#[cfg(feature = "verification")]
pub fn run_private_command(arguments: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    match arguments.get(1).map(String::as_str) {
        Some("squads-generate") => squads::generate(output_path(arguments)?),
        Some("squads-primary-audit") => squads::audit_primary(std::path::Path::new(
            arguments
                .get(2)
                .ok_or("private pinned primary IDL path required")?,
        )),
        Some("bubblegum-audit-primary") => bubblegum::oracle::audit(std::path::Path::new(
            arguments
                .get(2)
                .ok_or("private primary IDL path required")?,
        )),
        Some("bubblegum-generate") => bubblegum::generate(output_path(arguments)?),
        Some("mpl-core-generate") => mpl_core::generate(output_path(arguments)?),
        Some("mpl-core-primary-audit") => mpl_core::audit_primary(std::path::Path::new(
            arguments.get(2).ok_or("private primary input required")?,
        )),
        Some("attestation-generate") => attestation::generate(output_path(arguments)?),
        Some("attestation-primary-audit") => attestation::audit_primary(std::path::Path::new(
            arguments.get(2).ok_or("private pinned IDL path required")?,
        )),
        Some("generated-style") => generated_style::run(),
        Some("token-metadata-generate") => token_metadata::generate(output_path(arguments)?),
        Some("token-metadata-primary") => token_metadata::audit_primary(std::path::Path::new(
            arguments
                .get(2)
                .ok_or("private primary input path required")?,
        )),
        Some("program-preparation-audit") => program_preparation::audit(),
        Some("compression-normalize") => program_preparation::run(output_path(arguments)?),
        Some("compression-generate") => {
            program_preparation::generate_compression(output_path(arguments)?)
        }
        Some("subscriptions-generate") => {
            program_preparation::generate_subscriptions(output_path(arguments)?)
        }
        _ => run_foundation_audit(arguments),
    }
}

#[cfg(feature = "verification")]
fn run_foundation_audit(arguments: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    match arguments.get(1).map(String::as_str) {
        Some("inputs") => inputs::audit(),
        Some("alt-config-receipt") => alt_config_receipt::run(),
        Some("alt-config") => alt_config::run(),
        Some("stake-loader-data") => stake_loader::run(),
        Some("generate") => generate::run(),
        Some("compute-budget") => generic::run(),
        Some("compute-budget-receipt") => generic::receipt(),
        Some("token") => token::run(),
        Some("token-2022-proof") => token_2022_proof::run(std::path::Path::new(
            arguments.get(2).ok_or("output directory required")?,
        )),
        Some("token-2022-data") => token_2022::run(output_path(arguments)?),
        Some("token-normalize") => token::freeze(),
        Some("token-receipt") => token_receipt::run(),

        Some("system") => system::run(),
        Some("system-builders") => builders::run(),
        Some("system-errors") => system_errors::run(),
        Some("system-client-receipt") => client_receipt::run(),
        Some("system-receipt") => system_receipt::run(),
        Some("proof") => proof::run(),
        Some("renderer") => renderer::run(),
        _ => Err("usage: solana_kit_codegen inputs|generate|system|proof|renderer".into()),
    }
}

#[cfg(feature = "verification")]
fn output_path(arguments: &[String]) -> Result<&std::path::Path, Box<dyn std::error::Error>> {
    Ok(std::path::Path::new(
        arguments.get(2).ok_or("output path required")?,
    ))
}

mod public_renderer;
pub use public_renderer::{GeneratedFile, RenderError, render_root};
