//! Explicit trust roots with mandatory certificate and destination verification.
use crate::ConfigError;
use tokio_rustls::rustls::{self, ClientConfig, RootCertStore, pki_types::CertificateDer};

/// Certificate authorities selected explicitly by the caller.
#[derive(Clone, Default)]
pub enum TlsRoots {
    /// Mozilla's pinned WebPKI root set; no ambient operating-system trust files.
    #[default]
    WebPki,
    /// Only these DER-encoded CA certificates; peer/name verification remains on.
    Certificates(Vec<Vec<u8>>),
}

impl TlsRoots {
    pub(crate) fn build(&self) -> Result<ClientConfig, ConfigError> {
        let roots = root_store(self)?;
        ClientConfig::builder_with_provider(rustls::crypto::ring::default_provider().into())
            .with_safe_default_protocol_versions()
            .map_err(|_| ConfigError("TLS protocol configuration unavailable"))
            .map(|builder| builder.with_root_certificates(roots).with_no_client_auth())
    }
}

fn root_store(source: &TlsRoots) -> Result<RootCertStore, ConfigError> {
    let mut roots = RootCertStore::empty();
    let TlsRoots::Certificates(certificates) = source else {
        roots.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

        return Ok(roots);
    };

    for certificate in certificates {
        roots
            .add(CertificateDer::from(certificate.clone()))
            .map_err(|_| ConfigError("invalid TLS root certificate"))?;
    }

    Ok(roots)
}
