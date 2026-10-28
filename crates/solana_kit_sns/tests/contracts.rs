use solana_kit_core::Address;
use solana_kit_core::codecs::{Decoder, Encoder, Error as CodecError, Limits};
use solana_kit_sns::*;

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[test]
fn sns_hash_vectors() {
    assert_eq!(
        hex(sha256(b"").as_bytes()),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
    assert_eq!(
        hex(sha256(b"abc").as_bytes()),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    assert_eq!(
        hex(sha256(&[7; 200]).as_bytes()),
        "83ed56670125a357640623904bc0c1a17fdc835d75eef70363750f1a9c8ba724"
    );
    for (name, expected) in [
        (
            "bonfida",
            "8ee2d25c3d2b2a83a1fc209b90377aed03dc2539e8e238355edda8d1b2edab98",
        ),
        (
            "Ø",
            "4953761847398aedf790ff02f857b7af43c0fb2fe2bac1769ed4045186abda9e",
        ),
        (
            "\0sub",
            "4283e89c83ac6b464b0bcd51cf1391ed43cbff55c51a47c4c759596e2a5edc39",
        ),
        (
            "\u{1}url",
            "10edbb8fea0a71c36dffa9b24b2158f399107f336552f48d44dd42bf715568f1",
        ),
        (
            "\u{2}url",
            "e77214d976cb19019b46315c066d230374473ba561c1f3d7197fd1e644ac3585",
        ),
    ] {
        assert_eq!(hex(get_hashed_name(name).as_bytes()), expected);
    }
}

#[test]
fn sns_domain_and_pda_vectors() -> Result<(), Box<dyn std::error::Error>> {
    let parent = find_domain_key("bonfida", None)?;
    assert_eq!(
        parent.address.to_string(),
        "Crf8hzfthWGbGbLTVCiqRqV5MVnbpHB1L9KQMd6gsinb"
    );
    assert!(!parent.is_sub);
    assert_eq!(parent.parent_address, None);
    let child = find_domain_key("dex.bonfida", None)?;
    assert_eq!(
        child.address.to_string(),
        "HoFfFXqFHAC8RP3duuQNzag1ieUwJRBv1HtRNiWFq4Qu"
    );
    assert_eq!(child.parent_address, Some(parent.address));
    assert_eq!(
        find_domain_key("Ø.bonfida", None)?.address.to_string(),
        "9Cw7FSkJ1rNm4zZ7VBKA51daxj5QRpTempjmoZU19yk"
    );

    assert_eq!(
        find_name_account_key(&parent.hash, None, None)?.to_string(),
        "85v6oF1VnGNeT4oV2fH8HpVBj3k3U4m6uNnWYT8AcA5H"
    );
    assert_eq!(
        find_domain_key("url.a.bonfida", Some(SnsRecordVersion::V1))?
            .address
            .to_string(),
        "6Ck3zGPYfWuGv3CisTRW7AHm7ALQdWYSdK2pVSQTi96Q"
    );
    let subrecord = find_domain_key("url.a.bonfida", Some(SnsRecordVersion::V2))?;
    assert!(subrecord.is_sub_record);
    assert_eq!(subrecord.parent_address, Some(parent.address));
    assert_eq!(
        subrecord.address.to_string(),
        "Fwj5CpNUMqhfLe7tNM8NVz7MUDx3NoudxiSLqfxfAyjE"
    );
    for invalid in [
        "",
        ".bonfida",
        "bonfida.",
        "a..bonfida",
        "bonfida.sol",
        "bonfida.sns",
        "a.b.c",
        "a.b.c.d",
    ] {
        assert_eq!(
            find_domain_key(invalid, None),
            Err(DomainError::InvalidDomain)
        );
    }
    assert_eq!(
        find_domain_key("a.b.c.d", Some(SnsRecordVersion::V2)),
        Err(DomainError::InvalidDomain)
    );

    Ok(())
}

#[test]
fn sns_record_and_reverse_pda_vectors() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        find_record_v1_address("bonfida", SnsRecord::Url)?.to_string(),
        "CvhvqcxBbA4UdWuJFDMuuC4XbpCrAd9gidpW5wxEsjg5"
    );
    assert_eq!(
        find_record_v2_address("bonfida", SnsRecord::Url)?.to_string(),
        "EyXTEBK3xFkzkweB5PNR1zNjYchpyYyizunbdpcCEHVy"
    );
    assert_eq!(
        find_record_v2_address("bonfida", SnsRecord::Sol)?.to_string(),
        "ETARvCjLwjyM6Jux1ndxuXuYEYy56Nf5uvU3abL1WyW6"
    );
    assert_eq!(
        find_record_v2_address("a.bonfida", SnsRecord::Url)?.to_string(),
        "Fwj5CpNUMqhfLe7tNM8NVz7MUDx3NoudxiSLqfxfAyjE"
    );
    assert_eq!(
        find_reverse_address_for_domain("bonfida")?.to_string(),
        "DqgmWxe2PPrfy45Ja3UPyFGwcbRzkRuwXt3NyxjX8krg"
    );
    assert_eq!(
        find_reverse_address_for_domain("a.bonfida")?.to_string(),
        "rixHffj1oxKRGCB1MULj2BPEGL9Ac2VRhZdrjL91Evw"
    );
    Ok(())
}

