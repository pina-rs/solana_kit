use solana_kit_native::{HttpConfig, NativeHttp};

#[test]
fn explicit_ports_reject_non_decimal_text_and_invalid_ranges() {
    for port in [
        "+80",
        "+443",
        "-1",
        "",
        "８０",
        "٨٠",
        " 80",
        "80 ",
        "8\t0",
        "80\n",
        "abc",
        "80:90",
        "0",
        "00000",
        "65536",
        "18446744073709551616",
    ] {
        assert_port(port, false);
    }
}

#[test]
fn explicit_ports_accept_decimal_boundaries_and_leading_zeroes() {
    for port in ["1", "80", "443", "65535", "00001", "00080", "065535"] {
        assert_port(port, true);
    }
}

#[test]
fn omitted_ports_remain_valid() {
    for endpoint in [
        "http://localhost/",
        "https://localhost/",
        "http://127.0.0.1/",
        "https://127.0.0.1/",
        "http://[::1]/",
        "https://[::1]/",
    ] {
        assert!(NativeHttp::new(endpoint, HttpConfig::default()).is_ok());
    }
}

fn assert_port(port: &str, valid: bool) {
    for scheme in ["http", "https"] {
        for host in ["localhost", "127.0.0.1", "[::1]"] {
            let endpoint = format!("{scheme}://{host}:{port}/");
            let result = NativeHttp::new(&endpoint, HttpConfig::default());
            assert_eq!(result.is_ok(), valid, "{endpoint:?}");
        }
    }
}
