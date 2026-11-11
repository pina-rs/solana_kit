//! Prover base URLs. An API key is never accepted in the URL.

use std::net::{Ipv4Addr, Ipv6Addr};

use crate::error::ProverFailure;

/// A prover base URL with no userinfo and no `api-key` query parameter.
#[derive(Clone, PartialEq, Eq)]
pub struct ProverEndpoint {
    origin: String,
    path: String,
    host: String,
}

impl ProverEndpoint {
    /// Parses `input`.
    ///
    /// # Errors
    /// Rejects a URL that is not `http` or `https`, has userinfo, or carries
    /// an `api-key` query parameter. The key belongs on the transport, as a
    /// header, and must not be written into a URL or an error.
    pub fn parse(input: &str) -> Result<Self, ProverFailure> {
        let (scheme, authority, tail) = split_url(input)?;
        let (path, host) = path_and_host(authority, tail)?;

        Ok(Self {
            origin: format!("{scheme}://{authority}"),
            path,
            host,
        })
    }

    /// `scheme://host[:port]`, with no path and no query.
    #[must_use]
    pub fn origin(&self) -> &str {
        &self.origin
    }

    /// Absolute path prefix, beginning with `/`.
    #[must_use]
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Whether requests to this host stay on the machine.
    ///
    /// Loopback is `localhost`, a name ending in `.localhost`, any address in
    /// `127.0.0.0/8`, and IPv6 `::1`.
    #[must_use]
    pub fn is_loopback(&self) -> bool {
        let host = self.host.trim_end_matches('.');

        if host.eq_ignore_ascii_case("localhost")
            || host.to_ascii_lowercase().ends_with(".localhost")
        {
            return true;
        }

        if let Ok(address) = host.parse::<Ipv4Addr>() {
            return address.is_loopback();
        }

        host.parse::<Ipv6Addr>()
            .is_ok_and(|address| address.is_loopback())
    }
}

impl std::fmt::Debug for ProverEndpoint {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ProverEndpoint")
            .field("origin", &self.origin)
            .field("path", &self.path)
            .finish()
    }
}

/// Splits a prover URL into scheme, authority, and the path tail.
///
/// Userinfo is rejected here so an API key cannot hide in the authority.
fn split_url(input: &str) -> Result<(&str, &str, &str), ProverFailure> {
    let Some((scheme, rest)) = input.split_once("://") else {
        return Err(ProverFailure::Config);
    };

    if scheme != "http" && scheme != "https" {
        return Err(ProverFailure::Config);
    }

    if rest.is_empty() || rest.contains([' ', '\n', '\r', '\t', '#', '\\']) {
        return Err(ProverFailure::Config);
    }

    let (authority, tail) = match rest.split_once('/') {
        Some((authority, tail)) => (authority, tail),
        None => (rest, ""),
    };

    if authority.is_empty() || authority.contains('@') {
        return Err(ProverFailure::Config);
    }

    Ok((scheme, authority, tail))
}

fn path_and_host(authority: &str, tail: &str) -> Result<(String, String), ProverFailure> {
    let (path, query) = match tail.split_once('?') {
        Some((path, query)) => (path, Some(query)),
        None => (tail, None),
    };

    if query.is_some_and(has_api_key) {
        return Err(ProverFailure::Config);
    }

    let host = host_name(authority).ok_or(ProverFailure::Config)?;
    let path = if path.is_empty() {
        "/".to_owned()
    } else {
        format!("/{path}")
    };

    Ok((path, host))
}

fn host_name(authority: &str) -> Option<String> {
    if let Some(host) = authority.strip_prefix('[') {
        let (host, _port) = host.split_once(']')?;

        if host.is_empty() {
            return None;
        }

        return Some(host.to_owned());
    }

    let host = authority
        .rsplit_once(':')
        .map(|(host, _)| host)
        .unwrap_or(authority);

    if host.is_empty() || host.contains(':') {
        return None;
    }

    Some(host.to_owned())
}

fn has_api_key(query: &str) -> bool {
    query.split('&').any(|pair| {
        pair.split_once('=')
            .map(|(name, _)| name.eq_ignore_ascii_case("api-key"))
            .unwrap_or(false)
    })
}

#[cfg(test)]
mod tests {
    use super::ProverEndpoint;

    #[test]
    fn rejected_urls_and_debug_stay_keyless() {
        assert!(ProverEndpoint::parse("notaurl").is_err());
        assert!(ProverEndpoint::parse("ftp://example.com").is_err());
        assert!(
            ProverEndpoint::parse("http://api.localhost")
                .expect("localhost suffix")
                .is_loopback()
        );
        assert!(ProverEndpoint::parse("http://").is_err());
        assert!(ProverEndpoint::parse("http://a@b").is_err());
        assert!(ProverEndpoint::parse("http:// ").is_err());
        assert!(
            ProverEndpoint::parse("http://[::1]")
                .expect("ipv6")
                .is_loopback()
        );
        assert!(
            !ProverEndpoint::parse("http://[2001:db8::1]")
                .expect("ipv6")
                .is_loopback()
        );
        assert!(ProverEndpoint::parse("http://[]").is_err());
        assert!(ProverEndpoint::parse("http://:80").is_err());
        let endpoint = ProverEndpoint::parse("http://127.0.0.1:3001").expect("loopback");
        assert!(format!("{endpoint:?}").contains("127.0.0.1"));
    }
}
