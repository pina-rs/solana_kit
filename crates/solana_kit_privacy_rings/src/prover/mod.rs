//! Remote prover client.
//!
//! `PrivacyProverClient::prove` posts a caller-supplied witness to
//! `POST /prove/{circuit}` with `X-Sync: true`. A `429` is sent again with
//! `X-Async: true`, and a `jobId` is polled at `/prove/{circuit}/status` until
//! the proof is ready or the wait bound passes. The returned [`Proof`] is the
//! compressed Groth16 proof a transact instruction stores. Attach it with
//! [`TransactBody::with_proof`].
//!
//! The client speaks through [`solana_kit_client::rest::Transport`]. The
//! transport owns TLS, deadlines, and the API key. It sets `X-Sync` or
//! `X-Async` from [`ProverDestination::delivery`] and, when the caller has a
//! key, sends that key as a header. This client never puts an API key in a
//! URL, a request, or an error.
//!
//! A prover that is not on a loopback host refuses the call unless
//! [`PrivacyProverClient::allow_unauthenticated_remote_prover`] is set. The
//! flag sends the witness as ordinary JSON, which a local prover accepts and a
//! TEE-only gateway rejects. [`tee`] binds `report_data`. The `tee` feature
//! checks an AWS Nitro COSE document and seals the body with HPKE. The
//! transport still owns HTTP headers, so the caller attaches `Spr-Tee` and
//! `Spr-Tee-Enc`. [`PrivacyProverClient::prove_indexed`] posts to
//! `/prove/{circuit}/indexed` and polls a queued job at the same
//! `/prove/{circuit}/status` route. A remote URL must be `https`.

mod endpoint;
#[cfg(feature = "tee")]
mod hpke;
mod indexed;
mod keys;
#[cfg(feature = "tee")]
mod nitro;
mod proof;
mod tee;

use std::time::{Duration, Instant};

use serde_json::Value;
use solana_kit_client::rest::{self, Body, Method};

#[cfg(feature = "tee")]
use rand_core::{CryptoRng, RngCore};

use crate::encode::{Proof, TransactBody};
use crate::error::{Error, ProverFailure};
pub use endpoint::ProverEndpoint;
#[cfg(feature = "tee")]
pub use hpke::{
    OpenedRequest, SealedRequest, open_prover_request, open_prover_response, seal_prover_request,
};
pub use indexed::IndexedCircuit;
pub use keys::proving_key_sha256;
#[cfg(feature = "tee")]
pub use nitro::{
    NitroMeasurement, NitroPolicy, VerifiedNitro, verify_attestation, verify_nitro,
    verify_nitro_with,
};
pub use proof::parse_prover_proof;
pub use tee::{
    HEADER_CIPHERTEXT, HEADER_ENC, HEADER_VERSION, TEE_VERSION, gpu_nonce, hex_decode, hex_encode,
    report_data, request_aad,
};

const MAX_REPLY_BYTES: usize = 1024 * 1024;
const INITIAL_POLL_MS: u64 = 25;
const MAX_WAIT_SECS: u64 = 20 * 60;
const HTTP_TOO_MANY: u16 = 429;
const HTTP_NOT_FOUND: u16 = 404;
const HTTP_SERVER_ERROR: u16 = 500;
const JOB_ID_LIMIT: usize = 256;

/// How the transport delivers one prover call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Delivery {
    /// `POST` with header `X-Sync: true`.
    Sync,
    /// `POST` with header `X-Async: true`.
    Async,
    /// `GET` of a queued job. No delivery header.
    Status,
}

/// Destination identity for one prover HTTP call.
///
/// The transport reads [`Self::delivery`] to set `X-Sync` or `X-Async`. The
/// path and query are also on the [`rest::Request`], so a transport that only
/// forwards those still reaches the right URL, but it must set the header or
/// the prover will not treat the call as synchronous.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProverDestination {
    /// Base URL. It has no userinfo and no query or fragment.
    pub endpoint: ProverEndpoint,
    /// Header the transport adds.
    pub delivery: Delivery,
}

