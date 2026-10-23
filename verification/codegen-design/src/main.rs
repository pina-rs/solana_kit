//! Private reproducibility experiments; this is not a public program generator.

mod accounts_schema;
mod alt_config;
mod alt_config_receipt;
mod builders;
mod client_receipt;
mod generate;
mod generated;
mod generated_style;

mod generic;
mod inputs;
mod proof;
mod renderer;
mod schema;

mod system;
mod system_errors;
mod system_receipt;
mod token;
mod token_2022;
mod token_2022_proof;
mod token_2022_rust_values;
mod token_receipt;
#[cfg(test)]
mod token_tests;

#[cfg(test)]
mod client_oracle;
#[cfg(test)]
mod client_schema_tests;
#[cfg(test)]
mod system_oracle;
#[cfg(test)]
mod system_tests;

#[cfg(test)]
mod tests;

/// Dispatches the isolated input audit or the focused generator experiment.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments: Vec<_> = std::env::args().collect();

    match arguments.get(1).map(String::as_str) {
        Some("generated-style") => generated_style::run(),
        Some("inputs") => inputs::audit(),
        Some("alt-config-receipt") => alt_config_receipt::run(),
        Some("alt-config") => alt_config::run(),
        Some("generate") => generate::run(),
        Some("compute-budget") => generic::run(),
        Some("compute-budget-receipt") => generic::receipt(),
        Some("token") => token::run(),
        Some("token-2022-proof") => token_2022_proof::run(std::path::Path::new(
            arguments.get(2).ok_or("output directory required")?,
        )),
        Some("token-2022-data") => token_2022::run(std::path::Path::new(
            arguments.get(2).ok_or("output path required")?,
        )),
        Some("token-normalize") => token::freeze(),
        Some("token-receipt") => token_receipt::run(),

        Some("system") => system::run(),
        Some("system-builders") => builders::run(),
        Some("system-errors") => system_errors::run(),
        Some("system-client-receipt") => client_receipt::run(),
        Some("system-receipt") => system_receipt::run(),
        Some("proof") => proof::run(),
        Some("renderer") => renderer::run(),
        _ => Err("usage: solana_kit_codegen_design inputs|generate|system|proof|renderer".into()),
    }
}
