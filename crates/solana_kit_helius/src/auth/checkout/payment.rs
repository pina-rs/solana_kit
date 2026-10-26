//! Checkout orchestration resolves one price and never signs or submits funds.

use super::{
    CheckoutInitializeRequest, CheckoutInitializeResponse, CheckoutPreviewRequest,
    CreatePaymentRequest, GetPaymentIntent, PaymentError, PaymentPrice, resolve_price_id,
};
use crate::{
    auth::{BillingPeriod, PaymentLink, Plan, urls},
    service::{Service, ServiceClient, ServiceTransport},
};

/// Previews and initializes one checkout using the same exact provider price.
///
/// The client must bind its bearer token only to the approved Developer origin.
/// Explicit prices bypass configuration discovery. No payment is signed or sent.
/// Dropping this future drops its active exchange; initialization is never retried.
///
/// # Errors
/// Rejects empty references/prices, invalid payment bases and zero totals.
/// Every service or preview failure propagates before initialization.
pub async fn create_payment<T: ServiceTransport<Service>>(
    client: &ServiceClient<T>,
    request: CreatePaymentRequest,
) -> Result<PaymentLink, PaymentError> {
    if request.ref_id.is_empty() {
        return Err(PaymentError::InvalidReference);
    }

    let host = urls::payment_host(request.payment_host.as_deref())?;

    let price_id = match &request.price {
        PaymentPrice::Plan { plan, period } => resolve_price_id(client, *plan, *period).await?,

        PaymentPrice::Id(value) if !value.is_empty() => value.clone(),
        PaymentPrice::Id(_) => return Err(PaymentError::InvalidPrice),
    };

    let preview = client
        .call(CheckoutPreviewRequest {
            price_id: price_id.clone(),
            ref_id: request.ref_id.clone(),
            coupon_code: request.coupon_code.clone(),
            qty: request.qty.map(core::num::NonZeroU64::get),
        })
        .await?;

    if preview.due_today == 0 {
        return Err(PaymentError::ZeroAmount);
    }

    let intent = client
        .call(CheckoutInitializeRequest {
            price_id,
            ref_id: request.ref_id,
            email: request.email,
            first_name: request.first_name,
            last_name: request.last_name,
            wallet_address: request.wallet_address.map(|address| address.to_string()),
            coupon_code: request.coupon_code,
            qty: request.qty.map(core::num::NonZeroU64::get),
        })
        .await?;

    payment_link(
        intent,
        plan_name(request.price, request.plan_name_override),
        host,
    )
}

/// Reconstructs a payment link for one existing pending checkout intent.
///
/// This performs one read. It never initializes another intent, signs a
/// transaction or submits a payment. The caller owns JWT binding and approval.
///
/// # Errors
/// Rejects invalid inputs, non-pending or mismatched intents, zero totals and
/// service failures. Provider error bytes remain available through the typed cause.
pub async fn pay_renewal<T: ServiceTransport<Service>>(
    client: &ServiceClient<T>,
    payment_intent_id: &str,
    payment_host: Option<&str>,
) -> Result<PaymentLink, PaymentError> {
    if payment_intent_id.is_empty() || matches!(payment_intent_id, "." | "..") {
        return Err(PaymentError::InvalidPaymentIntent);
    }

    let host = urls::payment_host(payment_host)?;
    let intent = client
        .call(GetPaymentIntent {
            payment_intent_id: payment_intent_id.into(),
        })
        .await?;

    if intent.id != payment_intent_id {
        return Err(PaymentError::IntentMismatch);
    }

    if intent.status != "pending" {
        return Err(PaymentError::NotPending);
    }

    payment_link(intent, "Subscription renewal".into(), host)
}

fn payment_link(
    intent: CheckoutInitializeResponse,
    plan_name: String,
    host: &str,
) -> Result<PaymentLink, PaymentError> {
    if intent.amount == 0 {
        return Err(PaymentError::ZeroAmount);
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
        plan_name,
    })
}

fn plan_name(price: PaymentPrice, fallback: Option<String>) -> String {
    let PaymentPrice::Plan { plan, period } = price else {
        return fallback.unwrap_or_else(|| "Helius".into());
    };

    let name = match plan {
        Plan::Agent => return "Agent Plan".into(),
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
