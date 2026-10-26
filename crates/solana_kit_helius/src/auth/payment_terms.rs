//! Hosted payment terms are validated before any transaction builder or signer sees them.

use super::PaymentLink;
use solana_kit_core::Address;

/// Pinned mainnet USDC mint used by the reference payment helpers.
pub const USDC_MINT_MAINNET: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
/// Pinned devnet USDC mint, available for an explicitly separate test environment.
pub const USDC_MINT_DEVNET: &str = "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU";
/// Pinned provider treasury address used by the legacy one-USDC convenience.
pub const TREASURY: &str = "CEs84tEowsXpH8u4VBf8rJSVgSRypFMfXw9CpGRtQgb6";

/// Exact validated transfer terms. Constructing them does not sign or submit a payment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentTransfer {
    /// Explicit provider destination, parsed as a Kit address.
    pub recipient: Address,
    /// Mainnet USDC mint from the pinned payment contract.
    pub mint: Address,
    /// Positive raw six-decimal token amount, bounded to u64.
    pub amount: u64,
    /// Exact payment intent identity, to include as the signed memo.
    pub memo: String,
}

/// Payment input failures are observed before any I/O, signing or token instruction creation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentTermsError {
    /// The link is not a payment_required intent.
    Kind,
    /// The memo does not match the payment intent identity.
    Memo,
    /// The cent amount is zero or overflows the raw u64 token quantity.
    Amount,
    /// The recipient is not a valid Kit address.
    Recipient,
    /// The pinned mint constant is not a valid Kit address.
    Mint,
}

impl core::fmt::Display for PaymentTermsError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "invalid Helius payment terms: {self:?}")
    }
}

impl std::error::Error for PaymentTermsError {}

impl PaymentTransfer {
    /// Validates the source's kind, memo and exact cents-to-USDC conversion.
    pub fn from_link(link: &PaymentLink) -> Result<Self, PaymentTermsError> {
        if link.kind != "payment_required" {
            return Err(PaymentTermsError::Kind);
        }

        if link.memo != link.payment_intent_id {
            return Err(PaymentTermsError::Memo);
        }

        let amount = link
            .amount_cents
            .checked_mul(10_000)
            .filter(|amount| *amount > 0)
            .ok_or(PaymentTermsError::Amount)?;
        let recipient = link
            .destination_wallet
            .parse()
            .map_err(|_| PaymentTermsError::Recipient)?;
        let mint = USDC_MINT_MAINNET
            .parse()
            .map_err(|_| PaymentTermsError::Mint)?;
        Ok(Self {
            recipient,
            mint,
            amount,
            memo: link.payment_intent_id.clone(),
        })
    }
}
