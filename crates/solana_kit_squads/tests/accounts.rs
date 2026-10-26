//! Every complete account layout has independent nonzero bytes and malformed snapshots.
#[path = "support/errors.rs"]
mod errors;
#[path = "support/wire.rs"]
mod support;

use solana_kit_core::{
    Address,
    accounts::{Account, AccountError, EncodedAccount},
    codecs::{Decoder, Encoder, Error, Limits},
};
use solana_kit_squads::*;
use std::fmt::Debug;

const LIMITS: Limits = Limits::new(16384, 8192);

fn snapshot(bytes: Vec<u8>) -> EncodedAccount {
    Account {
        address: Address::from_bytes([91; 32]),
        owner: PROGRAM,
        lamports: 1234,
        executable: false,
        space: u64::try_from(bytes.len()).unwrap(),
        rent_epoch: Some(17),
        data: bytes,
    }
}

fn verify<T: Debug + Eq>(
    codec: &(impl Encoder<T> + Decoder<T>),
    header: &str,
    payload: Vec<u8>,
    kind: AccountKind,
    decode: fn(EncodedAccount, Limits) -> Result<Account<T>, AccountError>,
) {
    let bytes = [support::hex(header), payload].concat();
    let value = codec.decode(&bytes, LIMITS).unwrap();
    assert_eq!(codec.encode(&value, LIMITS.max_bytes).unwrap(), bytes);
    assert_eq!(identify_account(&bytes).unwrap(), kind);
    let original = snapshot(bytes.clone());
    let decoded = decode(original.clone(), LIMITS).unwrap();
    assert_eq!(decoded.data, value);
    assert_eq!(decoded.address, original.address);

    assert_eq!(decoded.owner, original.owner);
    assert_eq!(decoded.lamports, original.lamports);
    assert_eq!(decoded.executable, original.executable);
    assert_eq!(decoded.space, original.space);
    assert_eq!(decoded.rent_epoch, original.rent_epoch);

    for end in 0..bytes.len() {
        assert!(codec.decode(&bytes[..end], LIMITS).is_err());
    }

    let mut changed = bytes.clone();
    changed.push(0);
    assert!(codec.decode(&changed, LIMITS).is_err());
    changed = bytes;
    changed[0] ^= 0x80;
    assert!(codec.decode(&changed, LIMITS).is_err());
    let mut foreign = original.clone();
    foreign.owner = Address::from_bytes([92; 32]);

    assert!(matches!(
        decode(foreign, LIMITS),
        Err(AccountError::OwnerMismatch { .. })
    ));
    let mut size = original.clone();
    size.space += 1;
    assert!(matches!(
        decode(size, LIMITS),
        Err(AccountError::DataSizeMismatch(_))
    ));
    assert!(decode(original.clone(), Limits::new(original.data.len() - 1, 8192)).is_err());
    let limited_succeeds = decode(original, Limits::new(16384, 0)).is_ok();
    assert_eq!(
        limited_succeeds,
        matches!(kind, AccountKind::Batch | AccountKind::ProgramConfig)
    );
}

fn amount() -> Vec<u8> {
    0x0102_0304_0506_0708u64.to_le_bytes().to_vec()
}
fn sequence(value: Vec<u8>) -> Vec<u8> {
    [vec![1, 0, 0, 0], value].concat()
}
fn blob(value: &[u8]) -> Vec<u8> {
    [
        u32::try_from(value.len()).unwrap().to_le_bytes().to_vec(),
        value.to_vec(),
    ]
    .concat()
}

fn message() -> Vec<u8> {
    [
        vec![1, 1, 1],
        vec![3, 0, 0, 0],
        vec![7; 32],
        vec![8; 32],
        vec![9; 32],
        vec![1, 0, 0, 0, 2],
        blob(&[0, 1]),
        blob(&[0xa1, 0xb2]),
        sequence([vec![10; 32], blob(&[3]), blob(&[4])].concat()),
    ]
    .concat()
}

#[cfg_attr(test, test)]
pub fn batch() {
    verify(
        &BatchCodec,
        "9cc2462c1658892c",
        [
            vec![7; 32],
            vec![8; 32],
            amount(),
            vec![253, 2, 252],
            3u32.to_le_bytes().to_vec(),
            1u32.to_le_bytes().to_vec(),
        ]
        .concat(),
        AccountKind::Batch,
        decode_batch,
    );
}

