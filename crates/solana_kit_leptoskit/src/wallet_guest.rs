//! Guest session authority is a real pending bridge call, cancelled on drop.
use crate::{GuestWalletPlatform, WalletPlatformError, wallet_wire as wire};
use leptoskit_bridge::{
    Error, Event, MethodId,
    guest::{Call, Client, Events},
};
use serde_json::{Map, Value};
use solana_kit_core::{Address, Signature};
use solana_kit_wallet::mwa::{Method, ProtocolError};
use std::{
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll, ready},
};

/// Guest failures preserve the exact bridge, Android or wallet-reported cause.
pub enum WalletSessionError {
    /// Original service or host failure code; original native causes stay native.
    Bridge(Error),
    /// Original Android launch/availability failure.
    Platform(WalletPlatformError),
    /// The wallet explicitly rejected the operation.
    Rejected(ProtocolError),
    /// A bounded reply did not match its contract.
    Malformed,
    /// A method is outside the concrete association contract.
    Method,
}

impl fmt::Debug for WalletSessionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("WalletSessionError(REDACTED)")
    }
}

impl fmt::Display for WalletSessionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("wallet session request failed")
    }
}

impl std::error::Error for WalletSessionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Bridge(error) => Some(error),
            Self::Platform(error) => Some(error),
            _ => None,
        }
    }
}

struct Lease {
    client: Client,
    call: Call,
}

fn opened(
    event: &Event,
    client: Client,
    call: leptoskit_bridge::CallId,
) -> Result<Option<Vec<u8>>, WalletSessionError> {
    if event.plugin != client.plugin_id() || event.kind != wire::OPENED {
        return Ok(None);
    }

    let bytes = event.payload.as_ref();
    let prefix = bytes
        .get(..wire::CALL_BYTES)
        .ok_or(WalletSessionError::Malformed)?;

    if prefix != call.0.to_le_bytes() {
        return Ok(None);
    }

    Ok(Some(bytes[wire::CALL_BYTES..].to_vec()))
}

fn poll_opened(
    next: &mut leptoskit_bridge::guest::NextEvent<'_>,
    route: &mut impl FnMut(Event) -> Result<(), Error>,
    client: Client,
    call: leptoskit_bridge::CallId,
    context: &mut Context<'_>,
) -> Poll<Result<Vec<u8>, WalletSessionError>> {
    for _ in 0..wire::POLL_BUDGET {
        let event =
            ready!(Pin::new(&mut *next).poll(context)).map_err(WalletSessionError::Bridge)?;

        match opened(&event, client, call)? {
            Some(bytes) => return Poll::Ready(Ok(bytes)),
            None => route(event).map_err(WalletSessionError::Bridge)?,
        }
    }

    context.waker().wake_by_ref();
    Poll::Pending
}

impl Lease {
    /// Retain the pending lease before reading events, so cancellation during
    /// platform handover also disposes the native association or local key.
    async fn start(
        method: u32,
        payload: &[u8],
        events: &mut Events,
        mut route: impl FnMut(Event) -> Result<(), Error>,
    ) -> Result<(Self, Vec<u8>), WalletSessionError> {
        let client = Client::new("solana-wallet-session").map_err(WalletSessionError::Bridge)?;
        let mut call = client
            .call(MethodId(method), payload)
            .map_err(WalletSessionError::Bridge)?;
        let id = call.id().ok_or(WalletSessionError::Malformed)?;
        let mut next = events.recv();
        let bytes = std::future::poll_fn(|context| {
            if let Poll::Ready(result) = Pin::new(&mut call).poll(context) {
                return Poll::Ready(Err(result
                    .err()
                    .map(WalletSessionError::Bridge)
                    .unwrap_or(WalletSessionError::Malformed)));
            }

            poll_opened(&mut next, &mut route, client, id, context)
        })
        .await?;
        Ok((Self { client, call }, bytes))
    }

    async fn request(
        &mut self,
        method: u32,
        payload: &[u8],
    ) -> Result<Vec<u8>, WalletSessionError> {
        let mut request = self
            .client
            .call(MethodId(method), payload)
            .map_err(WalletSessionError::Bridge)?;
        std::future::poll_fn(|context| {
            let lease = Pin::new(&mut self.call).poll(context);
            let reply = Pin::new(&mut request).poll(context);

            match (lease, reply) {
                (Poll::Pending, reply) => reply.map(|result| {
                    result
                        .map(|bytes| bytes.as_ref().to_vec())
                        .map_err(WalletSessionError::Bridge)
                }),
                (Poll::Ready(_), Poll::Ready(Err(error))) => {
                    Poll::Ready(Err(WalletSessionError::Bridge(error)))
                }
                (Poll::Ready(result), _) => Poll::Ready(Err(result
                    .err()
                    .map(WalletSessionError::Bridge)
                    .unwrap_or(WalletSessionError::Malformed))),
            }
        })
        .await
    }

    async fn close(self) -> Result<(), WalletSessionError> {
        self.client
            .call(MethodId(wire::CLOSE), &[])
            .map_err(WalletSessionError::Bridge)?
            .await
            .map_err(WalletSessionError::Bridge)?;
        Ok(())
    }
}

/// Android-only association, real platform handover and one native binary owner.
/// Dropping this owner, including while Android launch awaits, cancels its lease.
pub struct GuestMobileWallet(Lease);

