//! Legacy/v0 independent wire and OpenSSL signature fixtures.

use alloc::{vec, vec::Vec};
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Hash, Instruction,
    crypto::Keypair,
    transaction::{Error, Lifetime, LookupTable, Message, Transaction, Version},
};

use crate::require;

const SIGNATURE: [u8; 64] = [
    179, 0, 170, 254, 234, 124, 169, 9, 108, 48, 231, 29, 49, 65, 187, 140, 250, 66, 209, 172, 185,
    218, 153, 109, 237, 47, 94, 221, 29, 25, 48, 15, 7, 49, 230, 121, 66, 20, 16, 106, 134, 62,
    167, 158, 192, 234, 125, 116, 47, 157, 240, 177, 177, 156, 47, 219, 251, 178, 18, 117, 245,
    113, 167, 10,
];
const SEED: [u8; 32] = [
    157, 97, 177, 157, 239, 253, 90, 96, 186, 132, 74, 244, 146, 236, 44, 196, 68, 73, 197, 105,
    123, 50, 105, 25, 112, 59, 172, 3, 28, 174, 127, 96,
];

/// Explicit field layout audited against pinned Dart and official Solana.
fn legacy_bytes() -> Vec<u8> {
    let mut bytes = vec![1, 0, 1, 2];
    bytes.extend([1; 32]);
    bytes.extend([2; 32]);
    bytes.extend([3; 32]);
    bytes.extend([1, 1, 0, 0]);
    bytes
}

fn compile(
    version: Version,
    payer: Address,
    accounts: Vec<AccountMeta>,
    tables: &[LookupTable],
) -> Result<Message, &'static str> {
    Message::compile(
        version,
        payer,
        Lifetime::Blockhash {
            blockhash: Hash::from_bytes([3; 32]),
            last_valid_block_height: 42,
        },
        &[Instruction {
            program_address: Address::from_bytes([2; 32]),
            accounts,
            data: Vec::new(),
        }],
        tables,
    )
    .map_err(|_| "message compilation failed")
}

pub(crate) fn legacy_wire() -> Result<(), &'static str> {
    let message = compile(
        Version::Legacy,
        Address::from_bytes([1; 32]),
        Vec::new(),
        &[],
    )?;
    require(
        message.bytes() == legacy_bytes(),
        "legacy independent bytes mismatch",
    )?;
    let decoded = Message::decode(&legacy_bytes()).map_err(|_| "legacy decoding failed")?;
    require(
        decoded.bytes() == message.bytes(),
        "decoded signed bytes changed",
    )
}

pub(crate) fn lookup_positions() -> Result<(), &'static str> {
    let loaded = Address::from_bytes([4; 32]);
    let table = LookupTable {
        address: Address::from_bytes([6; 32]),
        addresses: vec![loaded],
    };
    let account = AccountMeta {
        address: loaded,
        role: AccountRole::Writable,
    };
    let message = compile(
        Version::V0,
        Address::from_bytes([1; 32]),
        vec![account, account],
        &[table],
    )?;
    require(
        message.instructions()[0].accounts() == [2, 2],
        "duplicate positions lost",
    )?;
    require(
        message.lookups()[0].writable_indices() == [0],
        "lookup index mismatch",
    )?;
    let mut expected = vec![128, 1, 0, 1, 2];
    expected.extend([1; 32]);
    expected.extend([2; 32]);
    expected.extend([3; 32]);
    expected.extend([1, 1, 2, 2, 2, 0, 1]);
    expected.extend([6; 32]);
    expected.extend([1, 0, 0]);
    require(message.bytes() == expected, "v0 independent bytes mismatch")
}

pub(crate) fn malformed_wire() -> Result<(), &'static str> {
    let fixture = legacy_bytes();

    for end in 0..fixture.len() {
        require(
            Message::decode(&fixture[..end]).is_err(),
            "truncated message accepted",
        )?;
    }

    let mut alias = vec![1, 0, 1, 130, 0];
    alias.extend(&fixture[4..]);
    require(
        Message::decode(&alias) == Err(Error::InvalidLength),
        "short-u16 alias accepted",
    )?;
    let mut external = fixture;
    external[2] = 0;
    require(
        Message::decode(&external).is_ok(),
        "builder restriction imposed on valid wire",
    )
}

pub(crate) fn openssl_signature() -> Result<(), &'static str> {
    let keypair = Keypair::from_seed(&SEED);
    let message = compile(Version::Legacy, keypair.address(), Vec::new(), &[])?;
    let transaction = Transaction::new(message)
        .sign(&keypair)
        .map_err(|_| "signing failed")?;
    require(
        transaction.signatures()[0].map(|signature| signature.to_bytes()) == Some(SIGNATURE),
        "OpenSSL signature mismatch",
    )?;
    transaction
        .verify()
        .map_err(|_| "signature verification failed")?;
    let mut packet = transaction
        .encode(true)
        .map_err(|_| "packet encoding failed")?;
    packet[1] ^= 1;
    let altered = Transaction::decode(&packet).map_err(|_| "altered packet decoding failed")?;
    require(
        altered.verify() == Err(Error::InvalidSignature),
        "altered signature accepted",
    )
}
