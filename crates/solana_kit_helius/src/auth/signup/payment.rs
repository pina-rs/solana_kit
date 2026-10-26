//! New subscriptions validate contact data before using the shared checkout flow.

use super::{ContactField, SignupError};
use crate::{
    SensitiveString,
    auth::{
        AuthenticatedIdentity, CreatePaymentRequest, PaymentLink, PaymentPrice, SignupRequest,
        create_payment,
    },
    service::{Service, ServiceClient, ServiceTransport},
};

/// Signup alone requires contact data; checkout mechanics have one maintained implementation.
pub(super) async fn prepare<T: ServiceTransport<Service>>(
    client: &ServiceClient<T>,
    identity: &AuthenticatedIdentity,
    request: &SignupRequest,
) -> Result<PaymentLink, SignupError> {
    contact(request)?;
    let mut payment = CreatePaymentRequest::new(
        &identity.ref_id,
        PaymentPrice::Plan {
            plan: request.plan,
            period: request.period,
        },
    );
    payment.email = request.email.clone();
    payment.first_name = request.first_name.clone();
    payment.last_name = request.last_name.clone();
    payment.wallet_address = Some(identity.wallet_address);
    payment.coupon_code = request.coupon_code.clone();
    payment.payment_host = request.payment_host.clone();

    create_payment(client, payment)
        .await
        .map_err(SignupError::from)
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
