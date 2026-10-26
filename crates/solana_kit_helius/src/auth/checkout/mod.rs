//! Direct checkout calls remain separate from payment approval and transaction submission.

mod models;
#[cfg(feature = "checkout")]
mod payment;
#[cfg(feature = "checkout")]
mod payment_models;
mod polling;
pub use models::*;
#[cfg(feature = "checkout")]
pub use payment::{create_payment, pay_renewal};
#[cfg(feature = "checkout")]
pub use payment_models::{CreatePaymentRequest, PaymentError, PaymentPrice};
pub use polling::{PollError, PollOutcome, PollPolicy, poll_until_terminal};

use super::{BillingPeriod, Plan};
use crate::{
    Field,
    service::{Body, Error, Operation, Service, ServiceClient, ServiceTransport, Verb, pair},
};

request!(
    "Read checkout preview using one explicit price identifier.",
    CheckoutPreviewRequest {
        price_id: String => "priceId",
        ref_id: String => "refId",
    } optional { coupon_code: String => "couponCode",
    qty: u64 => "qty" }
);
request!(
    "Read one initialized payment intent.",
    GetPaymentIntent {
        payment_intent_id: String => "paymentIntentId",
    } optional {}
);
request!(
    "Read one payment intent's status.",
    GetPaymentStatus {
        payment_intent_id: String => "paymentIntentId",
    } optional {}
);
request!(
    "Read developer portal billing configuration.",
    DevPortalConfigsRequest {
        include_agent_plan: bool => "includeAgentPlan",
    } optional {}
);

rest!(
    CheckoutInitializeRequest,
    CheckoutInitializeResponse,
    Developer,
    Post,
    Json,
    |_request| vec!["checkout".into(), "initialize".into()]
);
rest!(
    GetPaymentIntent,
    CheckoutInitializeResponse,
    Developer,
    Get,
    Absent,
    |request| vec!["checkout".into(), request.payment_intent_id.clone()]
);
rest!(
    GetPaymentStatus,
    CheckoutStatusResponse,
    Developer,
    Get,
    Absent,
    |request| vec![
        "checkout".into(),
        request.payment_intent_id.clone(),
        "status".into()
    ]
);

impl Operation for CheckoutPreviewRequest {
    type Response = CheckoutPreviewResponse;
    const SERVICE: Service = Service::Developer;
    const VERB: Verb = Verb::Get;
    const BODY: Body = Body::Absent;

    fn path(&self) -> Vec<String> {
        vec!["checkout".into(), "preview".into()]
    }

    fn query(&self) -> Vec<(String, String)> {
        let mut query = vec![
            ("priceId".into(), self.price_id.clone()),
            ("refId".into(), self.ref_id.clone()),
        ];
        pair(&mut query, "couponCode", self.coupon_code.as_ref());
        pair(&mut query, "qty", self.qty);
        query
    }
}

impl Operation for DevPortalConfigsRequest {
    type Response = DevPortalConfigsResponse;
    const SERVICE: Service = Service::Developer;
    const VERB: Verb = Verb::Get;
    const BODY: Body = Body::Absent;

    fn path(&self) -> Vec<String> {
        vec!["dev-portal".into(), "configs".into()]
    }

    fn query(&self) -> Vec<(String, String)> {
        if self.include_agent_plan {
            vec![("agent".into(), "cli".into())]
        } else {
            Vec::new()
        }
    }
}

/// Selects an exact price identifier without an empty-string fallback.
pub fn price_id(prices: &PriceIds, plan: Plan, period: BillingPeriod) -> Option<&str> {
    let value = match plan {
        Plan::Agent => prices.agent_plan.value().map(String::as_str),
        _ => {
            let prices = match period {
                BillingPeriod::Monthly => &prices.monthly,
                BillingPeriod::Yearly => &prices.yearly,
            };

            prices.get(plan.usage_plan()).map(String::as_str)
        }
    };

    value.filter(|value| !value.is_empty())
}

/// Resolves one plan through the actual provider configuration operation.
pub async fn resolve_price_id<T: ServiceTransport<Service>>(
    client: &ServiceClient<T>,
    plan: Plan,
    period: BillingPeriod,
) -> Result<String, Error> {
    let configs = client
        .call(DevPortalConfigsRequest {
            include_agent_plan: plan == Plan::Agent,
        })
        .await?;
    price_id(&configs.stripe.price_ids, plan, period)
        .map(str::to_owned)
        .ok_or_else(|| {
            Error::Schema(<serde_json::Error as serde::de::Error>::custom(
                "missing provider price identifier",
            ))
        })
}

/// Returns the source's empty prepaid-map default only for genuinely absent or null data.
pub fn prepaid_price_ids(
    configs: DevPortalConfigsResponse,
) -> std::collections::BTreeMap<String, String> {
    match configs.stripe.prepaid_credits_plans {
        Field::Value(prices) => prices,
        Field::Missing | Field::Null => std::collections::BTreeMap::new(),
    }
}