#[test]
fn sns_registry_and_name_boundaries() -> Result<(), Box<dyn std::error::Error>> {
    let state = NameRegistryState {
        parent_name: SNS_ROOT_DOMAIN,
        owner: NAME_PROGRAM,
        registry_class: REVERSE_LOOKUP_CLASS,
        data: b"hello".to_vec(),
    };
    assert!(state.has_data());
    let encoded = state.encode();
    assert_eq!(NameRegistryState::decode(&encoded)?, state);

    for length in 0..NAME_REGISTRY_HEADER_LENGTH {
        assert_eq!(
            NameRegistryState::decode(&encoded[..length]),
            Err(DataError::Truncated)
        );
    }

    assert_eq!(encode_name_value("Ø")?.as_slice(), b"\x02\0\0\0\xc3\x98");
    assert_eq!(
        decode_name_value_prefix(b"\x03\0\0\0fooc")?,
        ("foo".to_string(), 4 + "foo".len())
    );

    assert_eq!(decode_name_value(b"\x04\0\0\0\xef\xbb\xbfa*")?, "a");
    assert_eq!(
        decode_name_value(&[1, 0, 0, 0, 0xff]),
        Err(DataError::InvalidUtf8)
    );
    assert_eq!(
        decode_name_value(&[2, 0, 0, 0, b'a']),
        Err(DataError::Truncated)
    );
    let reverse = encode_reverse_value("dex", true)?;
    assert_eq!(reverse.as_slice(), b"\x04\0\0\0\0dex");
    assert_eq!(decode_reverse_value(&reverse, false)?, "\0dex");
    assert_eq!(decode_reverse_value(&reverse, true)?, "dex");
    Ok(())
}

#[test]
fn sns_record_header_and_lazy_payload() -> Result<(), Box<dyn std::error::Error>> {
    let header = SnsRecordHeader {
        staleness_validation: SnsValidation::Solana,
        right_of_association_validation: SnsValidation::Ethereum,
        content_length: 3,
    };
    assert_eq!(header.encode(), [1, 0, 2, 0, 3, 0, 0, 0]);
    assert_eq!(
        SnsRecordHeader::decode_prefix(&header.encode())?,
        (header, RECORD_HEADER_LENGTH)
    );
    assert_eq!(
        SnsRecordHeader::decode_prefix(&[4, 0, 0, 0, 0, 0, 0, 0]),
        Err(DataError::InvalidValidation)
    );

    for length in 0..104 {
        assert_eq!(
            SnsRecordV2::deserialize(&vec![0; length]),
            Err(DataError::Truncated)
        );
    }

    let mut account = vec![0; 96];
    account.extend_from_slice(&header.encode());

    let view = SnsRecordV2::deserialize(&account)?;
    assert_eq!(view.staleness_id(), Err(DataError::Truncated));
    assert_eq!(view.roa_id(), Err(DataError::Truncated));
    assert_eq!(view.content(), Err(DataError::Truncated));
    account.extend_from_slice(&[1; 32]);
    account.extend_from_slice(&[2; 20]);
    account.extend_from_slice(b"abc");
    account.push(b'c');

    let view = SnsRecordV2::deserialize(&account)?;
    assert_eq!(view.staleness_id()?, &[1; 32]);
    assert_eq!(view.roa_id()?, &[2; 20]);
    assert_eq!(view.content()?, b"abc");
    Ok(())
}

