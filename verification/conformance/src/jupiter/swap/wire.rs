//! Independent v0 field bytes verify the SDK's promised transaction conversion.

use crate::require;
use alloc::{vec, vec::Vec};
use base64::{Engine, engine::general_purpose::STANDARD};
use solana_kit_core::{
    Hash,
    transaction::{Lifetime, Version},
};
use solana_kit_jupiter::decode_base64_swap_transaction;

pub(super) fn transactions() -> Result<(), &'static str> {
    let wire = wire_fixture();
    let encoded = STANDARD.encode(&wire);
    let transaction =
        decode_base64_swap_transaction(&encoded).map_err(|_| "base64 v0 transaction rejected")?;
    require(
        transaction.message().version() == Version::V0 && transaction.signatures() == [None],
        "transaction version or unsigned slot changed",
    )?;
    require(
        transaction.message().lifetime() == Lifetime::Unresolved(Hash::from_bytes([3; 32])),
        "wire decoding invented expiry metadata",
    )?;
    require(
        transaction
            .encode(false)
            .map_err(|_| "transaction reencoding failed")?
            == wire,
        "decoded transaction bytes changed",
    )?;
    require(
        transaction.encode(true).is_err(),
        "unsigned transaction gained a signature",
    )?;
    let unpadded = encoded.trim_end_matches('=');
    require(
        decode_base64_swap_transaction(unpadded).is_ok(),
        "reference unpadded base64 rejected",
    )?;
    signed_slots(&wire)?;
    invalid_wire(wire)
}

fn wire_fixture() -> Vec<u8> {
    let mut wire = vec![1];
    wire.extend([0; 64]);
    wire.extend([128, 1, 0, 1, 2]);
    wire.extend([1; 32]);
    wire.extend([2; 32]);
    wire.extend([3; 32]);
    wire.extend([1, 1, 0, 0, 0]);
    wire
}

fn signed_slots(wire: &[u8]) -> Result<(), &'static str> {
    let mut signed = wire.to_vec();
    signed[1..65].fill(7);
    let decoded = decode_base64_swap_transaction(&STANDARD.encode(&signed))
        .map_err(|_| "signed transaction rejected")?;
    require(
        decoded.signatures()[0].map(|signature| signature.to_bytes()) == Some([7; 64]),
        "signature bytes changed",
    )
}

fn invalid_wire(mut wire: Vec<u8>) -> Result<(), &'static str> {
    for encoded in ["%%%", "AQ==", "", "-_=="] {
        require(
            decode_base64_swap_transaction(encoded).is_err(),
            "invalid swap transaction accepted",
        )?;
    }

    wire.push(0);
    require(
        decode_base64_swap_transaction(&STANDARD.encode(wire)).is_err(),
        "transaction trailing bytes accepted",
    )?;
    let oversized = STANDARD.encode(vec![0; 4097]);
    require(
        decode_base64_swap_transaction(&oversized).is_err(),
        "oversized transaction allocated or accepted",
    )
}
