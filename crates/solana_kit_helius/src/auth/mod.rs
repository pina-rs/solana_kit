//! Explicit project/API-key calls. Credential resolution remains outside the SDK.

mod models;
pub use models::*;
mod developer;
pub use developer::*;
mod plans;
mod results;
pub use plans::{BillingPeriod, InvalidPlan, Plan, PlanInfo};
pub use results::*;
mod payment_terms;
pub use payment_terms::{
    PaymentTermsError, PaymentTransfer, TREASURY, USDC_MINT_DEVNET, USDC_MINT_MAINNET,
};
/// Explicit checkout endpoints, price resolution and finite observation.
pub mod checkout;
#[cfg(feature = "crypto")]
mod signing;
#[cfg(feature = "crypto")]
pub use signing::{SignedAuthMessage, VERIFICATION_TEXT, auth_message, sign_auth_message};

request!(
    "Read one project.",
    GetProject { project_id: String => "projectId" } optional {}
);
request!(
    "List authenticated projects.",
    ListProjects {} optional {}
);
request!(
    "Read authenticated credit balances.",
    CheckBalances {} optional {}
);

rest!(
    WalletSignupRequest,
    WalletSignupResponse,
    Auth,
    Post,
    Json,
    |_request| vec!["v0".into(), "auth".into(), "wallet-signup".into()]
);
rest!(
    CreateProjectRequest,
    HeliusProject,
    Auth,
    Post,
    Json,
    |_request| vec!["v0".into(), "auth".into(), "projects".into()]
);
rest!(
    CreateApiKeyRequest,
    HeliusApiKey,
    Auth,
    Post,
    Json,
    |_request| vec!["v0".into(), "auth".into(), "api-keys".into()]
);
rest!(
    GetProject,
    HeliusProject,
    Auth,
    Get,
    Absent,
    |request| vec![
        "v0".into(),
        "auth".into(),
        "projects".into(),
        request.project_id.clone()
    ]
);
rest!(
    ListProjects,
    Vec<HeliusProject>,
    Auth,
    Get,
    Absent,
    |_request| vec!["v0".into(), "auth".into(), "projects".into()]
);
rest!(
    CheckBalances,
    CheckBalancesResponse,
    Auth,
    Get,
    Absent,
    |_request| vec!["v0".into(), "auth".into(), "balances".into()]
);
