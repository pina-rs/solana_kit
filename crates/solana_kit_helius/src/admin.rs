//! Project administration uses the adapter's Admin origin and x-api-key policy.

model!(
    "Credit balances, subscription details and per-service usage.",
    ProjectUsage {
        credits_remaining: u64 => "creditsRemaining",
        credits_used: u64 => "creditsUsed",
        prepaid_credits_remaining: u64 => "prepaidCreditsRemaining",
        prepaid_credits_used: u64 => "prepaidCreditsUsed",
        subscription_details: AdminSubscriptionDetails => "subscriptionDetails",
        usage: AdminUsageBreakdown => "usage",
    } optional {}
);
model!(
    "The subscription plan and billing cycle.",
    AdminSubscriptionDetails {
        billing_cycle: AdminBillingCycle => "billingCycle",
        credits_limit: u64 => "creditsLimit",
        plan: String => "plan",
    } optional {}
);
model!(
    "The provider's billing cycle timestamps.",
    AdminBillingCycle {
        start: String => "start",
        end: String => "end",
    } optional {}
);
model!(
    "Per-service project usage counts.",
    AdminUsageBreakdown {
        api: u64 => "api",
        archival: u64 => "archival",
        das: u64 => "das",
        grpc: u64 => "grpc",
        grpc_geyser: u64 => "grpcGeyser",
        photon: u64 => "photon",
        rpc: u64 => "rpc",
        stream: u64 => "stream",
        webhook: u64 => "webhook",
        websocket: u64 => "websocket",
    } optional {}
);
request!(
    "Read project usage. The Admin adapter root includes v0.",
    GetProjectUsage {
        project_id: String => "projectId",
    } optional {}
);

rest!(
    GetProjectUsage,
    ProjectUsage,
    Admin,
    Get,
    Absent,
    |request| vec![
        "admin".into(),
        "projects".into(),
        request.project_id.clone(),
        "usage".into()
    ]
);
