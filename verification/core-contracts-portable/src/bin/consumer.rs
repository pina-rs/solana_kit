//! Execute every original public contract from an ordinary downstream binary.

fn main() {
    let cases = solana_kit_core_contracts_verification::cases();
    assert_eq!(cases.len(), 48);

    for (name, body) in cases {
        println!("running {name}");
        body();
        println!("passed {name}");
    }
}
