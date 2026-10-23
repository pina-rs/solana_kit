//! Explicit public context and bounded source-template interpolation.
use super::ErrorCode;
use alloc::{collections::BTreeMap, format, string::String, vec::Vec};
use core::fmt::{self, Write};

const MAX_FACTS: usize = 64;
const MAX_KEY: usize = 64;
const MAX_TEXT: usize = 1024;
const MAX_DEPTH: usize = 8;
const MAX_NODES: usize = 256;
const MAX_STORAGE: usize = 16384;
const MAX_MESSAGE: usize = 4096;

/// Public scalar facts. Opaque payloads and secret bytes are deliberately absent.
#[derive(Clone, Eq, PartialEq)]
pub enum ContextValue {
    /// An explicit null fact, distinct from an absent key.
    Null,
    /// A signed exact integer.
    Integer(i128),
    /// An unsigned exact integer.
    Unsigned(u128),
    /// A boolean fact.
    Boolean(bool),
    /// Public diagnostic text explicitly supplied by the caller.
    Text(String),
    /// A typed list of public facts.
    List(Vec<ContextValue>),
    /// A typed nested object of public facts.
    Object(BTreeMap<String, ContextValue>),
}

impl fmt::Display for ContextValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => formatter.write_str("null"),
            Self::Integer(value) => value.fmt(formatter),
            Self::Unsigned(value) => value.fmt(formatter),
            Self::Boolean(value) => value.fmt(formatter),
            Self::Text(value) => value.fmt(formatter),
            Self::List(values) => format_items(values.iter(), formatter),
            Self::Object(values) => format_object(values, formatter),
        }
    }
}
impl fmt::Debug for ContextValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ContextValue(<redacted>)")
    }
}

/// Immutable-by-borrow public context with validated insertion and bounded output.
#[derive(Clone, Default, Eq, PartialEq)]
pub struct Context(BTreeMap<String, ContextValue>);

/// Context construction or rendering exceeded its explicit contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContextError {
    /// At most 64 facts, 64 ASCII key bytes and 1024 text bytes are accepted.
    Limit,
    /// Keys must be ASCII words and may not designate secret input.
    Key,
    /// An exact numeric conversion failed.
    Number,
    /// Base64, percent escapes or UTF-8 were malformed.
    Encoding,
}
impl fmt::Display for ContextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "diagnostic context error: {self:?}")
    }
}
impl core::error::Error for ContextError {}

impl Context {
    /// Adds or replaces a public fact after validating its key and storage bound.
    pub fn insert(&mut self, key: &str, value: ContextValue) -> Result<(), ContextError> {
        validate_key(key, &value)?;
        let mut nodes = 0;
        let mut storage = key.len();

        validate_value(&value, 0, &mut nodes, &mut storage)?;

        for (existing, value) in &self.0 {
            if existing != key {
                storage = storage
                    .checked_add(existing.len())
                    .ok_or(ContextError::Limit)?;
                validate_value(value, 0, &mut nodes, &mut storage)?;
            }
        }

        if key.len() > MAX_KEY || self.0.len() >= MAX_FACTS && !self.0.contains_key(key) {
            return Err(ContextError::Limit);
        }

        self.0.insert(key.into(), value);
        Ok(())
    }
    /// Removes null top-level facts using the source helper's normalization rule.
    #[must_use]
    pub fn normalized(mut self) -> Self {
        self.0.retain(|_, value| *value != ContextValue::Null);
        self
    }
    /// Returns a public fact without copying it.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&ContextValue> {
        self.0.get(key)
    }
    /// Iterates public facts in deterministic key order.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &ContextValue)> {
        self.0.iter().map(|(key, value)| (key.as_str(), value))
    }
    /// Adds an infallible numeric fact under a compile-time internal key.
    pub(crate) fn number(mut self, key: &'static str, value: impl Into<ContextValue>) -> Self {
        self.0.insert(key.into(), value.into());
        self
    }
    pub(super) fn render(&self, code: ErrorCode) -> Result<String, ContextError> {
        let template = code
            .definition()
            .map_or("", |definition| definition.template);

        if template.is_empty() {
            return Ok(format!("Solana error #{}", code.value()));
        }

        interpolate(template, self)
    }
}
impl fmt::Debug for Context {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Context")
            .field("entries", &self.0.len())
            .finish_non_exhaustive()
    }
}
impl From<u8> for ContextValue {
    fn from(value: u8) -> Self {
        Self::Unsigned(value.into())
    }
}
impl From<usize> for ContextValue {
    fn from(value: usize) -> Self {
        Self::Unsigned(value as u128)
    }
}
impl From<i64> for ContextValue {
    fn from(value: i64) -> Self {
        Self::Integer(value.into())
    }
}

