//! Normal verification rejects stale goldens; native capture emits only executed outcomes.

use std::process::ExitCode;

#[cfg(not(target_arch = "wasm32"))]
const USAGE: &str = "usage: solana_kit_helius_portable [--capture]";
#[cfg(target_arch = "wasm32")]
const USAGE: &str = "usage: solana_kit_helius_portable";

enum Mode {
    Verify,
    #[cfg(not(target_arch = "wasm32"))]
    Capture,
}

fn mode(arguments: &[String]) -> Result<Mode, &'static str> {
    match arguments {
        [] => Ok(Mode::Verify),

        #[cfg(not(target_arch = "wasm32"))]
        [argument] if argument == "--capture" => Ok(Mode::Capture),
        _ => Err(USAGE),
    }
}

fn main() -> ExitCode {
    let arguments: Vec<_> = std::env::args().skip(1).collect();

    let selected = match mode(&arguments) {
        Ok(mode) => mode,
        Err(usage) => {
            eprintln!("{usage}");

            return ExitCode::from(2);
        }
    };

    let completed: Result<usize, &'static str> = match selected {
        Mode::Verify => Ok(solana_kit_helius_portable::run()),
        #[cfg(not(target_arch = "wasm32"))]
        Mode::Capture => solana_kit_helius_portable::capture(),
    };

    finish(completed)
}

fn finish(completed: Result<usize, &'static str>) -> ExitCode {
    match completed {
        Ok(count) if count == solana_kit_helius_portable::COUNT => {
            println!("PASS all {count} Helius contract groups");
            ExitCode::SUCCESS
        }

        Ok(_) => ExitCode::FAILURE,
        Err(reason) => {
            eprintln!("{reason}");
            ExitCode::FAILURE
        }
    }
}
