//! Resume submitted signup payments without owning approval or submission.

use super::{SignupCompletionError, provisioned};
use crate::{
    auth::{
        AuthenticatedIdentity, DeveloperListProjects, PaymentLink, ProvisionedProject,
        SignupAndPayResult,
        checkout::{PollClock, PollError, PollOutcome, PollPolicy, poll_until_terminal},
    },
    service::{Service, ServiceClient, ServiceTransport},
};
use core::{future::Future, ops::ControlFlow};
use solana_kit_core::Signature;

/// Independent finite budgets for checkout and the subsequently provisioned project.
#[derive(Debug, Clone, Copy)]
pub struct SignupCompletionPolicy {
    /// Observe the existing checkout intent without submitting payment again.
    pub checkout: PollPolicy,
    /// Look for the first project after checkout reports completion.
    pub provisioning: PollPolicy,
}

/// Resumes a submitted payment receipt through checkout and API-key provisioning.
///
/// Pass the caller-owned `Pending` outcome after independently approving and
/// submitting its payment. All other outcomes return unchanged without requests.
/// The Developer client must bind the same JWT to its approved service origin.
/// No authentication, checkout initialization, signing, or submission runs here.
///
/// The input is borrowed so its identity, link and signature remain available on
/// every error and when this future is dropped. The caller owns monotonic time,
/// sleep futures and transport deadlines. Dropping releases the active exchange
/// or timer; concrete inputs preserve local or Send behavior without a task.
///
/// # Errors
/// Retains checkout, provisioning, clock and service causes. No project within
/// the explicit provisioning budget is an error; checkout timeout stays Pending.
pub async fn resume_signup_and_pay<T, N, S, F, E>(
    client: &ServiceClient<T>,
    result: &SignupAndPayResult,
    policy: SignupCompletionPolicy,
    mut now: N,
    mut sleep_until: S,
) -> Result<SignupAndPayResult, SignupCompletionError<E>>
where
    T: ServiceTransport<Service>,
    N: FnMut() -> u64,
    S: FnMut(u64) -> F,
    F: Future<Output = Result<(), E>>,
{
    let SignupAndPayResult::Pending {
        identity,
        payment_link,
        tx_signature,
    } = result
    else {
        return Ok(result.clone());
    };

    let payment_intent_id = payment_link.payment_intent_id.clone();
    let outcome = poll_until_terminal(
        client,
        payment_intent_id.clone(),
        policy.checkout,
        &mut now,
        &mut sleep_until,
    )
    .await
    .map_err(SignupCompletionError::Checkout)?;

    if let ControlFlow::Break(result) =
        checkout_result(identity, payment_link, *tx_signature, outcome)
    {
        return Ok(result);
    }

    let project = provision_after_payment(
        client,
        identity,
        policy.provisioning,
        &mut now,
        &mut sleep_until,
    )
    .await?;

    Ok(SignupAndPayResult::Completed {
        identity: identity.clone(),
        project,
        tx_signature: *tx_signature,
        payment_intent_id,
    })
}

/// Only confirmed checkout continues to project I/O; every other outcome is data.
fn checkout_result(
    identity: &AuthenticatedIdentity,
    payment_link: &PaymentLink,
    tx_signature: Signature,
    outcome: PollOutcome,
) -> ControlFlow<SignupAndPayResult> {
    let result = match outcome {
        PollOutcome::Completed(_) => return ControlFlow::Continue(()),
        PollOutcome::Expired(_) => SignupAndPayResult::Expired {
            identity: identity.clone(),
            payment_intent_id: payment_link.payment_intent_id.clone(),
        },
        PollOutcome::Failed(status) => SignupAndPayResult::Failed {
            identity: identity.clone(),
            payment_intent_id: payment_link.payment_intent_id.clone(),
            reason: status.message,
        },
        PollOutcome::Timeout(_) => SignupAndPayResult::Pending {
            identity: identity.clone(),
            payment_link: payment_link.clone(),
            tx_signature,
        },
    };

    ControlFlow::Break(result)
}

/// A fresh window begins only after confirmed checkout. Empty lists permit another
/// bounded read; discovering the first project immediately reuses key provisioning.
async fn provision_after_payment<T, N, S, F, E>(
    client: &ServiceClient<T>,
    identity: &AuthenticatedIdentity,
    policy: PollPolicy,
    now: &mut N,
    sleep_until: &mut S,
) -> Result<ProvisionedProject, SignupCompletionError<E>>
where
    T: ServiceTransport<Service>,
    N: FnMut() -> u64,
    S: FnMut(u64) -> F,
    F: Future<Output = Result<(), E>>,
{
    let mut clock = PollClock::new(policy, now()).map_err(SignupCompletionError::Provisioning)?;

    for poll in 1..=policy.max_polls {
        if !clock
            .advance(now())
            .map_err(SignupCompletionError::Provisioning)?
        {
            return Err(SignupCompletionError::ProvisioningTimeout);
        }

        let projects = client
            .call(DeveloperListProjects {})
            .await
            .map_err(PollError::Service)
            .map_err(SignupCompletionError::Provisioning)?;

        if let Some(project) = projects.into_iter().next() {
            return provisioned(client, identity, project.id)
                .await
                .map_err(SignupCompletionError::Project);
        }

        if poll == policy.max_polls
            || !clock
                .advance(now())
                .map_err(SignupCompletionError::Provisioning)?
        {
            return Err(SignupCompletionError::ProvisioningTimeout);
        }

        sleep_until(clock.wake(policy.interval_ms))
            .await
            .map_err(PollError::Clock)
            .map_err(SignupCompletionError::Provisioning)?;
    }

    Err(SignupCompletionError::ProvisioningTimeout)
}
