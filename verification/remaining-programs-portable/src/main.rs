//! Emits the same complete named contract stream on native and WASI hosts.
fn main() {
    println!(
        "remaining-programs: {}/{}",
        solana_kit_remaining_programs_portable::run(),
        solana_kit_remaining_programs_portable::COUNT,
    );
}
