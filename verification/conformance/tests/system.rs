//! Native execution of the cases before combined runtime-suite integration.

#[test]
fn system_data_vectors_and_malformed_inputs() {
    solana_kit_conformance::system::create_account().expect("CreateAccount literal");
    solana_kit_conformance::system::create_seeded().expect("seeded literal");
    solana_kit_conformance::system::malformed().expect("strict decoding");
}
