use super::{Error, PublicPoint};
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use base64::{
    Engine,
    engine::general_purpose::{URL_SAFE, URL_SAFE_NO_PAD},
};
use iri_string::types::UriStr;

const MIN_PORT: u16 = 49152;
const MAX_PORT: u16 = 65535;
const BYTE_BITS: u32 = 8;
const HEX_RADIX: u32 = 16;

/// A dynamic/private local association port.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AssociationPort(u16);

impl AssociationPort {
    /// Accepts only the inclusive range 49152 through 65535.
    pub fn new(port: u32) -> Result<Self, Error> {
        match u16::try_from(port) {
            Ok(port @ MIN_PORT..=MAX_PORT) => Ok(Self(port)),
            _ => Err(Error::Association),
        }
    }

    /// Returns the validated port.
    pub fn value(self) -> u16 {
        self.0
    }

    /// Maps 14 uniformly random caller-provided bits to the complete port range.
    pub fn from_entropy(bytes: [u8; 2]) -> Self {
        Self(MIN_PORT + (u16::from_be_bytes(bytes) & 0x3fff))
    }
}

/// A reflector ID representable by a JavaScript safe integer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReflectorId(u64);

impl ReflectorId {
    fn token(self) -> String {
        let bytes = self.value().to_be_bytes();
        let first = bytes
            .iter()
            .position(|byte| *byte != 0)
            .unwrap_or(bytes.len() - 1);
        URL_SAFE_NO_PAD.encode(&bytes[first..])
    }

    /// Accepts values from zero through 2^53 - 1.
    pub fn new(id: u64) -> Result<Self, Error> {
        if id > 9_007_199_254_740_991 {
            return Err(Error::Association);
        }

        Ok(Self(id))
    }

    /// Returns the validated integer.
    pub fn value(self) -> u64 {
        self.0
    }
}

/// A local association or remote reflector endpoint, with parsed protocol text.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Association {
    /// Local transport selected by the platform adapter.
    Local {
        key: PublicPoint,
        protocol: String,
        port: AssociationPort,
    },
    /// Remote transport selected by the platform adapter.
    Remote {
        key: PublicPoint,
        protocol: String,
        host: String,
        id: ReflectorId,
    },
}

/// Builds a wallet association URI. An optional wallet prefix must use HTTPS.
pub fn association_uri(
    association: &Association,
    wallet_base: Option<&str>,
) -> Result<String, Error> {
    let (kind, key, protocol, endpoint) = match association {
        Association::Local {
            key,
            protocol,
            port,
        } => ("local", key, protocol, format!("port={}", port.value())),
        Association::Remote {
            key,
            protocol,
            host,
            id,
        } => {
            validate_host(host)?;
            (
                "remote",
                key,
                protocol,
                format!("reflector={}&id={}", escape(host), id.token()),
            )
        }
    };

    let prefix = wallet_prefix(wallet_base)?;

    Ok(format!(
        "{prefix}/v1/associate/{kind}?association={}&{endpoint}&v={}",
        key.association_token(),
        escape(protocol)
    ))
}

fn wallet_prefix(wallet_base: Option<&str>) -> Result<String, Error> {
    let prefix = match wallet_base {
        None => String::from("solana-wallet:"),
        Some(base) => {
            let uri = parse_uri(base)?;

            if !uri.scheme_str().eq_ignore_ascii_case("https") || uri.authority_str().is_none() {
                return Err(Error::Association);
            }

            base.split(['?', '#'])
                .next()
                .ok_or(Error::Association)?
                .trim_end_matches('/')
                .to_string()
        }
    };

    Ok(prefix)
}

/// Parses only recognized association paths and validates the public key and endpoint.
pub fn parse_association_uri(input: &str) -> Result<Association, Error> {
    let uri = parse_uri(input)?;
    let path = uri.path_str();
    let query = uri.query_str().ok_or(Error::Association)?;
    let pairs = query_pairs(query)?;
    let get = |name: &str| query_value(&pairs, name);
    let key = PublicPoint::from_bytes(&base64url(get("association")?)?)?;
    let protocol = if pairs.iter().any(|(key, _)| key == "v") {
        get("v")?.into()
    } else {
        "v1".into()
    };

    if path.ends_with("/v1/associate/local") {
        let port = get("port")?.parse().map_err(|_| Error::Association)?;
        return Ok(Association::Local {
            key,
            protocol,
            port: AssociationPort::new(port)?,
        });
    }

    if !path.ends_with("/v1/associate/remote") {
        return Err(Error::Association);
    }

    let host = get("reflector")?.to_string();
    validate_host(&host)?;
    let id = parse_reflector(get("id")?)?;
    Ok(Association::Remote {
        key,
        protocol,
        host,
        id,
    })
}

