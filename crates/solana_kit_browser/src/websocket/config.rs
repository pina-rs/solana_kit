//! Validated browser socket boundary.
use super::WsError;
use std::{fmt, time::Duration};
use web_sys::Url;
const DEFAULT_MESSAGE_BYTES: usize = 256 * 1024;
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_PROTOCOLS: usize = 16;
const MAX_PROTOCOL_BYTES: usize = 128;
const MAX_ENDPOINT_BYTES: usize = 8192;

/// Explicit receive, send and operation budgets. No arbitrary headers are supported.
#[derive(Clone)]
pub struct WsConfig {
    /// Browser subprotocol offers, at most 16 unique HTTP tokens of 128 bytes each.
    pub protocols: Vec<String>,
    /// Maximum queued complete text messages.
    pub queue_messages: usize,
    /// Maximum retained UTF-8 bytes across the receive queue.
    pub queue_bytes: usize,
    /// Maximum encoded bytes in one incoming or outgoing text message.
    pub message_bytes: usize,
    /// Maximum browser bufferedAmount after accepting an outgoing message.
    pub buffered_bytes: u32,
    /// Positive per-operation deadline, at most i32::MAX milliseconds.
    pub operation_timeout: Duration,
}
impl Default for WsConfig {
    fn default() -> Self {
        Self {
            protocols: Vec::new(),
            queue_messages: 64,
            queue_bytes: 1024 * 1024,
            message_bytes: DEFAULT_MESSAGE_BYTES,
            buffered_bytes: 1024 * 1024,
            operation_timeout: DEFAULT_TIMEOUT,
        }
    }
}
impl fmt::Debug for WsConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WsConfig").finish_non_exhaustive()
    }
}
pub fn validate(config: &WsConfig, endpoint: &str) -> Result<String, WsError> {
    let millis = config.operation_timeout.as_millis();

    if millis == 0 || millis > i32::MAX as u128 {
        return Err(WsError::Config);
    }

    budgets(config)?;
    protocols(&config.protocols)?;
    endpoint_url(endpoint)
}
fn budgets(config: &WsConfig) -> Result<(), WsError> {
    if config.queue_messages == 0
        || config.message_bytes == 0
        || config.queue_bytes < config.message_bytes
        || config.message_bytes > config.buffered_bytes as usize
    {
        return Err(WsError::Config);
    }

    Ok(())
}
fn protocols(protocols: &[String]) -> Result<(), WsError> {
    if protocols.len() > MAX_PROTOCOLS {
        return Err(WsError::Config);
    }

    for (index, protocol) in protocols.iter().enumerate() {
        if !valid_protocol(protocol) || protocols[..index].contains(protocol) {
            return Err(WsError::Config);
        }
    }

    Ok(())
}
fn valid_protocol(protocol: &str) -> bool {
    !protocol.is_empty() && protocol.len() <= MAX_PROTOCOL_BYTES && protocol.bytes().all(token)
}
fn token(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(&byte)
}
fn endpoint_url(endpoint: &str) -> Result<String, WsError> {
    spelling(endpoint)?;
    let url = Url::new(endpoint).map_err(|_| WsError::Config)?;

    if url.host().is_empty() || url.port() == "0" {
        return Err(WsError::Config);
    }

    Ok(url.href())
}
fn spelling(endpoint: &str) -> Result<(), WsError> {
    let forbidden_character = endpoint.chars().any(forbidden);

    if endpoint.len() > MAX_ENDPOINT_BYTES || forbidden_character {
        return Err(WsError::Config);
    }

    let (scheme, rest) = endpoint.split_once("://").ok_or(WsError::Config)?;
    let authority = rest.split(['/', '?']).next().ok_or(WsError::Config)?;

    if (!scheme.eq_ignore_ascii_case("ws") && !scheme.eq_ignore_ascii_case("wss"))
        || authority.is_empty()
        || authority.contains('@')
    {
        return Err(WsError::Config);
    }

    Ok(())
}
fn forbidden(character: char) -> bool {
    const FORBIDDEN: [char; 4] = ['#', '\\', ' ', '\u{7f}'];
    character.is_ascii_control() || FORBIDDEN.contains(&character)
}
