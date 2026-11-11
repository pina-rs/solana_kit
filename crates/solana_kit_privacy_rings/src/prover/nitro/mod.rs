//! AWS Nitro COSE attestation documents.
//!
//! `verify_nitro` checks the COSE_Sign1 signature, the certificate chain back
//! to the AWS Nitro Enclaves root, the PCR pins, and the `report_data`
//! binding. The `tee` feature is what makes an attested session more than a
//! header name.

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
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NitroPolicy {
    /// Images this caller will encrypt to.
    pub measurements: Vec<NitroMeasurement>,
    /// Optional HPKE key pin. A KMS deployment can keep the same key across boots.
    pub hpke_public_key: Option<[u8; 32]>,
}

/// A document that passed the chain, the pins, and the session binding.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifiedNitro {
    /// HPKE public key later requests are sealed to.
    pub hpke_public_key: [u8; 32],
    /// PCR0 of the enclave image.
    pub image_id: [u8; 48],
    /// 64-byte report value carried in `user_data`.
    pub report_data: [u8; 64],
}

/// Verifies `document` against the embedded AWS Nitro Enclaves root.
///
/// `document` is the COSE_Sign1 bytes from `evidence.document`. `now_secs` is
/// the unix time used for certificate validity.
///
/// # Errors
/// Rejects a document whose signature, chain, measurements, nonce, or
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
/// Rejects a document whose signature, chain, measurements, nonce, or
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
    })
}

/// Reads a `GET /attestation` body and verifies the Nitro document inside it.
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
