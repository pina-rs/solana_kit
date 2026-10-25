//! Every named source contract has nonzero independent bytes and strict framing.
use core::fmt::Debug;
use solana_kit_account_compression::*;
use solana_kit_core::{
    Address,
    codecs::{Decoder, Encoder, Limits},
};
const BYTE_LIMIT: usize = 4096;
const FIRST_ERROR: u32 = 6000;
const UNKNOWN_CODES: &[u32] = &[0, 5999, 6009, u32::MAX];

fn check<T: PartialEq + Debug + core::fmt::Display + core::hash::Hash>(
    codec: impl Encoder<T> + Decoder<T>,
    value: T,
    bytes: Vec<u8>,
) {
    let limits = Limits::new(BYTE_LIMIT, 1024);
    assert_eq!(codec.encode(&value, BYTE_LIMIT).unwrap(), bytes);
    assert_eq!(codec.decode(&bytes, limits).unwrap(), value);
    assert_eq!(format!("{value}"), format!("{value:?}"));

    for end in 0..bytes.len() {
        assert!(codec.decode(&bytes[..end], limits).is_err());
    }

    let mut trailing = bytes;
    trailing.push(0);
    assert!(codec.decode(&trailing, limits).is_err());
}

#[cfg_attr(test, test)]
pub fn application_and_change_log_records_and_all_event_variants() {
    application_events();
    change_log_events();
}

fn application_events() {
    let application = ApplicationDataEventV1 {
        application_data: vec![51, 52, 53],
    };
    let bytes = vec![3, 0, 0, 0, 51, 52, 53];
    check(
        ApplicationDataEventV1Codec,
        application.clone(),
        bytes.clone(),
    );

    let mut tagged = vec![0];
    tagged.extend(bytes);
    let application = ApplicationDataEvent::V1(application);
    check(
        ApplicationDataEventCodec,
        application.clone(),
        tagged.clone(),
    );

    let mut outer = vec![1];
    outer.extend(tagged);
    check(
        AccountCompressionEventCodec,
        AccountCompressionEvent::ApplicationData(application),
        outer,
    );
}

fn change_log_events() {
    let node = PathNode {
        node: vec![61; 32],
        index: 0x12345678,
    };
    let mut path = vec![61; 32];
    path.extend([0x78, 0x56, 0x34, 0x12]);
    check(PathNodeCodec, node.clone(), path.clone());
    let log = ChangeLogEventV1 {
        id: Address::from_bytes([62; 32]),
        path: vec![node],
        seq: 0x0807060504030201,
        index: 0x87654321,
    };
    let mut bytes = vec![62; 32];
    bytes.extend([1, 0, 0, 0]);
    bytes.extend(path);

    const LOG_TAIL: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 0x21, 0x43, 0x65, 0x87];
    bytes.extend(LOG_TAIL);
    check(ChangeLogEventV1Codec, log.clone(), bytes.clone());

    let log = ChangeLogEvent::V1(log);
    let mut tagged = vec![0];
    tagged.extend(bytes);
    check(ChangeLogEventCodec, log.clone(), tagged.clone());

    let mut outer = vec![0];
    outer.extend(tagged);
    check(
        AccountCompressionEventCodec,
        AccountCompressionEvent::ChangeLog(log),
        outer,
    );
}

#[cfg_attr(test, test)]
pub fn complete_header_records_version_and_account_types() {
    check(
        CompressionAccountTypeCodec,
        CompressionAccountType::Uninitialized,
        vec![0],
    );
    check(
        CompressionAccountTypeCodec,
        CompressionAccountType::ConcurrentMerkleTree,
        vec![1],
    );

    let header = ConcurrentMerkleTreeHeaderDataV1 {
        max_buffer_size: 256,
        max_depth: 14,
        authority: Address::from_bytes([63; 32]),
        creation_slot: 0x0807060504030201,
        padding: vec![81, 82, 83, 84, 85, 86],
    };
    let mut bytes = vec![0, 1, 0, 0, 14, 0, 0, 0];
    bytes.extend([63; 32]);
    const HEADER_TAIL: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 81, 82, 83, 84, 85, 86];
    bytes.extend(HEADER_TAIL);
    check(
        ConcurrentMerkleTreeHeaderDataV1Codec,
        header.clone(),
        bytes.clone(),
    );

    let header = ConcurrentMerkleTreeHeaderData::V1(header);
    let mut tagged = vec![0];
    tagged.extend(bytes);
    check(
        ConcurrentMerkleTreeHeaderDataCodec,
        header.clone(),
        tagged.clone(),
    );

    let header = ConcurrentMerkleTreeHeader {
        account_type: CompressionAccountType::ConcurrentMerkleTree,
        header,
    };
    let mut bytes = vec![1];
    bytes.extend(tagged);
    assert_eq!(bytes.len(), HEADER_SIZE_V1);
    check(ConcurrentMerkleTreeHeaderCodec, header, bytes);
}

#[cfg_attr(test, test)]
pub fn every_error_code_and_unknown_code_is_lossless() {
    let expected = [
        SplAccountCompressionError::IncorrectLeafLength,
        SplAccountCompressionError::ConcurrentMerkleTreeError,
        SplAccountCompressionError::ZeroCopyError,
        SplAccountCompressionError::ConcurrentMerkleTreeConstantsError,
        SplAccountCompressionError::CanopyLengthMismatch,
        SplAccountCompressionError::IncorrectAuthority,
        SplAccountCompressionError::IncorrectAccountOwner,
        SplAccountCompressionError::IncorrectAccountType,
        SplAccountCompressionError::LeafIndexOutOfBounds,
    ];

    for (index, error) in expected.into_iter().enumerate() {
        let code = FIRST_ERROR + u32::try_from(index).unwrap();
        assert_eq!(SplAccountCompressionError::from_code(code), error);
        assert_eq!(error.code(), code);
    }

    for &code in UNKNOWN_CODES {
        assert_eq!(
            SplAccountCompressionError::from_code(code),
            SplAccountCompressionError::Unknown(code)
        );
        assert_eq!(SplAccountCompressionError::from_code(code).code(), code);
    }
}
