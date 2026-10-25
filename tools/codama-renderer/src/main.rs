//! Private reproducibility command adapter for the shared generation library.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    solana_kit_codegen::run_private_command(&std::env::args().collect::<Vec<_>>())
}
