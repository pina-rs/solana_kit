//! Small fixed binary envelope; destination policy never crosses from the guest.
use solana_kit_client::{Reply, Request, TransportError, TransportFailureKind as Kind};

const SUCCESS_TAG: u8 = 0;
const ERROR_TAG: u8 = 1;
const BODY_LIMIT: u8 = 5;
const HEADER_LIMIT: u8 = 6;
const INVALID_RESPONSE: u8 = 7;
#[cfg(all(feature = "guest", target_arch = "wasm32"))]
const HTTP_MAX_STATUS: u16 = 599;

pub(crate) const POST: leptoskit_bridge::MethodId = leptoskit_bridge::MethodId(0);

#[cfg(all(feature = "guest", target_arch = "wasm32"))]
pub(crate) fn request(request: Request) -> Result<Vec<u8>, TransportError> {
    let limit = u32::try_from(request.max_reply_bytes).map_err(|_| failure(Kind::BodyLimit))?;
    let mut bytes = Vec::with_capacity(request.body.len().saturating_add(4));
    bytes.extend_from_slice(&limit.to_le_bytes());
    bytes.extend_from_slice(&request.body);
    Ok(bytes)
}

#[cfg(feature = "host")]
pub(crate) fn parse_request(bytes: &[u8]) -> Result<Request, TransportError> {
    let budget: [u8; 4] = bytes
        .get(..4)
        .ok_or_else(|| failure(Kind::InvalidResponse))?
        .try_into()
        .map_err(|_| failure(Kind::InvalidResponse))?;
    Ok(Request {
        body: bytes[4..].to_vec(),
        max_reply_bytes: u32::from_le_bytes(budget) as usize,
    })
}

#[cfg(feature = "host")]
pub(crate) fn reply(reply: Result<Reply, TransportError>) -> Vec<u8> {
    match reply {
        Ok(reply) => {
            let mut bytes = Vec::with_capacity(reply.body.len().saturating_add(3));
            bytes.push(SUCCESS_TAG);
            bytes.extend_from_slice(&reply.status.to_le_bytes());
            bytes.extend_from_slice(&reply.body);
            bytes
        }
        Err(error) => vec![ERROR_TAG, kind_code(error.kind)],
    }
}

#[cfg(all(feature = "guest", target_arch = "wasm32"))]
pub(crate) fn parse_reply(bytes: &[u8], limit: usize) -> Result<Reply, TransportError> {
    if let [ERROR_TAG, code] = bytes {
        return Err(failure(code_kind(*code)?));
    }

    let [SUCCESS_TAG, low, high, body @ ..] = bytes else {
        return Err(failure(Kind::InvalidResponse));
    };
    let status = u16::from_le_bytes([*low, *high]);

    if !(100..=HTTP_MAX_STATUS).contains(&status) {
        return Err(failure(Kind::InvalidResponse));
    }

    if body.len() > limit {
        return Err(failure(Kind::BodyLimit));
    }

    Ok(Reply {
        status,
        body: body.to_vec(),
    })
}

#[cfg(all(feature = "guest", target_arch = "wasm32"))]
pub(crate) fn bridge_error(error: leptoskit_bridge::Error) -> TransportError {
    use leptoskit_bridge::Error as Bridge;

    let kind = match error {
        Bridge::Unavailable => Kind::Unavailable,
        Bridge::PermissionDenied => Kind::Denied,
        Bridge::Cancelled | Bridge::InvalidCall => Kind::Cancelled,
        Bridge::TooLarge | Bridge::Busy => Kind::BodyLimit,
        Bridge::InvalidArgument => Kind::InvalidResponse,
        Bridge::Native | Bridge::Exhausted => Kind::Io,
    };

    TransportError::with_source(kind, error)
}

fn failure(kind: Kind) -> TransportError {
    TransportError::new(kind)
}

#[cfg(feature = "host")]
fn kind_code(kind: Kind) -> u8 {
    match kind {
        Kind::Unavailable => 0,
        Kind::Denied => 1,
        Kind::Deadline => 2,
        Kind::Cancelled => 3,
        Kind::Io => 4,
        Kind::BodyLimit => BODY_LIMIT,
        Kind::HeaderLimit => HEADER_LIMIT,
        Kind::InvalidResponse => INVALID_RESPONSE,
    }
}

#[cfg(all(feature = "guest", target_arch = "wasm32"))]
fn code_kind(code: u8) -> Result<Kind, TransportError> {
    match code {
        0 => Ok(Kind::Unavailable),
        1 => Ok(Kind::Denied),
        2 => Ok(Kind::Deadline),
        3 => Ok(Kind::Cancelled),
        4 => Ok(Kind::Io),
        BODY_LIMIT => Ok(Kind::BodyLimit),
        HEADER_LIMIT => Ok(Kind::HeaderLimit),
        INVALID_RESPONSE => Ok(Kind::InvalidResponse),
        _ => Err(failure(Kind::InvalidResponse)),
    }
}