fn query_pairs(input: &str) -> Result<Vec<(String, String)>, Error> {
    input
        .split('&')
        .map(|pair| {
            let (key, value) = pair.split_once('=').ok_or(Error::Association)?;
            Ok((decode(key)?, decode(value)?))
        })
        .collect()
}

fn query_value<'a>(pairs: &'a [(String, String)], name: &str) -> Result<&'a str, Error> {
    let mut matches = pairs.iter().filter(|(key, _)| key == name);
    let value = matches.next().ok_or(Error::Association)?;

    if matches.next().is_some() {
        return Err(Error::Association);
    }

    Ok(&value.1)
}

fn parse_reflector(input: &str) -> Result<ReflectorId, Error> {
    let bytes = base64url(input)?;

    if bytes.len() > core::mem::size_of::<u64>() {
        return Err(Error::Association);
    }

    let id = bytes
        .iter()
        .fold(0_u64, |value, byte| (value << BYTE_BITS) | u64::from(*byte));
    ReflectorId::new(id)
}

fn validate_host(host: &str) -> Result<(), Error> {
    let input = format!("https://{host}/");
    let uri = parse_uri(&input)?;
    let authority = uri.authority_components().ok_or(Error::Association)?;

    if host.is_empty()
        || uri.authority_str() != Some(host)
        || authority.userinfo().is_some()
        || uri.path_str() != "/"
        || uri.query_str().is_some()
        || uri.fragment_str().is_some()
    {
        return Err(Error::Association);
    }

    canonical_authority(authority.host(), authority.port())
}

fn parse_uri(input: &str) -> Result<&UriStr, Error> {
    let uri = UriStr::new(input).map_err(|_| Error::Association)?;

    if let Some(authority) = uri.authority_components() {
        // RFC 3986 permits arbitrary digit ports and IPvFuture. The existing
        // protocol boundary accepts only u16 ports and supported IPv6 literals.
        port_value(authority.port())?;
        ipv6_value(authority.host())?;
    }

    Ok(uri)
}

fn port_value(port: Option<&str>) -> Result<Option<u16>, Error> {
    port.filter(|port| !port.is_empty())
        .map(str::parse)
        .transpose()
        .map_err(|_| Error::Association)
}

fn ipv6_value(host: &str) -> Result<Option<core::net::Ipv6Addr>, Error> {
    host.strip_prefix('[')
        .map(|address| address.trim_end_matches(']').parse())
        .transpose()
        .map_err(|_| Error::Association)
}

fn canonical_authority(host: &str, port: Option<&str>) -> Result<(), Error> {
    // Preserve the original authority round-trip rule: no port or IPv6 rewriting.
    let canonical_port = port_value(port)?.map(|port| port.to_string());
    let canonical_host = ipv6_value(host)?.map(|address| format!("[{address}]"));

    if canonical_port.as_deref() != port {
        return Err(Error::Association);
    }

    if canonical_host.as_deref().is_some_and(|value| value != host) {
        return Err(Error::Association);
    }

    Ok(())
}

fn base64url(value: &str) -> Result<Vec<u8>, Error> {
    let trimmed = value.trim_end_matches('=');

    if value.len() - trimmed.len() > 2 {
        return Err(Error::Association);
    }

    URL_SAFE_NO_PAD
        .decode(trimmed)
        .or_else(|_| URL_SAFE.decode(value))
        .map_err(|_| Error::Association)
}

fn escape(value: &str) -> String {
    let mut result = String::new();

    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(char::from(byte))
            }
            b' ' => result.push('+'),
            _ => result.push_str(&format!("%{byte:02X}")),
        }
    }

    result
}

fn decode(value: &str) -> Result<String, Error> {
    let mut result = Vec::new();
    let mut bytes = value.bytes();

    while let Some(byte) = bytes.next() {
        match byte {
            b'+' => result.push(b' '),
            b'%' => {
                let high = char::from(bytes.next().ok_or(Error::Association)?)
                    .to_digit(HEX_RADIX)
                    .ok_or(Error::Association)?;
                let low = char::from(bytes.next().ok_or(Error::Association)?)
                    .to_digit(HEX_RADIX)
                    .ok_or(Error::Association)?;
                result.push(u8::try_from(high * HEX_RADIX + low).map_err(|_| Error::Association)?);
            }
            _ => result.push(byte),
        }
    }

    String::from_utf8(result).map_err(|_| Error::Association)
}
