//! Payment-link validation does not execute a transfer or resolve a secret.

use solana_kit_helius::auth::{
    PaymentLink, PaymentTermsError, PaymentTransfer, TREASURY, USDC_MINT_MAINNET,
};

/// Exact one-cent conversion, overflow, kind and memo failures remain observable.
pub fn positive_and_adverse_terms() {
    let mut link = terms();
    exact_conversion(&mut link);
    invalid_terms(&mut link);
}

fn terms() -> PaymentLink {
    PaymentLink {
        kind: "payment_required".into(),
        payment_intent_id: "intent".into(),
        amount_cents: 1,
        destination_wallet: TREASURY.into(),
        memo: "intent".into(),
        expires_at: "caller-checks-expiry".into(),
        payment_url: "https://example.invalid/pay".into(),
        solana_pay_url: "solana:recipient".into(),
        plan_name: "plan".into(),
    }
}

fn exact_conversion(link: &mut PaymentLink) {
    const RAW_UNITS_PER_CENT: u64 = 10_000;
    let transfer = PaymentTransfer::from_link(link).expect("valid source terms");
    assert_eq!(transfer.recipient.to_string(), TREASURY);
    assert_eq!(transfer.mint.to_string(), USDC_MINT_MAINNET);
    assert_eq!(transfer.amount, RAW_UNITS_PER_CENT);
    assert_eq!(transfer.memo, "intent");
    link.amount_cents = u64::MAX / RAW_UNITS_PER_CENT;
    assert_eq!(
        PaymentTransfer::from_link(link)
            .expect("maximum raw amount")
            .amount,
        (u64::MAX / RAW_UNITS_PER_CENT) * RAW_UNITS_PER_CENT
    );

    link.amount_cents += 1;
    assert_eq!(
        PaymentTransfer::from_link(link),
        Err(PaymentTermsError::Amount)
    );
    link.amount_cents = 0;
    assert_eq!(
        PaymentTransfer::from_link(link),
        Err(PaymentTermsError::Amount)
    );
    link.amount_cents = 1;
}

fn invalid_terms(link: &mut PaymentLink) {
    link.memo = "different-intent".into();
    assert_eq!(
        PaymentTransfer::from_link(link),
        Err(PaymentTermsError::Memo)
    );
    link.memo = link.payment_intent_id.clone();
    link.kind = "other".into();
    assert_eq!(
        PaymentTransfer::from_link(link),
        Err(PaymentTermsError::Kind)
    );
    link.kind = "payment_required".into();
    link.destination_wallet = "invalid-address".into();
    assert_eq!(
        PaymentTransfer::from_link(link),
        Err(PaymentTermsError::Recipient)
    );
}
