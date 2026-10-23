//! Runs the same complete Reader controls on native and WASI command hosts.

fn main() {
    assert_eq!(solana_kit_reader_transactions::run(), 5);
    println!("reader-transactions: 5/5");
}