impl GuestMobileWallet {
    /// Starts after a user gesture; caller owns the singleton event router.
    pub async fn connect_android(
        events: &mut Events,
        route: impl FnMut(Event) -> Result<(), Error>,
    ) -> Result<Self, WalletSessionError> {
        let platform = GuestWalletPlatform::new().map_err(WalletSessionError::Platform)?;

        if !platform
            .available()
            .await
            .map_err(WalletSessionError::Platform)?
        {
            return Err(WalletSessionError::Bridge(Error::Unavailable));
        }

        let (mut lease, bytes) = Lease::start(wire::BEGIN_MOBILE, &[], events, route).await?;
        let uri = std::str::from_utf8(&bytes).map_err(|_| WalletSessionError::Malformed)?;
        platform
            .launch(uri)
            .await
            .map_err(WalletSessionError::Platform)?;
        lease.request(wire::CONNECT_MOBILE, &[]).await?;
        Ok(Self(lease))
    }

    /// Invokes only implemented MWA methods, preserving explicit wallet rejection.
    pub async fn call(
        &mut self,
        method: Method,
        params: Map<String, Value>,
    ) -> Result<Map<String, Value>, WalletSessionError> {
        let method = match method {
            Method::Authorize => wire::AUTHORIZE,
            Method::SignTransactions => wire::SIGN_TRANSACTIONS,
            Method::SignMessages => wire::SIGN_MESSAGES,
            Method::SignAndSendTransactions => wire::SEND_TRANSACTIONS,
            Method::Deauthorize => wire::DEAUTHORIZE,
            _ => return Err(WalletSessionError::Method),
        };

        let payload = serde_json::to_vec(&params).map_err(|_| WalletSessionError::Malformed)?;
        let bytes = self.0.request(method, &payload).await?;
        let mut envelope: Map<String, Value> =
            serde_json::from_slice(&bytes).map_err(|_| WalletSessionError::Malformed)?;

        if let Some(error) = envelope.remove("error") {
            let error = serde_json::from_value(error).map_err(|_| WalletSessionError::Malformed)?;

            return Err(WalletSessionError::Rejected(error));
        }

        match envelope.remove("result") {
            Some(Value::Object(value)) => Ok(value),
            _ => Err(WalletSessionError::Malformed),
        }
    }

    /// Awaits protocol close and native owner completion before releasing its lease.
    pub async fn close(self) -> Result<(), WalletSessionError> {
        self.0.close().await
    }
}

/// An explicitly selected local signer. Secret bytes never enter the guest.
pub struct GuestLocalWallet {
    lease: Lease,
    address: Address,
}

impl GuestLocalWallet {
    /// Generates a new in-memory key from the real host entropy provider.
    pub async fn create(
        events: &mut Events,
        route: impl FnMut(Event) -> Result<(), Error>,
    ) -> Result<Self, WalletSessionError> {
        Self::open(wire::CREATE_LOCAL, &[], events, route).await
    }

    /// Loads a private previously saved wallet only after a user requests it.
    pub async fn load(
        events: &mut Events,
        route: impl FnMut(Event) -> Result<(), Error>,
    ) -> Result<Self, WalletSessionError> {
        Self::open(wire::LOAD_LOCAL, &[0], events, route).await
    }

    /// Restores the intentionally saved private copy without overwriting the
    /// primary saved wallet. This still depends on the app sandbox surviving.
    pub async fn restore_backup(
        events: &mut Events,
        route: impl FnMut(Event) -> Result<(), Error>,
    ) -> Result<Self, WalletSessionError> {
        Self::open(wire::LOAD_LOCAL, &[1], events, route).await
    }

    async fn open(
        method: u32,
        payload: &[u8],
        events: &mut Events,
        route: impl FnMut(Event) -> Result<(), Error>,
    ) -> Result<Self, WalletSessionError> {
        let (lease, bytes) = Lease::start(method, payload, events, route).await?;
        let address = std::str::from_utf8(&bytes)
            .map_err(|_| WalletSessionError::Malformed)?
            .parse()
            .map_err(|_| WalletSessionError::Malformed)?;
        Ok(Self { lease, address })
    }

    /// Current public signing account.
    pub fn address(&self) -> Address {
        self.address
    }

    /// Persists a private app-sandbox key file, never overwriting an existing file.
    pub async fn save(&mut self) -> Result<(), WalletSessionError> {
        self.lease.request(wire::SAVE_LOCAL, &[0]).await.map(|_| ())
    }

    /// Creates a separate private copy. This stays inside the app sandbox and is
    /// not an off-device recovery backup. The guest never receives private bytes.
    pub async fn backup(&mut self) -> Result<(), WalletSessionError> {
        self.lease.request(wire::SAVE_LOCAL, &[1]).await.map(|_| ())
    }

    /// Owns the wallet while awaiting approval and signing, so cancelling this
    /// future disposes its native lease even before a signing call was made.
    /// Rejection returns the unchanged wallet and no signature.
    pub async fn sign_with_approval(
        mut self,
        message: &[u8],
        approval: impl Future<Output = bool>,
    ) -> Result<(Self, Option<Signature>), WalletSessionError> {
        if !approval.await {
            return Ok((self, None));
        }

        let signature = self.sign_approved(message).await?;
        Ok((self, Some(signature)))
    }

    pub(crate) async fn sign_approved(
        &mut self,
        message: &[u8],
    ) -> Result<Signature, WalletSessionError> {
        let bytes = self.lease.request(wire::SIGN_LOCAL, message).await?;
        std::str::from_utf8(&bytes)
            .map_err(|_| WalletSessionError::Malformed)?
            .parse()
            .map_err(|_| WalletSessionError::Malformed)
    }

    /// Ends local authority and awaits native erasure before guest replacement.
    pub async fn close(self) -> Result<(), WalletSessionError> {
        self.lease.close().await
    }
}