/// HTTP client for the Helius remote prover.
///
/// The witness is posted as JSON, unchanged apart from that encoding.
/// `confidential_transfer_witness` builds that JSON from notes and Merkle
/// paths when the `poseidon` feature is on.
pub struct PrivacyProverClient<T> {
    transport: T,
    endpoint: ProverEndpoint,
    /// Sends the witness in the clear to a non-loopback prover.
    pub allow_unauthenticated_remote_prover: bool,
    poll_interval: Duration,
    max_poll_interval: Duration,
    max_wait: Duration,
}

impl<T> PrivacyProverClient<T> {
    /// Creates a client for `endpoint`, for example `http://127.0.0.1:3001`
    /// or `https://devnet.helius-rpc.com/v1/spr`.
    ///
    /// # Errors
    /// Rejects a URL that is not `http` or `https`, that has userinfo, that
    /// carries any query or fragment, or whose host or port does not parse.
    /// An `api-key` therefore cannot ride in the origin, and it belongs on
    /// the transport as a header.
    pub fn new(endpoint: &str, transport: T) -> Result<Self, Error> {
        Ok(Self {
            transport,
            endpoint: ProverEndpoint::parse(endpoint)?,
            allow_unauthenticated_remote_prover: false,
            poll_interval: Duration::from_millis(INITIAL_POLL_MS),
            max_poll_interval: Duration::from_secs(1),
            max_wait: Duration::from_secs(MAX_WAIT_SECS),
        })
    }

    /// Allows a cleartext witness to a remote `https` prover.
    #[must_use]
    pub fn allow_unauthenticated_remote_prover(mut self, allow: bool) -> Self {
        self.allow_unauthenticated_remote_prover = allow;
        self
    }

    /// Sets the status-poll cadence and the wall-clock bound.
    ///
    /// The first gap is `interval`. Later gaps double until `max_interval`.
    /// `max_wait` bounds the whole poll, including those gaps.
    #[must_use]
    pub fn with_poll(
        mut self,
        interval: Duration,
        max_interval: Duration,
        max_wait: Duration,
    ) -> Self {
        self.poll_interval = interval;
        self.max_poll_interval = max_interval.max(interval);
        self.max_wait = max_wait;
        self
    }
}

impl<T> std::fmt::Debug for PrivacyProverClient<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PrivacyProverClient")
            .field("endpoint", &self.endpoint)
            .field(
                "allow_unauthenticated_remote_prover",
                &self.allow_unauthenticated_remote_prover,
            )
            .field("max_wait", &self.max_wait)
            .finish_non_exhaustive()
    }
}

