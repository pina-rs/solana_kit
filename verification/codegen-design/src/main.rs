//! Private reproducibility experiments; this is not a public program generator.

mod generate;
mod generated;
mod inputs;
mod proof;
mod renderer;
mod schema;
mod system;
mod system_receipt;

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
        Some("system") => system::run(),
        Some("system-receipt") => system_receipt::run(),
        Some("proof") => proof::run(),
        Some("renderer") => renderer::run(),
        _ => Err("usage: solana_kit_codegen_design inputs|generate|system|proof|renderer".into()),
    }
}
