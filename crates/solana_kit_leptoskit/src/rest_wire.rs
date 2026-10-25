//! Additive REST wire. Named host services select all destination policy.

use solana_kit_client::{TransportError, TransportFailureKind as Kind, rest};

pub(crate) const REST: leptoskit_bridge::MethodId = leptoskit_bridge::MethodId(1);
const VERSION: u8 = 1;

#[cfg(all(feature = "guest", target_arch = "wasm32"))]
pub(crate) fn encode(
    request: rest::Request<()>,
    maximum: usize,
) -> Result<Vec<u8>, TransportError> {
    let mut output = Writer {
        bytes: Vec::new(),
        maximum,
    };

    let method = match request.method {
        rest::Method::Get => 0,
        rest::Method::Post => 1,
        rest::Method::Put => 2,
        rest::Method::Patch => 3,
        rest::Method::Delete => 4,
    };

    let body = match &request.body {
        rest::Body::Absent => 0,
        rest::Body::Json(_) => 1,
        rest::Body::Form(_) => 2,
    };

    output.push(&[VERSION, method, body])?;
    output.number(request.max_reply_bytes)?;
    output.number(request.path_segments.len())?;

    for segment in request.path_segments {
        output.field(segment.as_bytes())?;
    }

    output.number(request.query.len())?;

    for (key, value) in request.query {
        output.field(key.as_bytes())?;
        output.field(value.as_bytes())?;
    }

    output.field(request.body.as_deref().unwrap_or(&[]))?;
    Ok(output.bytes)
}

#[cfg(all(feature = "guest", target_arch = "wasm32"))]
struct Writer {
    bytes: Vec<u8>,
    maximum: usize,
}

#[cfg(all(feature = "guest", target_arch = "wasm32"))]
impl Writer {
    fn push(&mut self, bytes: &[u8]) -> Result<(), TransportError> {
        if bytes.len() > self.maximum.saturating_sub(self.bytes.len()) {
            return Err(failure(Kind::BodyLimit));
        }

        self.bytes
            .try_reserve_exact(bytes.len())
            .map_err(|error| TransportError::with_source(Kind::BodyLimit, error))?;
        self.bytes.extend_from_slice(bytes);
        Ok(())
    }

    fn number(&mut self, value: usize) -> Result<(), TransportError> {
        let value = u32::try_from(value).map_err(|_| failure(Kind::BodyLimit))?;
        self.push(&value.to_le_bytes())
    }

    fn field(&mut self, bytes: &[u8]) -> Result<(), TransportError> {
        self.number(bytes.len())?;
        self.push(bytes)
    }
}

#[cfg(feature = "host")]
/// Validates the wire framing before constructing a request owned by the native service.
pub(crate) fn decode(bytes: &[u8]) -> Result<rest::Request<()>, TransportError> {
    let mut input = Reader(bytes);

    if input.byte()? != VERSION {
        return Err(failure(Kind::InvalidResponse));
    }

    let method = match input.byte()? {
        0 => rest::Method::Get,
        1 => rest::Method::Post,
        2 => rest::Method::Put,
        3 => rest::Method::Patch,
        4 => rest::Method::Delete,
        _ => return Err(failure(Kind::InvalidResponse)),
    };

    let representation = input.byte()?;

    if representation > 2 {
        return Err(failure(Kind::InvalidResponse));
    }

    let max_reply_bytes = input.number()?;
    let path_segments = input.path_segments()?;
    let query = input.query()?;

    let bytes = input.field()?;

    let body = match representation {
        0 if bytes.is_empty() => rest::Body::Absent,
        1 => rest::Body::Json(copy(bytes)?),
        2 => rest::Body::Form(copy(bytes)?),
        _ => return Err(failure(Kind::InvalidResponse)),
    };

    if !input.0.is_empty() {
        return Err(failure(Kind::InvalidResponse));
    }

    Ok(rest::Request {
        service: (),
        method,
        path_segments,
        query,
        body,
        max_reply_bytes,
    })
}

#[cfg(feature = "host")]
struct Reader<'a>(&'a [u8]);

#[cfg(feature = "host")]
impl<'a> Reader<'a> {
    fn path_segments(&mut self) -> Result<Vec<String>, TransportError> {
        const LENGTH_BYTES: usize = core::mem::size_of::<u32>();
        let count = self.count(LENGTH_BYTES)?;
        let mut segments = Vec::new();

        for _ in 0..count {
            segments
                .try_reserve(1)
                .map_err(|error| TransportError::with_source(Kind::BodyLimit, error))?;
            segments.push(self.text()?);
        }

        Ok(segments)
    }

    fn query(&mut self) -> Result<Vec<(String, String)>, TransportError> {
        const PAIR_LENGTH_BYTES: usize = 2 * core::mem::size_of::<u32>();
        let count = self.count(PAIR_LENGTH_BYTES)?;
        let mut pairs = Vec::new();

        for _ in 0..count {
            pairs
                .try_reserve(1)
                .map_err(|error| TransportError::with_source(Kind::BodyLimit, error))?;
            pairs.push((self.text()?, self.text()?));
        }

        Ok(pairs)
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], TransportError> {
        let bytes = self
            .0
            .get(..length)
            .ok_or_else(|| failure(Kind::InvalidResponse))?;
        self.0 = &self.0[length..];
        Ok(bytes)
    }

    fn byte(&mut self) -> Result<u8, TransportError> {
        Ok(self.take(1)?[0])
    }

    fn number(&mut self) -> Result<usize, TransportError> {
        let bytes = self
            .take(4)?
            .try_into()
            .map_err(|_| failure(Kind::InvalidResponse))?;
        usize::try_from(u32::from_le_bytes(bytes)).map_err(|_| failure(Kind::BodyLimit))
    }

    fn count(&mut self, minimum: usize) -> Result<usize, TransportError> {
        let count = self.number()?;

        if count > self.0.len() / minimum {
            return Err(failure(Kind::InvalidResponse));
        }

        Ok(count)
    }

    fn field(&mut self) -> Result<&'a [u8], TransportError> {
        let length = self.number()?;
        self.take(length)
    }

    fn text(&mut self) -> Result<String, TransportError> {
        let bytes = self.field()?;
        let text = std::str::from_utf8(bytes).map_err(|_| failure(Kind::InvalidResponse))?;
        let mut output = String::new();
        output
            .try_reserve_exact(text.len())
            .map_err(|error| TransportError::with_source(Kind::BodyLimit, error))?;
        output.push_str(text);
        Ok(output)
    }
}

#[cfg(feature = "host")]
fn copy(bytes: &[u8]) -> Result<Vec<u8>, TransportError> {
    let mut output = Vec::new();
    output
        .try_reserve_exact(bytes.len())
        .map_err(|error| TransportError::with_source(Kind::BodyLimit, error))?;
    output.extend_from_slice(bytes);
    Ok(output)
}

fn failure(kind: Kind) -> TransportError {
    TransportError::new(kind)
}