fn interpolate(template: &str, context: &Context) -> Result<String, ContextError> {
    let mut output = String::new();
    let mut characters = template.chars().peekable();

    while let Some(character) = characters.next() {
        match character {
            '\\' => output.push(characters.next().unwrap_or(character)),
            '$' => interpolate_value(&mut characters, context, &mut output)?,
            _ => output.push(character),
        }

        if output.len() > MAX_MESSAGE {
            return Err(ContextError::Limit);
        }
    }

    Ok(output)
}

fn validate_value(
    value: &ContextValue,
    depth: usize,
    nodes: &mut usize,
    storage: &mut usize,
) -> Result<(), ContextError> {
    *nodes = nodes.checked_add(1).ok_or(ContextError::Limit)?;
    let text_bytes = match value {
        ContextValue::Text(text) => text.len(),
        _ => 0,
    };
    *storage = storage.checked_add(text_bytes).ok_or(ContextError::Limit)?;

    if depth > MAX_DEPTH || *nodes > MAX_NODES || *storage > MAX_STORAGE {
        return Err(ContextError::Limit);
    }

    match value {
        ContextValue::Text(text) if text.len() > MAX_TEXT => Err(ContextError::Limit),
        ContextValue::List(values) => validate_children(values.iter(), depth, nodes, storage),
        ContextValue::Object(values) => validate_object(values, depth, nodes, storage),
        _ => Ok(()),
    }
}

fn format_items<'a>(
    values: impl Iterator<Item = &'a ContextValue>,
    formatter: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    formatter.write_str("[")?;

    for (index, value) in values.enumerate() {
        if index != 0 {
            formatter.write_str(", ")?;
        }

        write!(formatter, "{value}")?;
    }

    formatter.write_str("]")
}
fn format_object(
    values: &BTreeMap<String, ContextValue>,
    formatter: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    formatter.write_str("{")?;

    for (index, (key, value)) in values.iter().enumerate() {
        if index != 0 {
            formatter.write_str(", ")?;
        }

        write!(formatter, "{key}: {value}")?;
    }

    formatter.write_str("}")
}
fn validate_key(key: &str, value: &ContextValue) -> Result<(), ContextError> {
    if key.len() > MAX_KEY {
        return Err(ContextError::Limit);
    }

    let lower = key.to_ascii_lowercase().replace('_', "");
    let secret = [
        "private",
        "secret",
        "credential",
        "authorization",
        "password",
    ]
    .iter()
    .any(|part| lower.contains(part))
        || matches!(
            lower.as_str(),
            "token" | "accesstoken" | "refreshtoken" | "idtoken" | "apikey"
        );
    let seed_text = lower.contains("seed")
        && matches!(
            value,
            ContextValue::Text(_) | ContextValue::List(_) | ContextValue::Object(_)
        );
    let valid = !key.is_empty()
        && key
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_');

    if !valid || secret || seed_text {
        return Err(ContextError::Key);
    }

    Ok(())
}
fn validate_children<'a>(
    values: impl Iterator<Item = &'a ContextValue>,
    depth: usize,
    nodes: &mut usize,
    storage: &mut usize,
) -> Result<(), ContextError> {
    for value in values {
        validate_value(value, depth + 1, nodes, storage)?;
    }

    Ok(())
}
fn validate_object(
    values: &BTreeMap<String, ContextValue>,
    depth: usize,
    nodes: &mut usize,
    storage: &mut usize,
) -> Result<(), ContextError> {
    // Every object entry needs a value node before key normalization can be bounded.
    if values.len() > MAX_NODES - *nodes {
        return Err(ContextError::Limit);
    }

    for (key, value) in values {
        validate_key(key, value)?;
        *storage = storage.checked_add(key.len()).ok_or(ContextError::Limit)?;

        if *storage > MAX_STORAGE {
            return Err(ContextError::Limit);
        }
    }

    validate_children(values.values(), depth, nodes, storage)
}
fn interpolate_value(
    characters: &mut core::iter::Peekable<core::str::Chars<'_>>,
    context: &Context,
    output: &mut String,
) -> Result<(), ContextError> {
    let name: String = core::iter::from_fn(|| {
        characters.next_if(|character| character.is_ascii_alphanumeric() || *character == '_')
    })
    .collect();

    if let Some(value) = context.get(&name) {
        return write!(LimitedWriter(output), "{value}").map_err(|_| ContextError::Limit);
    }

    output.push('$');
    output.push_str(&name);
    Ok(())
}

struct LimitedWriter<'a>(&'a mut String);

impl Write for LimitedWriter<'_> {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        if self.0.len() + text.len() > MAX_MESSAGE {
            return Err(fmt::Error);
        }

        self.0.push_str(text);
        Ok(())
    }
}
