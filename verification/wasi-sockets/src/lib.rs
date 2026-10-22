//! Private TLS configuration for an explicitly named off-chain socket experiment.

use std::sync::Arc;

use anyhow::Result;
use rustls::pki_types::{CertificateDer, PrivatePkcs8KeyDer, ServerName};
use rustls::{ClientConfig, ClientConnection, RootCertStore, ServerConfig};

/// Builds a normal validating TLS client with exactly one fixture trust anchor.
pub fn client(wrong_ca: bool, wrong_name: bool) -> Result<ClientConnection> {
    let bytes = if wrong_ca {
        include_bytes!("../fixtures/wrong-ca.der").as_slice()
    } else {
        include_bytes!("../fixtures/ca.der").as_slice()
    };

    let mut roots = RootCertStore::empty();
    roots.add(CertificateDer::from(bytes.to_vec()))?;
    let config = ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();

    let name = if wrong_name {
        "wrong.invalid"
    } else {
        "localhost"
    };

    Ok(ClientConnection::new(
        Arc::new(config),
        ServerName::try_from(name)?,
    )?)
}

/// Builds the loopback peer with a public, deliberately disposable fixture key.
pub fn server() -> Result<Arc<ServerConfig>> {
    let certificate = CertificateDer::from(include_bytes!("../fixtures/server.der").to_vec());
    let key = PrivatePkcs8KeyDer::from(include_bytes!("../fixtures/server-key.der").to_vec());
    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![certificate], key.into())?;

    Ok(Arc::new(config))
}
