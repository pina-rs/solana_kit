//! AWS Nitro COSE attestation documents.
//!
//! `verify_nitro` checks the COSE_Sign1 signature, the certificate chain back
//! to the AWS Nitro Enclaves root, the PCR pins, the `report_data` binding,
//! and the document's signed age against the policy window. The `tee`
//! feature is what makes an attested session more than a header name.

mod document;
mod scan;

use serde_json::Value;

use crate::Error;
use crate::prover::hex_decode;
use crate::prover::tee::report_data;
use document::inspect_document;

const AWS_NITRO_ROOT: &[u8] = include_bytes!("aws_nitro_enclaves_root_g1.der");

/// PCR0, PCR1, and PCR2 the caller accepts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NitroMeasurement {
    /// Enclave image.
    pub pcr0: [u8; 48],
    /// Kernel and bootstrap.
    pub pcr1: [u8; 48],
    /// Application.
    pub pcr2: [u8; 48],
}

/// Pins for one Nitro prover.
///
/// Time is caller-owned: every bound is evaluated against the `now_secs` the
/// caller passes to [`verify_nitro`], and no ambient clock is read here.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NitroPolicy {
    /// Images this caller will encrypt to.
    pub measurements: Vec<NitroMeasurement>,
    /// Optional HPKE key pin. A KMS deployment can keep the same key across boots.
    pub hpke_public_key: Option<[u8; 32]>,
    /// Maximum age of the document's signed timestamp, in seconds. A document
    /// older than this at the caller's `now_secs` is rejected, even while its
    /// certificates are still valid. Zero rejects every timestamp strictly
    /// older than the caller's clock; it never disables the check. Future
    /// timestamps remain subject to `max_future_secs`.
    pub max_age_secs: u64,
    /// How far ahead of the caller's `now_secs` the signed timestamp may sit,
    /// in seconds. Zero rejects any timestamp in the future.
    pub max_future_secs: u64,
}

/// A document that passed the chain, the pins, the session binding, and the
/// policy age window.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifiedNitro {
    /// HPKE public key later requests are sealed to.
    pub hpke_public_key: [u8; 32],
    /// PCR0 of the enclave image.
    pub image_id: [u8; 48],
    /// 64-byte report value carried in `user_data`.
    pub report_data: [u8; 64],
    /// Signed NSM timestamp of the document, in milliseconds since the
    /// epoch. The policy window held at the caller's `now_secs` when this
    /// value was verified; deciding how long a verified session may be
    /// reused stays with the caller.
    pub timestamp_ms: u64,
}

/// Verifies `document` against the embedded AWS Nitro Enclaves root.
///
/// `document` is the COSE_Sign1 bytes from `evidence.document`. `now_secs` is
/// the caller-owned unix time: it drives certificate validity and the policy
/// age window over the document's signed timestamp, which is exposed on
/// [`VerifiedNitro::timestamp_ms`]. Prefer
/// [`PrivacyProverClient::attest`](crate::PrivacyProverClient::attest) for
/// normal Nitro use; it draws and binds the challenge nonce itself.
///
/// # Errors
/// Rejects a document whose signature, chain, measurements, age, nonce, or
/// `report_data` do not match.
pub fn verify_nitro(
    document: &[u8],
    hpke_public_key: &[u8; 32],
    nonce: &[u8; 32],
    policy: &NitroPolicy,
    now_secs: u64,
) -> Result<VerifiedNitro, Error> {
    verify_nitro_with(
        document,
        hpke_public_key,
        nonce,
        policy,
        now_secs,
        AWS_NITRO_ROOT,
    )
}

