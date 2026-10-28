//! Offline SNS vectors shared across every conformance host.

use alloc::{string::ToString, vec};
use solana_kit_core::{
    Address,
    codecs::{Decoder, Encoder, Limits},
};
use solana_kit_sns::{
    DataError, DomainError, NAME_PROGRAM, NAME_REGISTRY_HEADER_LENGTH, NameRegistryCodec,
    NameRegistryState, NameValueCodec, RECORD_HEADER_LENGTH, REVERSE_LOOKUP_CLASS, SNS_ROOT_DOMAIN,
    SnsRecord, SnsRecordHeader, SnsRecordHeaderCodec, SnsRecordV2, SnsRecordVersion, SnsValidation,
    decode_name_value, decode_record_content, decode_reverse_value, encode_record_content,
    encode_reverse_value, find_domain_key, find_record_v1_address, find_record_v2_address,
    find_reverse_address_for_domain, get_hashed_name, sha256,
};

/// Returns independently reported source-compatible offline SNS groups.
pub(crate) fn cases() -> [crate::CaseResult; 4] {
    [
        crate::case("sns-hash-domain-pda", hash_domain()),
        crate::case("sns-record-reverse-pda", record_reverse()),
        crate::case("sns-registry-value-codecs", registry_value()),
        crate::case("sns-record-content-lazy-bounds", record_payload()),
    ]
}

/// Exercises deterministic hash and domain derivation with invalid input.
fn hash_domain() -> Result<(), &'static str> {
    let expected = [
        0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea, 0x41, 0x41, 0x40, 0xde, 0x5d, 0xae, 0x22,
        0x23, 0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c, 0xb4, 0x10, 0xff, 0x61, 0xf2, 0x00,
        0x15, 0xad,
    ];
    crate::require(sha256(b"abc").as_bytes() == &expected, "SNS SHA-256")?;
    let expected_hash = [
        0x8e, 0xe2, 0xd2, 0x5c, 0x3d, 0x2b, 0x2a, 0x83, 0xa1, 0xfc, 0x20, 0x9b, 0x90, 0x37, 0x7a,
        0xed, 0x03, 0xdc, 0x25, 0x39, 0xe8, 0xe2, 0x38, 0x35, 0x5e, 0xdd, 0xa8, 0xd1, 0xb2, 0xed,
        0xab, 0x98,
    ];
    crate::require(
        get_hashed_name("bonfida").as_bytes() == &expected_hash,
        "SNS name hash",
    )?;
    let key = find_domain_key("dex.bonfida", None).map_err(|_| "SNS subdomain")?;
    let expected: Address = "HoFfFXqFHAC8RP3duuQNzag1ieUwJRBv1HtRNiWFq4Qu"
        .parse()
        .map_err(|_| "SNS address literal")?;
    crate::require(key.address == expected && key.is_sub, "SNS subdomain PDA")?;
    crate::require(
        find_domain_key("bonfida.sol", None) == Err(DomainError::InvalidDomain),
        "SNS suffix validation",
    )
}

/// Exercises V1, V2, subdomain records and reverse address derivation.
fn record_reverse() -> Result<(), &'static str> {
    for (actual, expected) in [
        (
            find_record_v1_address("bonfida", SnsRecord::Url),
            "CvhvqcxBbA4UdWuJFDMuuC4XbpCrAd9gidpW5wxEsjg5",
        ),
        (
            find_record_v2_address("bonfida", SnsRecord::Url),
            "EyXTEBK3xFkzkweB5PNR1zNjYchpyYyizunbdpcCEHVy",
        ),
        (
            find_record_v2_address("a.bonfida", SnsRecord::Url),
            "Fwj5CpNUMqhfLe7tNM8NVz7MUDx3NoudxiSLqfxfAyjE",
        ),
        (
            find_reverse_address_for_domain("a.bonfida"),
            "rixHffj1oxKRGCB1MULj2BPEGL9Ac2VRhZdrjL91Evw",
        ),
    ] {
        let address = actual.map_err(|_| "SNS address derivation")?;
        let expected: Address = expected.parse().map_err(|_| "SNS address literal")?;
        crate::require(address == expected, "SNS record or reverse PDA")?;
    }
    let key = find_domain_key("url.a.bonfida", Some(SnsRecordVersion::V2))
        .map_err(|_| "SNS subrecord")?;
    crate::require(key.is_sub_record, "SNS subrecord context")
}

