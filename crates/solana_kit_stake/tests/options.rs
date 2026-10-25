use solana_kit_core::{
    Address,
    codecs::{Decoder, Encoder, Error, Limits},
    plans::{InstructionLeaf, Plan},
};
use solana_kit_stake::*;

const LIMITS: Limits = Limits::new(4096, 4096);
const ADDRESS: Address = Address::from_bytes([37; 32]);

/// Every independently optional field retains its own tag and complete payload.
#[cfg_attr(test, test)]
pub fn all_lockup_option_combinations_have_literal_frames() {
    for mask in 0_u8..8 {
        let value = SetLockup {
            unix_timestamp: (mask & 1 != 0).then_some(UnixTimestamp { value: i64::MIN }),
            epoch: (mask & 2 != 0).then_some(Epoch { value: u64::MAX }),
            custodian: (mask & 4 != 0).then_some(ADDRESS),
        };
        let mut bytes = vec![6, 0, 0, 0];
        option_bytes(&mut bytes, mask & 1 != 0, &i64::MIN.to_le_bytes());
        option_bytes(&mut bytes, mask & 2 != 0, &u64::MAX.to_le_bytes());
        option_bytes(&mut bytes, mask & 4 != 0, &[37; 32]);
        assert_eq!(SetLockupCodec.encode(&value, 4096).unwrap(), bytes);
        assert_eq!(SetLockupCodec.decode(&bytes, LIMITS).unwrap(), value);
        reject_prefixes(&bytes, |bytes| SetLockupCodec.decode(bytes, LIMITS));
    }
}

/// Checked lockups use the same independent timestamp and epoch encoding.
#[cfg_attr(test, test)]
pub fn checked_lockup_options_and_invalid_tags_are_independent() {
    for mask in 0_u8..4 {
        let value = SetLockupChecked {
            unix_timestamp: (mask & 1 != 0).then_some(UnixTimestamp { value: -17 }),
            epoch: (mask & 2 != 0).then_some(Epoch { value: u64::MAX }),
        };
        let mut bytes = vec![12, 0, 0, 0];
        option_bytes(&mut bytes, mask & 1 != 0, &(-17_i64).to_le_bytes());
        option_bytes(&mut bytes, mask & 2 != 0, &u64::MAX.to_le_bytes());
        assert_eq!(SetLockupCheckedCodec.encode(&value, 4096).unwrap(), bytes);
        assert_eq!(SetLockupCheckedCodec.decode(&bytes, LIMITS).unwrap(), value);
        reject_prefixes(&bytes, |bytes| SetLockupCheckedCodec.decode(bytes, LIMITS));
    }

    for position in 4..7 {
        let mut bytes = vec![6, 0, 0, 0, 0, 0, 0];
        bytes[position] = 2;
        assert!(SetLockupCodec.decode(&bytes, LIMITS).is_err());
    }
}

/// Explicit source defaults remain ordinary caller-owned account and wire values.
#[cfg_attr(test, test)]
pub fn explicit_lockup_plan_preserves_nondefault_values() {
    let lockup = Lockup {
        unix_timestamp: UnixTimestamp { value: i64::MIN },
        epoch: Epoch { value: u64::MAX },
        custodian: ADDRESS,
    };
    let plan = create_stake_account_plan(
        ADDRESS,
        ADDRESS,
        0,
        Authorized {
            staker: ADDRESS,
            withdrawer: ADDRESS,
        },
        Some(lockup.clone()),
        LIMITS,
    )
    .unwrap();
    let Plan::Sequential { divisible, plans } = plan else {
        panic!("indivisible creation plan")
    };
    assert!(!divisible);
    assert_eq!(plans.len(), 2);
    let Plan::Single(InstructionLeaf::Instruction(initialize)) = &plans[1] else {
        panic!("initialization instruction")
    };
    assert_eq!(
        Initialize::parse_instruction(initialize, PROGRAM, LIMITS)
            .unwrap()
            .arg1,
        lockup
    );
}

fn option_bytes(bytes: &mut Vec<u8>, present: bool, payload: &[u8]) {
    bytes.push(u8::from(present));

    if present {
        bytes.extend_from_slice(payload);
    }
}

fn reject_prefixes<T>(bytes: &[u8], decode: impl Fn(&[u8]) -> Result<T, Error>) {
    for end in 0..bytes.len() {
        assert!(
            decode(&bytes[..end]).is_err(),
            "accepted truncated prefix {end}"
        );
    }

    let mut trailing = bytes.to_vec();
    trailing.push(0);
    assert!(decode(&trailing).is_err());
}
