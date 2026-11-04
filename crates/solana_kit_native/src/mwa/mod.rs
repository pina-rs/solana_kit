//! Caller-owned local and remote Mobile Wallet Adapter transports. Android
//! launch remains an explicit platform action. No runtime or background task
//! is created by this module.
mod error;
mod remote;
#[cfg(test)]
mod tests;
mod transport;

use crate::{ConfigError, WsConfig, keys::NativeEntropy};
pub use error::MwaError;
pub use remote::{PendingRemoteAssociation, RemoteReflector};
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
const DEFAULT_USER_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(300);

/// Finite policy for one MWA connection. Outbound headers are deliberately rejected.
#[derive(Clone, Debug)]
pub struct MwaConfig {
    /// Shared bounded WebSocket parser, TLS roots and connect/operation deadlines.
    pub socket: WsConfig,
    /// Maximum encrypted calls before a new association is required.
    pub max_requests: usize,
    /// Deadline for waiting for a remote wallet to join the reflector.
    /// Defaults to five minutes, independently of the socket operation deadline.
    pub join_timeout: std::time::Duration,
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
            join_timeout: DEFAULT_USER_TIMEOUT,
            approval_timeout: DEFAULT_USER_TIMEOUT,
        }
    }
}

impl MwaConfig {
    fn validate(&self) -> Result<(), MwaError> {
        if self.max_requests == 0 || self.max_requests > MAX_REQUESTS {
            return Err(MwaError::Capacity);
        }

        for duration in [
            self.socket.connect_timeout,
            self.socket.operation_timeout,
            self.join_timeout,
            self.approval_timeout,
        ] {
            if duration.is_zero() || std::time::Instant::now().checked_add(duration).is_none() {
                return Err(MwaError::Config(ConfigError(
                    "MWA timeouts must be positive and representable",
                )));
            }
        }

        Ok(())
    }
}

/// Inert local association material. Keys and provider state have no Debug or export API.
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
        config.validate()?;
        let deadline = config.socket.connect_timeout;
        tokio::time::timeout(deadline, self.establish(config))
            .await
            .map_err(MwaError::Deadline)?
    }

    async fn establish(self, config: MwaConfig) -> Result<NativeMobileWallet, MwaError> {
        if config.socket.max_frame_bytes < HELLO_BYTES {
            return Err(MwaError::Capacity);
        }

        let Self {
            association,
            mut agreement,
            port,
            mut entropy,
        } = self;
        let mut connection = transport::connect(port, &config.socket).await?;
        let hello = hello(
            &mut connection,
            &mut agreement,
            &association,
            &mut entropy,
            config.socket.max_frame_bytes,
        )
        .await?;
        let seed = WalletSeed {
            association,
            agreement,
            entropy,
            config,
        };

        establish_wallet(connection, seed, &hello)
    }
}

/// The private inputs shared by local and remote session establishment.
struct WalletSeed {
    association: AssociationKey,
    agreement: AgreementKey,
    entropy: NativeEntropy,
    config: MwaConfig,
}

// An APP_PING restarts HELLO with a fresh agreement key, within the same deadline.
async fn hello(
    connection: &mut transport::Connection,
    agreement: &mut AgreementKey,
    association: &AssociationKey,
    entropy: &mut NativeEntropy,
    max_frame: usize,
) -> Result<Vec<u8>, MwaError> {
    for _ in 0..MAX_APP_PINGS {
        let request = agreement.hello_request(association).to_vec();
        transport::send(connection, request, max_frame).await?;
        let response = transport::receive(connection, max_frame).await?;

        if !response.is_empty() {
            return Ok(response);
        }

        *agreement = scalar(entropy, AgreementKey::from_scalar).await?;
    }

    Err(MwaError::Capacity)
}

fn establish_wallet(
    connection: transport::Connection,
    seed: WalletSeed,
    hello: &[u8],
) -> Result<NativeMobileWallet, MwaError> {
    let WalletSeed {
        association,
        agreement,
        entropy,
        config,
    } = seed;
    let (key, properties) = agreement.accept_hello(hello, &association.public_key())?;
    // A real wallet consumes sequence 1 for HELLO_RSP session properties.
    // Receive therefore starts at 1, leaving 2 for its first actual reply.
    // The client's first request still uses sequence 1.
    let mut session = Session::new(key, 1, 1);
    let capacity = config
        .max_requests
        .checked_mul(2)
        .and_then(|value| value.checked_add(1))
        .ok_or(MwaError::Capacity)?;
    let mut nonces = Vec::new();
    nonces
        .try_reserve_exact(capacity)
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
            connection,
            session,
            nonces,
        }),
        config,
        properties,
        entropy,
        next_id: 1,
    })
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
    connection: transport::Connection,
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
            transport::close(&mut active.connection).await
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
        transport::send(
            &mut self.connection,
            self.session.encrypt(&message, &nonce)?,
            max_frame,
        )
        .await?;
        let frame = transport::receive_encrypted(&mut self.connection, max_frame).await?;
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