/// Exercises account and length-prefixed value codecs, including trailing allocation.
fn registry_value() -> Result<(), &'static str> {
    let registry = NameRegistryState {
        parent_name: SNS_ROOT_DOMAIN,
        owner: NAME_PROGRAM,
        registry_class: REVERSE_LOOKUP_CLASS,
        data: vec![1, 2, 3],
    };
    let bytes = NameRegistryCodec
        .encode(&registry, NAME_REGISTRY_HEADER_LENGTH + 3)
        .map_err(|_| "SNS registry encode")?;
    let decoded = NameRegistryCodec
        .decode(&bytes, Limits::new(bytes.len(), 3))
        .map_err(|_| "SNS registry decode")?;
    crate::require(decoded == registry, "SNS registry roundtrip")?;
    crate::require(
        NameRegistryState::decode(&bytes[..95]) == Err(DataError::Truncated),
        "SNS short registry",
    )?;
    let name = NameValueCodec
        .encode("foo", 7)
        .map_err(|_| "SNS name encode")?;
    let decoded = NameValueCodec
        .decode(&name, Limits::new(name.len(), 3))
        .map_err(|_| "SNS name decode")?;
    crate::require(decoded == "foo", "SNS name roundtrip")?;

    crate::require(
        decode_name_value(b"\x04\0\0\0\xef\xbb\xbfa*").map_err(|_| "SNS name BOM")? == "a",
        "SNS BOM and trailing bytes",
    )?;
    let reverse = encode_reverse_value("dex", true).map_err(|_| "SNS reverse encode")?;
    crate::require(
        decode_reverse_value(&reverse, true).map_err(|_| "SNS reverse decode")? == "dex",
        "SNS reverse marker",
    )
}

/// Exercises lazy V2 payload bounds and content representation.
fn record_payload() -> Result<(), &'static str> {
    let header = SnsRecordHeader {
        staleness_validation: SnsValidation::Solana,
        right_of_association_validation: SnsValidation::Ethereum,
        content_length: 3,
    };
    let encoded = SnsRecordHeaderCodec
        .encode(&header, RECORD_HEADER_LENGTH)
        .map_err(|_| "SNS header encode")?;
    crate::require(
        SnsRecordHeaderCodec
            .decode(&encoded, Limits::new(encoded.len(), 0))
            .map_err(|_| "SNS header decode")?
            == header,
        "SNS header roundtrip",
    )?;
    let mut account = vec![0; NAME_REGISTRY_HEADER_LENGTH];
    account.extend_from_slice(&encoded);
    let view = SnsRecordV2::deserialize(&account).map_err(|_| "SNS lazy header")?;
    crate::require(
        view.content() == Err(DataError::Truncated)
            && view.staleness_id() == Err(DataError::Truncated),
        "SNS lazy bounds",
    )?;
    account.extend_from_slice(&[1; 32]);

    account.extend_from_slice(&[2; 20]);
    account.extend_from_slice(b"abc!");
    let view = SnsRecordV2::deserialize(&account).map_err(|_| "SNS record")?;
    crate::require(
        view.content().map_err(|_| "SNS content")? == b"abc",
        "SNS content length",
    )?;
    let evm = "0xc0ffee254729296a45a3885639ac7e10f9d54979";
    let bytes = encode_record_content(evm, SnsRecord::Eth).map_err(|_| "SNS EVM encode")?;
    let decoded = decode_record_content(&bytes, SnsRecord::Eth).map_err(|_| "SNS EVM decode")?;
    crate::require(decoded == evm.to_string(), "SNS EVM content")
}
