//! GET orchestration owns no runtime, global browser object or detached task.
use super::{AssetType, Error, Policy, PriceFeedMetadata, PriceUpdate, UpdateOptions};
use super::{
    decode_price_feeds, decode_price_update,
    requests::{self, Route},
};
use alloc::{string::ToString, vec::Vec};
use core::{
    future::{Future, poll_fn},
    pin::pin,
    task::Poll,
};
use solana_kit_client::subscriptions::{Clock, ClockError};
use solana_kit_client::{Reply, TransportError, TransportFailureKind, rest};

/// The adapter's independently approved Hermes root, headers and authentication.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Service;

/// Typed Hermes GET client with caller-owned transport, clock and resource limits.
pub struct Client<T, C> {
    transport: T,
    clock: C,
    policy: Policy,
    budgets: rest::Budgets,
}

impl<T: rest::Transport<Service>, C: Clock> Client<T, C> {
    /// Associates existing approved host adapters without starting I/O.
    pub const fn new(transport: T, clock: C, policy: Policy, budgets: rest::Budgets) -> Self {
        Self {
            transport,
            clock,
            policy,
            budgets,
        }
    }

    /// Lists available feeds with optional symbol and asset-category filters.
    pub async fn get_price_feeds(
        &self,
        query: Option<&str>,
        asset: Option<AssetType>,
    ) -> Result<Vec<PriceFeedMetadata>, Error> {
        let query = requests::listing(query, asset, self.budgets)?;
        let bytes = self.get(Route::new(&["v2", "price_feeds"], query)).await?;
        decode_price_feeds(&bytes, self.budgets).map_err(Error::Json)
    }

    /// Reads the latest updates with ordered repeated IDs and explicit options.
    pub async fn get_latest_price_updates(
        &self,
        ids: &[&str],
        options: UpdateOptions,
    ) -> Result<PriceUpdate, Error> {
        self.updates("latest", ids, options).await
    }

    /// Reads updates at an exact signed Unix timestamp in seconds.
    pub async fn get_price_updates_at_timestamp(
        &self,
        publish_time: i64,
        ids: &[&str],
        options: UpdateOptions,
    ) -> Result<PriceUpdate, Error> {
        self.updates(&publish_time.to_string(), ids, options).await
    }

    async fn updates(
        &self,
        time: &str,
        ids: &[&str],
        options: UpdateOptions,
    ) -> Result<PriceUpdate, Error> {
        let query = requests::updates(ids, options, self.budgets)?;
        let bytes = self
            .get(Route::new(&["v2", "updates", "price", time], query))
            .await?;
        decode_price_update(&bytes, self.budgets).map_err(Error::Json)
    }

    /// Repeats only admitted transient GET failures, after dropping each prior exchange.
    async fn get(&self, route: Route) -> Result<Vec<u8>, Error> {
        rest::request_target("/", &route.path, &route.query, self.budgets.bytes)
            .map_err(Error::Transport)?;
        let mut retries = self.policy.http_retries;
        let mut backoff = self.policy.backoff_ms;

        loop {
            let error = match self.attempt(&route).await {
                Ok(bytes) => return Ok(bytes),
                Err(error) => error,
            };

            if retries == 0 || !error.retryable() {
                return Err(error);
            }

            let deadline = self.deadline(backoff)?;
            self.clock
                .sleep_until(deadline)
                .await
                .map_err(Error::Clock)?;
            retries -= 1;

            if retries > 0 {
                backoff *= 2;
            }
        }
    }

    async fn attempt(&self, route: &Route) -> Result<Vec<u8>, Error> {
        let reply = self.exchange(route).await?;
        self.budgets.check_bytes(&reply.body)?;

        if !(200..300).contains(&reply.status) {
            return Err(Error::Http {
                status: reply.status,
                body: reply.body,
            });
        }

        Ok(reply.body)
    }

    fn deadline(&self, delay: u64) -> Result<u64, Error> {
        self.clock
            .now()
            .checked_add(delay)
            .ok_or(Error::Clock(ClockError::DeadlineRange))
    }

    /// Deadline precedence is deterministic; the losing exchange is dropped before retry.
    async fn exchange(&self, route: &Route) -> Result<Reply, Error> {
        let deadline = self.deadline(self.policy.timeout_ms)?;
        let request = rest::Request {
            service: Service,
            method: rest::Method::Get,
            path_segments: route.path.clone(),
            query: route.query.clone(),
            body: rest::Body::Absent,
            max_reply_bytes: self.budgets.bytes,
        };
        let mut timer = pin!(self.clock.sleep_until(deadline));
        let mut exchange = pin!(self.transport.exchange(request));

        poll_fn(|context| {
            if let Poll::Ready(result) = timer.as_mut().poll(context) {
                return Poll::Ready(Err(elapsed(result)));
            }

            exchange
                .as_mut()
                .poll(context)
                .map(|result| result.map_err(Error::Transport))
        })
        .await
    }
}

/// Distinguishes a completed deadline from a timer owner that failed to run.
fn elapsed(result: Result<(), ClockError>) -> Error {
    match result {
        Ok(()) => Error::Transport(TransportError::new(TransportFailureKind::Deadline)),
        Err(error) => Error::Clock(error),
    }
}
