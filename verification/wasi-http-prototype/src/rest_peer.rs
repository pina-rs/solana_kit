//! Independent HTTP peer validates the actual public REST request bytes.
use std::io::{Read, Write};

pub fn respond(stream: &mut (impl Read + Write), mode: &str) {
    let mut bytes = Vec::new();
    let mut byte = [0];

    while !bytes.ends_with(b"\r\n\r\n") {
        assert!(bytes.len() < 8192);
        stream.read_exact(&mut byte).expect("REST headers");
        bytes.push(byte[0]);
    }

    let headers = std::str::from_utf8(&bytes).expect("UTF8 headers");
    assert!(headers.starts_with("POST /rpc/a%2Fb?key=a%20b&key=%26 HTTP/1.1\r\n"));
    let headers = headers.to_ascii_lowercase();
    assert!(headers.contains("content-type: application/json\r\n"));
    assert!(headers.contains("authorization: bearer fixture-only\r\n"));
    assert!(headers.contains("content-length: 4\r\n"));
    let mut body = [0; 4];
    stream.read_exact(&mut body).expect("REST body");
    assert_eq!(&body, b"null");
    let response = if mode == "rest-malformed" {
        b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\nConnection: close\r\n\r\nabc".as_slice()
    } else {
        b"HTTP/1.1 200 OK\r\nContent-Length: 4\r\nConnection: close\r\n\r\ndone"
    };

    stream.write_all(response).expect("REST response");
    stream.flush().expect("REST flush");
}
