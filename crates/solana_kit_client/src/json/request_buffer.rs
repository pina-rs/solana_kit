//! Keep both request-serialization buffers independent of rejected input size.

use std::io::{self, Write};

use serde::Serialize;

use crate::Error;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Root {
    Unknown,
    Container,
    Scalar,
}

impl Root {
    fn observe(&mut self, bytes: &[u8]) {
        if *self != Self::Unknown {
            return;
        }

        *self = match bytes.first() {
            Some(b'[' | b'{') => Self::Container,
            Some(_) => Self::Scalar,
            None => Self::Unknown,
        };
    }
}

struct Buffer {
    bytes: Vec<u8>,
    limit: usize,
    root: Root,
    exceeded: bool,
}

impl Buffer {
    fn finish(self, result: serde_json::Result<()>) -> Result<Vec<u8>, Error> {
        if self.exceeded {
            return Err(Error::BodyLimit);
        }

        result?;

        if self.root != Root::Container {
            return Err(Error::Envelope);
        }

        Ok(self.bytes)
    }
}

impl Write for Buffer {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.root.observe(bytes);

        // Invalid scalar output needs no buffer. Let the producer finish so its
        // own serialization error still precedes the client's envelope error.
        if self.root != Root::Container {
            return Ok(bytes.len());
        }

        if self.exceeded || bytes.len() > self.limit.saturating_sub(self.bytes.len()) {
            self.exceeded = true;

            return Err(io::Error::other("request byte budget exceeded"));
        }

        self.bytes.extend_from_slice(bytes);

        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Serializes a container with a byte cap before retaining any oversized output.
pub(super) fn encode(value: &impl Serialize, limit: usize) -> Result<Vec<u8>, Error> {
    let mut buffer = Buffer {
        bytes: Vec::new(),
        limit,
        root: Root::Unknown,
        exceeded: false,
    };
    let result = serde_json::to_writer(&mut buffer, value);

    buffer.finish(result)
}
