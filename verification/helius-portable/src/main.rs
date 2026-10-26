fn main() {
    let completed = solana_kit_helius_portable::run();
    assert_eq!(completed, solana_kit_helius_portable::COUNT);
    println!("PASS all {completed} Helius contract groups");
}
