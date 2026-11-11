//! Attested prover session binding.
//!
//! A TEE prover publishes an HPKE key inside a quote. `report_data` is the
//! 64-byte value the quote must carry, and `gpu_nonce` binds GPU evidence to
//! the same nonce. `request_aad` is the additional data the HPKE seal covers.
//! These three match `prover/tee/testdata/vectors.json`. Quote parsing, the
//! Intel and Nitro signature checks, and the HPKE seal itself stay in the
//! upstream `spr-prover-tee` crate: this module does not claim a quote is valid.

use sha2::{Digest, Sha256, Sha512};

const REPORT_DOMAIN: &[u8] = b"zolana/prover-tee/v1/report";
const GPU_DOMAIN: &[u8] = b"zolana/prover-tee/v1/gpu";
const GATEWAY_PREFIX: &str = "/v1/spr";
const API_KEY_NAME: &str = "api-key";
const NONCE_BYTES: usize = 32;
const DIGEST_BYTES: usize = 32;
const REPORT_BYTES: usize = 64;
const HEX_RADIX: u32 = 16;
const PERCENT_WIDTH: usize = 2;

/// Header that names the TEE wire version.
pub const HEADER_VERSION: &str = "Spr-Tee";

/// Header that carries the encapsulated HPKE key.
pub const HEADER_ENC: &str = "Spr-Tee-Enc";

/// Header that carries ciphertext when the method cannot have a body.
pub const HEADER_CIPHERTEXT: &str = "Spr-Tee-Ciphertext";

/// Wire version both sides require.
pub const TEE_VERSION: &str = "v1";

/// `SHA-512(domain || nonce || hpke_public_key || gpu_digest)`.
///
/// `gpu_token` is hashed with SHA-256. `None` uses 32 zero bytes, which is
/// the digest the quote uses when the prover has no GPU evidence.
#[must_use]
pub fn report_data(
    nonce: &[u8; NONCE_BYTES],
    hpke_public_key: &[u8; NONCE_BYTES],
    gpu_token: Option<&[u8]>,
) -> [u8; REPORT_BYTES] {
    let gpu_digest = gpu_digest(gpu_token);
    let mut hasher = Sha512::new();
    hasher.update(REPORT_DOMAIN);
    hasher.update(nonce);
    hasher.update(hpke_public_key);
    hasher.update(gpu_digest);
    hasher.finalize().into()
}

/// `SHA-256(gpu domain || nonce || hpke_public_key)`.
#[must_use]
pub fn gpu_nonce(
    nonce: &[u8; NONCE_BYTES],
    hpke_public_key: &[u8; NONCE_BYTES],
) -> [u8; DIGEST_BYTES] {
    let mut hasher = Sha256::new();
    hasher.update(GPU_DOMAIN);
    hasher.update(nonce);
    hasher.update(hpke_public_key);
    hasher.finalize().into()
}

/// Binds the method and the route, with every `api-key` query pair removed.
///
/// Trailing slashes are stripped before the `/v1/spr` gateway prefix. A prefix
/// is removed only when the remainder starts with `/`, which is how the
/// gateway forwards the request.
#[must_use]
pub fn request_aad(method: &str, request_uri: &str) -> String {
    let (path, query) = split_target(request_uri);
    let route = gateway_route(&trim_slashes(path));
    let kept = kept_query(query);

    if kept.is_empty() {
        return format!("{method} {route}");
    }

    format!("{method} {route}?{kept}")
}

/// Lowercase hex for the attestation nonce query.
#[must_use]
pub fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut text = String::with_capacity(bytes.len() * PERCENT_WIDTH);

    for byte in bytes {
        text.push(HEX[(byte >> 4) as usize] as char);
        text.push(HEX[(byte & 0x0f) as usize] as char);
    }

    text
}

fn gpu_digest(token: Option<&[u8]>) -> [u8; DIGEST_BYTES] {
    match token {
        Some(bytes) => Sha256::digest(bytes).into(),
        None => [0; DIGEST_BYTES],
    }
}

