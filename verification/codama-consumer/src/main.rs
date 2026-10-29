fn main() {
    assert_eq!(
        packed_counter_client::run(),
        packed_counter_client::CONTRACT_GROUPS
    );
    println!("generated-client-contracts:9");
}
