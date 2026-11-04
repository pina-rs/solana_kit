//! Explicit caller-owned remote association through one WSS reflector.
use super::{
    HELLO_BYTES, MwaConfig, MwaError, NativeMobileWallet, WalletSeed, establish_wallet, scalar,
    transport::{self, Connection},
};
use crate::{keys::NativeEntropy, websocket_connect};
use solana_kit_wallet::mwa::{self, AgreementKey, AssociationKey, ReflectorId};
use std::fmt;

const REFLECTOR_PATH: &str = "/reflect";
const MAX_VARINT_BYTES: usize = 10;
const MAX_REFLECTOR_ID_BYTES: usize = 8;
const BITS_PER_VARINT_BYTE: u32 = 7;
const BYTE_BASE: u64 = 256;

/// An explicitly selected WSS reflector authority.
///
/// The value is a bare `host` or `host:port`; schemes, userinfo, paths, queries,
/// fragments and noncanonical ports reject. The selected authority is both the
/// TLS destination and the exact `reflector` value in the returned association
/// URI. No default host or public reflector is installed.
#[derive(Clone, Eq, PartialEq)]
pub struct RemoteReflector {
    authority: String,
}

impl fmt::Debug for RemoteReflector {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RemoteReflector")
            .finish_non_exhaustive()
    }
}

impl RemoteReflector {
    /// Validates one reflector authority before any socket or TLS work.
    ///
    /// # Errors
    ///
    /// Returns [`MwaError::Reflector`] for an authority rejected by the existing
    /// remote association URI contract or unusable as a TLS destination.
    pub fn new(authority: impl Into<String>) -> Result<Self, MwaError> {
        let authority = authority.into();
        mwa::validate_reflector_authority(&authority).map_err(|_| MwaError::Reflector)?;
        let endpoint = format!("wss://{authority}{REFLECTOR_PATH}");
        let parsed = websocket_connect::endpoint(&endpoint).map_err(|_| MwaError::Reflector)?;

        if !parsed.secure || parsed.path != REFLECTOR_PATH || parsed.authority != authority {
            return Err(MwaError::Reflector);
        }

        Ok(Self { authority })
    }

    /// Returns the exact validated authority.
    #[must_use]
    pub fn authority(&self) -> &str {
        &self.authority
    }

    /// Connects once to `/reflect` and waits for the reflector's first ID frame.
    ///
    /// The returned pending owner retains the only socket, association key and
    /// agreement key. Dropping it, or dropping this future, closes the socket and
    /// erases the keys; no SDK-owned task, retry or fallback endpoint survives. The pending
    /// association URI is available before any wallet join or HELLO exchange.
    ///
    /// # Errors
    ///
    /// Returns a TLS, handshake, deadline, reflector ID or URI validation failure.
    /// Certificate chain and destination-name verification always stay enabled.
    pub async fn connect(
        self,
        mut entropy: NativeEntropy,
        config: MwaConfig,
    ) -> Result<PendingRemoteAssociation, MwaError> {
        config.validate()?;

        if config.socket.max_frame_bytes < HELLO_BYTES {
            return Err(MwaError::Capacity);
        }

        let association = scalar(&mut entropy, AssociationKey::from_scalar).await?;
        let agreement = scalar(&mut entropy, AgreementKey::from_scalar).await?;
        let RemoteReflector { authority } = self;
        let endpoint = format!("wss://{authority}{REFLECTOR_PATH}");
        let deadline = config.socket.connect_timeout;

        tokio::time::timeout(deadline, async move {
            let (connection, payload) =
                transport::connect_remote(&endpoint, &config.socket).await?;
            let id = decode_reflector_id(&payload)?;
            let uri = mwa::association_uri(
                &mwa::Association::Remote {
                    key: association.public_key(),
                    protocol: "v1".into(),
                    host: authority,
                    id,
                },
                None,
            )?;

            Ok(PendingRemoteAssociation {
                connection,
                association,
                agreement,
                entropy,
                config,
                uri,
                id,
            })
        })
        .await
        .map_err(MwaError::Deadline)?
    }
}

/// One reflector connection waiting for a wallet to join.
///
/// The caller displays [`Self::uri`], then awaits [`Self::join`]. The URI contains
/// the actual selected authority and the server-assigned reflector ID. Dropping
/// this owner or a pending join future closes its socket and key state.
pub struct PendingRemoteAssociation {
    connection: Connection,
    association: AssociationKey,
    agreement: AgreementKey,
    entropy: NativeEntropy,
    config: MwaConfig,
    uri: String,
    id: ReflectorId,
}

