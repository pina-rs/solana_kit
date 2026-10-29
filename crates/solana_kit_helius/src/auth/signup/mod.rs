//! Unified signup leaves JWT binding, time, payment approval and submission with the caller.

mod authenticate;
mod completion;
mod completion_error;
mod error;
mod payment;

pub use completion::{SignupCompletionPolicy, resume_signup_and_pay};
pub use completion_error::SignupCompletionError;
pub use error::{ContactField, SignupError};

use super::{
    AuthenticatedIdentity, BillingPeriod, DeveloperCreateApiKey, DeveloperGetProject,
    DeveloperListProjects, DeveloperProjectSummary, Plan, ProvisionedProject, SignupRequest,
    SignupResult, build_endpoints,
};
use crate::{
    SensitiveString,
    service::{self, Service, ServiceClient, ServiceTransport},
};
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

const MONTHLY_HOURS: core::ops::RangeInclusive<i64> = 600..=840;
const YEARLY_HOURS: core::ops::RangeInclusive<i64> = 8_400..=9_120;

/// Authenticates, examines the first project and prepares the matching signup outcome.
///
/// `timestamp` is caller-owned Unix milliseconds, used only for secret-key authentication.
/// The session factory runs once with the actual developer JWT. It must bind that token
/// only to the caller's approved Developer origin; a project API key never replaces it.
/// No payment is signed or submitted. Dropping this future drops its active exchange.
///
/// # Errors
/// Returns typed input or service failures. Preview failures propagate without retry.
pub async fn auth_signup<T, U>(
    bootstrap: &ServiceClient<T>,
    request: SignupRequest,
    timestamp: u64,
    session: impl FnOnce(&SensitiveString) -> Result<ServiceClient<U>, service::Error>,
) -> Result<SignupResult, SignupError>
where
    T: ServiceTransport<Service>,
    U: ServiceTransport<Service>,
{
    let identity =
        authenticate::authenticate(bootstrap, &request.authentication, timestamp).await?;
    let client = session(&identity.jwt)?;
    let projects = client.call(DeveloperListProjects {}).await?;

    let Some(project) = projects.into_iter().next() else {
        let payment_link = payment::prepare(&client, &identity, &request).await?;

        return Ok(SignupResult::PaymentRequired {
            identity,
            payment_link,
        });
    };

    if !matches_plan(&project, request.plan, request.period) {
        return Ok(SignupResult::UpgradeRequired {
            identity,
            current_plan: project.subscription.plan,
            requested_plan: request.plan.as_str().into(),
        });
    }

    let project = provisioned(&client, &identity, project.id).await?;

    Ok(SignupResult::AlreadySubscribed { identity, project })
}

/// Only an actual empty key list permits creation; service failures must not create keys.
async fn provisioned<T: ServiceTransport<Service>>(
    client: &ServiceClient<T>,
    identity: &AuthenticatedIdentity,
    project_id: String,
) -> Result<ProvisionedProject, SignupError> {
    let details = client
        .call(DeveloperGetProject {
            project_id: project_id.clone(),
        })
        .await?;

    let api_key = match details.api_keys.into_iter().next() {
        Some(key) => key,
        None => {
            client
                .call(DeveloperCreateApiKey {
                    project_id: project_id.clone(),
                    wallet_address: identity.wallet_address.to_string(),
                })
                .await?
        }
    };

    let endpoints = build_endpoints(&api_key.key_id);

    Ok(ProvisionedProject {
        project_id,
        api_key: api_key.key_id,
        endpoints,
    })
}

/// Explicit offsets keep billing independent of host timezone; whole hours match Dart.
fn matches_plan(project: &DeveloperProjectSummary, plan: Plan, period: BillingPeriod) -> bool {
    let subscription = &project.subscription;

    if subscription.plan != plan.usage_plan() {
        return false;
    }

    if plan == Plan::Agent {
        return true;
    }

    let Ok(start) = OffsetDateTime::parse(&subscription.billing_period_start, &Rfc3339) else {
        return false;
    };
    let Ok(end) = OffsetDateTime::parse(&subscription.billing_period_end, &Rfc3339) else {
        return false;
    };

    if end <= start {
        return false;
    }

    let hours = (end - start).whole_hours();

    match period {
        BillingPeriod::Monthly => MONTHLY_HOURS.contains(&hours),
        BillingPeriod::Yearly => YEARLY_HOURS.contains(&hours),
    }
}
