//! Three isolated version-one cases, awaiting integration into the host registry.

use crate::{CaseResult, case, require};
use alloc::vec::Vec;
use solana_kit_core::{
    Address, Hash,
    crypto::Keypair,
    transaction::{Error, HeapSize, Lifetime, Message, Transaction, V1Config},
};

const HEAP_BYTES: u32 = 32768;

/// Runs the candidate cases without changing the established shared suite.
#[must_use]
pub fn cases() -> [CaseResult; 3] {
    [
        case("transaction-v1-message-first", message_first()),
        case("transaction-v1-inline-config", inline_config()),
        case("transaction-v1-signature-suffix", signature_suffix()),
    ]
}

fn lifetime() -> Lifetime {
    Lifetime::Blockhash {
        blockhash: Hash::from_bytes([2; 32]),
        last_valid_block_height: 42,
    }
}

fn message_first() -> Result<(), &'static str> {
    let message = Message::compile_v1(
        Address::from_bytes([3; 32]),
        lifetime(),
        &[],
        V1Config::default(),
    )
    .map_err(|_| "v1 compilation failed")?;
    let mut literal = alloc::vec![129, 1, 0, 0, 0, 0, 0, 0];
    literal.extend([2; 32]);
    literal.extend([0, 1]);
    literal.extend([3; 32]);
    require(message.bytes() == literal, "v1 literal message mismatch")?;
    literal.extend([0; 64]);
    let decoded = Transaction::decode(&literal).map_err(|_| "v1 literal envelope rejected")?;
    require(
        decoded.encode(false).map_err(|_| "v1 encoding failed")? == literal,
        "v1 exact received bytes changed",
    )
}

fn inline_config() -> Result<(), &'static str> {
    let config = V1Config::default()
        .with_priority_fee(u64::MAX)
        .with_heap_size(HeapSize::new(HEAP_BYTES).map_err(|_| "valid heap rejected")?);
    let message = Message::compile_v1(Address::from_bytes([3; 32]), lifetime(), &[], config)
        .map_err(|_| "v1 config compilation failed")?;
    let mut expected = Vec::from(u64::MAX.to_le_bytes());
    expected.extend(HEAP_BYTES.to_le_bytes());
    require(
        message.bytes().ends_with(&expected),
        "v1 config order mismatch",
    )?;
    let mut invalid = message.bytes().to_vec();
    invalid[4] = 1;
    require(
        Message::decode(&invalid) == Err(Error::InvalidConfig),
        "half priority mask accepted",
    )
}

fn signature_suffix() -> Result<(), &'static str> {
    let key = Keypair::from_seed(&[9; 32]);
    let message = Message::compile_v1(key.address(), lifetime(), &[], V1Config::default())
        .map_err(|_| "v1 signing message failed")?;
    let tx = Transaction::new(message)
        .sign(&key)
        .map_err(|_| "v1 signing failed")?;
    let mut wire = tx.encode(true).map_err(|_| "signed v1 encoding failed")?;
    require(
        wire.starts_with(tx.message().bytes()),
        "v1 envelope is not message-first",
    )?;
    tx.verify()
        .map_err(|_| "v1 signature verification failed")?;
    let signature = wire.last_mut().ok_or("empty v1 envelope")?;
    *signature ^= 1;
    require(
        Transaction::decode(&wire)
            .map_err(|_| "altered v1 decode failed")?
            .verify()
            == Err(Error::InvalidSignature),
        "v1 altered signature accepted",
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn candidate_v1_cases_pass_independently() {
        for case in super::cases() {
            assert!(case.outcome.is_ok(), "{case:?}");
        }
    }
}
