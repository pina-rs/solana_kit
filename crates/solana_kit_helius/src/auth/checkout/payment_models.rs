//! Checkout preparation keeps price selection explicit and never owns a signer.

use super::super::{BillingPeriod, Plan};
use crate::{SensitiveString, service};
use solana_kit_core::Address;
use std::num::NonZeroU64;

/// A provider plan to resolve, or one exact caller-selected price identifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaymentPrice {
    /// Resolve one provider price for the selected plan and billing period.
    Plan {
        /// Requested subscription plan.
        plan: Plan,
        /// Requested billing period.
        period: BillingPeriod,
    },
    /// Use this exact identifier for both preview and initialization.
    Id(String),
}

/// Prepare a checkout using an already authenticated Developer service client.
///
/// The adapter owns JWT authentication. For upgrades, `ref_id` is the existing
/// project ID. Contact details remain optional outside new-account signup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreatePaymentRequest {
    /// Provider reference identity or existing project ID.
    pub ref_id: String,
    /// Mutually exclusive price selection.
    pub price: PaymentPrice,
    /// Optional positive quantity, forwarded to preview and initialization.
    pub qty: Option<NonZeroU64>,
    /// Optional billing email, redacted when formatted.
    pub email: Option<SensitiveString>,
    /// Optional billing first name, redacted when formatted.
    pub first_name: Option<SensitiveString>,
    /// Optional billing last name, redacted when formatted.
    pub last_name: Option<SensitiveString>,
    /// Optional wallet associated with the checkout.
    pub wallet_address: Option<Address>,
    /// Optional provider coupon code.
    pub coupon_code: Option<String>,
    /// Optional explicit HTTP(S) payment-page base URL.
    pub payment_host: Option<String>,
    /// Display label for an explicit price. Named plans retain their standard label.
    pub plan_name_override: Option<String>,
}

impl CreatePaymentRequest {
    /// Creates a checkout with no optional contact, coupon or quantity fields.
    pub fn new(ref_id: impl Into<String>, price: PaymentPrice) -> Self {
        Self {
            ref_id: ref_id.into(),
            price,
            qty: None,
            email: None,
            first_name: None,
            last_name: None,
            wallet_address: None,
            coupon_code: None,
            payment_host: None,
            plan_name_override: None,
        }
    }
}

/// Input, provider and lifecycle failures during unsigned payment preparation.
#[derive(Debug)]
pub enum PaymentError {
    /// The provider reference or project ID is empty.
    InvalidReference,
    /// An explicit price identifier is empty.
    InvalidPrice,
    /// The payment base URL is not accepted.
    InvalidPaymentHost,
    /// The preview or initialized intent requires no payment.
    ZeroAmount,
    /// A payment intent identifier is empty or is a URL dot segment.
    InvalidPaymentIntent,
    /// The fetched intent does not match the requested identity.
    IntentMismatch,
    /// An existing intent is not pending and cannot be renewed.
    NotPending,
    /// Original service failure, with bounded response bytes and redacted formatting.
    Service(service::Error),
}

impl core::fmt::Display for PaymentError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "Helius payment preparation failed: {self:?}")
    }
}

impl std::error::Error for PaymentError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Service(error) => Some(error),
            _ => None,
        }
    }
}

impl From<service::Error> for PaymentError {
    fn from(error: service::Error) -> Self {
        Self::Service(error)
    }
}
