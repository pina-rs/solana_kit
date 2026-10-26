//! Public signup journeys use local transports with the same ownership rules as Wasm.

mod billing;
mod payment;
#[cfg(feature = "crypto")]
mod secret;

pub use billing::{billing_windows, existing_projects};
pub use payment::{checkout_failures, new_subscription};
#[cfg(feature = "crypto")]
pub use secret::signed_authentication;

use solana_kit_client::{Reply, TransportError};
use solana_kit_core::Address;
use solana_kit_helius::{SensitiveString, auth::*, service::*};
use std::{
    cell::RefCell,
    collections::VecDeque,
    future::{Ready, ready},
    rc::Rc,
};

type Trace = Rc<RefCell<Vec<ServiceRequest>>>;

struct Replay {
    replies: RefCell<VecDeque<Reply>>,
    requests: Trace,
}

impl ServiceTransport<Service> for Replay {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: ServiceRequest) -> Self::Exchange<'_> {
        assert_eq!(request.service, Service::Developer);
        self.requests.borrow_mut().push(request);
        ready(Ok(self
            .replies
            .borrow_mut()
            .pop_front()
            .expect("scripted exchange")))
    }
}

fn client(replies: Vec<Reply>) -> (ServiceClient<Replay>, Trace) {
    let requests = Rc::new(RefCell::new(Vec::new()));
    let transport = Replay {
        replies: RefCell::new(replies.into()),
        requests: Rc::clone(&requests),
    };
    (ServiceClient::new(transport, Budgets::default()), requests)
}

fn reply(body: &str) -> Reply {
    Reply {
        status: 200,
        body: body.as_bytes().to_vec(),
    }
}

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

fn body(request: &ServiceRequest) -> serde_json::Value {
    serde_json::from_slice(request.body.as_deref().expect("JSON request")).expect("object body")
}