#[test]
fn sns_record_content_and_constants() -> Result<(), Box<dyn std::error::Error>> {
    let wallet = "GtEGr7hhAqt53JbGrQNgWY2rrHHtNSj4aNquNqaDPaUH";
    let address: Address = wallet.parse()?;
    assert_eq!(
        encode_record_content(wallet, SnsRecord::Sol)?.as_slice(),
        address.as_bytes()
    );
    assert_eq!(
        decode_record_content(address.as_bytes(), SnsRecord::Sol)?,
        wallet
    );
    let evm = "0xc0ffee254729296a45a3885639ac7e10f9d54979";
    assert_eq!(
        decode_record_content(&encode_record_content(evm, SnsRecord::Eth)?, SnsRecord::Eth)?,
        evm
    );
    assert_eq!(
        encode_record_content("0x12", SnsRecord::Eth),
        Err(DataError::InvalidContent)
    );
    assert_eq!(
        decode_record_content(&[0; 19], SnsRecord::Eth),
        Err(DataError::InvalidContent)
    );

    assert_eq!(
        encode_record_content("1.1.1.4", SnsRecord::Ipv4),
        Err(DataError::UnsupportedRecord)
    );
    assert_eq!(
        decode_record_content(&[0xff], SnsRecord::Url),
        Err(DataError::InvalidUtf8)
    );
    assert_eq!(SnsRecord::Aaaa.label(), "AAAA");
    assert_eq!(SnsRecord::Bio.label(), "bio");
    assert_eq!(
        NAME_PROGRAM.to_string(),
        "namesLPneVptA9Z5rqUDD9tMTWEJwofgaYwp8cawRkX"
    );
    assert_eq!(
        CENTRAL_STATE_SNS_RECORDS.to_string(),
        "2pMnqHvei2N5oDcVGCRdZx48gqti199wr5CsyTTafsbo"
    );
    Ok(())
}

#[test]
fn sns_kit_codecs_prefix_and_limits() -> Result<(), Box<dyn std::error::Error>> {
    let name = NameValueCodec.encode("foo", 7)?;
    let mut padded = name.clone();
    padded.push(42);
    let limits = Limits::new(padded.len(), 3);
    assert_eq!(
        NameValueCodec.decode_prefix(&padded, 0, limits)?,
        ("foo".to_string(), name.len())
    );
    assert_eq!(
        NameValueCodec.decode(&padded, limits),
        Err(CodecError::TrailingBytes)
    );
    assert_eq!(
        NameValueCodec.decode(&name, Limits::new(name.len(), 2)),
        Err(CodecError::LimitExceeded)
    );

    let header = SnsRecordHeader {
        staleness_validation: SnsValidation::Solana,
        right_of_association_validation: SnsValidation::None,
        content_length: 3,
    };
    let encoded = SnsRecordHeaderCodec.encode(&header, RECORD_HEADER_LENGTH)?;
    assert_eq!(
        SnsRecordHeaderCodec.decode(&encoded, Limits::new(encoded.len(), 0))?,
        header
    );

    let registry = NameRegistryState {
        parent_name: SNS_ROOT_DOMAIN,
        owner: NAME_PROGRAM,
        registry_class: REVERSE_LOOKUP_CLASS,
        data: vec![1, 2, 3],
    };
    let encoded = NameRegistryCodec.encode(&registry, NAME_REGISTRY_HEADER_LENGTH + 3)?;
    assert_eq!(
        NameRegistryCodec.decode(&encoded, Limits::new(encoded.len(), 3))?,
        registry
    );
    assert_eq!(
        NameRegistryCodec.decode(&encoded, Limits::new(encoded.len(), 2)),
        Err(CodecError::LimitExceeded)
    );
    Ok(())
}

