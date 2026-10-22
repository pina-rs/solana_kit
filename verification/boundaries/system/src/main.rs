//! Native and WASI entry point uses exactly the same standalone constructor.

fn main() {
    let corrupt = std::env::args().nth(1).as_deref() == Some("--corrupt-fixture");
    let result = solana_kit_system_boundary::verify(corrupt);
    println!("solana-kit-system-create-account-v1: {result:?}");

    if result.is_err() {
        std::process::exit(1);
    }
}
