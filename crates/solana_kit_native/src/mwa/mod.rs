//! Caller-owned local binary MWA transport. Android launch remains an explicit
//! platform action. No runtime or background task is created by this module.
mod error;
#[cfg(test)]
mod tests;
mod transport;

use crate::{WsConfig, keys::NativeEntropy, websocket_connect::Socket};
pub use error::MwaError;
use serde_json::{Map, Value};
use solana_kit_core::crypto::Entropy;
use solana_kit_wallet::mwa::{
    self, AgreementKey, AssociationKey, AssociationPort, Method, Request, Response, Session,
    SessionProperties,
};
use zeroize::Zeroizing;

const MAX_REQUESTS: usize = 4096;
const HELLO_BYTES: usize = 129;
const FRAME_OVERHEAD: usize = 32;
const MAX_APP_PINGS: usize = 32;

/// Finite policy for one MWA connection. Outbound headers are deliberately rejected.
#[derive(Clone, Debug)]
pub struct MwaConfig {
    /// Shared bounded WebSocket parser and connect/operation deadlines.
    pub socket: WsConfig,
    /// Maximum encrypted calls before a new association is required.
    pub max_requests: usize,
    /// Deadline for wallet authorization and signing, including user approval.
    /// Defaults to five minutes, independently of the socket operation deadline.
    pub approval_timeout: std::time::Duration,
}

impl Default for MwaConfig {
    fn default() -> Self {
        Self {
            socket: WsConfig {
                connect_timeout: std::time::Duration::from_millis(u64::from(
                    mwa::CONNECTION_TIMEOUT_MS,
                )),
                ..WsConfig::default()
            },
            max_requests: 128,
            approval_timeout: std::time::Duration::from_secs(300),
        }
    }
}

/// Inert association material. Keys and provider state have no Debug or export API.
/// Generate this after user approval, launch `uri`, then consume it in `connect`.
pub struct NativeAssociation {
    association: AssociationKey,
    agreement: AgreementKey,
    port: AssociationPort,
    entropy: NativeEntropy,
}

impl NativeAssociation {
    /// Creates fresh association/agreement scalars and a private-range port.
    pub async fn generate(mut entropy: NativeEntropy) -> Result<Self, MwaError> {
        let association = scalar(&mut entropy, AssociationKey::from_scalar).await?;
        let agreement = scalar(&mut entropy, AgreementKey::from_scalar).await?;
        let mut bytes = [0; 2];
        entropy.fill(&mut bytes).await.map_err(MwaError::Entropy)?;
        Ok(Self {
            association,
            agreement,
            port: AssociationPort::from_entropy(bytes),
            entropy,
        })
    }

    /// Returns the selected local wallet port.
    pub fn port(&self) -> AssociationPort {
        self.port
    }

    /// Produces the real Android association URI, optionally under a wallet HTTPS base.
    pub fn uri(&self, wallet_base: Option<&str>) -> Result<String, MwaError> {
        mwa::association_uri(
            &mwa::Association::Local {
                key: self.association.public_key(),
                protocol: "v1".into(),
                port: self.port,
            },
            wallet_base,
        )
        .map_err(Into::into)
    }

    /// Connects only to the selected loopback port and performs the binary HELLO.
    /// Only TCP refusal before HELLO is retried, within the total connect deadline.
    /// Cancelling drops the socket and all private material; no task is detached.
    pub async fn connect(self, config: MwaConfig) -> Result<NativeMobileWallet, MwaError> {
        if config.max_requests == 0 || config.max_requests > MAX_REQUESTS {
            return Err(MwaError::Capacity);
        }

        let deadline = config.socket.connect_timeout;
        tokio::time::timeout(deadline, self.establish(config))
            .await
            .map_err(MwaError::Deadline)?
    }

    async fn establish(mut self, config: MwaConfig) -> Result<NativeMobileWallet, MwaError> {
        if config.socket.max_frame_bytes < HELLO_BYTES {
            return Err(MwaError::Capacity);
        }

        let mut socket = transport::connect(self.port, &config.socket).await?;
        let hello = self.hello(&mut socket).await?;
        let (key, properties) = self
            .agreement
            .accept_hello(&hello, &self.association.public_key())?;
        let incoming = if properties.is_some() { 0 } else { 1 };
        let mut session = Session::new(key, 1, incoming);
        let mut nonces = Vec::new();
        nonces
            .try_reserve_exact(config.max_requests * 2 + 1)
            .map_err(|_| MwaError::Capacity)?;

        let properties = match properties {
            Some(bytes) => {
                remember(&mut nonces, &bytes)?;
                session.properties(&bytes)?
            }
            None => SessionProperties {
                protocol_version: mwa::ProtocolVersion::Legacy,
            },
        };

        Ok(NativeMobileWallet {
            active: Some(Active {
                socket,
                session,
                nonces,
            }),
            config,
            properties,
            entropy: self.entropy,
            next_id: 1,
        })
    }

