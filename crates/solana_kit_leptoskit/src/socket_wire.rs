//! Bounded socket envelopes contain only host-issued correlation and text.
use leptoskit_bridge::{CallId, MethodId};

pub const OPEN: MethodId = MethodId(1);
pub const SEND: MethodId = MethodId(2);
pub const CLOSE: MethodId = MethodId(3);
pub const EVENT: u32 = 1;
pub const OPENED: u8 = 1;
pub const TEXT: u8 = 2;
pub const ENDED: u8 = 3;
/// Additive binary event kind. Existing method IDs and event kinds are unchanged.
pub const BINARY: u8 = 4;
pub const CALL_BYTES: usize = 8;

pub const HEADER_BYTES: usize = CALL_BYTES + 1;
#[cfg(feature = "host")]
const PROTOCOL_ERROR: u8 = 5;

/// Redacted guest transport failure; peer data and host policy are never retained.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketFailure {
    /// The host bridge refused or lost this capability.
    Bridge,
    /// A frame, queue or connection budget was exceeded.
    Limit,
    /// Native socket or protocol I/O failed.
    Io,
    /// The socket ended or the caller cancelled uncertain work.
    Closed,
    /// The versioned envelope or socket state was invalid.
    Protocol,
}

impl std::fmt::Display for SocketFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "guest socket failure: {self:?}")
    }
}

impl std::error::Error for SocketFailure {}

pub fn envelope(call: CallId, kind: u8, body: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(HEADER_BYTES + body.len());
    bytes.extend_from_slice(&call.0.to_le_bytes());
    bytes.push(kind);
    bytes.extend_from_slice(body);
    bytes
}

pub fn parse(bytes: &[u8]) -> Result<(CallId, u8, &[u8]), SocketFailure> {
    let id = bytes.get(..CALL_BYTES).ok_or(SocketFailure::Protocol)?;
    let id = u64::from_le_bytes(id.try_into().map_err(|_| SocketFailure::Protocol)?);
    let kind = *bytes.get(CALL_BYTES).ok_or(SocketFailure::Protocol)?;
    Ok((CallId(id), kind, &bytes[HEADER_BYTES..]))
}

#[cfg(feature = "host")]
pub fn reply(result: Result<(), SocketFailure>) -> Vec<u8> {
    vec![match result {
        Ok(()) => 0,
        Err(SocketFailure::Bridge) => 1,
        Err(SocketFailure::Limit) => 2,
        Err(SocketFailure::Io) => 3,
        Err(SocketFailure::Closed) => 4,
        Err(SocketFailure::Protocol) => PROTOCOL_ERROR,
    }]
}

#[cfg(all(feature = "guest", target_arch = "wasm32"))]
pub fn parse_reply(bytes: &[u8]) -> Result<(), SocketFailure> {
    match bytes {
        [0] => Ok(()),
        [1] => Err(SocketFailure::Bridge),
        [2] => Err(SocketFailure::Limit),
        [3] => Err(SocketFailure::Io),
        [4] => Err(SocketFailure::Closed),
        _ => Err(SocketFailure::Protocol),
    }
}
