//! High-level attestation flow: a fresh challenge per attempt, one attempt
//! per draw, and every verification failure propagated from [`attest`].

use std::num::NonZeroU32;
use std::sync::{Arc, Mutex};

use rand_core::{CryptoRng, Error as RngError, RngCore};
use serde_json::Value;
use solana_kit_client::{Reply, TransportError, TransportFailureKind};

use super::*;

const HTTP_OK: u16 = 200;
const LOOPBACK: &str = "http://127.0.0.1:9";
const PINNED_TIMESTAMP_MS: u64 = 1_791_307_410_659;

fn live_body() -> Value {
    serde_json::from_str(include_str!(
        "../../tests/fixtures/nitro-live-attestation.json"
    ))
    .expect("live fixture")
}

fn live_policy() -> NitroPolicy {
    let policy: Value =
        serde_json::from_str(include_str!("../../tests/fixtures/nitro-live-policy.json"))
            .expect("policy fixture");
    let deployment = &policy["deployment"];
    let measurements = deployment["measurements"]
        .as_array()
        .expect("measurements")
        .iter()
        .map(|item| NitroMeasurement {
            pcr0: hex48(item["pcr0"].as_str().expect("pcr0")),
            pcr1: hex48(item["pcr1"].as_str().expect("pcr1")),
            pcr2: hex48(item["pcr2"].as_str().expect("pcr2")),
        })
        .collect();

    // The fixture deployment declares `max_age_secs: 600`; the skew stays a
    // small explicit value because the format predates the field.
    NitroPolicy {
        measurements,
        hpke_public_key: None,
        max_age_secs: deployment["max_age_secs"].as_u64().expect("max age"),
        max_future_secs: 60,
    }
}

fn hex_array(hex: &str) -> [u8; 32] {
    hex_decode(hex)
        .expect("hex")
        .try_into()
        .expect("nonce width")
}

fn hex48(hex: &str) -> [u8; 48] {
    hex_decode(hex).expect("hex").try_into().expect("pcr width")
}

fn fixture_nonce() -> [u8; 32] {
    hex_array(live_body()["nonce"].as_str().expect("nonce"))
}

/// A transport that answers every call with one canned reply and records the
/// requests, the way a cached or replaying quote endpoint would.
struct QuoteTransport {
    /// Status answered for every call; `0` fails the exchange itself.
    status: u16,
    body: String,
    calls: Arc<Mutex<usize>>,
    queries: Arc<Mutex<Vec<Vec<(String, String)>>>>,
}

struct QuoteFixture {
    client: PrivacyProverClient<QuoteTransport>,
    calls: Arc<Mutex<usize>>,
    queries: Arc<Mutex<Vec<Vec<(String, String)>>>>,
}

fn quote_transport(status: u16, body: String) -> QuoteFixture {
    let calls = Arc::new(Mutex::new(0));
    let queries = Arc::new(Mutex::new(Vec::new()));
    let transport = QuoteTransport {
        status,
        body,
        calls: Arc::clone(&calls),
        queries: Arc::clone(&queries),
    };
    QuoteFixture {
        client: PrivacyProverClient::new(LOOPBACK, transport).expect("client"),
        calls,
        queries,
    }
}

impl rest::Transport<ProverDestination> for QuoteTransport {
    type Exchange<'a>
        = std::future::Ready<Result<Reply, TransportError>>
    where
        Self: 'a;

    fn exchange(&self, request: rest::Request<ProverDestination>) -> Self::Exchange<'_> {
        *self.calls.lock().expect("calls") += 1;
        self.queries.lock().expect("queries").push(request.query);

        if self.status == 0 {
            return std::future::ready(Err(TransportError::new(TransportFailureKind::Denied)));
        }

        std::future::ready(Ok(Reply {
            status: self.status,
            body: self.body.as_bytes().to_vec(),
        }))
    }
}

/// An advancing cryptographic-RNG double whose first draw is the nonce the
/// pinned live quote was minted for. Later draws move the stream forward, so
/// a second attest never repeats the first nonce.
struct PinnedNonceRng {
    draws: usize,
}

impl PinnedNonceRng {
    fn new() -> Self {
        Self { draws: 0 }
    }
}

