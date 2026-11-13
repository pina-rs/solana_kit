//! Prover base URLs. An API key is never accepted in the URL: any query is
//! refused outright, so a key cannot survive in the origin or a request
//! target, not even before the first path segment.

use std::net::{Ipv4Addr, Ipv6Addr};

use crate::error::ProverFailure;

/// A prover base URL with no userinfo, no query, and no fragment.
///
/// The shape is `scheme://host[:port][/base-path]`. Every query is rejected
/// instead of silently dropped, so a key can never ride in the origin.
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
    /// Rejects a URL that is not `http` or `https`, has userinfo, carries a
    /// query or a fragment, or whose host, port, or bracket tail does not
    /// parse. The key belongs on the transport, as a header, and must not be
    /// written into a URL or an error.
    pub fn parse(input: &str) -> Result<Self, ProverFailure> {
        let (scheme, authority, tail) = split_url(input)?;
        let host = host_name(authority).ok_or(ProverFailure::Config)?;
        let path = if tail.is_empty() {
            "/".to_owned()
        } else {
            format!("/{tail}")
        };

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
/// Userinfo and query or fragment syntax are rejected here so a key or a
/// marker cannot hide in the authority, before or after the first path
/// segment.
fn split_url(input: &str) -> Result<(&str, &str, &str), ProverFailure> {
    let Some((scheme, rest)) = input.split_once("://") else {
        return Err(ProverFailure::Config);
    };

    if scheme != "http" && scheme != "https" {
        return Err(ProverFailure::Config);
    }

    // Security: a '?' that precedes the first '/' stays inside the authority
    // of a spec-compliant URI parser, so every query is refused instead of
    // split off and silently dropped from the origin.
    if rest.is_empty()
        || rest.contains(['?', '#', '\\'])
        || rest.bytes().any(|byte| byte < 0x21 || byte == 0x7f)
    {
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

/// Host without brackets or port, or `None` for an authority this client
/// will not send to.
///
/// The port tail must be empty or a `:`-prefixed 16-bit number, so the
/// origin never keeps text the loopback decision ignored.
fn host_name(authority: &str) -> Option<String> {
    if let Some(host) = authority.strip_prefix('[') {
        let (host, tail) = host.split_once(']')?;

        if host.is_empty() || !valid_port_tail(tail) {
            return None;
        }

        return Some(host.to_owned());
    }

    let (host, port) = authority
        .rsplit_once(':')
        .map(|(host, port)| (host, Some(port)))
        .unwrap_or((authority, None));

    if host.is_empty() || host.contains(':') || port.is_some_and(|port| !valid_port(port)) {
        return None;
    }

    Some(host.to_owned())
}

fn valid_port_tail(tail: &str) -> bool {
    tail.strip_prefix(':').map_or(tail.is_empty(), valid_port)
}

fn valid_port(port: &str) -> bool {
    (1..=5).contains(&port.len())
        && port.bytes().all(|byte| byte.is_ascii_digit())
        && port.parse::<u16>().is_ok()
}

#[cfg(test)]
mod tests {
    use super::ProverEndpoint;
    use crate::error::ProverFailure;

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

    #[test]
    fn a_query_anywhere_is_refused_without_echoing_a_key() {
        for url in [
            "http://127.0.0.1:3001?api-key=DUMMY",
            "https://prover.example?api-key=DUMMY",
            "http://127.0.0.1:3001?%61pi-key=DUMMY",
            "http://127.0.0.1:3001/?api-key=DUMMY",
            "http://127.0.0.1:3001/base?jobId=1",
            "http://127.0.0.1:3001??api-key=keep",
            "https://prover.example?api+key=DUMMY",
        ] {
            let error = ProverEndpoint::parse(url).expect_err(url);
            assert!(matches!(error, ProverFailure::Config), "{url}");
            assert!(!format!("{error:?}").contains("DUMMY"));
        }
    }

    #[test]
    fn a_query_marker_before_the_path_is_never_loopback() {
        let error = ProverEndpoint::parse("http://attacker.example?.localhost")
            .expect_err("marker before path");
        assert!(matches!(error, ProverFailure::Config));
        assert!(
            ProverEndpoint::parse("http://attacker.localhost")
                .expect("localhost suffix")
                .is_loopback()
        );
        assert!(
            !ProverEndpoint::parse("http://attacker.example")
                .expect("remote host")
                .is_loopback()
        );
    }

    #[test]
    fn a_host_or_port_tail_that_does_not_parse_is_refused() {
        for url in [
            "http://127.0.0.1:3001:",
            "http://127.0.0.1:3001x",
            "http://example.com:99999",
            "http://example.com:",
            "http://[::1]junk",
            "http://[::1]:",
            "http://a:b:c",
        ] {
            assert!(ProverEndpoint::parse(url).is_err(), "{url}");
        }

        for url in [
            "http://example.com:1",
            "http://example.com:65535",
            "http://[::1]:443",
            "http://127.0.0.1:3001",
        ] {
            assert!(ProverEndpoint::parse(url).is_ok(), "{url}");
        }
    }

    #[test]
    fn a_base_path_is_kept_and_the_origin_stays_host_only() {
        let endpoint =
            ProverEndpoint::parse("https://devnet.helius-rpc.com/v1/spr").expect("base path");
        assert_eq!(endpoint.origin(), "https://devnet.helius-rpc.com");
        assert_eq!(endpoint.path(), "/v1/spr");
        assert!(!endpoint.is_loopback());
        let loopback = ProverEndpoint::parse("http://127.0.0.1:3001/v1/spr").expect("base path");
        assert_eq!(loopback.path(), "/v1/spr");
        assert!(loopback.is_loopback());
    }
}
