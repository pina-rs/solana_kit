use solana_kit_core::{
    AccountRole, Address,
    codecs::{Decoder, Encoder, Error, Limits},
};
use solana_kit_loader::{v3, v4};
const LIMITS: Limits = Limits::new(4096, 4096);
const WRITE_BOUND: usize = 64;
const EMPTY_TAGS: [u32; 5] = [0, 3, 4, 5, 7];

#[cfg_attr(test, test)]
pub fn pinned_loader_byte_lengths_and_tags_remain_distinct() {
    let old = v3::Write {
        offset: u32::MAX,
        bytes: vec![1, 2, 3],
    };
    let new = v4::Write {
        offset: u32::MAX,
        bytes: vec![1, 2, 3],
    };
    let a = v3::WriteCodec.encode(&old, WRITE_BOUND).unwrap();
    let b = v4::WriteCodec.encode(&new, WRITE_BOUND).unwrap();
    assert_eq!(
        a,
        [
            vec![1, 0, 0, 0],
            u32::MAX.to_le_bytes().to_vec(),
            3_u64.to_le_bytes().to_vec(),
            vec![1, 2, 3]
        ]
        .concat()
    );
    assert_eq!(
        b,
        [
            vec![0],
            u32::MAX.to_le_bytes().to_vec(),
            3_u32.to_le_bytes().to_vec(),
            vec![1, 2, 3]
        ]
        .concat()
    );
    assert_eq!(v3::WriteCodec.decode(&a, LIMITS).unwrap(), old);
    assert_eq!(v4::WriteCodec.decode(&b, LIMITS).unwrap(), new);

    for end in 0..a.len() {
        assert!(v3::WriteCodec.decode(&a[..end], LIMITS).is_err());
    }

    let mut hostile = vec![1, 0, 0, 0, 0, 0, 0, 0];
    hostile.extend_from_slice(&u64::MAX.to_le_bytes());
    assert!(v3::WriteCodec.decode(&hostile, LIMITS).is_err());
}

#[cfg_attr(test, test)]
pub fn every_empty_variant_and_unknown_code_uses_exact_width() {
    for tag in EMPTY_TAGS {
        let bytes = tag.to_le_bytes();
        let value = v3::LoaderV3InstructionDataCodec
            .decode(&bytes, LIMITS)
            .unwrap();
        assert_eq!(
            v3::LoaderV3InstructionDataCodec.encode(&value, 4).unwrap(),
            bytes
        );
    }

    for tag in 2_u8..6 {
        let value = v4::LoaderV4InstructionDataCodec
            .decode(&[tag], LIMITS)
            .unwrap();
        assert_eq!(
            v4::LoaderV4InstructionDataCodec.encode(&value, 1).unwrap(),
            [tag]
        );
    }

    assert_eq!(
        v4::LoaderV4InstructionDataCodec.decode(&[255], LIMITS),
        Err(Error::InvalidDiscriminator)
    );
}

#[cfg_attr(test, test)]
pub fn optional_account_omission_and_exact_privileges_follow_source() {
    let address = Address::from_bytes([1; 32]);
    let accounts = v4::TruncateAccounts {
        program: address,
        authority: address,
        destination: None,
    };
    let instruction = accounts
        .instruction(address, &v4::Truncate { new_size: 0 }, &[], LIMITS)
        .unwrap();
    assert_eq!(instruction.accounts.len(), 2);
    assert_eq!(instruction.accounts[0].role, AccountRole::WritableSigner);
    assert_eq!(instruction.accounts[1].role, AccountRole::ReadonlySigner);
}

#[cfg_attr(test, test)]
pub fn actual_headers_cover_optional_authority_and_tail() {
    use solana_kit_loader::*;

    for authority in [None, Some(Address::from_bytes([7; 32]))] {
        let buffer = UpgradeableState::Buffer { authority };
        let bytes = UpgradeableStateCodec
            .encode(&buffer, BUFFER_HEADER_SIZE)
            .unwrap();
        assert_eq!(bytes.len(), BUFFER_HEADER_SIZE);
        assert_eq!(bytes[4], u8::from(authority.is_some()));
        assert_eq!(
            UpgradeableStateCodec.decode(&bytes, LIMITS).unwrap(),
            buffer
        );
        let value = UpgradeableState::ProgramData {
            slot: u64::MAX,
            authority,
        };
        let mut bytes = UpgradeableStateCodec
            .encode(&value, PROGRAM_DATA_HEADER_SIZE)
            .unwrap();

        assert_eq!(bytes.len(), PROGRAM_DATA_HEADER_SIZE);
        bytes.extend_from_slice(&[8, 9]);
        assert_eq!(
            UpgradeableStateCodec
                .decode_prefix(&bytes, 0, LIMITS)
                .unwrap(),
            (value, PROGRAM_DATA_HEADER_SIZE)
        );
        assert_eq!(
            UpgradeableStateCodec.decode(&bytes, LIMITS),
            Err(Error::TrailingBytes)
        );
    }
}

#[cfg_attr(test, test)]
pub fn actual_v4_status_headers_reject_unknown_u64_tags() {
    use solana_kit_loader::*;

    for status in [
        LoaderV4Status::Retracted,
        LoaderV4Status::Deployed,
        LoaderV4Status::Finalized,
    ] {
        let value = ProgramStateAccount {
            slot: u64::MAX,
            authority_address_or_next_version: Address::from_bytes([3; 32]),
            status,
        };
        let mut bytes = ProgramStateAccountCodec
            .encode(&value, PROGRAM_STATE_HEADER_SIZE)
            .unwrap();
        assert_eq!(
            ProgramStateAccountCodec.decode(&bytes, LIMITS).unwrap(),
            value
        );
        bytes[40..].copy_from_slice(&u64::MAX.to_le_bytes());
        assert_eq!(
            ProgramStateAccountCodec.decode(&bytes, LIMITS),
            Err(Error::InvalidDiscriminator)
        );
    }
}
