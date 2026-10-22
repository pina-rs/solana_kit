use super::support::*;
use solana_kit_client::{Transport, TransportFailureKind as Kind};
use solana_kit_native::{HttpConfig, NativeHttp, TlsRoots};
use std::{sync::Arc, time::Duration};
use tokio::io::AsyncWriteExt;
use tokio_rustls::{
    TlsAcceptor,
    rustls::{
        self, ServerConfig,
        pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer},
    },
};

const ROOT: &[u8] = include_bytes!("../fixtures/ca.der");
const CERT: &[u8] = include_bytes!("../fixtures/server.der");
const KEY: &[u8] = include_bytes!("../fixtures/server-key.der");

fn acceptor() -> TlsAcceptor {
    let config =
        ServerConfig::builder_with_provider(rustls::crypto::ring::default_provider().into())
            .with_safe_default_protocol_versions()
            .unwrap()
            .with_no_client_auth()
            .with_single_cert(
                vec![CertificateDer::from(CERT.to_vec())],
                PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(KEY.to_vec())),
            )
            .unwrap();
    TlsAcceptor::from(Arc::new(config))
}

#[tokio::test]
async fn tls_requires_trust_and_exact_destination_name() {
    for (hostname, trusted, succeeds) in [
        ("localhost", true, true),
        ("127.0.0.1", true, false),
        ("localhost", false, false),
    ] {
        tls_case(hostname, trusted, succeeds).await;
    }
}

async fn tls_case(hostname: &str, trusted: bool, succeeds: bool) {
    let listener = listener().await;
    let port = listener.local_addr().unwrap().port();
    let server = tokio::spawn(async move {
        let (socket, _) = listener.accept().await.unwrap();
        let stream = acceptor().accept(socket).await;

        if !succeeds {
            assert!(stream.is_err());
            return;
        }

        let mut stream = stream.unwrap();
        read_request(&mut stream).await;
        stream.write_all(&response(200, b"TLS")).await.unwrap();
    });
    let tls = if trusted {
        TlsRoots::Certificates(vec![ROOT.to_vec()])
    } else {
        TlsRoots::WebPki
    };

    let adapter = NativeHttp::new(
        &format!("https://{hostname}:{port}/"),
        HttpConfig {
            tls,
            ..HttpConfig::default()
        },
    )
    .unwrap();
    let result = adapter.exchange(request(3)).await;

    if succeeds {
        assert_eq!(result.unwrap().body, b"TLS");
    } else {
        assert_eq!(result.unwrap_err().kind, Kind::Io);
    }

    server.await.unwrap();
}

#[tokio::test]
async fn connect_deadline_includes_tls_handshake() {
    let listener = listener().await;
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        // Consume the ClientHello, then keep the TLS peer quiet until cancellation.
        use tokio::io::AsyncReadExt;
        let mut record = [0; 5];
        socket.read_exact(&mut record).await.unwrap();
        let length = u16::from_be_bytes([record[3], record[4]]);
        let mut body = vec![0; usize::from(length)];
        socket.read_exact(&mut body).await.unwrap();
        assert_eq!(eof(&mut socket).await.unwrap(), 0);
    });
    let config = HttpConfig {
        connect_timeout: Duration::from_millis(100),
        ..HttpConfig::default()
    };
    let adapter = NativeHttp::new(&format!("https://{address}/"), config).unwrap();
    assert_eq!(
        adapter.exchange(request(3)).await.unwrap_err().kind,
        Kind::Deadline
    );
    server.await.unwrap();
}
