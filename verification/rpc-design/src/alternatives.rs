//! Rejected erased-future candidate retained only as an executable design control.

use std::future::Future;
use std::pin::Pin;

use crate::json::{self, Error, Limits};
use crate::methods::Method;
use crate::{Reply, Request, TransportError};

/// Erasure loses Send even when the implementation stores only native Send state.
pub trait ErasedTransport {
    fn exchange(
        &self,
        request: Request,
    ) -> Pin<Box<dyn Future<Output = Result<Reply, TransportError>> + '_>>;
}

/// This actual alternate client shares the JSON boundary with candidate A.
pub struct ErasedClient<T>(pub T);

impl<T: ErasedTransport> ErasedClient<T> {
    pub async fn call<R: Method>(&self, id: u64, method: R) -> Result<R::Response, Error> {
        let limits = Limits::default();
        let request = json::request(id, &method, limits)?;
        let reply = self.0.exchange(request).await?;
        json::response(id, reply, limits)
    }
}

/// A universal local box prevents spawning the same algorithm natively.
///
/// ```compile_fail,E0277
/// use solana_kit_rpc_design::alternatives::{ErasedClient, ErasedTransport};
/// use solana_kit_rpc_design::methods::Balance;
/// use solana_kit_rpc_design::json::Error;
/// fn spawn<T: ErasedTransport + Send + Sync + 'static>(client: ErasedClient<T>) {
///     tokio::spawn(async move {
///         client.call(1, Balance("11111111111111111111111111111111".parse().unwrap(), None)).await
///     });
/// }
/// ```
pub struct ErasedSendControl;
