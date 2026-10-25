use solana_kit_core::{
    Address,
    codecs::{Decoder, Encoder, Error, Limits, Reader},
};
use solana_kit_stake::*;

const LIMITS: Limits = Limits::new(4096, 4096);
const EMPTY_TAGS: [u32; 5] = [2, 5, 7, 9, 13];
const UNKNOWN_TAGS: [u32; 4] = [15, 255, 256, u32::MAX];
const UNKNOWN_AUTHORITIES: [u32; 3] = [2, 256, u32::MAX];
const ACCOUNT_ALLOCATION: usize = 200;
const ACTIVATION_EPOCH: u64 = 5;
const DEACTIVATION_EPOCH: u64 = 6;
const FLAGS: u8 = 9;

#[cfg_attr(test, test)]
pub fn every_instruction_tag_dispatches_with_exact_u32_width() {
    for tag in EMPTY_TAGS {
        let bytes = tag.to_le_bytes();
        let value = StakeInstructionDataCodec.decode(&bytes, LIMITS).unwrap();
        assert_eq!(StakeInstructionDataCodec.encode(&value, 4).unwrap(), bytes);
    }

    for tag in UNKNOWN_TAGS {
        assert_eq!(
            StakeInstructionDataCodec.decode(&tag.to_le_bytes(), LIMITS),
            Err(Error::InvalidDiscriminator)
        );
    }
}

fn metadata() -> Meta {
    Meta {
        rent_exempt_reserve: u64::MAX,
        authorized: Authorized {
            staker: Address::from_bytes([1; 32]),
            withdrawer: Address::from_bytes([2; 32]),
        },
        lockup: Lockup {
            unix_timestamp: UnixTimestamp { value: i64::MIN },
            epoch: Epoch { value: u64::MAX },
            custodian: Address::from_bytes([3; 32]),
        },
    }
}

fn delegated(meta: Meta) -> StakeStateV2 {
    StakeStateV2::Stake(StakeStateV2StakePayload {
        item0: meta,
        item1: Stake {
            delegation: Delegation {
                voter_pubkey: Address::from_bytes([4; 32]),
                stake: u64::MAX,
                activation_epoch: Epoch {
                    value: ACTIVATION_EPOCH,
                },
                deactivation_epoch: Epoch {
                    value: DEACTIVATION_EPOCH,
                },
                reserved: vec![7; 8],
            },
            credits_observed: 8,
        },
        item2: StakeFlags { bits: FLAGS },
    })
}

#[cfg_attr(test, test)]
pub fn canonical_account_preserves_all_four_states_and_tuple_payload_order() {
    let meta = metadata();
    let states = [
        StakeStateV2::Uninitialized,
        StakeStateV2::Initialized(meta.clone()),
        delegated(meta),
        StakeStateV2::RewardsPool,
    ];

    for (tag, state) in states.into_iter().enumerate() {
        let value = StakeStateAccount { state };
        let bytes = StakeStateAccountCodec
            .encode(&value, ACCOUNT_ALLOCATION)
            .unwrap();
        assert_eq!(&bytes[..4], &u32::try_from(tag).unwrap().to_le_bytes());
        assert_eq!(
            StakeStateAccountCodec.decode(&bytes, LIMITS).unwrap(),
            value
        );

        if tag == 2 {
            assert_eq!(bytes.len(), 197);
            assert_eq!(&bytes[4..12], &u64::MAX.to_le_bytes());
            assert_eq!(&bytes[124..156], &[4; 32]);
            assert_eq!(&bytes[188..196], &8_u64.to_le_bytes());
            assert_eq!(bytes[196], FLAGS);
        }
    }
}

#[cfg_attr(test, test)]
pub fn u32_enum_unknown_truncation_budget_and_rollback_are_explicit() {
    for bytes in [&[0][..], &[0, 0], &[0, 0, 0]] {
        assert_eq!(
            StakeAuthorizeCodec.decode(bytes, LIMITS),
            Err(Error::Truncated)
        );
    }

    for tag in UNKNOWN_AUTHORITIES {
        let bytes = tag.to_le_bytes();
        let mut reader = Reader::new(&bytes, LIMITS).unwrap();
        assert_eq!(
            StakeAuthorizeCodec.read(&mut reader),
            Err(Error::InvalidDiscriminator)
        );
        assert_eq!(reader.position(), 0);
    }

    assert_eq!(
        StakeAuthorizeCodec
            .encode(&StakeAuthorize::Withdrawer, 4)
            .unwrap(),
        [1, 0, 0, 0]
    );
    assert_eq!(
        StakeStateV2Codec.decode(&[2, 0, 0, 0], Limits::new(4, 0)),
        Err(Error::Truncated)
    );
}

#[cfg_attr(test, test)]
pub fn ordered_sysvars_optional_signer_remaining_and_override_are_preserved() {
    let program = Address::from_bytes([9; 32]);
    let address = Address::from_bytes([1; 32]);
    let accounts = AuthorizeAccounts {
        stake: address,
        clock_sysvar: address,
        authority: address,
        lockup_authority: None,
    };
    let data = Authorize {
        arg0: address,
        arg1: StakeAuthorize::Staker,
    };
    let instruction = accounts.instruction(program, &data, &[], LIMITS).unwrap();
    assert_eq!(instruction.accounts.len(), 3);
    assert!(instruction.accounts[0].role.is_writable());
    assert!(!instruction.accounts[1].role.is_signer());

    assert!(instruction.accounts[2].role.is_signer());
    assert_eq!(
        parse_instruction(&instruction, program, LIMITS).unwrap(),
        StakeInstructionData::Authorize(data)
    );
    assert_eq!(
        parse_instruction(&instruction, address, LIMITS),
        Err(Error::InvalidValue)
    );
}

#[cfg_attr(test, test)]
pub fn all_error_numbers_and_failure_attribution_are_lossless() {
    use solana_kit_core::program_error::{InstructionFailure, ProgramError};
    let program = Address::from_bytes([9; 32]);
    let other = Address::from_bytes([8; 32]);

    for code in 0_u32..17 {
        let error = StakeError::from_code(code);
        assert_eq!(error.code(), code);
        assert!(error.message().is_some());
        let failure = InstructionFailure {
            index: 1,
            error: ProgramError::Custom(code),
        };
        assert_eq!(
            StakeError::from_failure(&failure, &[other, program], program),
            Some(error)
        );
        assert_eq!(
            StakeError::from_failure(&failure, &[program, other], program),
            None
        );
    }

    assert_eq!(
        StakeError::from_code(u32::MAX),
        StakeError::Unknown(u32::MAX)
    );
    assert_eq!(StakeError::from_code(u32::MAX).code(), u32::MAX);
}
