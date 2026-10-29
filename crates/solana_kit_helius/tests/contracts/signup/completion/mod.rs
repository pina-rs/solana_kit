//! Submitted-signature completion contracts shared by all signup feature hosts.

mod cancellation;
mod failures;
mod flows;

pub use cancellation::dropped_completion;
pub use failures::completion_failures;
pub use flows::{completed_provisioning, completion_outcomes, terminal_exits};

use super::*;
use solana_kit_core::Signature;
use std::{cell::Cell, future::ready, rc::Rc};

const COMPLETE: &str = r#"{"status":"complete","phase":"active","subscriptionActive":true,"readyToRedirect":true,"message":"ready"}"#;
const PENDING: &str = r#"{"status":"pending","phase":"confirming","subscriptionActive":false,"readyToRedirect":false,"message":"waiting"}"#;
const PROJECTS: &str = concat!(
    r#"[{"id":"project/opaque","subscription":{"plan":"agent_v4","#,
    r#""billingPeriodStart":"ignored","billingPeriodEnd":"ignored"}},"#,
    r#"{"id":"unused-project","subscription":{"plan":"business_v4","#,
    r#""billingPeriodStart":"ignored","billingPeriodEnd":"ignored"}}]"#,
);
const EXISTING_KEY: &str = r#"{"apiKeys":[{"keyId":"existing-key"},{"keyId":"unused-key"}]}"#;
const EMPTY_KEYS: &str = r#"{"apiKeys":[]}"#;
const NEW_KEY: &str = r#"{"keyId":"new-key"}"#;
const SIGNATURE: Signature = Signature::from_bytes([7; 64]);
const WINDOW: u64 = 10;
const MAX_POLLS: usize = 4;

type CompletionResult = Result<SignupAndPayResult, SignupCompletionError<()>>;

fn policy() -> SignupCompletionPolicy {
    let polling = checkout::PollPolicy {
        timeout_ms: WINDOW,
        interval_ms: 1,
        max_polls: MAX_POLLS,
    };
    SignupCompletionPolicy {
        checkout: polling,
        provisioning: polling,
    }
}

fn identity() -> AuthenticatedIdentity {
    AuthenticatedIdentity {
        jwt: SensitiveString::new("fixture-jwt"),
        ref_id: "reference".into(),
        wallet_address: Address::from_bytes([1; 32]),
    }
}

fn pending() -> SignupAndPayResult {
    let payment_link = serde_json::from_str(
        r#"{
        "kind":"payment_required","paymentIntentId":"intent/opaque","amountCents":4900,
        "destinationWallet":"fixture-destination","memo":"intent/opaque",
        "expiresAt":"2026-01-01T00:00:00Z","paymentUrl":"https://fixture.invalid/pay/intent",
        "solanaPayUrl":"solana:fixture","planName":"Developer"
    }"#,
    )
    .expect("payment link fixture");
    SignupAndPayResult::Pending {
        identity: identity(),
        payment_link,
        tx_signature: SIGNATURE,
    }
}

fn provisioned_project() -> ProvisionedProject {
    let api_key = SensitiveString::new("existing-key");
    ProvisionedProject {
        project_id: "project/opaque".into(),
        endpoints: build_endpoints(&api_key),
        api_key,
    }
}

fn execute(
    input: &SignupAndPayResult,
    replies: Vec<Reply>,
    policy: SignupCompletionPolicy,
) -> (CompletionResult, Vec<ServiceRequest>) {
    let (client, trace) = client(replies);
    let ticks = Rc::new(Cell::new(0));
    let timer = ticks.clone();
    let original = input.clone();
    let result = futures::executor::block_on(resume_signup_and_pay(
        &client,
        input,
        policy,
        || ticks.get(),
        move |deadline| {
            timer.set(deadline);
            ready(Ok::<_, ()>(()))
        },
    ));
    assert_eq!(input, &original, "borrowed payment receipt changed");
    let requests = std::mem::take(&mut *trace.borrow_mut());

    (result, requests)
}

fn assert_read(request: &ServiceRequest, path: &[&str]) {
    assert_eq!(request.service, Service::Developer);
    assert_eq!(request.method, Verb::Get);
    assert_eq!(request.path_segments.as_slice(), path);
    assert!(request.query.is_empty());
    assert!(request.body.as_deref().is_none());
}
