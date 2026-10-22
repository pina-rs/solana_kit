//! Rejected local erasure retained only as an executable type-system control.
use solana_kit_client::{
    Client, Error, Limits, Reply, Request, Transport, TransportError, methods::Method,
};
use std::{future::Future, pin::Pin};

/// Erasure loses Send even when the underlying implementation is native.
pub trait ErasedTransport {
    fn exchange(
        &self,
        request: Request,
    ) -> Pin<Box<dyn Future<Output = Result<Reply, TransportError>> + '_>>;
}

struct Erased<T>(T);
impl<T: ErasedTransport> Transport for Erased<T> {
    type Exchange<'a>
        = Pin<Box<dyn Future<Output = Result<Reply, TransportError>> + 'a>>
    where
        T: 'a;
    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        self.0.exchange(request)
    }
}

/// The alternate future representation still uses the public protocol owner.
pub struct ErasedClient<T>(pub T);
impl<T: ErasedTransport> ErasedClient<T> {
    pub async fn call<R: Method>(&self, method: R) -> Result<R::Response, Error> {
        Client::new(Erased(&self.0), Limits::default())
            .call(method)
            .await
    }
}
impl<T: ErasedTransport> ErasedTransport for &T {
    fn exchange(
        &self,
        request: Request,
    ) -> Pin<Box<dyn Future<Output = Result<Reply, TransportError>> + '_>> {
        T::exchange(self, request)
    }
}

/// A universal local box prevents spawning the same public algorithm natively.
///
/// ```compile_fail,E0277
/// use solana_kit_rpc_design::alternatives::{ErasedClient, ErasedTransport};
/// use solana_kit_rpc_design::methods::Balance;
/// fn spawn<T: ErasedTransport + Send + Sync + 'static>(client: ErasedClient<T>) {
///     tokio::spawn(async move {
///         client.call(Balance("11111111111111111111111111111111".parse().unwrap(), None)).await
///     });
/// }
/// ```
pub struct ErasedSendControl;
