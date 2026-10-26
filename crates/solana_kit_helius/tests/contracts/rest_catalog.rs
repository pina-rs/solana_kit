//! Literal contracts for every authored REST operation family.

use super::service::exchange;
use serde::de::DeserializeOwned;
use solana_kit_helius::{SensitiveString, admin::*, auth::*, enhanced::*, service::*};

fn route<O: Operation + DeserializeOwned>(
    input: &str,
    response: &[u8],
    service: Service,
    verb: Verb,
    path: &[&str],
) {
    let operation: O = serde_json::from_str(input).expect("literal operation");
    let request = exchange(operation, response);
    assert_eq!(request.service, service);
    assert_eq!(request.method, verb);
    assert_eq!(request.path_segments, path);
    assert!(request.query.is_empty());

    if matches!(verb, Verb::Post | Verb::Put | Verb::Patch) {
        assert_eq!(request.body.content_type(), Some("application/json"));
        let body: serde_json::Value =
            serde_json::from_slice(request.body.as_deref().expect("JSON body")).expect("body");
        assert_eq!(
            body,
            serde_json::from_str::<serde_json::Value>(input).expect("literal request")
        );
    } else {
        assert!(request.body.as_deref().is_none());
        assert_eq!(request.body.content_type(), None);
    }
}

/// Basic auth services preserve distinct paths and sensitive credentials.
pub fn auth_routes() {
    let project = br#"{"id":"p","name":"project","apiKey":"secret","createdAt":-1}"#;
    route::<CreateProjectRequest>(
        r#"{"name":"project"}"#,
        project,
        Service::Auth,
        Verb::Post,
        &["v0", "auth", "projects"],
    );
    route::<GetProject>(
        r#"{"projectId":"p"}"#,
        project,
        Service::Auth,
        Verb::Get,
        &["v0", "auth", "projects", "p"],
    );
    route::<ListProjects>(
        "{}",
        b"[]",
        Service::Auth,
        Verb::Get,
        &["v0", "auth", "projects"],
    );
    route::<CreateApiKeyRequest>(
        r#"{"projectId":"p","name":"key"}"#,
        br#"{"id":"k","key":"secret","name":"key","createdAt":0}"#,
        Service::Auth,
        Verb::Post,
        &["v0", "auth", "api-keys"],
    );
    route::<WalletSignupRequest>(
        r#"{"walletAddress":"w","signature":"s","message":"m"}"#,
        br#"{"apiKey":"secret","projectId":"p"}"#,
        Service::Auth,
        Verb::Post,
        &["v0", "auth", "wallet-signup"],
    );
    route::<CheckBalances>(
        "{}",
        br#"{"credits":18446744073709551615,"creditsUsed":0}"#,
        Service::Auth,
        Verb::Get,
        &["v0", "auth", "balances"],
    );
    let project: HeliusProject = serde_json::from_slice(project).expect("project");

    assert!(!format!("{project:?}").contains("secret"));
}

/// Wallet v1 and Enhanced Transactions v0 retain distinct service slots and schemas.
pub fn wallet_and_enhanced_routes() {
    super::wallet::vectors();
    super::enhanced::history_query();
    route::<GetTransactionsRequest>(
        r#"{"transactions":["s"]}"#,
        b"[]",
        Service::Api,
        Verb::Post,
        &["v0", "transactions"],
    );
    route::<GetTransactionsByAddressRequest>(
        r#"{"address":"a"}"#,
        b"[]",
        Service::Api,
        Verb::Get,
        &["v0", "addresses", "a", "transactions"],
    );
}

/// OAuth encoding is form data, including Unicode, spaces and reserved characters.
pub fn oauth_form_and_developer_routes() {
    let operation = OAuthTokenExchangeRequest {
        code: SensitiveString::new("a+b &"),
        code_verifier: SensitiveString::new("é"),
        client_id: "client".into(),
        redirect_uri: "https://example.invalid/cb?x=1&y=2".into(),
    };
    let result = br#"{"access_token":"secret","token_type":"Bearer","expires_in":3600,"user":{"id":"u","email":"hidden"}}"#;
    let request = exchange(operation, result);
    assert_eq!(request.service, Service::Developer);
    assert_eq!(request.path_segments, ["oauth", "token"]);
    assert_eq!(
        request.body.content_type(),
        Some("application/x-www-form-urlencoded")
    );
    assert_eq!(request.body.as_deref(), Some(b"grant_type=authorization_code&code=a%2Bb+%26&code_verifier=%C3%A9&client_id=client&redirect_uri=https%3A%2F%2Fexample.invalid%2Fcb%3Fx%3D1%26y%3D2".as_slice()));
    route::<DeveloperWalletSignup>(
        r#"{"message":"m","signature":"s","userID":"w"}"#,
        br#"{"token":"secret","refId":"r","newUser":false}"#,
        Service::Developer,
        Verb::Post,
        &["wallet-signup"],
    );

    route::<DeveloperListProjects>("{}", b"[]", Service::Developer, Verb::Get, &["projects"]);
    route::<DeveloperGetProject>(
        r#"{"projectId":"p"}"#,
        br#"{"apiKeys":[{"keyId":"secret"}]}"#,
        Service::Developer,
        Verb::Get,
        &["projects", "p"],
    );
    let request = exchange(
        DeveloperCreateApiKey {
            project_id: "p".into(),
            wallet_address: "w".into(),
        },
        br#"{"keyId":"secret"}"#,
    );
    assert_eq!(request.path_segments, ["projects", "p", "add-key"]);
    assert_eq!(
        request.body.as_deref(),
        Some(br#"{"userId":"w"}"#.as_slice())
    );
    assert!(
        serde_json::from_str::<DeveloperSignupResponse>(
            r#"{"token":"","refId":"r","newUser":true}"#
        )
        .is_err()
    );
    assert!(
        serde_json::from_str::<DeveloperProjectDetails>(r#"{"apiKeys":[{"keyId":null}]}"#).is_err()
    );
}

/// Admin usage retains the service root distinction and full required count schema.
pub fn admin_usage_route() {
    route::<GetProjectUsage>(r#"{"projectId":"p"}"#, br#"{"creditsRemaining":1,"creditsUsed":2,"prepaidCreditsRemaining":3,"prepaidCreditsUsed":4,"subscriptionDetails":{"billingCycle":{"start":"s","end":"e"},"creditsLimit":5,"plan":"p"},"usage":{"api":1,"archival":2,"das":3,"grpc":4,"grpcGeyser":5,"photon":6,"rpc":7,"stream":8,"webhook":9,"websocket":10}}"#, Service::Admin, Verb::Get, &["admin", "projects", "p", "usage"]);
}
