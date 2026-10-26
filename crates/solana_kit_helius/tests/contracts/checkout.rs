//! Checkout operations use actual service calls and explicit local time controls.

use super::service::exchange;
use solana_kit_client::{Reply, TransportError};
use solana_kit_helius::{
    Field,
    auth::{BillingPeriod, Plan, checkout::*},
    service::*,
};
use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    future::{Ready, ready},
    rc::Rc,
};

const INTENT: &[u8] = br#"{"id":"i","status":"pending","destinationWallet":"d","amount":9007199254740993,"solanaPayUrl":"solana:address","expiresAt":"later"}"#;
const PENDING: &[u8] = br#"{"status":"pending","phase":"confirming","subscriptionActive":false,"readyToRedirect":false,"message":"waiting"}"#;
const COMPLETE: &[u8] = br#"{"status":"complete","phase":"active","subscriptionActive":true,"readyToRedirect":true,"message":"ready"}"#;

/// Every checkout endpoint preserves body/query distinctions and dynamic ID segments.
pub fn endpoint_contracts() {
    let operation: CheckoutInitializeRequest =
        serde_json::from_str(r#"{"priceId":"p","refId":"r","qty":2}"#).expect("initialize");
    let request = exchange(operation, INTENT);
    assert_eq!(request.path_segments, ["checkout", "initialize"]);
    assert_eq!(request.method, Verb::Post);
    assert_eq!(
        request.body.as_deref(),
        Some(br#"{"priceId":"p","refId":"r","qty":2}"#.as_slice())
    );
    let request = exchange(
        GetPaymentIntent {
            payment_intent_id: "a/b?c".into(),
        },
        INTENT,
    );
    assert_eq!(request.path_segments, ["checkout", "a/b?c"]);
    let request = exchange(
        GetPaymentStatus {
            payment_intent_id: "a/b?c".into(),
        },
        PENDING,
    );
    assert_eq!(request.path_segments, ["checkout", "a/b?c", "status"]);
    let request = exchange(
        DevPortalConfigsRequest {
            include_agent_plan: true,
        },
        br#"{"stripe":{"priceIds":{"Monthly":{},"Yearly":{}}}}"#,
    );
    assert_eq!(request.path_segments, ["dev-portal", "configs"]);
    assert_eq!(request.query, [("agent".to_owned(), "cli".to_owned())]);

    let preview = br#"{"planName":"Developer","period":"monthly","baseAmount":100,"subtotal":90,"appliedCredits":10,"proratedCredits":0,"discounts":5,"dueToday":75,"destinationWallet":"d","note":"n"}"#;
    let operation = CheckoutPreviewRequest {
        price_id: "p&1".into(),
        ref_id: "r".into(),
        coupon_code: Some("c".into()),
        qty: Some(2),
    };
    let request = exchange(operation, preview);
    assert_eq!(request.path_segments, ["checkout", "preview"]);
    assert_eq!(
        request.query,
        [
            ("priceId".to_owned(), "p&1".to_owned()),
            ("refId".to_owned(), "r".to_owned()),
            ("couponCode".to_owned(), "c".to_owned()),
            ("qty".to_owned(), "2".to_owned())
        ]
    );
}

/// Plan price lookup keeps case-sensitive provider keys and rejects missing or empty prices.
pub fn price_contracts() {
    let prices: PriceIds = serde_json::from_str(r#"{"Monthly":{"developer_v4":"month"},"Yearly":{"developer_v4":"year"},"AgentPlan":"agent"}"#).expect("price schema");
    assert_eq!(
        price_id(&prices, Plan::Developer, BillingPeriod::Monthly),
        Some("month")
    );
    assert_eq!(
        price_id(&prices, Plan::Developer, BillingPeriod::Yearly),
        Some("year")
    );
    assert_eq!(
        price_id(&prices, Plan::Agent, BillingPeriod::Yearly),
        Some("agent")
    );
    assert_eq!(
        price_id(&prices, Plan::Business, BillingPeriod::Monthly),
        None
    );
    assert!(
        serde_json::from_str::<PriceIds>(r#"{"Monthly":{"developer_v4":1},"Yearly":{}}"#).is_err()
    );
    assert!(
        serde_json::from_str::<DevPortalConfigsResponse>(
            r#"{"stripe":{"priceIds":{"Monthly":{},"Yearly":{}}},"openPay":false}"#
        )
        .is_err()
    );
    assert_eq!(
        "PROFESSIONAL".parse::<Plan>().expect("normalized plan"),
        Plan::Professional
    );

    assert!("professional ".parse::<Plan>().is_err());
    assert!(matches!(prices.agent_plan, Field::Value(_)));
}

struct Replies {
    replies: RefCell<VecDeque<Reply>>,
    requests: Rc<Cell<usize>>,
}

impl ServiceTransport<Service> for Replies {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, _request: ServiceRequest) -> Self::Exchange<'_> {
        self.requests.set(self.requests.get() + 1);
        ready(Ok(self
            .replies
            .borrow_mut()
            .pop_front()
            .expect("one planned response per exchange")))
    }
}

fn client(replies: &[(u16, &[u8])]) -> (ServiceClient<Replies>, Rc<Cell<usize>>) {
    let requests = Rc::new(Cell::new(0));
    let transport = Replies {
        requests: Rc::clone(&requests),
        replies: RefCell::new(
            replies
                .iter()
                .map(|(status, body)| Reply {
                    status: *status,
                    body: body.to_vec(),
                })
                .collect(),
        ),
    };
    (ServiceClient::new(transport, Budgets::default()), requests)
}

/// A local Rc clock advances only through an explicitly supplied sleep future.
pub fn polling_terminal_and_expiry() {
    let (client, requests) = client(&[(200, PENDING), (200, COMPLETE)]);
    let clock = Rc::new(Cell::new(0));
    let sleep_clock = Rc::clone(&clock);
    let outcome = futures::executor::block_on(poll_until_terminal(
        &client,
        "i".into(),
        PollPolicy::default(),
        || clock.get(),
        move |wake| {
            sleep_clock.set(wake);
            ready(Ok::<(), ()>(()))
        },
    ))
    .expect("polling");
    assert!(matches!(outcome, PollOutcome::Completed(_)));
    assert_eq!(requests.get(), 2);
    assert_eq!(clock.get(), 1_000);
    let (expired_client, requests) = self::client(&[(410, b"gone")]);

    let outcome = futures::executor::block_on(poll_until_terminal(
        &expired_client,
        "i".into(),
        PollPolicy::default(),
        || 0,
        |_| ready(Ok::<(), ()>(())),
    ))
    .expect("expiry is explicit");
    assert!(matches!(outcome, PollOutcome::Expired(None)));
    assert_eq!(requests.get(), 1);
}

/// Timeouts retain the last real response. Timer and decreasing-clock failures remain errors.
pub fn polling_limits_and_clock_errors() {
    let (client, requests) = client(&[(200, PENDING)]);
    let outcome = futures::executor::block_on(poll_until_terminal(
        &client,
        "i".into(),
        PollPolicy {
            max_polls: 1,
            ..PollPolicy::default()
        },
        || 0,
        |_| ready(Ok::<(), ()>(())),
    ))
    .expect("finite observation limit");

    match outcome {
        PollOutcome::Timeout(Some(status)) => assert_eq!(status.message, "waiting"),
        other => panic!("wrong timeout {other:?}"),
    }

    assert_eq!(requests.get(), 1);
    let (client, requests) = self::client(&[(200, PENDING)]);
    let result = futures::executor::block_on(poll_until_terminal(
        &client,
        "i".into(),
        PollPolicy::default(),
        || 0,
        |_| ready(Err::<(), &str>("timer failed")),
    ));
    assert!(matches!(result, Err(PollError::Clock("timer failed"))));
    assert_eq!(requests.get(), 1);
    let (client, requests) = self::client(&[]);
    let ticks = RefCell::new(VecDeque::from([10, 9]));
    let result = futures::executor::block_on(poll_until_terminal(
        &client,
        "i".into(),
        PollPolicy::default(),
        || ticks.borrow_mut().pop_front().expect("clock sample"),
        |_| ready(Ok::<(), ()>(())),
    ));

    assert!(matches!(result, Err(PollError::Policy)));
    assert_eq!(requests.get(), 0);
}
