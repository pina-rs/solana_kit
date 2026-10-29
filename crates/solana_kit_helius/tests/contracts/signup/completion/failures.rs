//! Failure stages retain provider/timer causes and never consume payment receipts.

use super::*;
use checkout::PollError;

const UNAVAILABLE: u16 = 503;

/// Project timeout, policy, service and timer failures remain distinguishable.
pub fn completion_failures() {
    project_timeout();
    service_failures();
    project_failures();
    policy_failures();
    backwards_clock();
    timer_failure(vec![reply(PENDING)], false);
    timer_failure(vec![reply(COMPLETE), reply("[]")], true);
}

fn project_timeout() {
    let input = pending();
    let mut polling = policy();
    polling.provisioning.max_polls = 2;
    let (actual, calls) = execute(
        &input,
        vec![reply(COMPLETE), reply("[]"), reply("[]")],
        polling,
    );
    assert!(matches!(
        actual,
        Err(SignupCompletionError::ProvisioningTimeout)
    ));
    assert_eq!(calls.len(), 3);

    let mut polling = policy();
    polling.provisioning.timeout_ms = 0;
    let (actual, calls) = execute(&input, vec![reply(COMPLETE)], polling);
    assert!(matches!(
        actual,
        Err(SignupCompletionError::ProvisioningTimeout)
    ));
    assert_eq!(calls.len(), 1);
}

fn failed() -> Reply {
    Reply {
        status: UNAVAILABLE,
        body: b"private-provider-message".to_vec(),
    }
}

fn service_failures() {
    let input = pending();
    let (actual, calls) = execute(&input, vec![failed()], policy());
    assert!(matches!(
        &actual,
        Err(SignupCompletionError::Checkout(PollError::Service(
            Error::Http {
                status: UNAVAILABLE,
                ..
            }
        )))
    ));
    assert_eq!(calls.len(), 1);
    assert!(!format!("{actual:?}").contains("private-provider-message"));

    let (actual, calls) = execute(&input, vec![reply(COMPLETE), failed()], policy());
    assert!(matches!(
        actual,
        Err(SignupCompletionError::Provisioning(PollError::Service(
            Error::Http {
                status: UNAVAILABLE,
                ..
            }
        )))
    ));
    assert_eq!(calls.len(), 2);
}

fn project_failures() {
    let input = pending();

    for details in [failed(), reply(r#"{"apiKeys":false}"#)] {
        let (actual, calls) = execute(
            &input,
            vec![reply(COMPLETE), reply(PROJECTS), details],
            policy(),
        );
        assert!(matches!(
            actual,
            Err(SignupCompletionError::Project(SignupError::Service(_)))
        ));
        assert_eq!(calls.len(), 3, "details failure must not create an API key");
    }

    let (actual, calls) = execute(
        &input,
        vec![
            reply(COMPLETE),
            reply(PROJECTS),
            reply(EMPTY_KEYS),
            failed(),
        ],
        policy(),
    );
    assert!(matches!(
        actual,
        Err(SignupCompletionError::Project(SignupError::Service(
            Error::Http {
                status: UNAVAILABLE,
                ..
            }
        )))
    ));
    assert_eq!(calls.len(), 4, "API key creation must not retry");
    assert_eq!(calls[3].method, Verb::Post);
}

fn policy_failures() {
    let input = pending();

    for invalid in [
        checkout::PollPolicy {
            interval_ms: 0,
            ..policy().checkout
        },
        checkout::PollPolicy {
            max_polls: 0,
            ..policy().checkout
        },
    ] {
        let mut polling = policy();
        polling.checkout = invalid;
        let (actual, calls) = execute(&input, Vec::new(), polling);
        assert!(matches!(
            actual,
            Err(SignupCompletionError::Checkout(PollError::Policy))
        ));
        assert!(calls.is_empty());

        polling = policy();
        polling.provisioning = invalid;
        let (actual, calls) = execute(&input, vec![reply(COMPLETE)], polling);
        assert!(matches!(
            actual,
            Err(SignupCompletionError::Provisioning(PollError::Policy))
        ));
        assert_eq!(calls.len(), 1);
    }

    let mut polling = policy();
    polling.checkout.timeout_ms = u64::MAX;
    let (client, trace) = client(Vec::new());
    let actual = futures::executor::block_on(resume_signup_and_pay(
        &client,
        &input,
        polling,
        || 1,
        |_| ready(Ok::<_, ()>(())),
    ));
    assert!(matches!(
        actual,
        Err(SignupCompletionError::Checkout(PollError::Policy))
    ));
    assert!(trace.borrow().is_empty());
}

fn backwards_clock() {
    let input = pending();
    let (client, trace) = client(vec![reply(COMPLETE)]);
    let mut ticks = [0, 0, 1, 0].into_iter();
    let actual = futures::executor::block_on(resume_signup_and_pay(
        &client,
        &input,
        policy(),
        || ticks.next().expect("clock script"),
        |_| ready(Ok::<_, ()>(())),
    ));
    assert!(matches!(
        actual,
        Err(SignupCompletionError::Provisioning(PollError::Policy))
    ));
    assert_eq!(trace.borrow().len(), 1);
}

fn timer_failure(replies: Vec<Reply>, provisioning: bool) {
    let (client, trace) = client(replies);
    let input = pending();
    let actual = futures::executor::block_on(resume_signup_and_pay(
        &client,
        &input,
        policy(),
        || 0,
        |_| ready(Err::<(), _>("clock failure")),
    ));

    if provisioning {
        assert!(matches!(
            actual,
            Err(SignupCompletionError::Provisioning(PollError::Clock(
                "clock failure"
            )))
        ));
    } else {
        assert!(matches!(
            actual,
            Err(SignupCompletionError::Checkout(PollError::Clock(
                "clock failure"
            )))
        ));
    }

    assert_eq!(trace.borrow().len(), if provisioning { 2 } else { 1 });
}