/// [`verify_nitro`] with a caller-supplied trust anchor.
///
/// The published CBOR cases use a test root. Production callers use
/// [`verify_nitro`].
///
/// # Errors
/// Rejects a document whose signature, chain, measurements, age, nonce, or
/// `report_data` do not match.
pub fn verify_nitro_with(
    document: &[u8],
    hpke_public_key: &[u8; 32],
    nonce: &[u8; 32],
    policy: &NitroPolicy,
    now_secs: u64,
    root: &[u8],
) -> Result<VerifiedNitro, Error> {
    let measured = inspect_document(document, root, now_secs)?;

    // Security: the timestamp is signed payload data, so it is fresh
    // evidence only once the quote, the chain, and the certificate windows
    // have authenticated it. Only then is its age compared with the caller's
    // clock; a certificate being valid never means the document is fresh.
    check_document_age(measured.timestamp_ms, policy, now_secs)?;

    let measurement = NitroMeasurement {
        pcr0: measured.pcr0,
        pcr1: measured.pcr1,
        pcr2: measured.pcr2,
    };

    if !policy.measurements.contains(&measurement) {
        return Err(Error::Attestation("measurement"));
    }

    if measured.public_key.as_deref() != Some(hpke_public_key.as_slice()) {
        return Err(Error::Attestation("hpke"));
    }

    if measured.nonce.as_deref() != Some(nonce.as_slice()) {
        return Err(Error::Attestation("nonce"));
    }

    if policy
        .hpke_public_key
        .is_some_and(|pinned| pinned != *hpke_public_key)
    {
        return Err(Error::Attestation("hpke pin"));
    }
    let bound = report_data(nonce, hpke_public_key, None);

    if measured.report_data != bound {
        return Err(Error::Attestation("report"));
    }

    Ok(VerifiedNitro {
        hpke_public_key: *hpke_public_key,
        image_id: measured.pcr0,
        report_data: measured.report_data,
        timestamp_ms: measured.timestamp_ms,
    })
}

/// Compares the signed timestamp with the caller's clock under the policy
/// window.
///
/// Both sides are converted to milliseconds so a `max_age_secs` bound is
/// exact at the edge, and the arithmetic runs in `u128`: a `u64` seconds
/// value times 1,000 and summed with the same-width skew cannot approach
/// `u128`'s range, so no step can overflow, and `saturating_sub` keeps a
/// caller clock below the age limit from rejecting every document. A zero
/// `max_age_secs` rejects every timestamp strictly older than the caller's
/// clock; it never disables the check. The future bound remains separate.
fn check_document_age(timestamp_ms: u64, policy: &NitroPolicy, now_secs: u64) -> Result<(), Error> {
    const MS_PER_SEC: u128 = 1_000;

    let now_ms = u128::from(now_secs) * MS_PER_SEC;
    let oldest_ms = now_ms.saturating_sub(u128::from(policy.max_age_secs) * MS_PER_SEC);
    let newest_ms = now_ms + u128::from(policy.max_future_secs) * MS_PER_SEC;
    let timestamp = u128::from(timestamp_ms);

    if timestamp < oldest_ms {
        return Err(Error::Attestation("age"));
    }

    if timestamp > newest_ms {
        return Err(Error::Attestation("future"));
    }

    Ok(())
}

/// Reads a `GET /attestation` body and verifies the Nitro document inside it.
///
/// The nonce must be a fresh value the caller drew for this one attempt and
/// sent with the fetch; a nonce that repeats an earlier attempt accepts a
/// replayed body within its policy age window and certificate validity. Prefer
/// [`PrivacyProverClient::attest`](crate::PrivacyProverClient::attest),
/// which draws the nonce, fetches, and verifies in one step.
///
/// # Errors
/// Rejects a body that is not an `aws-nitro` attestation, a GPU token on
/// Nitro, or a document [`verify_nitro`] would reject.
pub fn verify_attestation(
    body: &Value,
    nonce: &[u8; 32],
    policy: &NitroPolicy,
    now_secs: u64,
) -> Result<VerifiedNitro, Error> {
    let attestation = body.get("attestation").unwrap_or(body);

    if attestation.get("platform").and_then(Value::as_str) != Some("aws-nitro") {
        return Err(Error::Attestation("platform"));
    }

    if attestation.get("gpu").is_some_and(|gpu| !gpu.is_null()) {
        return Err(Error::Attestation("gpu"));
    }

    let key = attestation
        .get("hpke_public_key")
        .and_then(Value::as_str)
        .ok_or(Error::Attestation("hpke"))?;
    let key = hex_decode(key)?
        .try_into()
        .map_err(|_| Error::Attestation("hpke"))?;
    let document = attestation
        .pointer("/evidence/document")
        .and_then(Value::as_str)
        .ok_or(Error::Attestation("document"))?;
    verify_nitro(&hex_decode(document)?, &key, nonce, policy, now_secs)
}

#[cfg(test)]
mod tests;
