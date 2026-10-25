//! Lexical budgets precede duplicate-aware syntax parsing and typed allocation.

use self::JsonError as Error;

/// A lexical budget or JSON syntax failure before typed decoding.
pub enum JsonError {
    /// Request or reply exceeded its byte budget.
    Bytes,
    /// Container nesting exceeded its budget.
    Depth,
    /// Values and object keys exceeded their aggregate budget.
    Items,
    /// Encoded string content exceeded its budget.
    String,
    /// Invalid JSON syntax or duplicate object keys.
    Schema(serde_json::Error),
}

impl std::fmt::Debug for JsonError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Bytes => "Bytes",
            Self::Depth => "Depth",
            Self::Items => "Items",
            Self::String => "String",
            Self::Schema(_) => "Schema",
        })
    }
}

impl std::fmt::Display for JsonError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "JSON admission failed: {self:?}")
    }
}

impl std::error::Error for JsonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Schema(error) => Some(error),
            _ => None,
        }
    }
}

/// Explicit request and response allocation budgets.
#[derive(Debug, Clone, Copy)]
pub struct Budgets {
    /// Maximum serialized request and received reply bytes.
    pub bytes: usize,
    /// Maximum nested object/array containers.
    pub depth: usize,
    /// Maximum JSON values plus object keys across one document.
    pub items: usize,
    /// Maximum encoded UTF-8 content bytes in one JSON string, including escapes.
    pub string_bytes: usize,
}

impl Default for Budgets {
    fn default() -> Self {
        const DOCUMENT_BYTES: usize = 1_048_576;
        const DOCUMENT_ITEMS: usize = 16_384;
        const STRING_BYTES: usize = 65_536;

        Self {
            bytes: DOCUMENT_BYTES,
            depth: 32,
            items: DOCUMENT_ITEMS,
            string_bytes: STRING_BYTES,
        }
    }
}

impl Budgets {
    /// Checks the byte ceiling without parsing or allocating a JSON value.
    pub fn check_bytes(self, bytes: &[u8]) -> Result<(), Error> {
        if bytes.len() > self.bytes {
            return Err(Error::Bytes);
        }

        Ok(())
    }

    /// Admits lexical bounds, syntax and unique object keys before typed decoding.
    pub fn validate_json(self, bytes: &[u8]) -> Result<(), Error> {
        self.check_bytes(bytes)?;
        let mut scanner = Scanner {
            limits: self,
            depth: 0,
            items: 0,
            token: Token::Outside,
        };

        for byte in bytes {
            scanner.byte(*byte)?;
        }

        crate::validate::unique(bytes).map_err(Error::Schema)
    }

    /// Serializes through a capped writer, then admits JSON before returning it.
    pub fn serialize(self, value: &impl serde::Serialize) -> Result<Vec<u8>, Error> {
        let mut output = LimitedOutput {
            bytes: Vec::new(),
            maximum: self.bytes,
            exceeded: false,
        };
        let result = serde_json::to_writer(&mut output, value);

        if output.exceeded {
            return Err(Error::Bytes);
        }

        result.map_err(Error::Schema)?;
        self.validate_json(&output.bytes)?;
        Ok(output.bytes)
    }
}

struct LimitedOutput {
    bytes: Vec<u8>,
    maximum: usize,
    exceeded: bool,
}

impl std::io::Write for LimitedOutput {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if bytes.len() > self.maximum.saturating_sub(self.bytes.len()) {
            self.exceeded = true;

            return Err(std::io::Error::other("request byte budget exceeded"));
        }

        self.bytes.extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Clone, Copy)]
enum Token {
    Outside,
    Literal,
    String { bytes: usize, escaped: bool },
}

struct Scanner {
    limits: Budgets,
    depth: usize,
    items: usize,
    token: Token,
}

impl Scanner {
    fn item(&mut self) -> Result<(), Error> {
        if self.items >= self.limits.items {
            return Err(Error::Items);
        }

        self.items += 1;
        Ok(())
    }

    fn byte(&mut self, byte: u8) -> Result<(), Error> {
        match self.token {
            Token::Outside => self.outside(byte),

            Token::Literal if byte.is_ascii_whitespace() || b",]}:".contains(&byte) => {
                self.token = Token::Outside;
                self.outside(byte)
            }

            Token::Literal => Ok(()),
            Token::String { bytes, escaped } => self.string(byte, bytes, escaped),
        }
    }

    fn string(&mut self, byte: u8, bytes: usize, escaped: bool) -> Result<(), Error> {
        if !escaped && byte == b'"' {
            self.token = Token::Outside;

            return Ok(());
        }

        if bytes >= self.limits.string_bytes {
            return Err(Error::String);
        }

        self.token = Token::String {
            bytes: bytes + 1,
            escaped: !escaped && byte == b'\\',
        };
        Ok(())
    }

    fn open_container(&mut self) -> Result<(), Error> {
        self.item()?;

        if self.depth >= self.limits.depth {
            return Err(Error::Depth);
        }

        self.depth += 1;
        Ok(())
    }

    fn outside(&mut self, byte: u8) -> Result<(), Error> {
        match byte {
            b'{' | b'[' => self.open_container()?,
            b'}' | b']' => self.depth = self.depth.saturating_sub(1),
            b'"' => {
                self.item()?;
                self.token = Token::String {
                    bytes: 0,
                    escaped: false,
                };
            }
            b',' | b':' | b' ' | b'\t' | b'\r' | b'\n' => {}
            _ => {
                self.item()?;
                self.token = Token::Literal;
            }
        }

        Ok(())
    }
}
