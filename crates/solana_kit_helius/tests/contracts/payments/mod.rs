//! Public payment APIs execute over the same local transport used by signup.

mod cancellation;
mod failures;
mod renewal;

pub use cancellation::dropped_preparation;
pub use failures::payment_failures;
pub use renewal::existing_intents;

use super::payment_support::{body, client as scripted, reply};
use solana_kit_core::Address;
use solana_kit_helius::{SensitiveString, auth::*, service::*};
use std::num::NonZeroU64;

const PREVIEW: &str = concat!(
    r#"{"planName":"Provider label","period":"monthly","baseAmount":4900,"subtotal":4900,"#,
    r#""appliedCredits":0,"proratedCredits":0,"discounts":0,"dueToday":4900,"#,
    r#""destinationWallet":"destination","note":"preview"}"#,
);
const INTENT: &str = concat!(
    r#"{"id":"intent/opaque","status":"pending","destinationWallet":"destination","amount":4900,"#,
    r#""solanaPayUrl":"solana:fixture","expiresAt":"2026-01-01T00:00:00Z"}"#,
);

fn request() -> CreatePaymentRequest {
    CreatePaymentRequest::new("project/opaque", PaymentPrice::Id("price&opaque".into()))
}

use solana_kit_helius::auth::BillingPeriod::{Monthly, Yearly};

const CONFIG: &str = concat!(
    r#"{"stripe":{"priceIds":{"Monthly":{"developer_v4":"developer-month","#,
    r#""business_v4":"business-month","professional_v4":"professional-month"},"#,
    r#""Yearly":{"developer_v4":"developer-year","business_v4":"business-year","#,
    r#""professional_v4":"professional-year"},"AgentPlan":"agent-price"}}}"#,
);
const UPGRADE_CASES: [(Plan, BillingPeriod, &str, &str); 8] = [
    (Plan::Agent, Monthly, "agent-price", "Agent Plan"),
    (Plan::Agent, Yearly, "agent-price", "Agent Plan"),
    (
        Plan::Developer,
        Monthly,
        "developer-month",
        "Developer (Monthly)",
    ),
    (
        Plan::Developer,
        Yearly,
        "developer-year",
        "Developer (Yearly)",
    ),
    (
        Plan::Business,
        Monthly,
        "business-month",
        "Business (Monthly)",
    ),
    (Plan::Business, Yearly, "business-year", "Business (Yearly)"),
    (
        Plan::Professional,
        Monthly,
        "professional-month",
        "Professional (Monthly)",
    ),
    (
        Plan::Professional,
        Yearly,
        "professional-year",
        "Professional (Yearly)",
    ),
];

/// Every named plan and billing period resolves exactly once before preparing an upgrade.
pub fn plan_upgrades() {
    for (plan, period, price, label) in UPGRADE_CASES {
        let (client, trace) = scripted(vec![reply(CONFIG), reply(PREVIEW), reply(INTENT)]);
        let mut request =
            CreatePaymentRequest::new("project/opaque", PaymentPrice::Plan { plan, period });
        request.plan_name_override = Some("ignored for named plans".into());
        let link = futures::executor::block_on(upgrade_plan(&client, request)).expect("upgrade");
        let calls = trace.borrow();

        assert_eq!(link.plan_name, label);
        assert_eq!(calls.len(), 3);
        let query = if plan == Plan::Agent {
            vec![("agent".into(), "cli".into())]
        } else {
            Vec::new()
        };

        assert_eq!(calls[0].query, query);
        assert_eq!(calls[1].query[0], ("priceId".into(), price.into()));
        assert_eq!(
            body(&calls[2]),
            serde_json::json!({"priceId":price,"refId":"project/opaque"})
        );
    }
}

/// An explicit price bypasses discovery and forwards the exact same checkout inputs twice.
pub fn explicit_prices() {
    let (client, trace) = scripted(vec![reply(PREVIEW), reply(INTENT)]);
    let request = explicit_request();
    assert!(!format!("{request:?}").contains("fixture@example.invalid"));
    let link = futures::executor::block_on(create_payment(&client, request)).expect("payment");
    let calls = trace.borrow();
    assert_eq!(calls.len(), 2);
    assert_eq!(calls[0].method, Verb::Get);
    assert_eq!(calls[0].path_segments, ["checkout", "preview"]);
    assert_eq!(
        calls[0].query,
        [
            ("priceId".into(), "price&opaque".into()),
            ("refId".into(), "project/opaque".into()),
            ("couponCode".into(), "coupon&opaque".into()),
            ("qty".into(), "3".into())
        ]
    );
    assert_eq!(calls[1].method, Verb::Post);
    assert_eq!(
        body(&calls[1]),
        serde_json::json!({
            "priceId":"price&opaque", "refId":"project/opaque", "qty":3,
            "couponCode":"coupon&opaque", "email":"fixture@example.invalid",
            "firstName":"fixture-first", "lastName":"fixture-last",
            "walletAddress":Address::from_bytes([1;32]).to_string()
        })
    );
    assert_eq!(link.plan_name, "One-time credits");
    assert_eq!(link.memo, "intent/opaque");
    assert_eq!(
        link.payment_url,
        "https://checkout.example.invalid/base/pay/intent%2Fopaque"
    );
    assert_eq!(link.amount_cents, 4900);
    assert_eq!(link.solana_pay_url, "solana:fixture");
    drop(calls);
    default_label();
}

fn explicit_request() -> CreatePaymentRequest {
    let mut request = request();
    request.qty = NonZeroU64::new(3);
    request.coupon_code = Some("coupon&opaque".into());
    request.email = Some(SensitiveString::new("fixture@example.invalid"));
    request.first_name = Some(SensitiveString::new("fixture-first"));
    request.last_name = Some(SensitiveString::new("fixture-last"));

    request.wallet_address = Some(Address::from_bytes([1; 32]));
    request.payment_host = Some("https://checkout.example.invalid/base/".into());
    request.plan_name_override = Some("One-time credits".into());

    request
}

fn default_label() {
    let (client, _) = scripted(vec![reply(PREVIEW), reply(INTENT)]);
    let link = futures::executor::block_on(create_payment(&client, request())).expect("payment");
    assert_eq!(link.plan_name, "Helius");
    assert_eq!(
        link.payment_url,
        "https://dashboard.helius.dev/pay/intent%2Fopaque"
    );
}
