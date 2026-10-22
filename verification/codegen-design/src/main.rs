//! Private reproducibility experiments; this is not a public program generator.

mod accounts_schema;
mod builders;
mod client_receipt;
mod generate;
mod generated;
mod generic;
mod inputs;
mod proof;
mod renderer;
mod schema;

mod system;
mod system_errors;
mod system_receipt;

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
        Some("inputs") => inputs::audit(),
        Some("generate") => generate::run(),
        Some("compute-budget") => generic::run(),
        Some("compute-budget-receipt") => generic::receipt(),

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
