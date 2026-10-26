//! Endpoint and tip metadata from the immutable source, without automatic dialing.

pub use crate::enums::SenderRegion;

/// Number of lamports in one SOL.
pub const LAMPORTS_PER_SOL: u64 = 1_000_000_000;
/// Pinned Sender Max minimum tip in lamports.
pub const MIN_TIP_LAMPORTS_MAX: u64 = 1_000_000;
/// Pinned SWQOS-only minimum tip in lamports.
pub const MIN_TIP_LAMPORTS_SWQOS: u64 = 5_000;

/// Pinned tip-account strings. The caller selects and validates its payment recipient.
pub const TIP_ACCOUNTS: [&str; 10] = [
    "4ACfpUFoaSD9bfPdeu6DBt89gB6ENTeHBXCAi87NhDEE",
    "D2L6yPZ2FmmmTKPgzaMKdhu6EWZcTpLy1Vhx8uvZe7NZ",
    "9bnz4RShgq1hAnLnZbP8kbgBg1kEmcJBYQq3gQbmnSta",
    "5VY91ws6B2hMmBFRsXkoAAdsPHBJwRfBht4DXox3xkwn",
    "2nyhqdwKcJZR2vcqCyrYsaPVdAnFoJjiksCXJ7hfEYgD",
    "2q5pghRs6arqVjRvT5gfgWfWcHWmw1ZuCzphgd5KfWGJ",
    "wyvPkWjVZz1M8fHQnMMCDTQDbkManefNNhweYk5WkcF",
    "3KCKozbAaF75qEU33jtzozcJ29yJuaLJTy2jFdzUY8bT",
    "4vieeGHPYPG2MmyPRcYjdiDmmhN3ww7hsFNap8pVN3Ey",
    "4TQLFNWK8AovT1gFvda5jfw2oJeRMKEmw7aH6MGBJ3or",
];

impl Default for SenderRegion {
    fn default() -> Self {
        Self::Default
    }
}

impl SenderRegion {
    /// Returns the pinned source origin. Regional HTTP requires explicit adapter policy.
    pub const fn origin(self) -> &'static str {
        match self {
            Self::Default => "https://sender.helius-rpc.com",
            Self::UsSlc => "http://slc-sender.helius-rpc.com",
            Self::UsEast => "http://ewr-sender.helius-rpc.com",
            Self::EuWest => "http://lon-sender.helius-rpc.com",
            Self::EuCentral => "http://fra-sender.helius-rpc.com",
            Self::EuNorth => "http://ams-sender.helius-rpc.com",
            Self::ApSingapore => "http://sg-sender.helius-rpc.com",
            Self::ApTokyo => "http://tyo-sender.helius-rpc.com",
        }
    }

    /// Returns the explicit region's fast path without opening a connection.
    pub fn fast_url(self) -> String {
        format!("{}/fast", self.origin())
    }

    /// Returns the explicit region's ping path without opening a connection.
    pub fn ping_url(self) -> String {
        format!("{}/ping", self.origin())
    }
}
