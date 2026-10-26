//! Wire models use redacted secret wrappers for provider credentials.

use super::{BillingPeriod, Plan};
use crate::SensitiveString;

request!(
    "Authenticate a wallet using an explicitly signed message.",
    WalletSignupRequest {
        wallet_address: String => "walletAddress",
        signature: String => "signature",
        message: String => "message",
    } optional {}
);
model!(
    "The new project's API key and identity.",
    WalletSignupResponse {
        api_key: SensitiveString => "apiKey",
        project_id: String => "projectId",
    } optional {}
);
request!(
    "Create a named project.",
    CreateProjectRequest { name: String => "name" } optional {}
);
model!(
    "An authenticated Helius project.",
    HeliusProject {
        id: String => "id",
        name: String => "name",
        api_key: SensitiveString => "apiKey",
        created_at: i64 => "createdAt",
    } optional {}
);
request!(
    "Create a named API key in one project.",
    CreateApiKeyRequest {
        project_id: String => "projectId",
        name: String => "name",
    } optional {}
);
model!(
    "A named API key.",
    HeliusApiKey {
        id: String => "id",
        key: SensitiveString => "key",
        name: String => "name",
        created_at: i64 => "createdAt",
    } optional {}
);
model!(
    "Credit balance and consumed credits.",
    CheckBalancesResponse {
        credits: u64 => "credits",
        credits_used: u64 => "creditsUsed",
    } optional {}
);
model!(
    "A hosted payment link and exact cent amount.",
    PaymentLink {
        kind: String => "kind",
        payment_intent_id: String => "paymentIntentId",
        amount_cents: u64 => "amountCents",
        destination_wallet: String => "destinationWallet",
        memo: String => "memo",
        expires_at: String => "expiresAt",
        payment_url: String => "paymentUrl",
        solana_pay_url: String => "solanaPayUrl",
        plan_name: String => "planName",
    } optional {}
);

model!(
    "Provisioned mainnet and devnet endpoints, which may contain credentials.",
    SignupEndpoints {
        mainnet: SensitiveString => "mainnet",
        devnet: SensitiveString => "devnet",
    } optional {}
);

/// Explicit signup authentication. No missing credential branch can be constructed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignupAuthentication {
    /// Caller-provided encoded signing key. Signing stays local.
    SecretKey(SensitiveString),
    /// Caller-provided already authenticated identity.
    Authenticated {
        /// Authentication bearer token.
        jwt: SensitiveString,
        /// Provider reference identity.
        ref_id: String,
        /// Wallet address bound to the authenticated identity.
        wallet_address: solana_kit_core::Address,
    },
}

/// Signup plan details and one explicit authentication mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignupRequest {
    /// Authentication mode, mutually exclusive by construction.
    pub authentication: SignupAuthentication,
    /// Requested provider plan.
    pub plan: Plan,
    /// Billing period. The reference default is monthly.
    pub period: BillingPeriod,
    /// Optional caller-supplied email.
    pub email: Option<SensitiveString>,
    /// Optional caller-supplied first name.
    pub first_name: Option<SensitiveString>,
    /// Optional caller-supplied last name.
    pub last_name: Option<SensitiveString>,
    /// Optional coupon code.
    pub coupon_code: Option<String>,
    /// Optional approved payment-page origin.
    pub payment_host: Option<String>,
}

impl SignupRequest {
    /// Creates signup input with the pinned monthly billing default.
    pub fn new(authentication: SignupAuthentication, plan: Plan) -> Self {
        Self {
            authentication,
            plan,
            period: BillingPeriod::Monthly,
            email: None,
            first_name: None,
            last_name: None,
            coupon_code: None,
            payment_host: None,
        }
    }
}