fn split_target(request_uri: &str) -> (&str, &str) {
    match request_uri.split_once('?') {
        Some((path, query)) => (path, query),
        None => (request_uri, ""),
    }
}

fn trim_slashes(path: &str) -> String {
    path.trim_end_matches('/').to_owned()
}

fn gateway_route(path: &str) -> String {
    let Some(rest) = path.strip_prefix(GATEWAY_PREFIX) else {
        return root_if_empty(path);
    };

    if rest.starts_with('/') {
        return root_if_empty(rest);
    }

    root_if_empty(path)
}

fn root_if_empty(path: &str) -> String {
    if path.is_empty() {
        return "/".to_owned();
    }

    path.to_owned()
}

fn kept_query(query: &str) -> String {
    if query.is_empty() {
        return String::new();
    }

    query
        .split('&')
        .filter(|pair| !pair.is_empty() && !is_api_key(pair))
        .collect::<Vec<_>>()
        .join("&")
}

fn is_api_key(pair: &str) -> bool {
    let raw = pair.split_once('=').map(|(key, _)| key).unwrap_or(pair);
    form_decode(raw) == API_KEY_NAME
}

fn form_decode(raw: &str) -> String {
    let bytes = raw.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut cursor = 0;

    while cursor < bytes.len() {
        cursor = push_form_byte(bytes, cursor, &mut decoded);
    }

    String::from_utf8_lossy(&decoded).into_owned()
}

fn push_form_byte(bytes: &[u8], cursor: usize, decoded: &mut Vec<u8>) -> usize {
    if bytes[cursor] == b'+' {
        decoded.push(b' ');
        return cursor + 1;
    }

    if let Some(value) = percent_byte(bytes, cursor) {
        decoded.push(value);
        return cursor + 1 + PERCENT_WIDTH;
    }

    decoded.push(bytes[cursor]);
    cursor + 1
}

fn percent_byte(bytes: &[u8], cursor: usize) -> Option<u8> {
    if bytes[cursor] != b'%' || cursor + PERCENT_WIDTH >= bytes.len() {
        return None;
    }

    let high = hex_value(bytes[cursor + 1])?;
    let low = hex_value(bytes[cursor + PERCENT_WIDTH])?;
    Some((high << 4) | low)
}

