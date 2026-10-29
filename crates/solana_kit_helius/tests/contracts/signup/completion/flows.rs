//! Existing outcomes bypass observation; confirmed payments provision exactly once.

use super::*;

/// All non-pending outcomes are idempotent and do not evaluate clocks or make requests.
pub fn terminal_exits() {
    let outcomes = [
        SignupAndPayResult::AlreadySubscribed {
            identity: identity(),
            project: provisioned_project(),
        },
        SignupAndPayResult::UpgradeRequired {
            identity: identity(),
            current_plan: "developer_v4".into(),
            requested_plan: "business".into(),
        },
        SignupAndPayResult::Completed {
            identity: identity(),
            project: provisioned_project(),
            tx_signature: SIGNATURE,
            payment_intent_id: "intent/opaque".into(),
        },
        SignupAndPayResult::Expired {
            identity: identity(),
            payment_intent_id: "intent/opaque".into(),
        },
        SignupAndPayResult::Failed {
            identity: identity(),
            payment_intent_id: "intent/opaque".into(),
            reason: "previous failure".into(),
        },
    ];

    for input in outcomes {
        let (client, trace) = client(Vec::new());
        let actual = futures::executor::block_on(resume_signup_and_pay(
            &client,
            &input,
            policy(),
            || panic!("terminal outcome read the clock"),
            |_| -> std::future::Ready<Result<(), ()>> {
                panic!("terminal outcome started a timer")
            },
        ))
        .expect("terminal passthrough");
        assert_eq!(actual, input);
        assert!(trace.borrow().is_empty());
    }
}

/// Delayed first-project discovery and empty-key creation preserve exact Developer requests.
pub fn completed_provisioning() {
    provision_with_keys(EXISTING_KEY, "existing-key", false);
    provision_with_keys(EMPTY_KEYS, "new-key", true);
}

fn provision_with_keys(keys: &str, expected: &str, creates: bool) {
    let replies = vec![
        reply(COMPLETE),
        reply("[]"),
        reply("[]"),
        reply(PROJECTS),
        reply(keys),
        reply(NEW_KEY),
    ];
    let input = pending();
    let (actual, calls) = execute(&input, replies, policy());
    let SignupAndPayResult::Completed {
        identity,
        project,
        tx_signature,
        payment_intent_id,
    } = actual.expect("confirmed payment provisioned")
    else {
        panic!("completed payment did not provision a project");
    };
    assert_eq!(identity, super::identity());
    assert_eq!(project.project_id, "project/opaque");
    assert_eq!(project.api_key.expose(), expected);
    assert_eq!(
        project.endpoints.mainnet.expose(),
        format!("https://mainnet.helius-rpc.com/?api-key={expected}")
    );
    assert_eq!(
        project.endpoints.devnet.expose(),
        format!("https://devnet.helius-rpc.com/?api-key={expected}")
    );

    assert_eq!(tx_signature, SIGNATURE);
    assert_eq!(payment_intent_id, "intent/opaque");
    assert_completed_requests(&calls, creates);
}

fn assert_completed_requests(calls: &[ServiceRequest], creates: bool) {
    const KEY_CREATION: usize = 5;
    const WITHOUT_CREATION: usize = KEY_CREATION;
    assert_eq!(calls.len(), WITHOUT_CREATION + usize::from(creates));
    assert_read(&calls[0], &["checkout", "intent/opaque", "status"]);

    for call in &calls[1..4] {
        assert_read(call, &["projects"]);
    }

    assert_read(&calls[4], &["projects", "project/opaque"]);

    if creates {
        assert_eq!(calls[KEY_CREATION].method, Verb::Post);
        assert_eq!(
            calls[KEY_CREATION].path_segments,
            ["projects", "project/opaque", "add-key"]
        );
        assert_eq!(
            body(&calls[KEY_CREATION]),
            serde_json::json!({"userId": identity().wallet_address.to_string()})
        );
        assert!(calls[KEY_CREATION].query.is_empty());
    }
}

/// Checkout timeout retains the exact submitted receipt; expired and failed intents stop early.
pub fn completion_outcomes() {
    let mut budget = policy();
    budget.checkout.max_polls = 1;
    let input = pending();
    let (actual, calls) = execute(&input, vec![reply(PENDING)], budget);
    assert_eq!(actual.expect("nonterminal checkout is pending"), input);
    assert_eq!(calls.len(), 1);
    assert_read(&calls[0], &["checkout", "intent/opaque", "status"]);

    budget = policy();
    budget.checkout.timeout_ms = 0;
    let (actual, calls) = execute(&input, Vec::new(), budget);
    assert_eq!(
        actual.expect("expired observation window stays pending"),
        input
    );
    assert!(calls.is_empty());
    terminal_outcomes();

    let gone = Reply {
        status: 410,
        body: b"gone".to_vec(),
    };
    let (actual, calls) = execute(&input, vec![gone], policy());
    assert!(matches!(actual, Ok(SignupAndPayResult::Expired { .. })));
    assert_eq!(calls.len(), 1);
}

fn terminal_outcomes() {
    let cases = [
        (
            "expired",
            SignupAndPayResult::Expired {
                identity: identity(),
                payment_intent_id: "intent/opaque".into(),
            },
        ),
        (
            "failed",
            SignupAndPayResult::Failed {
                identity: identity(),
                payment_intent_id: "intent/opaque".into(),
                reason: "provider reason".into(),
            },
        ),
    ];

    for (phase, expected) in cases {
        let status = serde_json::json!({"status":"pending", "phase":phase,
            "subscriptionActive":false,"readyToRedirect":false,"message":"provider reason"})
        .to_string();
        let (actual, calls) = execute(&pending(), vec![reply(&status)], policy());
        assert_eq!(actual.expect("terminal checkout outcome"), expected);
        assert_eq!(calls.len(), 1);
    }
}