impl<T> PrivacyProverClient<T>
where
    T: rest::Transport<ProverDestination>,
{
    /// Asks the prover to prove `circuit` for `witness` and returns the compressed proof.
    ///
    /// `circuit` is the proving-key name, with or without `.key`, for example
    /// `transfer_confidential_1_2`. `witness` is encoded with `serde_json` and
    /// posted unchanged apart from that encoding.
    ///
    /// The wait inside status polling uses [`std::thread::sleep`]. Run `prove`
    /// on a blocking executor. A dropped future does not cancel an exchange the
    /// transport has already started sleeping outside of.
    ///
    /// # Errors
    /// Rejects a remote cleartext call, a bad circuit name, a proving-key
    /// mismatch, a prover HTTP error, or a poll that exceeds the wait bound.
    pub async fn prove(&self, circuit: &str, witness: &Value) -> Result<Proof, Error> {
        self.prove_route(circuit, witness, ProofRoute::Complete)
            .await
    }

    /// Posts the same witness to `/prove/{circuit}/indexed`.
    ///
    /// A `404` whose body has no `code` means this prover was not started with
    /// an indexer. A coded `404` is reported as [`ProverFailure::Http`]. A
    /// queued job is polled at `/prove/{circuit}/status`, the key status
    /// route the indexed reply names.
    ///
    /// # Errors
    /// Rejects a remote cleartext call, a bad circuit name, a proving-key
    /// mismatch, a prover HTTP error, or a poll that exceeds the wait bound.
    pub async fn prove_indexed(&self, circuit: &str, witness: &Value) -> Result<Proof, Error> {
        self.prove_route(circuit, witness, ProofRoute::Indexed)
            .await
    }

    /// `GET /attestation?nonce=` for the 32-byte client nonce, without
    /// verifying the reply.
    ///
    /// The body is the prover's attestation document, returned raw. This
    /// call verifies nothing. Replay protection exists only when the caller
    /// draws the nonce from a cryptographic generator for this one attempt
    /// and passes that same value to `verify_attestation` immediately after:
    /// a repeated nonce re-fetches a byte-identical, cacheable URL, and a
    /// body captured from an earlier attempt with that nonce still verifies
    /// within its policy age window and certificate validity.
    /// A raw fetch therefore does not prevent a replay on its own. Prefer
    /// `PrivacyProverClient::attest` with the `tee` feature, which draws the
    /// nonce and verifies in one step.
    ///
    /// # Errors
    /// Rejects a remote cleartext call, a non-success status, or a body that
    /// is not JSON.
    pub async fn fetch_unverified_attestation(&self, nonce: &[u8; 32]) -> Result<Value, Error> {
        self.require_transport()?;
        let reply = self
            .transport
            .exchange(rest::Request {
                service: ProverDestination {
                    endpoint: self.endpoint.clone(),
                    delivery: Delivery::Status,
                },
                method: Method::Get,
                path_segments: vec!["attestation".to_owned()],
                query: vec![("nonce".to_owned(), hex_encode(nonce))],
                body: Body::Absent,
                max_reply_bytes: MAX_REPLY_BYTES,
            })
            .await
            .map_err(|_| ProverFailure::Transport)?;

        if !(200..300).contains(&reply.status) {
            return Err(ProverFailure::Http(reply.status).into());
        }

        serde_json::from_slice(&reply.body).map_err(|_| ProverFailure::Response.into())
    }

    /// Runs one attestation round: draws the challenge, fetches the quote,
    /// and verifies it, in a single attempt.
    ///
    /// The 32-byte nonce is drawn from `rng`, a caller-supplied cryptographic
    /// generator, for every call. A caller-chosen nonce is not accepted here,
    /// and an entropy failure returns an error before the transport is
    /// touched instead of continuing with a zeroed or repeated nonce. The
    /// quote is then verified against `policy` at the time returned by the
    /// caller's `now_secs` callback, sampled after the fetch completes:
    /// certificate validity, measurements, the optional HPKE pin, an exact
    /// nonce match that pins the quote to this challenge, and the policy's
    /// age window over the document's signed timestamp. Keep the generator
    /// freshly seeded and never deliberately reset it, the way
    /// [`seal_prover_request`] expects.
    ///
    /// There is no attestation cache and no automatic reuse lifetime: call
    /// `attest` again for a new session, and decide any later reuse from
    /// [`VerifiedNitro::timestamp_ms`]. The transport still owns HTTP
    /// headers, so the caller attaches `Spr-Tee` and `Spr-Tee-Enc` to the
    /// requests this session seals.
    ///
    /// # Errors
    /// Rejects an entropy failure from `rng`, a remote cleartext call, a
    /// prover HTTP or transport error, a body that is not JSON, and any
    /// document [`verify_attestation`] would reject, including one minted
    /// for a different or replayed challenge.
    #[cfg(feature = "tee")]
    pub async fn attest<R, F>(
        &self,
        rng: &mut R,
        policy: &NitroPolicy,
        now_secs: F,
    ) -> Result<VerifiedNitro, Error>
    where
        R: RngCore + CryptoRng,
        F: FnOnce() -> u64,
    {
        self.require_transport()?;
        let mut nonce = [0u8; 32];

        // Security: a zeroed or repeated nonce would make the quote URL and
        // the report_data binding replayable, so an entropy failure returns
        // before any request instead of continuing with a known nonce.
        if rng.try_fill_bytes(&mut nonce).is_err() {
            return Err(Error::Attestation("entropy"));
        }

        let body = self.fetch_unverified_attestation(&nonce).await?;
        verify_attestation(&body, &nonce, policy, now_secs())
    }

    async fn prove_route(
        &self,
        circuit: &str,
        witness: &Value,
        route: ProofRoute,
    ) -> Result<Proof, Error> {
        self.require_transport()?;
        let name = circuit_name(circuit)?;
        require_known_circuit(name)?;
        let body = serde_json::to_vec(witness).map_err(|_| ProverFailure::ProofParse)?;
        self.prove_known(name, body, route).await
    }

    fn require_transport(&self) -> Result<(), Error> {
        if self.endpoint.is_loopback() {
            return Ok(());
        }

        if !self.allow_unauthenticated_remote_prover {
            return Err(ProverFailure::Cleartext.into());
        }

        if !self.endpoint.origin().starts_with("https://") {
            return Err(ProverFailure::Config.into());
        }

        Ok(())
    }

    /// Posts once in sync mode, then once in async mode when the prover is busy.
    ///
    /// The hosted prover answers 429 under load. The second post asks for a job
    /// id instead of holding the connection open.
    async fn prove_known(
        &self,
        name: &str,
        body: Vec<u8>,
        route: ProofRoute,
    ) -> Result<Proof, Error> {
        let mut queued = false;

        for _ in 0..2 {
            let reply = self
                .exchange(
                    delivery_for(queued),
                    Method::Post,
                    name,
                    None,
                    Body::Json(body.clone()),
                    route,
                )
                .await?;

            if let Some(proof) = self
                .interpret_post(name, &reply, &mut queued, route)
                .await?
            {
                return Ok(proof);
            }
        }

        Err(ProverFailure::Http(HTTP_TOO_MANY).into())
    }

    async fn interpret_post(
        &self,
        name: &str,
        reply: &solana_kit_client::Reply,
        queued: &mut bool,
        route: ProofRoute,
    ) -> Result<Option<Proof>, Error> {
        if reply.status == HTTP_TOO_MANY {
            *queued = true;
            return Ok(None);
        }

        if route == ProofRoute::Indexed
            && reply.status == HTTP_NOT_FOUND
            && !has_error_code(&reply.body)
        {
            return Err(ProverFailure::Indexer.into());
        }

        if !(200..300).contains(&reply.status) {
            return Err(ProverFailure::Http(reply.status).into());
        }

        let value =
            serde_json::from_slice::<Value>(&reply.body).map_err(|_| ProverFailure::Response)?;

        if let Some(job) = queued_job(&value) {
            return Ok(Some(self.poll(name, job, route).await?));
        }

        Ok(Some(parse_prover_proof(&value, name)?))
    }

    /// Polls an async job until it completes or the wait bound passes.
    ///
    /// Client errors are final. Server errors wait and retry, because the
    /// prover returns them while a job is still queued.
    async fn poll(&self, circuit: &str, job_id: &str, route: ProofRoute) -> Result<Proof, Error> {
        if !is_job_id(job_id) {
            return Err(ProverFailure::Job.into());
        }

        let deadline = Instant::now() + self.max_wait;
        let mut interval = self.poll_interval;

        loop {
            if Instant::now() >= deadline {
                return Err(ProverFailure::Timeout.into());
            }

            let reply = self
                .exchange(
                    Delivery::Status,
                    Method::Get,
                    circuit,
                    Some(job_id),
                    Body::Absent,
                    route,
                )
                .await?;

            match read_status(circuit, &reply)? {
                Poll::Retry => interval = self.wait(interval, deadline)?,
                Poll::Done(proof) => return Ok(proof),
            }
        }
    }

    fn wait(&self, interval: Duration, deadline: Instant) -> Result<Duration, Error> {
        let now = Instant::now();

        if now >= deadline {
            return Err(ProverFailure::Timeout.into());
        }

        let remaining = deadline.saturating_duration_since(now);
        std::thread::sleep(interval.min(remaining));
        Ok(interval.saturating_mul(2).min(self.max_poll_interval))
    }

    async fn exchange(
        &self,
        delivery: Delivery,
        method: Method,
        circuit: &str,
        job_id: Option<&str>,
        body: Body,
        route: ProofRoute,
    ) -> Result<solana_kit_client::Reply, Error> {
        let mut segments = vec!["prove".to_owned(), circuit.to_owned()];

        // The indexed route posts to /prove/{circuit}/indexed, but a queued
        // job is polled at the plain key status route: the prover registers
        // /prove/{key}, /prove/{key}/indexed and /prove/{key}/status, and no
        // path carries both segments.
        if route == ProofRoute::Indexed && delivery != Delivery::Status {
            segments.push("indexed".to_owned());
        }

        if delivery == Delivery::Status {
            segments.push("status".to_owned());
        }

        let query = job_id
            .map(|job| vec![("jobId".to_owned(), job.to_owned())])
            .unwrap_or_default();
        let reply = self
            .transport
            .exchange(rest::Request {
                service: ProverDestination {
                    endpoint: self.endpoint.clone(),
                    delivery,
                },
                method,
                path_segments: segments,
                query,
                body,
                max_reply_bytes: MAX_REPLY_BYTES,
            })
            .await
            .map_err(|_| ProverFailure::Transport)?;

        if reply.body.len() > MAX_REPLY_BYTES {
            return Err(ProverFailure::Response.into());
        }

        Ok(reply)
    }
}

