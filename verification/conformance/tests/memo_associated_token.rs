#[test]
fn isolated_memo() {
    solana_kit_conformance::memo_associated_token::memo().unwrap();
}

#[test]
fn isolated_associated_token() {
    solana_kit_conformance::memo_associated_token::associated_token().unwrap();
}
