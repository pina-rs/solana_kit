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
//! TypeScript client requires an attested TEE session for every remote prover
//! and encrypts the witness. That session is not implemented here. The flag
//! sends the witness as ordinary JSON, which a local prover accepts and a
//! TEE-only gateway rejects. A remote URL must be `https`.

mod endpoint;
mod keys;
mod proof;

use std::time::{Duration, Instant};

use serde_json::Value;
use solana_kit_client::rest::{self, Body, Method};

use crate::encode::{Proof, TransactBody};
use crate::error::{Error, ProverFailure};
pub use endpoint::ProverEndpoint;
pub use keys::proving_key_sha256;
pub use proof::parse_prover_proof;

const MAX_REPLY_BYTES: usize = 1024 * 1024;

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
    /// Base URL. It has no userinfo and no `api-key` query parameter.
    pub endpoint: ProverEndpoint,
    /// Header the transport adds.
    pub delivery: Delivery,
}

/// HTTP client for the Helius remote prover.
///
/// The witness is posted as JSON, unchanged apart from that encoding. This
/// crate does not assemble the witness: the TypeScript and Rust SDKs build it
/// with Poseidon, and inventing those field elements would produce a proof the
/// program rejects.
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
    /// Rejects a URL that is not `http` or `https`, or that contains an
    /// `api-key` query parameter.
    pub fn new(endpoint: &str, transport: T) -> Result<Self, Error> {
        Ok(Self {
            transport,
            endpoint: ProverEndpoint::parse(endpoint)?,
            allow_unauthenticated_remote_prover: false,
            poll_interval: Duration::from_millis(25),
            max_poll_interval: Duration::from_secs(1),
            max_wait: Duration::from_secs(20 * 60),
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
        self.require_transport()?;
        let name = circuit_name(circuit)?;
        if proving_key_sha256(name).is_none() {
            return Err(ProverFailure::UnknownCircuit.into());
        }
        let body = serde_json::to_vec(witness).map_err(|_| ProverFailure::ProofParse)?;
        let mut queued = false;
        for _ in 0..2 {
            let delivery = if queued {
                Delivery::Async
            } else {
                Delivery::Sync
            };
            let reply = self
                .exchange(delivery, Method::Post, name, None, Body::Json(body.clone()))
                .await?;
            if reply.status == 429 && !queued {
                queued = true;
                continue;
            }
            if !(200..300).contains(&reply.status) {
                return Err(ProverFailure::Http(reply.status).into());
            }
            let value = serde_json::from_slice::<Value>(&reply.body)
                .map_err(|_| ProverFailure::Response)?;
            if let Some(job) = queued_job(&value) {
                return self.poll(name, job).await;
            }
            return Ok(parse_prover_proof(&value, name)?);
        }
        Err(ProverFailure::Http(429).into())
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

    async fn poll(&self, circuit: &str, job_id: &str) -> Result<Proof, Error> {
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
                )
                .await?;
            if (400..500).contains(&reply.status) {
                return Err(ProverFailure::Http(reply.status).into());
            }
            if reply.status >= 500 {
                interval = self.wait(interval, deadline)?;
                continue;
            }
            let value = serde_json::from_slice::<Value>(&reply.body)
                .map_err(|_| ProverFailure::Response)?;
            let Some(object) = value.as_object() else {
                return Err(ProverFailure::Response.into());
            };
            match object.get("status").and_then(Value::as_str) {
                Some("failed") => return Err(ProverFailure::Server.into()),
                Some("completed") => {
                    let result = object.get("result").unwrap_or(&value);
                    return Ok(parse_prover_proof(result, circuit)?);
                }
                _ => interval = self.wait(interval, deadline)?,
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
    ) -> Result<solana_kit_client::Reply, Error> {
        let mut segments = vec!["prove".to_owned(), circuit.to_owned()];
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

#[cfg(test)]
mod tests;

fn is_job_id(job_id: &str) -> bool {
    !job_id.is_empty()
        && job_id.len() <= 256
        && job_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-')
}

impl From<ProverFailure> for Error {
    fn from(failure: ProverFailure) -> Self {
        Self::Prover(failure)
    }
}