impl TransactBody {
    /// Replaces the Groth16 proof with one returned by [`PrivacyProverClient::prove`].
    #[must_use]
    pub fn with_proof(mut self, proof: Proof) -> Self {
        self.proof = proof;
        self
    }
}

fn circuit_name(circuit: &str) -> Result<&str, Error> {
    let name = circuit.strip_suffix(".key").unwrap_or(circuit);

    if name.is_empty()
        || name.contains('/')
        || name.contains('\\')
        || name.contains("..")
        || name == "."
    {
        return Err(ProverFailure::Circuit.into());
    }
    Ok(name)
}

fn queued_job(value: &Value) -> Option<&str> {
    let object = value.as_object()?;

    if object.contains_key("proof") {
        return None;
    }

    object.get("jobId").and_then(Value::as_str)
}

#[cfg(all(test, feature = "tee"))]
mod attest_tests;
#[cfg(test)]
mod tests;

fn is_job_id(job_id: &str) -> bool {
    !job_id.is_empty()
        && job_id.len() <= JOB_ID_LIMIT
        && job_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-')
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ProofRoute {
    Complete,
    Indexed,
}

fn has_error_code(body: &[u8]) -> bool {
    serde_json::from_slice::<Value>(body)
        .ok()
        .and_then(|value| value.as_object().map(|object| object.contains_key("code")))
        .unwrap_or(false)
}

fn require_known_circuit(name: &str) -> Result<(), Error> {
    if proving_key_sha256(name).is_none() {
        return Err(ProverFailure::UnknownCircuit.into());
    }

    Ok(())
}

fn delivery_for(queued: bool) -> Delivery {
    if queued {
        Delivery::Async
    } else {
        Delivery::Sync
    }
}

enum Poll {
    Retry,
    Done(Proof),
}

fn read_status(circuit: &str, reply: &solana_kit_client::Reply) -> Result<Poll, Error> {
    if (400..500).contains(&reply.status) {
        return Err(ProverFailure::Http(reply.status).into());
    }

    if reply.status >= HTTP_SERVER_ERROR {
        return Ok(Poll::Retry);
    }

    let value =
        serde_json::from_slice::<Value>(&reply.body).map_err(|_| ProverFailure::Response)?;
    let Some(object) = value.as_object() else {
        return Err(ProverFailure::Response.into());
    };

    match object.get("status").and_then(Value::as_str) {
        Some("failed") => Err(ProverFailure::Server.into()),
        Some("completed") => {
            let result = object.get("result").unwrap_or(&value);
            Ok(Poll::Done(parse_prover_proof(result, circuit)?))
        }
        _ => Ok(Poll::Retry),
    }
}

impl From<ProverFailure> for Error {
    fn from(failure: ProverFailure) -> Self {
        Self::Prover(failure)
    }
}
