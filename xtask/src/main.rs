//! Reproducible runtime verification commands; unavailable tools are hard errors.
#![forbid(unsafe_code)]

mod client_gate;
mod consumer;
mod gates;
mod process;
mod runtimes;
mod style;
mod system_gate;

use anyhow::{Context, Result, bail};

/// Dispatches one explicit verification journey.
fn main() -> Result<()> {
    std::env::set_current_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/.."))?;
    let command = std::env::args()
        .nth(1)
        .context("usage: cargo xtask <command>")?;

    match command.as_str() {
        "style" => style::run(),
        "style-report" => style::check_report(),
        "native" => runtimes::native(),
        "browser" => runtimes::browser(),
        "wasi" => runtimes::wasi(),
        "leptoskit" => runtimes::leptoskit(),
        "gates" => gates::run(),
        "dependency-graphs" => gates::graphs(),
        "consumer" => consumer::run(),
        "imports" => gates::check_import_file(),
        "dependency-fixture" => gates::check_dependency_fixture(),
        "all" => {
            runtimes::native()?;
            runtimes::browser()?;
            runtimes::wasi()?;
            runtimes::leptoskit()?;
            gates::run()?;
            consumer::run()
        }
        _ => bail!("unknown verification command: {command}"),
    }
}