    // An APP_PING restarts HELLO with a fresh agreement key, within the same deadline.
    async fn hello(&mut self, socket: &mut Socket) -> Result<Vec<u8>, MwaError> {
        for _ in 0..MAX_APP_PINGS {
            let request = self.agreement.hello_request(&self.association).to_vec();
            transport::send(socket, request).await?;
            let response = transport::receive(socket).await?;

            if !response.is_empty() {
                return Ok(response);
            }

            self.agreement = scalar(&mut self.entropy, AgreementKey::from_scalar).await?;
        }

        Err(MwaError::Capacity)
    }
}

async fn scalar<T>(
    entropy: &mut NativeEntropy,
    create: fn(&[u8; 32]) -> Result<T, mwa::Error>,
) -> Result<T, MwaError> {
    let mut bytes = Zeroizing::new([0; 32]);

    for _ in 0..16 {
        entropy
            .fill(bytes.as_mut())
            .await
            .map_err(MwaError::Entropy)?;

        if let Ok(value) = create(&bytes) {
            return Ok(value);
        }
    }

    Err(MwaError::Scalar)
}

struct Active {
    socket: Socket,
    session: Session,
    nonces: Vec<[u8; 12]>,
}

/// One unique, bounded MWA session. Borrowed call cancellation closes local
/// authority and erases the session instead of retaining a half-written request.
pub struct NativeMobileWallet {
    active: Option<Active>,
    config: MwaConfig,
    properties: SessionProperties,
    entropy: NativeEntropy,
    next_id: u32,
}

impl NativeMobileWallet {
    /// Returns the authenticated negotiated version.
    pub fn properties(&self) -> SessionProperties {
        self.properties
    }

    /// Sends exactly once and validates its correlated encrypted response.
    /// A wallet-reported rejection is returned as Response::Error, preserving its data.
    pub async fn call(
        &mut self,
        method: Method,
        params: Map<String, Value>,
    ) -> Result<Response, MwaError> {
        if usize::try_from(self.next_id).map_err(|_| MwaError::Capacity)? > self.config.max_requests
        {
            return Err(MwaError::Capacity);
        }

        let deadline = match method {
            Method::GetCapabilities | Method::Deauthorize => self.config.socket.operation_timeout,
            Method::Authorize
            | Method::Reauthorize
            | Method::SignTransactions
            | Method::SignMessages
            | Method::SignAndSendTransactions
            | Method::CloneAuthorization => self.config.approval_timeout,
        };
        let request = Request::new(
            self.next_id,
            method,
            params,
            self.properties.protocol_version,
        )?;
        self.next_id = self.next_id.checked_add(1).ok_or(MwaError::Capacity)?;
        let mut active = self.active.take().ok_or(MwaError::Closed)?;
        let operation = active.exchange(
            &request,
            &mut self.entropy,
            self.config.socket.max_frame_bytes,
        );
        let result = tokio::time::timeout(deadline, operation)
            .await
            .map_err(MwaError::Deadline)??;
        self.active = Some(active);
        Ok(result)
    }

    /// Consumes this owner and awaits a bounded peer close. Drop closes locally.
    pub async fn close(mut self) -> Result<(), MwaError> {
        let Some(mut active) = self.active.take() else {
            return Ok(());
        };
        tokio::time::timeout(self.config.socket.operation_timeout, async {
            transport::close(&mut active.socket).await
        })
        .await
        .map_err(MwaError::Deadline)?
    }
}

impl Active {
    async fn exchange(
        &mut self,
        request: &Request,
        entropy: &mut NativeEntropy,
        max_frame: usize,
    ) -> Result<Response, MwaError> {
        let message = Zeroizing::new(request.to_json()?);

        if message
            .len()
            .checked_add(FRAME_OVERHEAD)
            .is_none_or(|size| size > max_frame)
        {
            return Err(MwaError::Capacity);
        }

        let mut nonce = [0; 12];
        entropy.fill(&mut nonce).await.map_err(MwaError::Entropy)?;
        remember_iv(&mut self.nonces, nonce)?;
        transport::send(&mut self.socket, self.session.encrypt(&message, &nonce)?).await?;
        let frame = transport::receive_encrypted(&mut self.socket).await?;
        remember(&mut self.nonces, &frame)?;
        let plaintext = Zeroizing::new(self.session.decrypt(&frame)?);
        Response::from_json(&plaintext, request.id()).map_err(Into::into)
    }
}

fn remember(nonces: &mut Vec<[u8; 12]>, frame: &[u8]) -> Result<(), MwaError> {
    let nonce = frame.get(4..16).ok_or(mwa::Error::Authentication)?;
    remember_iv(
        nonces,
        nonce.try_into().map_err(|_| mwa::Error::Authentication)?,
    )
}

fn remember_iv(nonces: &mut Vec<[u8; 12]>, iv: [u8; 12]) -> Result<(), MwaError> {
    if nonces.contains(&iv) || nonces.len() == nonces.capacity() {
        return Err(MwaError::Capacity);
    }

    nonces.push(iv);
    Ok(())
}
