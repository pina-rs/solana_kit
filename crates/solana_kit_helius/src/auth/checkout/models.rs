//! Checkout fields are exact integers; optional known fields reject malformed types.

use crate::SensitiveString;
use std::collections::BTreeMap;

model!(
    "An optional checkout coupon.",
    CheckoutCoupon {} optional {
        code: String => "code",
        valid: bool => "valid",
        percent_off: u64 => "percentOff",
        description: String => "description",
    }
);
model!(
    "Checkout preview amounts in smallest currency units.",
    CheckoutPreviewResponse {
        plan_name: String => "planName",
        period: String => "period",
        base_amount: u64 => "baseAmount",
        subtotal: u64 => "subtotal",
        applied_credits: u64 => "appliedCredits",
        prorated_credits: u64 => "proratedCredits",
        discounts: u64 => "discounts",
        due_today: u64 => "dueToday",
        destination_wallet: String => "destinationWallet",
        note: String => "note",
    } optional { coupon: CheckoutCoupon => "coupon" }
);
request!(
    "Initialize one checkout; bearer authentication belongs to the adapter.",
    CheckoutInitializeRequest {
        price_id: String => "priceId",
        ref_id: String => "refId",
    } optional {
        email: SensitiveString => "email",
        first_name: SensitiveString => "firstName",
        last_name: SensitiveString => "lastName",
        wallet_address: String => "walletAddress",
        coupon_code: String => "couponCode",
        qty: u64 => "qty",
    }
);
model!(
    "An initialized checkout payment intent.",
    CheckoutInitializeResponse {
        id: String => "id",
        status: String => "status",
        destination_wallet: String => "destinationWallet",
        amount: u64 => "amount",
        solana_pay_url: String => "solanaPayUrl",
        expires_at: String => "expiresAt",
    } optional { created_at: String => "createdAt",
    price_id: String => "priceId",
    ref_id: String => "refId" }
);
model!(
    "Provider-reported checkout status.",
    CheckoutStatusResponse {
        status: String => "status",
        phase: String => "phase",
        subscription_active: bool => "subscriptionActive",
        ready_to_redirect: bool => "readyToRedirect",
        message: String => "message",
    } optional {}
);
model!(
    "Developer portal billing configuration.",
    DevPortalConfigsResponse {
        stripe: StripeConfig => "stripe",
    } optional { open_pay: StripeConfig => "openPay" }
);
model!(
    "Stripe price and optional prepaid plan configuration.",
    StripeConfig {
        price_ids: PriceIds => "priceIds",
    } optional { prepaid_credits_plans: BTreeMap<String,
    String> => "prepaidCreditsPlans" }
);
model!(
    "Monthly, yearly and optional Agent plan identifiers.",
    PriceIds {
        monthly: BTreeMap<String,
        String> => "Monthly",
        yearly: BTreeMap<String,
        String> => "Yearly",
    } optional { agent_plan: String => "AgentPlan" }
);
