//! Rejected checkout input and preview failures cannot reach initialization.

use super::*;
use solana_kit_client::Reply;

/// Each input, protocol and service failure is visible at its actual public boundary.
pub fn payment_failures() {
    invalid_inputs();
    provider_failures();
    invalid_intents();
}

fn invalid_inputs() {
    let mut reference = request();
    reference.ref_id.clear();
    let mut price = request();
    price.price = PaymentPrice::Id(String::new());
    let mut host = request();
    host.payment_host = Some("https://user@host".into());

    let error = rejected(reference);
    assert_eq!(
        error.to_string(),
        "Helius payment preparation failed: payment reference or project ID is empty"
    );
    assert!(matches!(error, PaymentError::InvalidReference));
    assert!(matches!(rejected(price), PaymentError::InvalidPrice));
    assert!(matches!(rejected(host), PaymentError::InvalidPaymentHost));
}

fn rejected(request: CreatePaymentRequest) -> PaymentError {
    let (client, trace) = scripted(Vec::new());
    let error = futures::executor::block_on(create_payment(&client, request)).expect_err("input");
    assert!(trace.borrow().is_empty());
    error
}

fn provider_failures() {
    let zero = PREVIEW.replace("\"dueToday\":4900", "\"dueToday\":0");
    let malformed = PREVIEW.replace("\"dueToday\":4900", "\"dueToday\":\"4900\"");

    for preview in [zero, malformed] {
        let (client, trace) = scripted(vec![reply(&preview)]);
        assert!(futures::executor::block_on(create_payment(&client, request())).is_err());
        assert_eq!(trace.borrow().len(), 1);
    }

    let (client, trace) = scripted(vec![Reply {
        status: 503,
        body: b"private-provider-body".to_vec(),
    }]);
    let error =
        futures::executor::block_on(create_payment(&client, request())).expect_err("preview");
    assert!(!format!("{error:?} {error}").contains("private-provider-body"));
    assert!(error.to_string().contains("503"));
    assert!(std::error::Error::source(&error).is_some());
    let PaymentError::Service(Error::Http { status, body }) = error else {
        panic!("original HTTP cause")
    };
    assert_eq!(status, 503);
    assert_eq!(body, b"private-provider-body");
    assert_eq!(trace.borrow().len(), 1);
}

fn invalid_intents() {
    for id in ["", ".", ".."] {
        assert!(matches!(
            build_payment_url(id, None),
            Err(PaymentError::InvalidPaymentIntent)
        ));
    }

    let zero = INTENT.replace("\"amount\":4900", "\"amount\":0");
    let empty = INTENT.replace("\"id\":\"intent/opaque\"", "\"id\":\"\"");

    assert!(matches!(invalid_intent(&zero), PaymentError::ZeroAmount));
    let dot = INTENT.replace("intent/opaque", "..");
    assert!(matches!(
        invalid_intent(&dot),
        PaymentError::InvalidPaymentIntent
    ));
    assert!(matches!(
        invalid_intent(&empty),
        PaymentError::InvalidPaymentIntent
    ));
}

fn invalid_intent(intent: &str) -> PaymentError {
    let (client, trace) = scripted(vec![reply(PREVIEW), reply(intent)]);
    let error =
        futures::executor::block_on(create_payment(&client, request())).expect_err("intent");
    assert_eq!(trace.borrow().len(), 2);
    error
}
