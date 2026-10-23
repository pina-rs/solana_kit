use solana_kit_client::accounts::parse_base_account;
fn main() {
    let _ = parse_base_account(
        r#"{"executable":false,"lamports":1,"owner":"11111111111111111111111111111111","space":3}"#,
    );
}
