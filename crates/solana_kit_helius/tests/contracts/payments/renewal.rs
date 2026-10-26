//! Renewals read one existing identity and preserve its exact integer amount.

use super::*;

/// A renewal never initializes another intent or fabricates a terminal status.
pub fn existing_intents() {
    let intent = INTENT.replace("\"amount\":4900", "\"amount\":9007199254740993");
    let (client, trace) = scripted(vec![reply(&intent)]);
    let link =
        futures::executor::block_on(pay_renewal(&client, "intent/opaque", None)).expect("renewal");
    assert_eq!(link.amount_cents, 9_007_199_254_740_993);
    assert_eq!(link.plan_name, "Subscription renewal");
    assert_eq!(link.memo, "intent/opaque");
    assert_eq!(trace.borrow().len(), 1);
    assert_eq!(trace.borrow()[0].method, Verb::Get);
    assert_eq!(
        trace.borrow()[0].path_segments,
        ["checkout", "intent/opaque"]
    );
    assert!(trace.borrow()[0].body.as_deref().is_none());
    terminal_statuses();
    invalid_renewals();
}

fn terminal_statuses() {
    for status in ["complete", "expired", "failed", "Pending"] {
        let intent = INTENT.replace(
            "\"status\":\"pending\"",
            &format!("\"status\":\"{status}\""),
        );
        let (client, trace) = scripted(vec![reply(&intent)]);
        assert!(matches!(
            futures::executor::block_on(pay_renewal(&client, "intent/opaque", None)),
            Err(PaymentError::NotPending)
        ));
        assert_eq!(trace.borrow().len(), 1);
    }
}

fn invalid_renewals() {
    let (client, trace) = scripted(Vec::new());

    for id in ["", ".", ".."] {
        assert!(matches!(
            futures::executor::block_on(pay_renewal(&client, id, None)),
            Err(PaymentError::InvalidPaymentIntent)
        ));
    }

    assert!(matches!(
        futures::executor::block_on(pay_renewal(&client, "id", Some("/relative"))),
        Err(PaymentError::InvalidPaymentHost)
    ));
    assert!(trace.borrow().is_empty());

    let (client, trace) = scripted(vec![reply(INTENT)]);
    assert!(matches!(
        futures::executor::block_on(pay_renewal(&client, "another-id", None)),
        Err(PaymentError::IntentMismatch)
    ));
    assert_eq!(trace.borrow().len(), 1);
    let zero = INTENT.replace("\"amount\":4900", "\"amount\":0");
    let (client, trace) = scripted(vec![reply(&zero)]);
    assert!(matches!(
        futures::executor::block_on(pay_renewal(&client, "intent/opaque", None)),
        Err(PaymentError::ZeroAmount)
    ));
    assert_eq!(trace.borrow().len(), 1);
}