#[cfg_attr(test, test)]
pub fn vault_batch_transaction() {
    verify(
        &VaultBatchTransactionCodec,
        "c4792e240c13fc07",
        [vec![252], blob(&[1, 2]), message()].concat(),
        AccountKind::VaultBatchTransaction,
        decode_vault_batch_transaction,
    );
}

#[cfg_attr(test, test)]
pub fn config_transaction() {
    verify(
        &ConfigTransactionCodec,
        "5e080423718b8b70",
        [
            vec![7; 32],
            vec![8; 32],
            amount(),
            vec![252],
            support::actions(),
        ]
        .concat(),
        AccountKind::ConfigTransaction,
        decode_config_transaction,
    );
}

#[cfg_attr(test, test)]
pub fn multisig() {
    for rent in [vec![0], [vec![1], vec![9; 32]].concat()] {
        verify(
            &MultisigCodec,
            "e07479ba44a14fec",
            [
                vec![7; 32],
                vec![8; 32],
                vec![1, 0, 17, 0, 0, 0],
                amount(),
                8u64.to_le_bytes().to_vec(),
                rent,
                vec![252],
                sequence([vec![10; 32], vec![7]].concat()),
            ]
            .concat(),
            AccountKind::Multisig,
            decode_multisig,
        );
    }
}

#[cfg_attr(test, test)]
pub fn program_config() {
    verify(
        &ProgramConfigCodec,
        "c4d25ae790958c3f",
        [vec![7; 32], amount(), vec![8; 32], vec![9; 64]].concat(),
        AccountKind::ProgramConfig,
        decode_program_config,
    );
}

#[cfg_attr(test, test)]
pub fn proposal_every_status() {
    for status in 0..7 {
        let timestamp = if status == 4 {
            Vec::new()
        } else {
            (-17i64).to_le_bytes().to_vec()
        };

        verify(
            &ProposalCodec,
            "1a5ebdbb74883521",
            [
                vec![7; 32],
                amount(),
                vec![status],
                timestamp,
                vec![252],
                sequence(vec![8; 32]),
                sequence(vec![9; 32]),
                sequence(vec![10; 32]),
            ]
            .concat(),
            AccountKind::Proposal,
            decode_proposal,
        );
    }
}

#[cfg_attr(test, test)]
pub fn spending_limit_every_period() {
    for period in 0..4 {
        verify(
            &SpendingLimitCodec,
            "0ac91ba0dac3de98",
            [
                vec![7; 32],
                vec![8; 32],
                vec![2],
                vec![9; 32],
                amount(),
                vec![period],
                11u64.to_le_bytes().to_vec(),
                (-17i64).to_le_bytes().to_vec(),
                vec![252],
                sequence(vec![10; 32]),
                sequence(vec![11; 32]),
            ]
            .concat(),
            AccountKind::SpendingLimit,
            decode_spending_limit,
        );
    }
}

#[cfg_attr(test, test)]
pub fn transaction_buffer() {
    verify(
        &TransactionBufferCodec,
        "5a2423db5de16e60",
        [
            vec![7; 32],
            vec![8; 32],
            vec![3, 2],
            vec![9; 32],
            vec![2, 0],
            blob(&[0xa1, 0xb2]),
        ]
        .concat(),
        AccountKind::TransactionBuffer,
        decode_transaction_buffer,
    );
}

#[cfg_attr(test, test)]
pub fn vault_transaction() {
    verify(
        &VaultTransactionCodec,
        "a8faa264510ea2cf",
        [
            vec![7; 32],
            vec![8; 32],
            amount(),
            vec![252, 2, 251],
            blob(&[1, 2]),
            message(),
        ]
        .concat(),
        AccountKind::VaultTransaction,
        decode_vault_transaction,
    );
}

#[cfg_attr(test, test)]
pub fn every_vote_permission_byte_and_error_code_preserve_their_wire_values() {
    for tag in 0..3 {
        let vote = VoteCodec.decode(&[tag], LIMITS).unwrap();
        assert_eq!(VoteCodec.encode(&vote, 1).unwrap(), [tag]);
    }

    assert!(VoteCodec.decode(&[3], LIMITS).is_err());

    for mask in 0..=u8::MAX {
        let permissions = PermissionsCodec.decode(&[mask], LIMITS).unwrap();
        assert_eq!(permissions.mask, mask);
        assert_eq!(PermissionsCodec.encode(&permissions, 1).unwrap(), [mask]);
    }

    errors::verify();

    assert_eq!(identify_account(&[]), Err(Error::Truncated));
    assert_eq!(identify_account(&[0; 8]), Err(Error::InvalidDiscriminator));
}
