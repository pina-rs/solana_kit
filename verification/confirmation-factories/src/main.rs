//! Deterministic receipt for native Rust and both WASI previews.
fn main() {
    let count = solana_kit_confirmation_factory_verification::run();
    assert_eq!(count, 11);
    println!("confirmation-factory-contracts {count}/11");
    assert_eq!(
        solana_kit_confirmation_factory_verification::run_clock_contracts(),
        2
    );
    println!("confirmation-clock-contracts 2/2");
}
