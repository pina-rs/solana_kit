//! Signup failures retain typed service causes and never format credentials.

use crate::service;

/// Contact field required before a new subscription can initialize checkout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContactField {
    /// Billing email.
    Email,
    /// Billing first name.
    FirstName,
    /// Billing last name.
    LastName,
}

/// An observable authentication, provisioning or payment preparation failure.
#[derive(Debug)]
pub enum SignupError {
    /// The supplied JWT or provider reference identity is empty.
    InvalidIdentity,
    /// The encoded key is invalid or its public half does not match its seed.
    InvalidSecretKey,
    /// Secret-key authentication requires the optional crypto feature.
    CryptoUnavailable,
    /// A new subscription is missing one required contact field.
    MissingContact(ContactField),
    /// The explicit payment host is not an accepted HTTP(S) base URL.
    InvalidPaymentHost,
    /// The provider preview or initialized checkout requires no payment.
    ZeroAmount,
    /// The initialized payment intent has an empty identity.
    InvalidPaymentIntent,
    /// The original bounded service, schema, capability or transport failure.
    Service(service::Error),
}

impl core::fmt::Display for SignupError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "Helius signup failed: {self:?}")
    }
}

impl std::error::Error for SignupError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Service(error) => Some(error),
            _ => None,
        }
    }
}

impl From<service::Error> for SignupError {
    fn from(error: service::Error) -> Self {
        Self::Service(error)
    }
}
