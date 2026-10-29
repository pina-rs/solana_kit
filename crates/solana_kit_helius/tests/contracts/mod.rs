//! Maintained public-contract bodies shared by ordinary tests and portable consumers.

pub mod checkout;
pub mod compute_units;
pub mod das;
pub mod enhanced;
pub mod nested_models;
pub mod pagination;
#[cfg(feature = "checkout")]
mod payment_support;
pub mod payment_terms;
#[cfg(feature = "checkout")]
pub mod payments;
pub mod priority_fee_resolution;
pub mod rest_catalog;
pub mod rpc;
pub mod sender;
pub mod service;

pub mod sharing;
#[cfg(feature = "crypto")]
pub mod signing;
#[cfg(feature = "signup")]
pub mod signup;
pub mod staking;
pub mod subscriptions;
pub mod v2;
pub mod wallet;
pub mod webhooks;
pub mod wire;
pub mod zk;

/// Executes every maintained deterministic public-contract group.
pub fn run_all() {
    for (name, body) in CASES {
        body();
        println!("PASS {name}");
    }

    println!("PASS all {} Helius contract groups", CASES.len());
}

/// Maintained public behavior registry. Each host executes these exact bodies.
pub const CASES: &[(&str, fn())] = &[
    ("das-vectors", das::vectors),
    ("zk-vectors", zk::vectors),
    ("enhanced-rpc-vectors", rpc::enhanced_rpc_vectors),
    (
        "known-malformed-fields",
        rpc::known_fields_reject_malformed_values,
    ),
    (
        "presence-and-decimals",
        wire::presence_and_decimal_precision,
    ),
    ("preconf-wire", wire::preconfirmation_binary_contracts),
    ("webhook-methods", webhooks::vectors),
    ("wallet-query-order", service::wallet_query_order),
    (
        "http-error-custody",
        service::http_errors_retain_bytes_without_debug_leak,
    ),
    ("service-budgets", service::budgets_precede_typed_success),
    ("shared-rest-admission", service::shared_rest_admission),
    (
        "owned-cancellation",
        service::dropped_exchange_releases_owned_request,
    ),
    ("pagination-limits", pagination::records_and_limits),
    ("json-provider", subscriptions::json_provider_lifecycle),
    (
        "physical-provider-sharing",
        sharing::physical_registration_and_last_cancellation,
    ),
    (
        "provider-subscribe-uncertainty",
        sharing::subscribe_ack_uncertainty_requires_replacement,
    ),
    (
        "binary-provider-broadcast",
        subscriptions::binary_early_broadcast_and_cancellation,
    ),
    (
        "binary-provider-replacement",
        subscriptions::binary_failure_and_replacement,
    ),
    ("auth-routes", rest_catalog::auth_routes),
    #[cfg(feature = "signup")]
    ("signup-existing", signup::existing_projects),
    #[cfg(feature = "signup")]
    ("signup-billing", signup::billing_windows),
    #[cfg(feature = "signup")]
    ("signup-payment", signup::new_subscription),
    #[cfg(feature = "signup")]
    ("signup-failures", signup::checkout_failures),
    #[cfg(feature = "signup")]
    ("signup-completion-existing", signup::terminal_exits),
    #[cfg(feature = "signup")]
    (
        "signup-completion-provisioning",
        signup::completed_provisioning,
    ),
    #[cfg(feature = "signup")]
    ("signup-completion-outcomes", signup::completion_outcomes),
    #[cfg(feature = "signup")]
    ("signup-completion-failures", signup::completion_failures),
    #[cfg(feature = "signup")]
    ("signup-completion-cancellation", signup::dropped_completion),
    #[cfg(all(feature = "crypto", feature = "signup"))]
    ("signup-signing", signup::signed_authentication),
    #[cfg(feature = "checkout")]
    ("payment-plan-upgrade", payments::plan_upgrades),
    #[cfg(feature = "checkout")]
    ("payment-explicit-price", payments::explicit_prices),
    #[cfg(feature = "checkout")]
    ("payment-rejection", payments::payment_failures),
    #[cfg(feature = "checkout")]
    ("payment-renewal", payments::existing_intents),
    #[cfg(feature = "checkout")]
    ("payment-cancellation", payments::dropped_preparation),
    ("staking-preparation", staking::preparation),
    ("compute-units-estimation", compute_units::estimation),
    (
        "priority-fee-resolution",
        priority_fee_resolution::resolution,
    ),
    (
        "wallet-and-enhanced-routes",
        rest_catalog::wallet_and_enhanced_routes,
    ),
    (
        "oauth-and-developer-routes",
        rest_catalog::oauth_form_and_developer_routes,
    ),
    ("admin-route", rest_catalog::admin_usage_route),
    ("sender-request", sender::submission_frames),
    ("sender-errors", sender::response_failures),
    ("checkout-routes", checkout::endpoint_contracts),
    ("checkout-prices", checkout::price_contracts),
    ("checkout-polling", checkout::polling_terminal_and_expiry),
    (
        "checkout-clock-errors",
        checkout::polling_limits_and_clock_errors,
    ),
    ("nested-asset", nested_models::complete_asset_model),
    ("enhanced-models", enhanced::models),
    ("transfer-models", nested_models::transfer_amounts_and_nulls),
    ("payment-terms", payment_terms::positive_and_adverse_terms),
    #[cfg(feature = "crypto")]
    ("auth-default-signing", signing::default_message),
    #[cfg(feature = "crypto")]
    ("auth-caller-signing", signing::caller_message),
];
