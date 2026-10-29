//! Public signup journeys use local transports with the same ownership rules as Wasm.

mod billing;
mod completion;
mod payment;
#[cfg(feature = "crypto")]
mod secret;

pub use billing::{billing_windows, existing_projects};
pub use completion::{
    completed_provisioning, completion_failures, completion_outcomes, dropped_completion,
    terminal_exits,
};
pub use payment::{checkout_failures, new_subscription};
#[cfg(feature = "crypto")]
pub use secret::signed_authentication;

use super::payment_support::{Replay, body, client, reply};
use solana_kit_client::Reply;
use solana_kit_core::Address;
use solana_kit_helius::{SensitiveString, auth::*, service::*};

fn request(plan: Plan) -> SignupRequest {
    SignupRequest::new(
        SignupAuthentication::Authenticated {
            jwt: SensitiveString::new("fixture-jwt"),
            ref_id: "reference".into(),
            wallet_address: Address::from_bytes([1; 32]),
        },
        plan,
    )
}

fn execute(
    request: SignupRequest,
    replies: Vec<Reply>,
) -> (Result<SignupResult, SignupError>, Vec<ServiceRequest>) {
    let (bootstrap, bootstrap_trace) = client(Vec::new());
    let (session, trace) = client(replies);
    let result = futures::executor::block_on(auth_signup(&bootstrap, request, 0, |jwt| {
        assert_eq!(jwt.expose(), "fixture-jwt");
        Ok(session)
    }));
    assert!(bootstrap_trace.borrow().is_empty());
    let requests = std::mem::take(&mut *trace.borrow_mut());
    (result, requests)
}

fn project(plan: &str, start: &str, end: &str) -> String {
    serde_json::json!([{
        "id":"project/opaque",
        "subscription": {
            "plan":plan, "billingPeriodStart":start, "billingPeriodEnd":end
        }
    }])
    .to_string()
}
