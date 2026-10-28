//! Decoded query values preserve built-in positions and extra-parameter overrides.

use crate::{Error, Integer};
use solana_kit_client::{TransportError, TransportFailureKind};
use solana_kit_core::Address;
use std::collections::BTreeMap;

/// Whether the requested amount describes input or output tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwapMode {
    /// Pay an exact input amount.
    ExactIn,
    /// Receive an exact output amount.
    ExactOut,
}

impl SwapMode {
    const fn wire(self) -> &'static str {
        match self {
            Self::ExactIn => "ExactIn",
            Self::ExactOut => "ExactOut",
        }
    }
}

/// A Swap API v2 quote, managed order or self-landed build request.
///
/// Optional fields remain omitted so provider defaults apply. The API requires
/// a taker for a build or assembled order. Additional query parameters replace
/// matching built-in values, matching the pinned reference; they cannot select
/// a service origin or path. The provider validates economic parameter ranges.
#[derive(Clone, PartialEq, Eq)]
pub struct OrderRequest {
    /// Mint of the token being sold.
    pub input_mint: Address,
    /// Mint of the token being bought.
    pub output_mint: Address,
    /// Exact amount in the selected mint's smallest unit.
    pub amount: Integer,
    /// Maximum slippage in basis points.
    pub slippage_bps: Option<i64>,
    /// Input or output amount interpretation.
    pub swap_mode: Option<SwapMode>,
    /// Account receiving referral fees.
    pub referral_account: Option<Address>,
    /// Referral fee in basis points; the API documents 0 through 255.
    pub referral_fee: Option<i64>,
    /// Restrict routes through intermediate tokens.
    pub restrict_intermediate_tokens: Option<bool>,
    /// Platform fee in basis points for the build route.
    pub platform_fee_bps: Option<i64>,
    /// Exclude direct router routes.
    pub exclude_direct_router_routes: Option<bool>,
    /// Alternative transaction payer for gasless flows.
    pub payer: Option<Address>,
    /// Wallet initiating the swap; required by the provider for builds.
    pub taker: Option<Address>,
    /// Additional decoded query values, including explicit built-in overrides.
    ///
    /// New keys are appended in sorted order, unlike Dart's insertion-ordered map.
    /// Overrides retain the position of the corresponding built-in key.
    pub extra_query_parameters: BTreeMap<String, String>,
}

impl OrderRequest {
    /// Creates a request that leaves all optional settings to the provider.
    pub fn new(input_mint: Address, output_mint: Address, amount: Integer) -> Self {
        Self {
            input_mint,
            output_mint,
            amount,
            slippage_bps: None,
            swap_mode: None,
            referral_account: None,
            referral_fee: None,
            restrict_intermediate_tokens: None,
            platform_fee_bps: None,
            exclude_direct_router_routes: None,
            payer: None,
            taker: None,
            extra_query_parameters: BTreeMap::new(),
        }
    }

    /// Returns decoded query pairs; transport encoding happens only at the boundary.
    pub fn to_query_parameters(&self) -> Vec<(String, String)> {
        let mut pairs = vec![
            ("inputMint".into(), self.input_mint.to_string()),
            ("outputMint".into(), self.output_mint.to_string()),
            ("amount".into(), self.effective_amount().into()),
        ];
        append(&mut pairs, "slippageBps", self.slippage_bps);
        append(&mut pairs, "swapMode", self.swap_mode.map(SwapMode::wire));
        append(&mut pairs, "referralAccount", self.referral_account);
        append(&mut pairs, "referralFee", self.referral_fee);
        append(
            &mut pairs,
            "restrictIntermediateTokens",
            self.restrict_intermediate_tokens,
        );
        append(&mut pairs, "platformFeeBps", self.platform_fee_bps);

        append(
            &mut pairs,
            "excludeDirectRouterRoutes",
            self.exclude_direct_router_routes,
        );
        append(&mut pairs, "payer", self.payer);
        append(&mut pairs, "taker", self.taker);

        let built_in_count = pairs.len();

        for (key, value) in &self.extra_query_parameters {
            override_query(&mut pairs, built_in_count, key, value);
        }

        pairs
    }

    /// Rejects inputs whose raw size already exceeds the request budget before cloning.
    ///
    /// The effective amount is borrowed, including an extra-parameter override. The
    /// raw checks are only lower bounds; the transport boundary enforces the final
    /// percent-encoded request target budget.
    pub(crate) fn bounded_query(&self, maximum: usize) -> Result<Vec<(String, String)>, Error> {
        let too_large = || Error::Transport(TransportError::new(TransportFailureKind::BodyLimit));

        // Each extra pair consumes at least a separator and an equals sign.
        if self.effective_amount().len() > maximum
            || self.extra_query_parameters.len() > maximum / 2
        {
            return Err(too_large());
        }

        let mut minimum = 0_usize;

        for (key, value) in &self.extra_query_parameters {
            minimum = minimum
                .saturating_add(key.len())
                .saturating_add(value.len())
                .saturating_add(2);

            if minimum > maximum {
                return Err(too_large());
            }
        }

        Ok(self.to_query_parameters())
    }

    fn effective_amount(&self) -> &str {
        self.extra_query_parameters
            .get("amount")
            .map_or(self.amount.as_str(), String::as_str)
    }
}

fn override_query(
    query: &mut Vec<(String, String)>,
    built_in_count: usize,
    key: &str,
    value: &str,
) {
    if let Some((_, current)) = query[..built_in_count]
        .iter_mut()
        .find(|(name, _)| name == key)
    {
        current.clear();
        current.push_str(value);

        return;
    }

    query.push((key.into(), value.into()));
}

impl std::fmt::Debug for OrderRequest {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("OrderRequest { .. }")
    }
}

fn append(query: &mut Vec<(String, String)>, name: &str, value: Option<impl ToString>) {
    if let Some(value) = value {
        query.push((name.into(), value.to_string()));
    }
}
