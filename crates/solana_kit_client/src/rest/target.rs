//! Relative request-target construction never selects a destination.

use crate::{TransportError, TransportFailureKind as Kind};

/// Appends decoded path segments and ordered query pairs to an approved root.
///
/// The root is an already validated relative HTTP path and optional fixed query.
/// Every segment, key and value is separately percent encoded. Repeated pairs
/// remain ordered. The limit applies before allocation or copying.
pub fn request_target(
    root: &str,
    segments: &[String],
    query: &[(String, String)],
    maximum: usize,
) -> Result<String, TransportError> {
    let (path, fixed_query) = root_parts(root)?;
    let mut output = Target {
        text: String::new(),
        maximum,
    };
    output.path(path, segments)?;
    output.query(fixed_query, query)?;

    Ok(output.text)
}

fn root_parts(root: &str) -> Result<(&str, Option<&str>), TransportError> {
    if !root.starts_with('/')
        || root.contains(['#', '\\'])
        || root
            .bytes()
            .any(|byte| byte.is_ascii_control() || byte == b' ')
    {
        return Err(TransportError::new(Kind::Denied));
    }

    Ok(root
        .split_once('?')
        .map_or((root, None), |(path, query)| (path, Some(query))))
}

struct Target {
    text: String,
    maximum: usize,
}

impl Target {
    // Dots must be rejected before encoding so server normalization cannot escape the root.
    fn path(&mut self, path: &str, segments: &[String]) -> Result<(), TransportError> {
        self.push(path)?;

        // Reject decoded dots so server normalization cannot escape the approved root.
        for (index, segment) in segments.iter().enumerate() {
            if segment == "." || segment == ".." {
                return Err(TransportError::new(Kind::Denied));
            }

            if index != 0 || !path.ends_with('/') {
                self.push("/")?;
            }

            self.encode(segment)?;
        }

        Ok(())
    }

    fn query(
        &mut self,
        fixed: Option<&str>,
        pairs: &[(String, String)],
    ) -> Result<(), TransportError> {
        if let Some(fixed) = fixed {
            self.push("?")?;
            self.push(fixed)?;
        }

        for (index, (key, value)) in pairs.iter().enumerate() {
            self.push(query_separator(index, fixed))?;
            self.encode(key)?;
            self.push("=")?;
            self.encode(value)?;
        }

        Ok(())
    }

    fn push(&mut self, text: &str) -> Result<(), TransportError> {
        const INITIAL_CAPACITY: usize = 16;
        const GROWTH_FACTOR: usize = 2;

        if text.len() > self.maximum.saturating_sub(self.text.len()) {
            return Err(TransportError::new(Kind::BodyLimit));
        }

        let required = self.text.len() + text.len();

        if required > self.text.capacity() {
            let capacity = self
                .text
                .capacity()
                .max(INITIAL_CAPACITY)
                .saturating_mul(GROWTH_FACTOR)
                .max(required)
                .min(self.maximum);
            self.text
                .try_reserve_exact(capacity - self.text.len())
                .map_err(|error| TransportError::with_source(Kind::BodyLimit, error))?;
        }

        self.text.push_str(text);

        Ok(())
    }

    fn encode(&mut self, text: &str) -> Result<(), TransportError> {
        for byte in text.bytes() {
            self.encode_byte(byte)?;
        }

        Ok(())
    }

    fn encode_byte(&mut self, byte: u8) -> Result<(), TransportError> {
        const HEX_DIGITS: usize = 16;
        const NIBBLE_BITS: u32 = 4;
        const LOW_NIBBLE: u8 = 15;
        const HEX: &[u8; HEX_DIGITS] = b"0123456789ABCDEF";

        if byte.is_ascii_alphanumeric() || b"-._~".contains(&byte) {
            return self.push(
                std::str::from_utf8(&[byte])
                    .map_err(|error| TransportError::with_source(Kind::Denied, error))?,
            );
        }

        let encoded = [
            b'%',
            HEX[usize::from(byte >> NIBBLE_BITS)],
            HEX[usize::from(byte & LOW_NIBBLE)],
        ];

        self.push(
            std::str::from_utf8(&encoded)
                .map_err(|error| TransportError::with_source(Kind::Denied, error))?,
        )
    }
}

fn query_separator(index: usize, fixed: Option<&str>) -> &'static str {
    match (index, fixed) {
        (0, None) => "?",
        (0, Some("")) => "",
        _ => "&",
    }
}