impl RngCore for PinnedNonceRng {
    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    fn next_u64(&mut self) -> u64 {
        u64::from_be_bytes(*fixture_nonce()) ^ (self.draws as u64).wrapping_mul(0x9E37_79B9)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.draws += 1;
        let pinned = fixture_nonce();

        if self.draws == 1 && dest.len() == pinned.len() {
            dest.copy_from_slice(&pinned);

            return;
        }

        // The stamp is never zero, so draw two and every later draw differ
        // from the pinned nonce in at least the first byte.
        let stamp = (self.draws % 255).max(2) as u8;

        for (index, slot) in dest.iter_mut().enumerate() {
            *slot = pinned[index % pinned.len()] ^ stamp;
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), RngError> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl CryptoRng for PinnedNonceRng {}

/// A cryptographic-RNG double whose entropy source is exhausted, the way an
/// OS outage surfaces through `try_fill_bytes`.
struct ExhaustedRng;

impl RngCore for ExhaustedRng {
    fn next_u32(&mut self) -> u32 {
        0
    }

    fn next_u64(&mut self) -> u64 {
        0
    }

    fn fill_bytes(&mut self, _dest: &mut [u8]) {
        // The very panic production code avoids by calling `try_fill_bytes`
        // instead.
        panic!("entropy source is unavailable");
    }

    fn try_fill_bytes(&mut self, _dest: &mut [u8]) -> Result<(), RngError> {
        Err(RngError::from(NonZeroU32::new(7).expect("nonzero")))
    }
}

impl CryptoRng for ExhaustedRng {}

#[test]
fn attest_binds_a_fresh_nonce_to_one_quote() {
    let fixture = quote_transport(HTTP_OK, live_body().to_string());
    let policy = live_policy();
    let now = live_body()["captured_at"].as_u64().expect("captured_at");
    let mut rng = PinnedNonceRng::new();

    let verified =
        futures::executor::block_on(fixture.client.attest(&mut rng, &policy, now)).expect("attest");
    assert_eq!(rng.draws, 1);
    assert_eq!(*fixture.calls.lock().expect("calls"), 1);

    // The outgoing query carried exactly the nonce this call drew.
    let queries = fixture.queries.lock().expect("queries").clone();
    assert_eq!(
        queries,
        vec![vec![("nonce".to_owned(), hex_encode(&fixture_nonce()))]]
    );

    // The returned session is the pinned one: its HPKE key and signed
    // timestamp come from the fixture document, verified at the caller's
    // clock inside the policy window.
    let hpke = hex_array(
        live_body()["attestation"]["hpke_public_key"]
            .as_str()
            .expect("hpke"),
    );
    assert_eq!(verified.hpke_public_key, hpke);
    assert_eq!(verified.timestamp_ms, PINNED_TIMESTAMP_MS);

    // A second call draws a new nonce, and the transport still answers with
    // the first quote: that quote belongs to the previous challenge and must
    // not verify.
    let replayed = futures::executor::block_on(fixture.client.attest(&mut rng, &policy, now));
    assert!(matches!(replayed, Err(Error::Attestation("nonce"))));
    assert_eq!(rng.draws, 2);
    assert_eq!(*fixture.calls.lock().expect("calls"), 2);
}

#[test]
fn attest_propagates_http_transport_and_verification_failures() {
    let now = live_body()["captured_at"].as_u64().expect("captured_at");

    let fixture = quote_transport(500, "{}".to_owned());
    let error = futures::executor::block_on(fixture.client.attest(
        &mut PinnedNonceRng::new(),
        &live_policy(),
        now,
    ))
    .expect_err("http");
    assert!(matches!(error, Error::Prover(ProverFailure::Http(500))));
    assert_eq!(*fixture.calls.lock().expect("calls"), 1);

    let fixture = quote_transport(0, String::new());
    let error = futures::executor::block_on(fixture.client.attest(
        &mut PinnedNonceRng::new(),
        &live_policy(),
        now,
    ))
    .expect_err("transport");
    assert!(matches!(error, Error::Prover(ProverFailure::Transport)));
    assert_eq!(*fixture.calls.lock().expect("calls"), 1);

    let fixture = quote_transport(HTTP_OK, "not-json".to_owned());
    let error = futures::executor::block_on(fixture.client.attest(
        &mut PinnedNonceRng::new(),
        &live_policy(),
        now,
    ))
    .expect_err("json");
    assert!(matches!(error, Error::Prover(ProverFailure::Response)));

    let fixture = quote_transport(
        HTTP_OK,
        r#"{"attestation":{"platform":"dstack-tdx"}}"#.to_owned(),
    );
    let error = futures::executor::block_on(fixture.client.attest(
        &mut PinnedNonceRng::new(),
        &live_policy(),
        now,
    ))
    .expect_err("platform");
    assert!(matches!(error, Error::Attestation("platform")));

    // The pinned quote itself is refused when the policy pins a different
    // HPKE key: every verification step still runs on the fetched body.
    let mut pinned = live_policy();
    pinned.hpke_public_key = Some([9; 32]);
    let fixture = quote_transport(HTTP_OK, live_body().to_string());
    let error = futures::executor::block_on(fixture.client.attest(
        &mut PinnedNonceRng::new(),
        &pinned,
        now,
    ))
    .expect_err("pin");
    assert!(matches!(error, Error::Attestation("hpke pin")));
}

#[test]
fn an_entropy_failure_returns_before_any_transport_call() {
    let fixture = quote_transport(HTTP_OK, live_body().to_string());
    let error =
        futures::executor::block_on(fixture.client.attest(&mut ExhaustedRng, &live_policy(), 0))
            .expect_err("exhausted entropy");

    // The error is a fixed label: no nonce, clock, or RNG detail rides out,
    // and the transport was never asked for a quote.
    assert!(matches!(error, Error::Attestation("entropy")));
    assert_eq!(format!("{error:?}"), "Attestation(\"entropy\")");
    assert_eq!(*fixture.calls.lock().expect("calls"), 0);
    assert!(fixture.queries.lock().expect("queries").is_empty());
}
