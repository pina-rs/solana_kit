//! Existing subscriptions retain first-project and whole-hour billing semantics.

use super::*;

const START: &str = "2024-01-01T00:00:00Z";
const WINDOWS: &[(BillingPeriod, &str, bool)] = &[
    (BillingPeriod::Monthly, "2024-01-25T23:59:59Z", false),
    (BillingPeriod::Monthly, "2024-01-26T00:00:00Z", true),
    (BillingPeriod::Monthly, "2024-02-05T00:59:59Z", true),
    (BillingPeriod::Monthly, "2024-02-05T01:00:00Z", false),
    (BillingPeriod::Yearly, "2024-12-15T23:59:59Z", false),
    (BillingPeriod::Yearly, "2024-12-16T00:00:00Z", true),
    (BillingPeriod::Yearly, "2025-01-15T00:59:59Z", true),
    (BillingPeriod::Yearly, "2025-01-15T01:00:00Z", false),
    (BillingPeriod::Monthly, "2024-01-26T01:00:00+01:00", true),
    (BillingPeriod::Monthly, START, false),
    (BillingPeriod::Monthly, "2023-12-01T00:00:00Z", false),
    (BillingPeriod::Monthly, "malformed", false),
];

/// Reuses the first key or creates exactly one key through the developer route.
pub fn existing_projects() {
    existing_key(r#"[{"keyId":"existing-key"}]"#, "existing-key");
    existing_key("[]", "created-key");
    first_project();
}

fn existing_key(keys: &str, expected: &str) {
    let summary = project("developer_v4", START, "2024-02-01T00:00:00Z");
    let details = format!(r#"{{"apiKeys":{keys}}}"#);
    let replies = vec![
        reply(&summary),
        reply(&details),
        reply(r#"{"keyId":"created-key"}"#),
    ];
    let (result, calls) = execute(request(Plan::Developer), replies);

    let SignupResult::AlreadySubscribed { identity, project } = result.expect("matching plan")
    else {
        panic!("existing matching subscription");
    };

    assert_eq!(identity.jwt.expose(), "fixture-jwt");
    assert_eq!(project.api_key.expose(), expected);
    assert_eq!(project.project_id, "project/opaque");
    assert_eq!(calls[1].path_segments, ["projects", "project/opaque"]);
    assert_eq!(
        project.endpoints.mainnet.expose(),
        format!("https://mainnet.helius-rpc.com/?api-key={expected}")
    );

    if keys == "[]" {
        assert_eq!(calls.len(), 3);
        assert_eq!(
            calls[2].path_segments,
            ["projects", "project/opaque", "add-key"]
        );
        assert_eq!(
            body(&calls[2]),
            serde_json::json!({"userId":identity.wallet_address.to_string()})
        );
    } else {
        assert_eq!(calls.len(), 2);
    }
}

fn first_project() {
    let first = project("business_v4", START, "2024-02-01T00:00:00Z");
    let second = project("developer_v4", START, "2024-02-01T00:00:00Z");
    let summaries = format!(
        "{},{}",
        first.trim_end_matches(']'),
        second.trim_start_matches('[')
    );
    let (result, calls) = execute(request(Plan::Developer), vec![reply(&summaries)]);
    assert!(matches!(result,
        Ok(SignupResult::UpgradeRequired { current_plan, requested_plan, .. })
        if current_plan=="business_v4" && requested_plan=="developer"));
    assert_eq!(calls.len(), 1);
}

/// Boundary fixtures include truncated fractional hours, offsets and malformed windows.
pub fn billing_windows() {
    for &(period, end, expected) in WINDOWS {
        billing_case(period, end, expected);
    }

    let summary = project("agent_v4", "not a date", "not a date");
    let (result, _) = execute(
        request(Plan::Agent),
        vec![reply(&summary), reply(r#"{"apiKeys":[{"keyId":"k"}]}"#)],
    );
    assert!(matches!(result, Ok(SignupResult::AlreadySubscribed { .. })));
}

fn billing_case(period: BillingPeriod, end: &str, expected: bool) {
    let mut options = request(Plan::Developer);
    options.period = period;
    let summary = project("developer_v4", START, end);
    let (result, calls) = execute(
        options,
        vec![reply(&summary), reply(r#"{"apiKeys":[{"keyId":"k"}]}"#)],
    );
    assert_eq!(
        matches!(result, Ok(SignupResult::AlreadySubscribed { .. })),
        expected,
        "{end}"
    );
    assert_eq!(calls.len(), if expected { 2 } else { 1 });
}
