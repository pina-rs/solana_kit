//! Fixed verification grants cannot redirect a generated app to a public node.
use leptoskit_bridge::Error;
use serde::Deserialize;
use std::{fs, io::Read, net::SocketAddr, path::Path};

const CONFIG_BYTES: u64 = 4096;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Endpoints {
    pub http: String,
    pub socket: String,
    pub quiet: Option<String>,
    pub quiet_socket: Option<String>,
}

impl Endpoints {
    pub(super) fn read(data: &Path) -> Result<Self, Error> {
        let path = data.join("journey.json");
        let metadata = fs::symlink_metadata(&path).map_err(|_| Error::InvalidArgument)?;

        if !metadata.is_file() || metadata.len() > CONFIG_BYTES {
            return Err(Error::InvalidArgument);
        }

        let mut bytes = Vec::new();
        fs::File::open(path)
            .map_err(crate::native)?
            .take(CONFIG_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(crate::native)?;

        if bytes.len() as u64 > CONFIG_BYTES {
            return Err(Error::InvalidArgument);
        }

        Self::parse(&bytes, cfg!(target_os = "android"))
    }

    fn parse(bytes: &[u8], android: bool) -> Result<Self, Error> {
        let endpoints: Self = serde_json::from_slice(bytes).map_err(|_| Error::InvalidArgument)?;

        for (endpoint, scheme) in [
            (Some(&endpoints.http), "http://"),
            (Some(&endpoints.socket), "ws://"),
            (endpoints.quiet.as_ref(), "http://"),
            (endpoints.quiet_socket.as_ref(), "ws://"),
        ] {
            if let Some(endpoint) = endpoint {
                local_endpoint(endpoint, scheme, android)?;
            }
        }

        Ok(endpoints)
    }
}

fn local_endpoint(endpoint: &str, scheme: &str, android: bool) -> Result<(), Error> {
    let address = endpoint
        .strip_prefix(scheme)
        .and_then(|address| address.parse::<SocketAddr>().ok())
        .ok_or(Error::InvalidArgument)?;
    let gateway = android && address.ip() == std::net::Ipv4Addr::new(10, 0, 2, 2);

    if address.port() == 0 || (!address.ip().is_loopback() && !gateway) {
        return Err(Error::InvalidArgument);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_only_explicit_local_socket_authorities() {
        for endpoint in ["http://127.0.0.1:8899", "http://[::1]:8899"] {
            assert!(local_endpoint(endpoint, "http://", false).is_ok());
        }

        for endpoint in [
            "http://localhost:8899",
            "http://127.0.0.1:0",
            "http://127.0.0.1:8899/",
            "http://127.0.0.1:8899?x=1",
            "http://name@127.0.0.1:8899",
            "http://127.0.0.1:8899#x",
            "https://127.0.0.1:8899",
            "http://192.0.2.1:8899",
        ] {
            assert!(local_endpoint(endpoint, "http://", false).is_err());
        }
    }

    #[test]
    fn android_gateway_is_an_explicit_platform_exception() {
        assert!(local_endpoint("http://10.0.2.2:8899", "http://", true).is_ok());
        assert!(local_endpoint("http://10.0.2.2:8899", "http://", false).is_err());
        assert!(local_endpoint("http://10.0.2.3:8899", "http://", true).is_err());
    }

    #[test]
    fn checks_every_configured_grant_and_rejects_unknown_fields() {
        let valid = br#"{"http":"http://127.0.0.1:8899","socket":"ws://[::1]:8900"}"#;
        assert!(Endpoints::parse(valid, false).is_ok());

        for (field, value) in [
            ("http", serde_json::json!("ws://127.0.0.1:8899")),
            ("socket", serde_json::json!("http://127.0.0.1:8900")),
            ("quiet", serde_json::json!("http://192.0.2.1:8899")),
            ("quiet_socket", serde_json::json!("http://127.0.0.1:8900")),
            ("typo", serde_json::json!(true)),
        ] {
            let mut config: serde_json::Value = serde_json::from_slice(valid).unwrap();
            config[field] = value;
            let bytes = serde_json::to_vec(&config).unwrap();
            assert!(Endpoints::parse(&bytes, false).is_err());
        }
    }
}
