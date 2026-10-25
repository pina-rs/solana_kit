use solana_kit_core::{
    Address,
    codecs::{Decoder, Encoder, Error, Limits},
};
use solana_kit_stake::*;

const LIMITS: Limits = Limits::new(4096, 4096);
const ADDRESS_BYTES: usize = 32;
const INITIALIZE: u32 = 0;
const AUTHORIZE: u32 = 1;
const DELEGATE: u32 = 2;
const SPLIT: u32 = 3;
const WITHDRAW: u32 = 4;
const DEACTIVATE: u32 = 5;
const LOCKUP: u32 = 6;
const MERGE: u32 = 7;
const SEED: u32 = 8;
const INITIALIZE_CHECKED: u32 = 9;
const AUTHORIZE_CHECKED: u32 = 10;
const SEED_CHECKED: u32 = 11;
const LOCKUP_CHECKED: u32 = 12;
const MINIMUM: u32 = 13;
const DELINQUENT: u32 = 14;
const MOVE_STAKE: u32 = 16;
const MOVE_LAMPORTS: u32 = 17;

fn verify(value: StakeInstructionData, bytes: Vec<u8>) {
    assert_eq!(
        StakeInstructionDataCodec
            .encode(&value, LIMITS.max_bytes)
            .unwrap(),
        bytes
    );
    assert_eq!(
        StakeInstructionDataCodec.decode(&bytes, LIMITS).unwrap(),
        value
    );

    for end in 0..bytes.len() {
        assert!(
            StakeInstructionDataCodec
                .decode(&bytes[..end], LIMITS)
                .is_err()
        );
    }

    let mut trailing = bytes;
    trailing.push(0);
    assert_eq!(
        StakeInstructionDataCodec.decode(&trailing, LIMITS),
        Err(Error::TrailingBytes)
    );
}

fn bytes(tag: u32, payload: &[u8]) -> Vec<u8> {
    [tag.to_le_bytes().as_slice(), payload].concat()
}

#[cfg_attr(test, test)]
pub fn all_seventeen_stake_literals_and_malformed_boundaries() {
    initialization();
    lifecycle();
    seeded();
    moves_and_checked_lockup();
}

fn initialization() {
    let address = Address::from_bytes([7; 32]);
    let amount = 0x0102_0304_0506_0708_u64;
    let authorizations = Authorized {
        staker: address,
        withdrawer: address,
    };
    let lockup = Lockup {
        unix_timestamp: UnixTimestamp { value: -1 },
        epoch: Epoch { value: amount },
        custodian: address,
    };
    let initialization = [
        vec![7; 64],
        vec![255; 8],
        amount.to_le_bytes().to_vec(),
        vec![7; 32],
    ]
    .concat();
    verify(
        StakeInstructionData::Initialize(Initialize {
            arg0: authorizations,
            arg1: lockup,
        }),
        bytes(INITIALIZE, &initialization),
    );
    verify(
        StakeInstructionData::Authorize(Authorize {
            arg0: address,
            arg1: StakeAuthorize::Withdrawer,
        }),
        bytes(AUTHORIZE, &[vec![7; 32], vec![1, 0, 0, 0]].concat()),
    );
}

fn lifecycle() {
    let amount = 0x0102_0304_0506_0708_u64;
    verify(
        StakeInstructionData::DelegateStake(DelegateStake {}),
        bytes(DELEGATE, &[]),
    );

    verify(
        StakeInstructionData::Split(Split { args: amount }),
        bytes(SPLIT, &amount.to_le_bytes()),
    );
    verify(
        StakeInstructionData::Withdraw(Withdraw { args: amount }),
        bytes(WITHDRAW, &amount.to_le_bytes()),
    );
    verify(
        StakeInstructionData::Deactivate(Deactivate {}),
        bytes(DEACTIVATE, &[]),
    );
    verify(
        StakeInstructionData::SetLockup(SetLockup {
            unix_timestamp: None,
            epoch: None,
            custodian: None,
        }),
        bytes(LOCKUP, &[0, 0, 0]),
    );
    verify(StakeInstructionData::Merge(Merge {}), bytes(MERGE, &[]));
}

fn seeded() {
    let address = Address::from_bytes([7; ADDRESS_BYTES]);
    let seeded = AuthorizeWithSeed {
        new_authorized_pubkey: address,
        stake_authorize: StakeAuthorize::Withdrawer,
        authority_seed: "é".into(),
        authority_owner: address,
    };
    let seed_payload = [
        vec![7; 32],
        vec![1, 0, 0, 0],
        vec![2, 0, 0, 0, 0, 0, 0, 0],
        vec![195, 169],
        vec![7; 32],
    ]
    .concat();
    verify(
        StakeInstructionData::AuthorizeWithSeed(seeded),
        bytes(SEED, &seed_payload),
    );
    verify(
        StakeInstructionData::InitializeChecked(InitializeChecked {}),
        bytes(INITIALIZE_CHECKED, &[]),
    );
    verify(
        StakeInstructionData::AuthorizeChecked(AuthorizeChecked {
            stake_authorize: StakeAuthorize::Withdrawer,
        }),
        bytes(AUTHORIZE_CHECKED, &[1, 0, 0, 0]),
    );
    let checked = AuthorizeCheckedWithSeed {
        stake_authorize: StakeAuthorize::Withdrawer,
        authority_seed: "é".into(),
        authority_owner: address,
    };
    verify(
        StakeInstructionData::AuthorizeCheckedWithSeed(checked),
        bytes(SEED_CHECKED, &seed_payload[ADDRESS_BYTES..]),
    );
}

fn moves_and_checked_lockup() {
    let amount = 0x0102_0304_0506_0708_u64;
    verify(
        StakeInstructionData::SetLockupChecked(SetLockupChecked {
            unix_timestamp: None,
            epoch: None,
        }),
        bytes(LOCKUP_CHECKED, &[0, 0]),
    );
    verify(
        StakeInstructionData::GetMinimumDelegation(GetMinimumDelegation {}),
        bytes(MINIMUM, &[]),
    );
    verify(
        StakeInstructionData::DeactivateDelinquent(DeactivateDelinquent {}),
        bytes(DELINQUENT, &[]),
    );
    verify(
        StakeInstructionData::MoveStake(MoveStake { args: amount }),
        bytes(MOVE_STAKE, &amount.to_le_bytes()),
    );
    verify(
        StakeInstructionData::MoveLamports(MoveLamports { args: amount }),
        bytes(MOVE_LAMPORTS, &amount.to_le_bytes()),
    );
}