impl fmt::Debug for PendingRemoteAssociation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("PendingRemoteAssociation")
            .finish_non_exhaustive()
    }
}

impl PendingRemoteAssociation {
    /// Returns the validated `solana-wallet:` remote association URI to display.
    #[must_use]
    pub fn uri(&self) -> &str {
        &self.uri
    }

    /// Returns the server-assigned reflector ID.
    #[must_use]
    pub fn id(&self) -> ReflectorId {
        self.id
    }

    /// Waits for the wallet's empty join frame, sends HELLO, and returns the wallet.
    ///
    /// The join wait uses [`MwaConfig::join_timeout`]. HELLO and HELLO_RSP use the
    /// socket operation deadline. No HELLO is sent before the join frame arrives.
    /// An empty HELLO_RSP is invalid for remote associations, matching the pinned
    /// source, unlike the local APP_PING behavior.
    ///
    /// # Errors
    ///
    /// Returns a deadline, nonempty join, peer close, invalid HELLO_RSP or
    /// encryption/agreement failure. Cancelling this future drops all state.
    pub async fn join(self) -> Result<NativeMobileWallet, MwaError> {
        let Self {
            mut connection,
            association,
            agreement,
            entropy,
            config,
            ..
        } = self;
        let max_frame = config.socket.max_frame_bytes;
        let joined = tokio::time::timeout(
            config.join_timeout,
            transport::receive(&mut connection, max_frame),
        )
        .await
        .map_err(MwaError::Deadline)??;

        if !joined.is_empty() {
            return Err(MwaError::Reflector);
        }

        let operation_timeout = config.socket.operation_timeout;
        tokio::time::timeout(operation_timeout, async move {
            let hello = agreement.hello_request(&association).to_vec();
            transport::send(&mut connection, hello, max_frame).await?;
            let response = transport::receive(&mut connection, max_frame).await?;

            if response.is_empty() {
                return Err(MwaError::Reflector);
            }

            establish_wallet(
                connection,
                WalletSeed {
                    association,
                    agreement,
                    entropy,
                    config,
                },
                &response,
            )
        })
        .await
        .map_err(MwaError::Deadline)?
    }
}

/// Decodes the reflector's LEB128-prefixed ID payload.
///
/// The source accepts arbitrary payload bytes and only later constrains the
/// numeric ID. Rust retains [`ReflectorId`]'s existing JavaScript-safe integer
/// contract, so a raw ID wider than eight bytes or above 2^53 - 1 rejects, and
/// the URI uses the canonical big-endian integer token rather than leading zero
/// bytes.
pub(super) fn decode_reflector_id(payload: &[u8]) -> Result<ReflectorId, MwaError> {
    let (length, offset) = decode_var_uint_length(payload)?;
    let end = offset.checked_add(length).ok_or(MwaError::Reflector)?;

    if length == 0 || end > payload.len() {
        return Err(MwaError::Reflector);
    }

    let bytes = &payload[offset..end];

    if bytes.len() > MAX_REFLECTOR_ID_BYTES {
        return Err(MwaError::Reflector);
    }

    let value = bytes
        .iter()
        .try_fold(0_u64, |value, byte| {
            value.checked_mul(BYTE_BASE)?.checked_add(u64::from(*byte))
        })
        .ok_or(MwaError::Reflector)?;

    ReflectorId::new(value).map_err(|_| MwaError::Reflector)
}

fn decode_var_uint_length(payload: &[u8]) -> Result<(usize, usize), MwaError> {
    let mut value = 0_u64;
    let mut shift = 0_u32;
    let mut offset = 0;

    loop {
        if offset >= payload.len() || offset >= MAX_VARINT_BYTES {
            return Err(MwaError::Reflector);
        }

        let byte = payload[offset];
        offset += 1;
        let chunk = u64::from(byte & 0x7f);
        let shifted = chunk
            .checked_shl(shift)
            .filter(|shifted| shifted >> shift == chunk)
            .ok_or(MwaError::Reflector)?;
        value = value.checked_add(shifted).ok_or(MwaError::Reflector)?;

        if byte & 0x80 == 0 {
            break;
        }

        shift = shift
            .checked_add(BITS_PER_VARINT_BYTE)
            .ok_or(MwaError::Reflector)?;
    }

    let length = usize::try_from(value).map_err(|_| MwaError::Reflector)?;

    Ok((length, offset))
}