fn hex_value(byte: u8) -> Option<u8> {
    char::from(byte)
        .to_digit(HEX_RADIX)
        .and_then(|value| u8::try_from(value).ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    const NONCE: [u8; 32] = [0x11; 32];
    const HPKE_KEY: [u8; 32] = [
        0xf4, 0x49, 0xa5, 0x1a, 0xe9, 0x08, 0x98, 0xef, 0xc4, 0x9a, 0xfb, 0x64, 0x30, 0x5f, 0x1e,
        0xbc, 0x2e, 0x0b, 0x47, 0xf6, 0xd8, 0x39, 0x19, 0xbc, 0x3f, 0x98, 0xd3, 0xe7, 0xf2, 0xfe,
        0x6b, 0x0b,
    ];
    const REPORT: &str = "00e611aac3df68da4c51dfe291e0b9093b426c3a985952f9be71cc11f3eef90113c540c58fdde93b451fb22f80d87143d42b0fc3de8b79fefe88ececba955e03";
    const REPORT_NO_GPU: &str = "b862900309ded091670546a72ac72e6aefcd3bb1da1516d739933ce9fc025ad366fbddd3915b948749c809916909f902e57e354ffdff2c0e731080994283216a";
    const GPU_NONCE: &str = "a60303f260939f6351f9ae36813887f29d56809a05f5379712dc1f3ce039405d";

    fn decode_hex(text: &str) -> Vec<u8> {
        (0..text.len())
            .step_by(PERCENT_WIDTH)
            .map(|index| {
                u8::from_str_radix(&text[index..index + PERCENT_WIDTH], HEX_RADIX).unwrap()
            })
            .collect()
    }

    #[test]
    fn report_and_gpu_nonce_match_the_tee_vector() {
        let token = b"[[\"JWT\",\"header.payload.signature\"],{\"GPU-0\":\"device\"}]";
        assert_eq!(
            hex_encode(&report_data(&NONCE, &HPKE_KEY, Some(token))),
            REPORT
        );
        assert_eq!(
            hex_encode(&report_data(&NONCE, &HPKE_KEY, None)),
            REPORT_NO_GPU
        );
        assert_eq!(hex_encode(&gpu_nonce(&NONCE, &HPKE_KEY)), GPU_NONCE);
        assert_eq!(HEADER_VERSION, "Spr-Tee");
        assert_eq!(TEE_VERSION, "v1");
        let _ = (HEADER_ENC, HEADER_CIPHERTEXT);
    }

    #[test]
    fn request_aad_matches_the_published_cases() {
        let cases = [
            ("GET", "/x?%zz=1", "GET /x?%zz=1"),
            ("GET", "/", "GET /"),
            (
                "POST",
                "/v1/spr/prove/sample/?id=a%20b",
                "POST /prove/sample?id=a%20b",
            ),
            (
                "POST",
                "/prove/sample//?id=a%20b",
                "POST /prove/sample?id=a%20b",
            ),
            (
                "POST",
                "/prove/sample%2F/?id=a%20b",
                "POST /prove/sample%2F?id=a%20b",
            ),
            ("GET", "/v1/spr/?jobId=a", "GET /v1/spr?jobId=a"),
            ("GET", "/v1/spr//", "GET /v1/spr"),
            ("GET", "/health", "GET /health"),
            (
                "POST",
                "/v1/spr/prove/transfer_2_2?api-key=k",
                "POST /prove/transfer_2_2",
            ),
            ("POST", "/v1/spr/v1/spr/prove/x", "POST /v1/spr/prove/x"),
            ("POST", "/v1/sprx/prove/x", "POST /v1/sprx/prove/x"),
            ("GET", "/v1/spr?jobId=a", "GET /v1/spr?jobId=a"),
            (
                "GET",
                "/prove/merge_8_1/status?api-key=k&jobId=a1",
                "GET /prove/merge_8_1/status?jobId=a1",
            ),
            (
                "GET",
                "/prove/merge_8_1/status?jobId=a1&api-key=k",
                "GET /prove/merge_8_1/status?jobId=a1",
            ),
            (
                "GET",
                "/attestation?nonce=ab&api-key=k1&api-key=k2",
                "GET /attestation?nonce=ab",
            ),
            (
                "GET",
                "/prove/x?api-keys=v&api-key",
                "GET /prove/x?api-keys=v",
            ),
            ("GET", "/prove/x?&&jobId=a&", "GET /prove/x?jobId=a"),
            (
                "POST",
                "/v1/spr/prove/transfer_2_2?api%2Dkey=k&jobId=%61%31&q=a%20b+c",
                "POST /prove/transfer_2_2?jobId=%61%31&q=a%20b+c",
            ),
            (
                "GET",
                "/prove/x?%61pi-key=a&api-%6Bey=b&%61%70%69%2d%6b%65%79=c&jobId=a",
                "GET /prove/x?jobId=a",
            ),
            (
                "GET",
                "/prove/x?api-key=k&API-KEY=keep&api+key=keep&api%2Bkey=keep&bad%=keep",
                "GET /prove/x?API-KEY=keep&api+key=keep&api%2Bkey=keep&bad%=keep",
            ),
            (
                "GET",
                "/prove/x??api-key=keep&jobId=a",
                "GET /prove/x??api-key=keep&jobId=a",
            ),
        ];

        for (method, target, expected) in cases {
            assert_eq!(request_aad(method, target), expected);
        }

        assert_eq!(decode_hex("0f"), vec![0x0f]);
    }
}
