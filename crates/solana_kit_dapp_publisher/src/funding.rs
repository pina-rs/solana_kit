//! Caller-facing funding preflight for the publisher CLI boundary.
//!
//! The SDK never fetches a balance or chooses an RPC endpoint. A caller supplies
//! either an explicit endpoint or local-development intent, performs the read
//! through its approved transport, and passes the observed lamports here.

use solana_kit_core::Address;
use std::{fmt, string::String};

/// Minimum signer balance required before publishing, about 0.016 SOL.
pub const MIN_PUBLICATION_SIGNER_BALANCE_LAMPORTS: u64 = 16_000_000;

/// Default mainnet endpoint used only when a caller asks for a non-local check.
pub const DEFAULT_MAINNET_RPC_URL: &str = "https://api.mainnet-beta.solana.com";

const MICRO_LAMPORTS_PER_SOL: u128 = 1_000_000;
const LAMPORTS_PER_MICRO_LAMPORT: u128 = 1_000;
const MICRO_LAMPORT_ROUNDING: u128 = 500;

/// One bounded funding observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FundingCheck {
    /// The observed balance meets the publication minimum.
    Sufficient {
        /// Observed lamports.
        balance: u64,
        /// Required lamports.
        required: u64,
    },
    /// The observed balance is below the publication minimum.
    Insufficient {
        /// Observed lamports.
        balance: u64,
        /// Required lamports.
        required: u64,
    },
}

/// Invalid caller input for a funding check.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FundingError {
    /// The signer text is not a canonical address.
    InvalidAddress,
}

impl fmt::Display for FundingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidAddress => formatter.write_str("invalid signer address"),
        }
    }
}

impl std::error::Error for FundingError {}

/// Resolves the RPC endpoint for a funding preflight.
///
/// An explicit nonempty URL wins. Local development without one skips the
/// check; otherwise the caller receives the default mainnet endpoint. The SDK
/// does not validate or contact the URL here.
pub fn resolve_funding_rpc_url(local_dev: bool, explicit: Option<&str>) -> Option<&str> {
    let explicit = explicit.map(str::trim).filter(|value| !value.is_empty());

    if let Some(url) = explicit {
        return Some(url);
    }

    if local_dev {
        return None;
    }

    Some(DEFAULT_MAINNET_RPC_URL)
}

/// Formats lamports as a six-decimal SOL amount using integer rounding.
pub fn format_sol_amount(lamports: u64) -> String {
    let micros = (u128::from(lamports) + MICRO_LAMPORT_ROUNDING) / LAMPORTS_PER_MICRO_LAMPORT;
    let whole = micros / MICRO_LAMPORTS_PER_SOL;
    let fraction = micros % MICRO_LAMPORTS_PER_SOL;

    format!("{whole}.{fraction:06}")
}

/// Compares an observed balance with the publication minimum.
///
/// The caller owns the RPC read, endpoint policy, retries and deadline. A failed
/// read must remain an explicit caller warning; it is never silently treated as
/// a zero balance.
pub fn check_signer_balance(public_key: &str, balance: u64) -> Result<FundingCheck, FundingError> {
    public_key
        .parse::<Address>()
        .map_err(|_| FundingError::InvalidAddress)?;

    if balance < MIN_PUBLICATION_SIGNER_BALANCE_LAMPORTS {
        Ok(FundingCheck::Insufficient {
            balance,
            required: MIN_PUBLICATION_SIGNER_BALANCE_LAMPORTS,
        })
    } else {
        Ok(FundingCheck::Sufficient {
            balance,
            required: MIN_PUBLICATION_SIGNER_BALANCE_LAMPORTS,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_endpoint_wins_and_local_dev_can_skip() {
        assert_eq!(
            resolve_funding_rpc_url(false, Some(" https://rpc.invalid ")),
            Some("https://rpc.invalid")
        );
        assert_eq!(resolve_funding_rpc_url(true, None), None);
        assert_eq!(
            resolve_funding_rpc_url(false, None),
            Some(DEFAULT_MAINNET_RPC_URL)
        );
        assert_eq!(resolve_funding_rpc_url(true, Some("  ")), None);
    }

    #[test]
    fn sol_formatting_uses_integer_six_decimal_rounding() {
        let half_micro_lamport = MICRO_LAMPORT_ROUNDING as u64;

        assert_eq!(format_sol_amount(0), "0.000000");
        assert_eq!(format_sol_amount(1), "0.000000");
        assert_eq!(format_sol_amount(half_micro_lamport), "0.000001");
        assert_eq!(
            format_sol_amount(MIN_PUBLICATION_SIGNER_BALANCE_LAMPORTS),
            "0.016000"
        );
        assert_eq!(format_sol_amount(1_234_567_890), "1.234568");
    }

    #[test]
    fn balance_check_preserves_the_observed_value_and_threshold() {
        let below_minimum = MIN_PUBLICATION_SIGNER_BALANCE_LAMPORTS - 1;

        assert_eq!(
            check_signer_balance("11111111111111111111111111111111", below_minimum),
            Ok(FundingCheck::Insufficient {
                balance: below_minimum,
                required: MIN_PUBLICATION_SIGNER_BALANCE_LAMPORTS,
            })
        );
        assert_eq!(
            check_signer_balance(
                "11111111111111111111111111111111",
                MIN_PUBLICATION_SIGNER_BALANCE_LAMPORTS
            ),
            Ok(FundingCheck::Sufficient {
                balance: MIN_PUBLICATION_SIGNER_BALANCE_LAMPORTS,
                required: MIN_PUBLICATION_SIGNER_BALANCE_LAMPORTS,
            })
        );
        assert_eq!(
            check_signer_balance("not-an-address", MIN_PUBLICATION_SIGNER_BALANCE_LAMPORTS),
            Err(FundingError::InvalidAddress)
        );
    }
}
