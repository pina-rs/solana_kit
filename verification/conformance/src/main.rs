//! Native and WASI Preview 1 entrypoint with observable failure propagation.

use std::process::ExitCode;

use solana_kit_conformance::{EXPECTED_CASE_COUNT, FixtureMode, SUITE_ID, run};

/// Parses only the documented negative-control flag.
fn fixture_mode() -> Result<FixtureMode, &'static str> {
    let arguments: Vec<_> = std::env::args().skip(1).collect();

    if arguments.is_empty() {
        return Ok(FixtureMode::Canonical);
    }

    if arguments == ["--corrupt-fixture"] {
        return Ok(FixtureMode::CorruptHash);
    }

    Err("usage: solana_kit_conformance [--corrupt-fixture]")
}

/// Executes and displays all cases; an incorrect or incomplete suite exits nonzero.
fn main() -> ExitCode {
    let mode = match fixture_mode() {
        Ok(mode) => mode,
        Err(error) => {
            eprintln!("{error}");

            return ExitCode::from(2);
        }
    };

    let report = run(mode);

    for case in &report.cases {
        println!("{}: {:?}", case.name, case.outcome);
    }

    println!(
        "{}: {}/{} cases passed",
        report.suite_id,
        report.passed_count(),
        report.cases.len()
    );

    if report.suite_id != SUITE_ID || report.cases.len() != EXPECTED_CASE_COUNT {
        eprintln!("unexpected suite identity or case count");

        return ExitCode::FAILURE;
    }

    if report.is_success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
