//! Wormhole v1 envelopes, header inspection and explicit signature reduction.
use alloc::{borrow::Cow, vec::Vec};
use solana_kit_core::codecs::{Decoder, Error, FixedBytes, Limits, Number, Reader};

/// Supported Wormhole envelope version.
pub const WORMHOLE_VAA_VERSION: u8 = 1;
/// Guardian index, 64 signature bytes and recovery ID.
pub const VAA_SIGNATURE_SIZE: usize = 66;
/// Reference helper's default reduced guardian count.
pub const DEFAULT_REDUCED_GUARDIAN_SET_SIZE: u8 = 5;
/// Wormhole emitter chain identifier for Pythnet.
pub const EMITTER_CHAIN_PYTHNET: u16 = 26;
const HEADER_SIZE: usize = 6;

/// One encoded guardian signature; parsing does not verify it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WormholeVaaSignature {
    /// Position in the referenced guardian set.
    pub guardian_index: u8,
    /// Raw recoverable ECDSA signature bytes.
    pub signature: [u8; 64],
    /// Recovery identifier, retained without authentication.
    pub recovery_id: u8,
}

/// A decoded v1 envelope whose payload borrows the caller's original bytes.
/// Signature entries retain their encoded order, including duplicates.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WormholeVaa<'a> {
    /// Encoded version, always [`WORMHOLE_VAA_VERSION`].
    pub version: u8,
    /// Index of the guardian set named by the envelope.
    pub guardian_set_index: u32,
    /// Ordered signature entries, without signature verification.
    pub signatures: Vec<WormholeVaaSignature>,
    /// Emitter's Unix timestamp in seconds.
    pub timestamp: u32,
    /// Emitter's deduplication nonce.
    pub nonce: u32,
    /// Wormhole chain identifier of the emitter.
    pub emitter_chain: u16,
    /// Emitter's exact 32-byte address.
    pub emitter_address: [u8; 32],
    /// Emitter's sequence number.
    pub sequence: u64,
    /// Requested consistency level.
    pub consistency_level: u8,
    /// Entire remaining payload, without an additional copy.
    pub payload: &'a [u8],
}

/// Reads only the guardian-set header field. No version or body is validated.
///
/// # Errors
/// Returns [`Error::Truncated`] when fewer than five bytes are available.
pub fn get_guardian_set_index(vaa: &[u8]) -> Result<u32, Error> {
    let bytes = vaa.get(1..5).ok_or(Error::Truncated)?;
    Number::<u32>::BE.decode(bytes, Limits::new(4, 0))
}

/// Reads only the encoded count. No version, signatures or body are validated.
///
/// # Errors
/// Returns [`Error::Truncated`] when the six-byte header is incomplete.
pub fn get_vaa_signature_count(vaa: &[u8]) -> Result<u8, Error> {
    vaa.get(HEADER_SIZE - 1).copied().ok_or(Error::Truncated)
}

/// Parses every v1 envelope field and borrows its remaining payload.
/// This validates structure, without authenticating guardians or the emitter.
///
/// # Errors
/// Rejects an unsupported version, truncation, exhausted byte/item limits or
/// failed signature-vector allocation. Each allocated signature claims one item.
pub fn parse_wormhole_vaa(bytes: &[u8], limits: Limits) -> Result<WormholeVaa<'_>, Error> {
    let mut reader = Reader::new(bytes, limits)?;
    let version = Number::<u8>::BE.read(&mut reader)?;

    if version != WORMHOLE_VAA_VERSION {
        return Err(Error::InvalidDiscriminator);
    }

    let guardian_set_index = Number::<u32>::BE.read(&mut reader)?;
    let count = usize::from(Number::<u8>::BE.read(&mut reader)?);
    reader.claim(count)?;
    let mut signatures = Vec::new();
    signatures
        .try_reserve_exact(count)
        .map_err(|_| Error::AllocationFailed)?;

    for _ in 0..count {
        signatures.push(WormholeVaaSignature {
            guardian_index: Number::<u8>::BE.read(&mut reader)?,
            signature: FixedBytes::<64>.read(&mut reader)?,
            recovery_id: Number::<u8>::BE.read(&mut reader)?,
        });
    }

    Ok(WormholeVaa {
        version,
        guardian_set_index,
        signatures,
        timestamp: Number::<u32>::BE.read(&mut reader)?,
        nonce: Number::<u32>::BE.read(&mut reader)?,
        emitter_chain: Number::<u16>::BE.read(&mut reader)?,
        emitter_address: FixedBytes::<32>.read(&mut reader)?,
        sequence: Number::<u64>::BE.read(&mut reader)?,
        consistency_level: Number::<u8>::BE.read(&mut reader)?,
        payload: reader.remaining(),
    })
}

/// Keeps the first `count` signatures and preserves the complete body and payload.
/// Returns a borrowed input when its encoded count already fits. Like the pinned
/// Dart helper, this does not validate the version or complete envelope body.
/// Use [`parse_wormhole_vaa`] separately when structural validation is required.
///
/// # Errors
/// Rejects a truncated header or signature region, exhausted input/output limits,
/// or allocation failure. A copied output claims one item per output byte.
pub fn trim_vaa_signatures(vaa: &[u8], count: u8, limits: Limits) -> Result<Cow<'_, [u8]>, Error> {
    let mut reader = Reader::new(vaa, limits)?;
    let current = get_vaa_signature_count(vaa)?;

    if count >= current {
        return Ok(Cow::Borrowed(vaa));
    }

    let body_start = HEADER_SIZE + usize::from(current) * VAA_SIGNATURE_SIZE;
    let body = vaa.get(body_start..).ok_or(Error::Truncated)?;
    let prefix_length = HEADER_SIZE + usize::from(count) * VAA_SIGNATURE_SIZE;
    let output_length = prefix_length + body.len();
    reader.claim(output_length)?;
    let mut output = Vec::new();
    output
        .try_reserve_exact(output_length)
        .map_err(|_| Error::AllocationFailed)?;
    output.extend_from_slice(&vaa[..prefix_length]);
    output.extend_from_slice(body);
    output[HEADER_SIZE - 1] = count;
    Ok(Cow::Owned(output))
}