#[test]
fn sns_all_record_labels_and_protocol_addresses() {
    let labels = [
        (SnsRecord::Ipfs, "IPFS"),
        (SnsRecord::Arwv, "ARWV"),
        (SnsRecord::Sol, "SOL"),
        (SnsRecord::Eth, "ETH"),
        (SnsRecord::Btc, "BTC"),
        (SnsRecord::Ltc, "LTC"),
        (SnsRecord::Doge, "DOGE"),
        (SnsRecord::Email, "email"),
        (SnsRecord::Url, "url"),
        (SnsRecord::Discord, "discord"),
        (SnsRecord::Github, "github"),
        (SnsRecord::Reddit, "reddit"),
        (SnsRecord::Twitter, "twitter"),
        (SnsRecord::Telegram, "telegram"),
        (SnsRecord::Pic, "pic"),
        (SnsRecord::Shdw, "SHDW"),
        (SnsRecord::Point, "POINT"),
        (SnsRecord::Bsc, "BSC"),
        (SnsRecord::Injective, "INJ"),
        (SnsRecord::Backpack, "backpack"),
        (SnsRecord::Ipv4, "A"),
        (SnsRecord::Aaaa, "AAAA"),
        (SnsRecord::Cname, "CNAME"),
        (SnsRecord::Txt, "TXT"),
        (SnsRecord::Background, "background"),
        (SnsRecord::Base, "BASE"),
        (SnsRecord::Ipns, "IPNS"),
        (SnsRecord::Bio, "bio"),
    ];

    for (record, label) in labels {
        assert_eq!(record.label(), label);
    }
    for (address, text) in [
        (NAME_PROGRAM, "namesLPneVptA9Z5rqUDD9tMTWEJwofgaYwp8cawRkX"),
        (
            SNS_ROOT_DOMAIN,
            "58PwtjSDuFHuUkYjH9BYnnQKHfwo9reZhC2zMJv9JPkx",
        ),
        (
            SNS_REGISTRAR_PROGRAM,
            "jCebN34bUfdeUYJT13J1yG16XWQpt5PDx6Mse9GUqhR",
        ),
        (
            SNS_RECORDS_PROGRAM,
            "HP3D4D1ZCmohQGFVms2SS4LCANgJyksBf5s1F77FuFjZ",
        ),
        (
            CENTRAL_STATE_SNS_RECORDS,
            "2pMnqHvei2N5oDcVGCRdZx48gqti199wr5CsyTTafsbo",
        ),
        (
            REVERSE_LOOKUP_CLASS,
            "33m47vH6Eav6jr5Ry86XjhRft2jRBLDnDgPSHoquXi2Z",
        ),
        (
            NAME_TOKENIZER_PROGRAM,
            "nftD3vbNkNqfj2Sd3HZwbpw4BxxKWr4AjGb9X38JeZk",
        ),
        (
            NAME_OFFERS_PROGRAM,
            "85iDfUvr3HJyLM2zcq5BXSiDvUWfw6cSE1FfNBo8Ap29",
        ),
        (
            TWITTER_VERIFICATION_AUTHORITY,
            "FvPH7PrVrLGKPfqaf3xJodFTjZriqrAXXLTVWEorTFBi",
        ),
        (
            TWITTER_ROOT_PARENT_REGISTRY,
            "4YcexoW3r78zz16J2aqmukBLRwGq6rAvWzJpkYAXqebv",
        ),
    ] {
        assert_eq!(address.to_string(), text);
    }
}
