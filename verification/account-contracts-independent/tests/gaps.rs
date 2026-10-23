//! Small source-supported counterexamples. These tests intentionally fail.
use solana_kit_client::accounts::{parse_base64_rpc_account, parse_json_rpc_account};
use solana_kit_core::Address;

#[test]
fn parsed_program_and_nested_space_are_optional_in_reference() {
    let raw = r#"{"executable":false,"lamports":1,"owner":"11111111111111111111111111111111","space":3,"data":{"parsed":{"info":{"x":7},"type":"custom"}}}"#;
    assert!(parse_json_rpc_account(Address::from_bytes([0; 32]), raw).is_ok());
}

#[test]
fn standalone_numeric_metadata_strings_are_accepted_in_reference() {
    let raw = r#"{"executable":false,"lamports":"1","owner":"11111111111111111111111111111111","space":"3","data":["AQID","base64"]}"#;
    assert!(parse_base64_rpc_account(Address::from_bytes([0; 32]), raw).is_ok());
}
