#![cfg(all(not(target_arch = "wasm32"), feature = "signup"))]
//! The caller's native executor owns the concrete completion future and receipt.
use solana_kit_client::{Reply, TransportError};
use solana_kit_core::{Address, Signature};
use solana_kit_helius::{SensitiveString, auth::*, service::*};
use std::{
    future::{Ready, ready},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

const SIGNATURE: Signature = Signature::from_bytes([7; 64]);
const HTTP_OK: u16 = 200;

struct Native {
    calls: Arc<AtomicUsize>,
}

impl ServiceTransport<Service> for Native {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: ServiceRequest) -> Self::Exchange<'_> {
        assert_eq!(request.service, Service::Developer);
        assert_eq!(request.method, Verb::Get);
        assert!(request.body.as_deref().is_none());
        self.calls.fetch_add(1, Ordering::SeqCst);
        let body = if request.path_segments == ["checkout", "intent", "status"] {
            r#"{"status":"complete","phase":"active","subscriptionActive":true,"readyToRedirect":true,"message":"ready"}"#
        } else if request.path_segments == ["projects"] {
            r#"[{"id":"project","subscription":{"plan":"agent_v4","billingPeriodStart":"ignored","billingPeriodEnd":"ignored"}}]"#
        } else if request.path_segments == ["projects", "project"] {
            r#"{"apiKeys":[{"keyId":"native-key"}]}"#
        } else {
            panic!("unexpected completion request")
        };

        ready(Ok(Reply {
            status: HTTP_OK,
            body: body.as_bytes().to_vec(),
        }))
    }
}

fn receipt() -> SignupAndPayResult {
    let payment_link = serde_json::from_str(
        r#"{
        "kind":"payment_required","paymentIntentId":"intent","amountCents":4900,
        "destinationWallet":"fixture","memo":"intent","expiresAt":"later",
        "paymentUrl":"https://fixture.invalid/pay/intent","solanaPayUrl":"solana:fixture",
        "planName":"Developer"
    }"#,
    )
    .expect("payment fixture");
    SignupAndPayResult::Pending {
        identity: AuthenticatedIdentity {
            jwt: SensitiveString::new("fixture-jwt"),
            ref_id: "reference".into(),
            wallet_address: Address::from_bytes([1; 32]),
        },
        payment_link,
        tx_signature: SIGNATURE,
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn submitted_signup_completion_retains_send_and_the_borrowed_receipt() {
    let calls = Arc::new(AtomicUsize::new(0));
    let client = ServiceClient::new(
        Native {
            calls: calls.clone(),
        },
        Budgets::default(),
    );
    let actual = tokio::spawn(async move {
        let receipt = receipt();
        let original = receipt.clone();
        let polling = checkout::PollPolicy {
            timeout_ms: 10,
            interval_ms: 1,
            max_polls: 2,
        };
        let result = resume_signup_and_pay(
            &client,
            &receipt,
            SignupCompletionPolicy {
                checkout: polling,
                provisioning: polling,
            },
            || 0,
            |_| ready(Ok::<_, ()>(())),
        )
        .await;
        assert_eq!(receipt, original);
        result
    })
    .await
    .expect("native completion task joined")
    .expect("signup completed");
    let SignupAndPayResult::Completed {
        project,
        tx_signature,
        payment_intent_id,
        ..
    } = actual
    else {
        panic!("payment did not reach a provisioned result");
    };
    assert_eq!(project.project_id, "project");
    assert_eq!(project.api_key.expose(), "native-key");
    assert_eq!(
        project.endpoints.mainnet.expose(),
        "https://mainnet.helius-rpc.com/?api-key=native-key"
    );
    assert_eq!(tx_signature, SIGNATURE);
    assert_eq!(payment_intent_id, "intent");
    assert_eq!(calls.load(Ordering::SeqCst), 3);
}
