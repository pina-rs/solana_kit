//! Emits the same complete named contract stream on native and WASI hosts.
fn main() {
    println!(
        "metaplex-clients: {}/{}",
        solana_kit_metaplex_portable::run(),
        solana_kit_metaplex_portable::COUNT,
    );
}
