//! Reproducible runtime verification commands; unavailable tools are hard errors.
#![forbid(unsafe_code)]

mod arguments;
mod artifact_capture;
mod client_gate;
mod consumer;
mod gates;
mod process;
mod runtimes;
mod style;
mod system_gate;

use anyhow::Result;
use arguments::Command;

/// Dispatches one explicit verification journey.
fn main() -> Result<()> {
    let arguments: Vec<_> = std::env::args_os().skip(1).collect();
    let command = arguments::parse(&arguments)?;
    std::env::set_current_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/.."))?;

    match command {
        Command::ArtifactCapture(path) => artifact_capture::run(&path),
        Command::Style(root) => style::run(root.as_deref()),
        Command::StyleReport(path) => style::check_report(&path),
        Command::Native => runtimes::native(),
        Command::Browser => runtimes::browser(),
        Command::Wasi => runtimes::wasi(),
        Command::Leptoskit => runtimes::leptoskit(),
        Command::Gates => gates::run(),
        Command::DependencyGraphs => gates::graphs(),
        Command::Consumer => consumer::run(),
        Command::Imports(path) => gates::check_import_file(&path),
        Command::DependencyFixture(path) => gates::check_dependency_fixture(&path),
        Command::All => {
            runtimes::native()?;
            runtimes::browser()?;
            runtimes::wasi()?;
            runtimes::leptoskit()?;
            gates::run()?;
            consumer::run()
        }
    }
}
