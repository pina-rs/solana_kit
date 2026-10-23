//! Executes all Token-2022 client contracts on native and WASI command hosts.

fn main() {
    assert_eq!(solana_kit_token_2022_portable::run(), 119);
    println!("token-2022-public: 119/119");
}
