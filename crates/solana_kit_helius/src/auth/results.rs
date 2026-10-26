//! Signup outcome variants share identities without invalid optional-field combinations.

use super::{PaymentLink, SignupEndpoints};
use crate::SensitiveString;
use solana_kit_core::{Address, Signature};

/// Authenticated wallet identity retained across provisioning outcomes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatedIdentity {
    /// Provider bearer token.
    pub jwt: SensitiveString,
    /// Provider reference identity.
    pub ref_id: String,
    /// Kit-owned wallet address.
    pub wallet_address: Address,
}

/// A provisioned project and its explicit credential-bearing endpoints.
#[derive(Debug, Clone, PartialEq)]
pub struct ProvisionedProject {
    /// Provider project identity.
    pub project_id: String,
    /// Project API key.
    pub api_key: SensitiveString,
    /// Mainnet and devnet endpoints, redacted when formatted.
    pub endpoints: SignupEndpoints,
}

/// The three mutually exclusive outcomes of unified signup.
#[derive(Debug, Clone, PartialEq)]
pub enum SignupResult {
    /// A matching existing project is ready.
    AlreadySubscribed {
        /// Authenticated identity.
        identity: AuthenticatedIdentity,
        /// Existing provisioned project.
        project: ProvisionedProject,
    },
    /// An existing plan requires an explicit upgrade decision.
    UpgradeRequired {
        /// Authenticated identity.
        identity: AuthenticatedIdentity,
        /// Current provider usage-plan name.
        current_plan: String,
        /// Requested signup plan name.
        requested_plan: String,
    },
    /// Checkout must complete before project provisioning.
    PaymentRequired {
        /// Authenticated identity.
        identity: AuthenticatedIdentity,
        /// Explicit payment details for caller review.
        payment_link: PaymentLink,
    },
}

/// Upgrade-plan preparation returns a payment link.
pub type UpgradePlanResult = PaymentLink;
/// Renewal preparation returns a payment link.
pub type PayRenewalResult = PaymentLink;

/// The six distinct outcomes of signup and payment orchestration.
#[derive(Debug, Clone, PartialEq)]
pub enum SignupAndPayResult {
    /// Payment and project provisioning completed.
    Completed {
        /// Authenticated identity.
        identity: AuthenticatedIdentity,
        /// Provisioned project.
        project: ProvisionedProject,
        /// Submitted payment transaction signature.
        tx_signature: Signature,
        /// Provider payment intent identity.
        payment_intent_id: String,
    },
    /// Checkout expired.
    Expired {
        /// Authenticated identity.
        identity: AuthenticatedIdentity,
        /// Expired payment intent identity.
        payment_intent_id: String,
    },
    /// Checkout failed with an explicit provider reason.
    Failed {
        /// Authenticated identity.
        identity: AuthenticatedIdentity,
        /// Failed payment intent identity.
        payment_intent_id: String,
        /// Provider failure reason.
        reason: String,
    },
    /// Submitted payment has no terminal outcome yet.
    Pending {
        /// Authenticated identity.
        identity: AuthenticatedIdentity,
        /// Payment link retained for reconciliation.
        payment_link: PaymentLink,
        /// Already submitted transaction signature. Never resubmit implicitly.
        tx_signature: Signature,
    },
    /// An existing matching project needs no payment.
    AlreadySubscribed {
        /// Authenticated identity.
        identity: AuthenticatedIdentity,
        /// Existing project.
        project: ProvisionedProject,
    },
    /// An existing project needs an explicit upgrade decision.
    UpgradeRequired {
        /// Authenticated identity.
        identity: AuthenticatedIdentity,
        /// Existing provider usage-plan name.
        current_plan: String,
        /// Requested signup plan name.
        requested_plan: String,
    },
}
