//! New subscriptions prepare one explicit checkout without signing or sending funds.

use super::{ContactField, SignupError, urls};
use crate::{
    SensitiveString,
    auth::{
        AuthenticatedIdentity, BillingPeriod, PaymentLink, Plan, SignupRequest,
        checkout::{CheckoutInitializeRequest, CheckoutPreviewRequest, resolve_price_id},
    },
    service::{Service, ServiceClient, ServiceTransport},
};

/// Freeze one provider price before preview and initialization so both refer to the same offer.
pub(super) async fn prepare<T: ServiceTransport<Service>>(
    client: &ServiceClient<T>,
    identity: &AuthenticatedIdentity,
    request: &SignupRequest,
) -> Result<PaymentLink, SignupError> {
    contact(request)?;
    let host = urls::payment_host(request.payment_host.as_deref())?;
    let price_id = resolve_price_id(client, request.plan, request.period).await?;
    let preview = client
        .call(CheckoutPreviewRequest {
            price_id: price_id.clone(),
            ref_id: identity.ref_id.clone(),
            coupon_code: request.coupon_code.clone(),
            qty: None,
        })
        .await?;

    if preview.due_today == 0 {
        return Err(SignupError::ZeroAmount);
    }

    let intent = client
        .call(CheckoutInitializeRequest {
            price_id,
            ref_id: identity.ref_id.clone(),
            email: request.email.clone(),
            first_name: request.first_name.clone(),
            last_name: request.last_name.clone(),
            wallet_address: Some(identity.wallet_address.to_string()),
            coupon_code: request.coupon_code.clone(),
            qty: None,
        })
        .await?;

    if intent.amount == 0 {
        return Err(SignupError::ZeroAmount);
    }

    let payment_url = urls::build_payment_url(&intent.id, Some(host))?;

    Ok(PaymentLink {
        kind: "payment_required".into(),
        payment_intent_id: intent.id.clone(),
        amount_cents: intent.amount,
        destination_wallet: intent.destination_wallet,
        memo: intent.id,
        expires_at: intent.expires_at,
        payment_url,
        solana_pay_url: intent.solana_pay_url,
        plan_name: plan_name(request.plan, request.period),
    })
}

fn contact(request: &SignupRequest) -> Result<(), SignupError> {
    for (field, value) in [
        (ContactField::Email, &request.email),
        (ContactField::FirstName, &request.first_name),
        (ContactField::LastName, &request.last_name),
    ] {
        if value.as_ref().is_none_or(blank) {
            return Err(SignupError::MissingContact(field));
        }
    }

    Ok(())
}

fn blank(value: &SensitiveString) -> bool {
    value
        .expose()
        .trim_matches(|character: char| character.is_whitespace() || character == '\u{feff}')
        .is_empty()
}

fn plan_name(plan: Plan, period: BillingPeriod) -> String {
    if plan == Plan::Agent {
        return "Agent Plan".into();
    }

    let name = match plan {
        Plan::Agent => "Agent",
        Plan::Developer => "Developer",
        Plan::Business => "Business",
        Plan::Professional => "Professional",
    };
    let period = match period {
        BillingPeriod::Monthly => "Monthly",
        BillingPeriod::Yearly => "Yearly",
    };

    format!("{name} ({period})")
}
