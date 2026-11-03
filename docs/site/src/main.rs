use std::process::ExitCode;

use clap::Parser;
use solana_kit_docs_site::Cli;

fn main() -> ExitCode {
    match Cli::parse().run() {
        Ok(server) => {
            if let Some(server) = server {
                server.wait();
            }

            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
