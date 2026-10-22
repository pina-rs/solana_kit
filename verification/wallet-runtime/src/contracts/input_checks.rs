//! Literal capability inputs asserted inside the concrete backend.

use solana_kit_wallet::*;

pub(super) fn transaction(input: &SignTransactionInput) {
    assert_eq!(input.account, super::account());
    assert_eq!(input.chain, None);
    let options = input.options.as_ref().unwrap();
    assert_eq!(
        options.preflight_commitment,
        Some(TransactionCommitment::Confirmed)
    );
    assert_eq!(options.min_context_slot, Some(-4));
}

pub(super) fn submission(input: &SignAndSendTransactionInput) {
    assert_eq!(input.account, super::account());
    assert_eq!(input.chain, "arbitrary");
    let options = input.options.as_ref().unwrap();
    assert_eq!(
        options.simulation.preflight_commitment,
        Some(TransactionCommitment::Confirmed)
    );
    assert_eq!(options.simulation.min_context_slot, Some(-4));
    assert_eq!(options.commitment, Some(TransactionCommitment::Finalized));
    assert_eq!(options.skip_preflight, Some(true));
    assert_eq!(options.max_retries, Some(-2));
}

pub(super) fn sign_in(input: &SignInInput) {
    assert_eq!(input.domain.as_deref(), Some("example.test"));
    assert_eq!(input.address.as_deref(), Some("display-address"));
    assert_eq!(input.statement.as_deref(), Some("Sign in"));
    assert_eq!(input.uri.as_deref(), Some("https://example.test"));
    assert_eq!(input.version.as_deref(), Some("1"));
    assert_eq!(input.chain_id.as_deref(), Some("1"));
    assert_eq!(input.nonce.as_deref(), Some("nonce"));
    assert_eq!(input.issued_at.as_deref(), Some("now"));
    assert_eq!(input.expiration_time.as_deref(), Some("later"));
    assert_eq!(input.not_before.as_deref(), Some("now"));
    assert_eq!(input.request_id.as_deref(), Some("request"));
    assert_eq!(
        input.resources.as_deref(),
        Some(&[String::from("resource")][..])
    );
}
