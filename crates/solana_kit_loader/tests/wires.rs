use solana_kit_core::codecs::{Decoder, Encoder, Error, Limits};
use solana_kit_loader::{v3, v4};

const LIMITS: Limits = Limits::new(4096, 4096);
const V3_WRITE: &[u8] = &[1, 0, 0, 0, 120, 86, 52, 18, 2, 0, 0, 0, 0, 0, 0, 0, 91, 92];
const V3_CAPACITY: &[u8] = &[2, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255];
const V4_WRITE: &[u8] = &[0, 120, 86, 52, 18, 2, 0, 0, 0, 91, 92];
const UPGRADE: &[u8] = &[3, 0, 0, 0];
const SET_AUTHORITY: &[u8] = &[4, 0, 0, 0];
const CLOSE: &[u8] = &[5, 0, 0, 0];

fn verify<T: core::fmt::Debug + PartialEq>(
    codec: impl Encoder<T> + Decoder<T>,
    value: T,
    bytes: Vec<u8>,
) {
    assert_eq!(codec.encode(&value, 4096).unwrap(), bytes);
    assert_eq!(codec.decode(&bytes, LIMITS).unwrap(), value);

    for end in 0..bytes.len() {
        assert!(codec.decode(&bytes[..end], LIMITS).is_err());
    }

    let mut trailing = bytes;
    trailing.push(0);
    assert_eq!(codec.decode(&trailing, LIMITS), Err(Error::TrailingBytes));
}

#[cfg_attr(test, test)]
pub fn every_v3_instruction_uses_exact_literal_data() {
    verify(
        v3::InitializeBufferCodec,
        v3::InitializeBuffer {},
        vec![0, 0, 0, 0],
    );
    verify(
        v3::WriteCodec,
        v3::Write {
            offset: 0x12345678,
            bytes: vec![91, 92],
        },
        V3_WRITE.to_vec(),
    );
    verify(
        v3::DeployWithMaxDataLenCodec,
        v3::DeployWithMaxDataLen {
            max_data_len: u64::MAX,
        },
        V3_CAPACITY.to_vec(),
    );
    verify(v3::UpgradeCodec, v3::Upgrade {}, UPGRADE.to_vec());
    verify(
        v3::SetAuthorityCodec,
        v3::SetAuthority {},
        SET_AUTHORITY.to_vec(),
    );
    verify(v3::CloseCodec, v3::Close {}, CLOSE.to_vec());
    verify(
        v3::ExtendProgramCodec,
        v3::ExtendProgram {
            additional_bytes: u32::MAX,
        },
        vec![6, 0, 0, 0, 255, 255, 255, 255],
    );
    verify(
        v3::SetAuthorityCheckedCodec,
        v3::SetAuthorityChecked {},
        vec![7, 0, 0, 0],
    );
}

#[cfg_attr(test, test)]
pub fn every_v4_instruction_uses_exact_literal_data() {
    verify(
        v4::WriteCodec,
        v4::Write {
            offset: 0x12345678,
            bytes: vec![91, 92],
        },
        V4_WRITE.to_vec(),
    );
    verify(
        v4::TruncateCodec,
        v4::Truncate { new_size: u32::MAX },
        vec![1, 255, 255, 255, 255],
    );
    verify(v4::DeployCodec, v4::Deploy {}, vec![2]);
    verify(v4::RetractCodec, v4::Retract {}, vec![3]);
    verify(
        v4::TransferAuthorityCodec,
        v4::TransferAuthority {},
        vec![4],
    );
    verify(v4::FinalizeCodec, v4::Finalize {}, vec![5]);
}
