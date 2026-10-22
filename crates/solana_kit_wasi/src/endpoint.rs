//! A conservative explicit HTTP endpoint. Credentials never come from the URL.

/// Validated explicit HTTP(S) endpoint without URL credentials or fragments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Endpoint {
    pub(crate) secure: bool,
    pub(crate) authority: String,
    pub(crate) path: String,
}

impl Endpoint {
    /// Parses a conservative ASCII endpoint with explicit scheme and host.
    ///
    /// # Errors
    /// Rejects credentials, fragments, whitespace, invalid ports and authorities.
    pub fn parse(text: &str) -> Result<Self, &'static str> {
        if !text.is_ascii()
            || text.bytes().any(|byte| byte <= 32 || byte == 127)
            || text.contains(['#', '@', '\\'])
        {
            return Err("invalid endpoint characters");
        }
        let (secure, rest) = if let Some(rest) = text.strip_prefix("https://") {
            (true, rest)
        } else if let Some(rest) = text.strip_prefix("http://") {
            (false, rest)
        } else {
            return Err("explicit http or https scheme required");
        };

        let split = rest.find(['/', '?']).unwrap_or(rest.len());
        let authority = &rest[..split];
        let suffix = &rest[split..];
        let (host, port) = if authority.starts_with('[') {
            let end = authority.find(']').ok_or("invalid IPv6 authority")?;
            authority[1..end]
                .parse::<std::net::Ipv6Addr>()
                .map_err(|_| "invalid IPv6")?;
            let port = authority[end + 1..].strip_prefix(':');

            if end + 1 != authority.len() && port.is_none() {
                return Err("invalid IPv6 port separator");
            }

            (&authority[..=end], port)
        } else {
            let (host, port) = authority
                .split_once(':')
                .map_or((authority, None), |(host, port)| (host, Some(port)));

            if !host
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-'))
            {
                return Err("invalid DNS or IPv4 host");
            }
            (host, port)
        };

        if host.is_empty() {
            return Err("empty host");
        }

        if let Some(port) = port
            && (port.is_empty()
                || !port.bytes().all(|byte| byte.is_ascii_digit())
                || port.parse::<u16>().is_err())
        {
            return Err("invalid port");
        }
        let path = if suffix.is_empty() {
            "/".into()
        } else if suffix.starts_with('?') {
            format!("/{suffix}")
        } else {
            suffix.into()
        };

        Ok(Self {
            secure,
            authority: authority.into(),
            path,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Endpoint;

    #[test]
    fn explicit_endpoints_and_ports_are_validated() {
        for endpoint in [
            "http://localhost",
            "https://node.example/rpc?q=7",
            "http://[::1]:8899/rpc",
            "http://127.0.0.1:0?x=y",
        ] {
            assert!(Endpoint::parse(endpoint).is_ok(), "{endpoint}");
        }

        for endpoint in [
            "localhost",
            "ftp://host",
            "http://",
            "http://user:pass@host",
            "http://host#fragment",
            "http://host/path\r\nx: y",
            "http://host:+80",
            "http://host:",
            "http://host:65536",
            "http://[::1]junk",
            "http://[invalid]",
            "http://x\\evil",
            "http://x:8:9",
            "https://nonascii-☃/",
        ] {
            assert!(Endpoint::parse(endpoint).is_err(), "{endpoint}");
        }

        for port in ["0", "1", "80", "443", "65535"] {
            assert!(Endpoint::parse(&format!("http://host:{port}/")).is_ok());
            assert!(Endpoint::parse(&format!("http://host:+{port}/")).is_err());
            assert!(Endpoint::parse(&format!("http://[::1]:+{port}/")).is_err());
        }
    }
}
