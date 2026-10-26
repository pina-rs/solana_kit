//! Explicit project/API-key calls. Credential resolution remains outside the SDK.

mod models;
pub use models::*;
mod developer;
pub use developer::*;
mod plans;
mod results;
#[cfg(feature = "signup")]
mod signup;
#[cfg(feature = "checkout")]
mod urls;
/// Prepare an existing project's plan change: pass its project ID as `ref_id`.
/// The provider decides upgrade eligibility; this alias shares the checkout implementation.
#[cfg(feature = "checkout")]
pub use checkout::create_payment as upgrade_plan;
#[cfg(feature = "checkout")]
pub use checkout::{CreatePaymentRequest, PaymentError, PaymentPrice, create_payment, pay_renewal};
pub use plans::{BillingPeriod, InvalidPlan, Plan, PlanInfo};
pub use results::*;
#[cfg(feature = "signup")]
pub use signup::{ContactField, SignupError, auth_signup};
#[cfg(feature = "checkout")]
pub use urls::{build_endpoints, build_payment_url};
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
