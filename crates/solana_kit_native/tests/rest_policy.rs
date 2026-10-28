//! REST confidentiality is enforced during route admission, before any exchange.

use solana_kit_native::{HeaderValue, HttpConfig, NativeHttp, NativeRest};

const REGIONAL_HOSTS: [&str; 7] = [
    "slc-sender.helius-rpc.com",
    "ewr-sender.helius-rpc.com",
    "lon-sender.helius-rpc.com",
    "fra-sender.helius-rpc.com",
    "ams-sender.helius-rpc.com",
    "sg-sender.helius-rpc.com",
    "tyo-sender.helius-rpc.com",
];

fn route(
    endpoint: &str,
    credentials: bool,
) -> Result<NativeRest<()>, solana_kit_native::ConfigError> {
    let mut config = HttpConfig::default();

    if credentials {
        config.headers.insert(
            "authorization",
            HeaderValue::from_static("Bearer synthetic-test-token"),
        );
    }

    let http = NativeHttp::new(endpoint, config)?;
    NativeRest::new(vec![((), http)], 1024)
}

#[test]
fn regional_plaintext_routes_reject_even_without_credentials() {
    for host in REGIONAL_HOSTS {
        for credentials in [false, true] {
            for path in ["/", "/?api-key=synthetic-test-key"] {
                let error = route(&format!("http://{host}{path}"), credentials).unwrap_err();
                assert_eq!(error.to_string(), "remote REST routes require HTTPS");
                let diagnostic = format!("{error:?} {error}");
                assert!(!diagnostic.contains(host));
                assert!(!diagnostic.contains("synthetic-test"));
            }
        }
    }
}

#[test]
fn remote_plaintext_cannot_bypass_policy_using_names_or_addresses() {
    for host in [
        "localhost",
        "localhost.invalid",
        "127.0.0.1.invalid",
        "192.0.2.1",
        "[2001:db8::1]",
        "[::ffff:192.0.2.1]",
    ] {
        assert!(route(&format!("http://{host}/"), false).is_err());
    }
}

#[test]
fn https_and_literal_loopback_services_remain_available() {
    assert!(
        route(
            "https://sender.helius-rpc.com/?api-key=synthetic-test-key",
            true
        )
        .is_ok()
    );

    for host in ["127.0.0.1", "127.1.2.3", "[::1]"] {
        assert!(route(&format!("http://{host}/"), true).is_ok());
    }
}

#[tokio::test]
async fn route_admission_never_opens_a_socket() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    assert!(route(&format!("http://localhost:{port}/"), true).is_err());
    let listener = listener.into_std().unwrap();
    assert_eq!(
        listener.accept().unwrap_err().kind(),
        std::io::ErrorKind::WouldBlock
    );
}
