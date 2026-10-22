//! Actual browser certificate rejection. No trust override exists.
use rustls::{
    ServerConfig, ServerConnection,
    pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer},
};
use std::{
    net::{TcpListener, TcpStream},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};
const FIXTURE_TIMEOUT: Duration = Duration::from_secs(5);

pub fn reject(listener: TcpListener, stop: Arc<AtomicBool>) {
    let Some(mut stream) = accept(listener, stop) else {
        return;
    };
    stream.set_nonblocking(false).unwrap();
    stream.set_read_timeout(Some(FIXTURE_TIMEOUT)).unwrap();
    let mut connection = ServerConnection::new(Arc::new(config())).unwrap();
    let mut failed = false;

    while connection.is_handshaking() {
        if connection.complete_io(&mut stream).is_err() {
            failed = true;
            break;
        }
    }

    assert!(failed, "browser must reject untrusted certificate");
    println!("Actual WSS TLS handshake rejected by browser.");
}
// A failed browser process must still let the fixture owner join this listener.
fn accept(listener: TcpListener, stop: Arc<AtomicBool>) -> Option<TcpStream> {
    listener.set_nonblocking(true).unwrap();

    while !stop.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((stream, _)) => return Some(stream),

            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(2));
            }

            Err(error) => panic!("TLS listener failed {error}"),
        }
    }

    None
}
fn config() -> ServerConfig {
    ServerConfig::builder_with_provider(rustls::crypto::ring::default_provider().into())
        .with_safe_default_protocol_versions()
        .unwrap()
        .with_no_client_auth()
        .with_single_cert(
            vec![CertificateDer::from(
                include_bytes!("../fixtures/server.der").to_vec(),
            )],
            PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(
                include_bytes!("../fixtures/server-key.der").to_vec(),
            )),
        )
        .unwrap()
}
