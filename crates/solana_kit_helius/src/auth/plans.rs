//! Pinned plan metadata. These values are a source snapshot, not current pricing.

/// Supported signup plan, normalized by parsing before any network request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Plan {
    /// Agent plan.
    Agent,
    /// Developer plan.
    Developer,
    /// Business plan.
    Business,
    /// Professional plan.
    Professional,
}

/// Supported billing periods.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BillingPeriod {
    /// Monthly billing, the reference default.
    Monthly,
    /// Yearly billing.
    Yearly,
}

/// An unknown provider plan or billing period.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidPlan;

impl core::fmt::Display for InvalidPlan {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str("unknown Helius plan or billing period")
    }
}

impl std::error::Error for InvalidPlan {}

impl core::str::FromStr for Plan {
    type Err = InvalidPlan;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "agent" => Ok(Self::Agent),
            "developer" => Ok(Self::Developer),
            "business" => Ok(Self::Business),
            "professional" => Ok(Self::Professional),
            _ => Err(InvalidPlan),
        }
    }
}

impl core::str::FromStr for BillingPeriod {
    type Err = InvalidPlan;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_ascii_lowercase().as_str() {
            "monthly" => Ok(Self::Monthly),
            "yearly" => Ok(Self::Yearly),
            _ => Err(InvalidPlan),
        }
    }
}

impl Plan {
    /// Exact signup wire name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Agent => "agent",
            Self::Developer => "developer",
            Self::Business => "business",
            Self::Professional => "professional",
        }
    }

    /// Exact usage-plan identifier returned by the pinned developer API.
    pub const fn usage_plan(self) -> &'static str {
        match self {
            Self::Agent => "agent_v4",
            Self::Developer => "developer_v4",
            Self::Business => "business_v4",
            Self::Professional => "professional_v4",
        }
    }

    /// Pinned catalog entry. The source has no price/limit entry for Agent.
    pub const fn info(self) -> Option<PlanInfo> {
        match self {
            Self::Agent => None,
            Self::Developer => Some(DEVELOPER),
            Self::Business => Some(BUSINESS),
            Self::Professional => Some(PROFESSIONAL),
        }
    }
}

impl BillingPeriod {
    /// Exact billing period wire name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Monthly => "monthly",
            Self::Yearly => "yearly",
        }
    }
}

/// Plan metadata preserved from the immutable source, not a live service quote.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlanInfo {
    /// Display name.
    pub name: &'static str,
    /// Monthly price in smallest currency units.
    pub monthly_price: u64,
    /// Yearly price in smallest currency units.
    pub yearly_price: u64,
    /// Included credits.
    pub credits: u64,
    /// Source request-rate allowance.
    pub requests_per_second: u64,
}

const DEVELOPER_MONTHLY_CENTS: u64 = 4_900;
const DEVELOPER_YEARLY_CENTS: u64 = 49_000;
const DEVELOPER_CREDITS: u64 = 10_000_000;
const BUSINESS_MONTHLY_CENTS: u64 = 49_900;
const BUSINESS_YEARLY_CENTS: u64 = 499_000;
const BUSINESS_CREDITS: u64 = 100_000_000;
const PROFESSIONAL_MONTHLY_CENTS: u64 = 99_900;
const PROFESSIONAL_YEARLY_CENTS: u64 = 999_000;
const PROFESSIONAL_CREDITS: u64 = 200_000_000;

const DEVELOPER: PlanInfo = PlanInfo {
    name: "Developer",
    monthly_price: DEVELOPER_MONTHLY_CENTS,
    yearly_price: DEVELOPER_YEARLY_CENTS,
    credits: DEVELOPER_CREDITS,
    requests_per_second: 50,
};
const BUSINESS: PlanInfo = PlanInfo {
    name: "Business",
    monthly_price: BUSINESS_MONTHLY_CENTS,
    yearly_price: BUSINESS_YEARLY_CENTS,
    credits: BUSINESS_CREDITS,
    requests_per_second: 200,
};
const PROFESSIONAL: PlanInfo = PlanInfo {
    name: "Professional",
    monthly_price: PROFESSIONAL_MONTHLY_CENTS,
    yearly_price: PROFESSIONAL_YEARLY_CENTS,
    credits: PROFESSIONAL_CREDITS,
    requests_per_second: 500,
};
