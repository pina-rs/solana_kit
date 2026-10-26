//! Explicit dashboard API requests. The application adapter owns any JWT header.

use crate::{
    SensitiveString,
    service::{Body, Operation, Service, Verb},
};

request!(
    "OAuth code exchange with explicitly supplied PKCE values.",
    OAuthTokenExchangeRequest {
        code: SensitiveString => "code",
        code_verifier: SensitiveString => "codeVerifier",
        client_id: String => "clientId",
        redirect_uri: String => "redirectUri",
    } optional {}
);
model!(
    "OAuth token response with redacted token formatting.",
    OAuthTokenResponse {
        access_token: SensitiveString => "access_token",
        token_type: String => "token_type",
        expires_in: u64 => "expires_in",
        user: OAuthUser => "user",
    } optional {}
);
model!(
    "Authenticated OAuth user information.",
    OAuthUser {
        id: String => "id",
        email: SensitiveString => "email",
    } optional {}
);
model!(
    "Developer wallet authentication result.",
    DeveloperSignupResponse {
        #[serde(deserialize_with = "nonempty_secret")]
        token: SensitiveString => "token",
        #[serde(deserialize_with = "nonempty_string")]
        ref_id: String => "refId",
        new_user: bool => "newUser",
    } optional {}
);
model!(
    "Developer subscription billing window.",
    DeveloperSubscription {
        #[serde(deserialize_with = "nonempty_string")]
        plan: String => "plan",
        #[serde(deserialize_with = "nonempty_string")]
        billing_period_start: String => "billingPeriodStart",
        #[serde(deserialize_with = "nonempty_string")]
        billing_period_end: String => "billingPeriodEnd",
    } optional {}
);
model!(
    "Developer project summary.",
    DeveloperProjectSummary {
        #[serde(deserialize_with = "nonempty_string")]
        id: String => "id",
        subscription: DeveloperSubscription => "subscription",
    } optional {}
);
model!(
    "Developer API key identity.",
    DeveloperApiKey {
        #[serde(deserialize_with = "nonempty_secret")]
        key_id: SensitiveString => "keyId",
    } optional {}
);
model!(
    "Developer project's configured API keys.",
    DeveloperProjectDetails {
        api_keys: Vec<DeveloperApiKey> => "apiKeys",
    } optional {}
);

request!(
    "Authenticate a developer wallet.",
    DeveloperWalletSignup {
        message: String => "message",
        signature: String => "signature",
        wallet_address: String => "userID",
    } optional {}
);
request!(
    "Read developer projects using the adapter's bearer token.",
    DeveloperListProjects {} optional {}
);
request!(
    "Read developer project details.",
    DeveloperGetProject {
        project_id: String => "projectId",
    } optional {}
);

/// Add one project API key. Its project identity is only a path segment.
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeveloperCreateApiKey {
    /// Owning project identity.
    #[serde(skip)]
    pub project_id: String,
    /// Wallet identity, preserving the endpoint's distinct userId spelling.
    #[serde(rename = "userId")]
    pub wallet_address: String,
}

impl Operation for OAuthTokenExchangeRequest {
    type Response = OAuthTokenResponse;
    const SERVICE: Service = Service::Developer;
    const VERB: Verb = Verb::Post;
    const BODY: Body = Body::Form;

    fn path(&self) -> Vec<String> {
        vec!["oauth".into(), "token".into()]
    }

    fn form(&self) -> Vec<(String, String)> {
        vec![
            ("grant_type".into(), "authorization_code".into()),
            ("code".into(), self.code.expose().to_owned()),
            (
                "code_verifier".into(),
                self.code_verifier.expose().to_owned(),
            ),
            ("client_id".into(), self.client_id.clone()),
            ("redirect_uri".into(), self.redirect_uri.clone()),
        ]
    }
}

rest!(
    DeveloperWalletSignup,
    DeveloperSignupResponse,
    Developer,
    Post,
    Json,
    |_request| vec!["wallet-signup".into()]
);
rest!(
    DeveloperListProjects,
    Vec<DeveloperProjectSummary>,
    Developer,
    Get,
    Absent,
    |_request| vec!["projects".into()]
);
rest!(
    DeveloperGetProject,
    DeveloperProjectDetails,
    Developer,
    Get,
    Absent,
    |request| vec!["projects".into(), request.project_id.clone()]
);
rest!(
    DeveloperCreateApiKey,
    DeveloperApiKey,
    Developer,
    Post,
    Json,
    |request| vec![
        "projects".into(),
        request.project_id.clone(),
        "add-key".into()
    ]
);

fn nonempty_string<'de, D: serde::Deserializer<'de>>(decoder: D) -> Result<String, D::Error> {
    let value = <String as serde::Deserialize>::deserialize(decoder)?;

    if value.is_empty() {
        return Err(serde::de::Error::custom(
            "expected a nonempty provider string",
        ));
    }

    Ok(value)
}

fn nonempty_secret<'de, D: serde::Deserializer<'de>>(
    decoder: D,
) -> Result<SensitiveString, D::Error> {
    nonempty_string(decoder).map(SensitiveString::new)
}
