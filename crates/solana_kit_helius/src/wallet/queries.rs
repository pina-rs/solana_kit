//! Wallet v1 queries retain ordered values for the caller's approved HTTP adapter.

use super::{
    BalanceAtPoint, GetBalanceAtRequest, GetBalanceAtResponse, WalletBalances, WalletHistory,
    WalletTransfers, path,
};
use crate::service::{Body, Error, Operation, Service, Verb, pair};

/// Token-account participation in wallet history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum WalletTokenAccounts {
    /// Only transactions directly referencing the wallet.
    #[serde(rename = "none")]
    None,
    /// Transactions that changed owned token balances.
    #[serde(rename = "balanceChanged")]
    BalanceChanged,
    /// Include all owned token accounts.
    #[serde(rename = "all")]
    All,
}

impl WalletTokenAccounts {
    fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::BalanceChanged => "balanceChanged",
            Self::All => "all",
        }
    }
}

request!(
    #[serde(deny_unknown_fields)]
    "Read a wallet balance page; the provider limits each page to 100 tokens.",
    GetBalancesRequest { wallet: String => "wallet" } optional {
        page: u64 => "page",
        limit: u64 => "limit",
        show_zero_balance: bool => "showZeroBalance",
        show_native: bool => "showNative",
        show_nfts: bool => "showNfts",
    }
);
request!(
    #[serde(deny_unknown_fields)]
    "Read wallet history with cursor pagination.",
    GetHistoryRequest { wallet: String => "wallet" } optional {
        limit: u64 => "limit",
        before: String => "before",
        after: String => "after",
        transaction_type: String => "type",
        token_accounts: WalletTokenAccounts => "tokenAccounts",
    }
);
request!(
    #[serde(deny_unknown_fields)]
    "Read token transfers with an opaque pagination cursor.",
    GetTransfersRequest { wallet: String => "wallet" } optional {
        limit: u64 => "limit",
        cursor: String => "cursor",
    }
);

impl Operation for GetBalancesRequest {
    type Response = WalletBalances;
    const SERVICE: Service = Service::Wallet;
    const VERB: Verb = Verb::Get;
    const BODY: Body = Body::Absent;

    fn path(&self) -> Vec<String> {
        path(&self.wallet, "balances")
    }

    fn query(&self) -> Vec<(String, String)> {
        let mut query = Vec::new();
        pair(&mut query, "page", self.page);
        pair(&mut query, "limit", self.limit);
        pair(&mut query, "showZeroBalance", self.show_zero_balance);
        pair(&mut query, "showNative", self.show_native);
        pair(&mut query, "showNfts", self.show_nfts);
        query
    }
}

impl Operation for GetHistoryRequest {
    type Response = WalletHistory;
    const SERVICE: Service = Service::Wallet;
    const VERB: Verb = Verb::Get;
    const BODY: Body = Body::Absent;

    fn path(&self) -> Vec<String> {
        path(&self.wallet, "history")
    }

    fn query(&self) -> Vec<(String, String)> {
        let mut query = Vec::new();
        pair(&mut query, "limit", self.limit);
        pair(&mut query, "before", self.before.as_ref());
        pair(&mut query, "after", self.after.as_ref());
        pair(&mut query, "type", self.transaction_type.as_ref());
        pair(
            &mut query,
            "tokenAccounts",
            self.token_accounts.map(WalletTokenAccounts::as_str),
        );
        query
    }
}

impl Operation for GetTransfersRequest {
    type Response = WalletTransfers;
    const SERVICE: Service = Service::Wallet;
    const VERB: Verb = Verb::Get;
    const BODY: Body = Body::Absent;

    fn path(&self) -> Vec<String> {
        path(&self.wallet, "transfers")
    }

    fn query(&self) -> Vec<(String, String)> {
        let mut query = Vec::new();
        pair(&mut query, "limit", self.limit);
        pair(&mut query, "cursor", self.cursor.as_ref());
        query
    }
}

impl Operation for GetBalanceAtRequest {
    type Response = GetBalanceAtResponse;
    const SERVICE: Service = Service::Wallet;
    const VERB: Verb = Verb::Get;
    const BODY: Body = Body::Absent;

    fn path(&self) -> Vec<String> {
        path(&self.wallet, "balance-at")
    }

    fn query(&self) -> Vec<(String, String)> {
        let mut query = vec![("mint".into(), self.mint.clone())];

        match &self.point {
            BalanceAtPoint::Time(time) => pair(&mut query, "time", Some(time)),
            BalanceAtPoint::Datetime(datetime) => pair(&mut query, "datetime", Some(datetime)),
            BalanceAtPoint::Slot(slot) => pair(&mut query, "slot", Some(slot)),
        }

        query
    }

    fn decode(&self, bytes: &[u8]) -> Result<GetBalanceAtResponse, Error> {
        crate::models::checked_json(bytes, |response: &GetBalanceAtResponse| {
            response.wallet == self.wallet
                && response.mint == self.mint
                && self.point.matches(&response.requested)
        })
        .map_err(Error::Schema)
    }
}
