//! Isolated client cases awaiting combined host-registry integration.

use solana_kit_core::{
    AccountRole, Address,
    codecs::{Decoder, Encoder, Limits},
};
use solana_kit_system::*;

const SOURCE: Address = Address::from_bytes([1; 32]);
const DESTINATION: Address = Address::from_bytes([2; 32]);
const LIMITS: Limits = Limits::new(256, 64);

/// Exercises positional privileges, duplicate positions and strict parser rejection.
pub fn builder_parser() -> Result<(), &'static str> {
    let data = TransferSol { amount: u64::MAX };
    let mut instruction = data
        .instruction(
            TransferSolAccounts {
                source: SOURCE,
                destination: SOURCE,
            },
            LIMITS,
        )
        .map_err(|_| "transfer builder")?;

    if instruction.accounts.len() != 2
        || instruction.accounts[0].address != instruction.accounts[1].address
    {
        return Err("duplicate positions lost");
    }

    if parse_instruction(&instruction, LIMITS) != Ok(SystemInstructionData::TransferSol(data)) {
        return Err("canonical parser");
    }

    instruction.accounts[0].role = AccountRole::Writable;

    if parse_instruction(&instruction, LIMITS) != Err(ClientError::AccountRole(0)) {
        return Err("missing signer accepted");
    }

    Ok(())
}

/// Exercises initialized nonce identity, exact u64 fees and invalid state tags.
pub fn nonce_snapshot() -> Result<(), &'static str> {
    let data = Nonce {
        version: NonceVersion::Current,
        state: NonceState::Initialized(NonceData {
            authority: DESTINATION,
            blockhash: solana_kit_core::Hash::from_bytes([3; 32]),
            lamports_per_signature: u64::MAX,
        }),
    };
    let mut bytes = NonceCodec
        .encode(&data, NONCE_SIZE)
        .map_err(|_| "nonce encode")?;

    if NonceCodec.decode(&bytes, LIMITS) != Ok(data) || data.durable_data().is_none() {
        return Err("nonce snapshot roundtrip");
    }

    bytes[4] = 2;

    if NonceCodec.decode(&bytes, LIMITS).is_ok() {
        return Err("invalid nonce state accepted");
    }

    Ok(())
}
