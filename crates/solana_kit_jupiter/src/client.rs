//! One exchange per request; the adapter owns origins, credentials, deadlines and cancellation.

use crate::{Category, Error, Interval, MAX_PRICE_MINTS, Price, TokenItem, TokenLimit};
use solana_kit_client::rest::{self, Budgets, Transport};
use solana_kit_core::Address;
use std::collections::BTreeMap;

/// Independently approved Jupiter origin and optional x-api-key policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Service;

/// Jupiter price and token APIs without an executor or global networking state.
pub struct Client<T> {
    transport: T,
    budgets: Budgets,
}

impl<T: Transport<Service>> Client<T> {
    /// Associates an application-owned adapter with explicit JSON and receiving budgets.
    pub const fn new(transport: T, budgets: Budgets) -> Self {
        Self { transport, budgets }
    }

    /// Fetches at most fifty mint prices; unreliable prices may be omitted by the provider.
    pub async fn get_prices(&self, mints: &[Address]) -> Result<BTreeMap<Address, Price>, Error> {
        if mints.len() > MAX_PRICE_MINTS {
            return Err(Error::TooManyMints);
        }

        let ids = mints
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",");
        let body = self
            .get(&["price", "v3"], vec![("ids".into(), ids)])
            .await?;
        crate::decode_prices(&body, self.budgets)
    }

    /// Searches token names, symbols or mint text; query data remains a decoded query value.
    pub async fn search_tokens(
        &self,
        query: &str,
        limit: Option<u64>,
    ) -> Result<Vec<TokenItem>, Error> {
        self.budgets
            .check_bytes(query.as_bytes())
            .map_err(Error::Json)?;
        let mut pairs = vec![("query".into(), query.to_owned())];
        append_limit(&mut pairs, limit);
        self.tokens(&["tokens", "v2", "search"], pairs).await
    }

    /// Lists tokens with an arbitrary provider tag, including verified, strict and lfg.
    pub async fn tagged_tokens(&self, tag: &str) -> Result<Vec<TokenItem>, Error> {
        self.budgets
            .check_bytes(tag.as_bytes())
            .map_err(Error::Json)?;
        self.tokens(
            &["tokens", "v2", "tag"],
            vec![("query".into(), tag.to_owned())],
        )
        .await
    }

    /// Lists a closed category and interval, with an optional validated page limit.
    pub async fn category_tokens(
        &self,
        category: Category,
        interval: Interval,
        limit: Option<TokenLimit>,
    ) -> Result<Vec<TokenItem>, Error> {
        let path = ["tokens", "v2", category.segment(), interval.segment()];
        let mut pairs = Vec::new();
        append_limit(&mut pairs, limit.map(|value| u64::from(value.get())));
        self.tokens(&path, pairs).await
    }

    /// Lists recently active tokens in the provider's response order.
    pub async fn recent_tokens(&self) -> Result<Vec<TokenItem>, Error> {
        self.tokens(&["tokens", "v2", "recent"], Vec::new()).await
    }

    async fn tokens(
        &self,
        path: &[&str],
        query: Vec<(String, String)>,
    ) -> Result<Vec<TokenItem>, Error> {
        let bytes = self.get(path, query).await?;
        crate::decode_tokens(&bytes, self.budgets)
    }

    async fn get(&self, path: &[&str], query: Vec<(String, String)>) -> Result<Vec<u8>, Error> {
        let request = rest::Request {
            service: Service,
            method: rest::Method::Get,
            path_segments: path.iter().map(|part| (*part).to_owned()).collect(),
            query,
            body: rest::Body::Absent,
            max_reply_bytes: self.budgets.bytes,
        };
        rest::request_target(
            "/",
            &request.path_segments,
            &request.query,
            self.budgets.bytes,
        )
        .map_err(Error::Transport)?;
        let reply = self
            .transport
            .exchange(request)
            .await
            .map_err(|error| Error::Transport(rest::redact_error(error)))?;
        self.budgets.check_bytes(&reply.body).map_err(Error::Json)?;

        if !(200..300).contains(&reply.status) {
            return Err(Error::Http {
                status: reply.status,
                body: reply.body,
            });
        }

        Ok(reply.body)
    }
}

fn append_limit(query: &mut Vec<(String, String)>, limit: Option<u64>) {
    if let Some(limit) = limit {
        query.push(("limit".into(), limit.to_string()));
    }
}
