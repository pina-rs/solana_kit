use hex_literal::hex;
use solana_kit_core::codecs::{Error, Limits};
use solana_kit_pyth::*;
use std::borrow::Cow;

const ACCUMULATOR: &[u8] = include_bytes!("fixtures/accumulator.bin");
const VAA: &[u8] = include_bytes!("fixtures/vaa.bin");
const MESSAGE: &[u8] = include_bytes!("fixtures/wormhole-message.bin");
const PRICE: &[u8] = include_bytes!("fixtures/price-message.bin");
const SIGNATURES: usize = 13;
const PROOF_HASHES: usize = 12;
const VAA_BODY_BYTES: usize = 51;
const VAA_HEADER_BYTES: usize = 6;
const TEST_ITEM_LIMIT: usize = 4096;
const FUTURE_FIELD_BYTE: u8 = 99;

fn limits(bytes: &[u8]) -> Limits {
    Limits::new(bytes.len(), TEST_ITEM_LIMIT)
}

#[test]
fn upstream_accumulator_vaa_and_price_compose_through_public_apis() {
    let accumulator = parse_accumulator_update_data(ACCUMULATOR, limits(ACCUMULATOR)).unwrap();
    assert!(is_accumulator_update_data(ACCUMULATOR));
    assert_eq!(accumulator.minor_version, 0);
    assert!(accumulator.trailing.is_empty());
    assert!(accumulator.extension.is_empty());

    assert_eq!(accumulator.vaa, VAA);
    assert_eq!(accumulator.updates.len(), 1);
    assert_eq!(accumulator.updates[0].message, PRICE);
    assert_eq!(accumulator.updates[0].proof.len(), PROOF_HASHES);

    let vaa = parse_wormhole_vaa(accumulator.vaa, limits(VAA)).unwrap();
    assert_eq!(vaa.version, WORMHOLE_VAA_VERSION);
    assert_eq!(vaa.guardian_set_index, 4);

    assert_eq!(vaa.signatures.len(), SIGNATURES);
    assert_eq!(vaa.emitter_chain, EMITTER_CHAIN_PYTHNET);
    assert_eq!(vaa.sequence, 123_319_103);
    assert_eq!(vaa.payload, MESSAGE);

    let commitment = parse_pyth_wormhole_message(vaa.payload, limits(MESSAGE)).unwrap();
    assert_eq!(commitment.raw_payload, MESSAGE);
    assert_eq!(commitment.merkle_root.slot, 208_075_477);
    assert_eq!(commitment.merkle_root.ring_size, 10_000);
    assert_eq!(
        commitment.merkle_root.root,
        hex!("30c91600a5e258569a690c96d59f013978c78976")
    );

    let price =
        parse_pyth_price_feed_message(accumulator.updates[0].message, limits(PRICE)).unwrap();
    assert_eq!(
        price.feed_id,
        hex!("879551021853eec7a7dc827578e8e69da7e4fa8148339aa0d3d5296405be4b1a")
    );
    assert_eq!(
        price.feed_id_hex().unwrap(),
        "879551021853eec7a7dc827578e8e69da7e4fa8148339aa0d3d5296405be4b1a"
    );
    assert_eq!(price.price, 960_310_184);
    assert_eq!(price.confidence, 11_022_234);
    assert_eq!(price.exponent, -8);
    assert_eq!(price.publish_time, 1_743_631_834);
    assert_eq!(price.prev_publish_time, 1_743_631_834);

    assert_eq!(price.ema_price, 1_022_389_000);
    assert_eq!(price.ema_conf, 2_406_225);
}

#[test]
fn every_truncated_known_structure_is_rejected() {
    for end in 0..ACCUMULATOR.len() {
        assert!(parse_accumulator_update_data(&ACCUMULATOR[..end], limits(ACCUMULATOR)).is_err());
    }

    let minimum_vaa = VAA_HEADER_BYTES + SIGNATURES * VAA_SIGNATURE_SIZE + VAA_BODY_BYTES;

    for end in 0..minimum_vaa {
        assert!(parse_wormhole_vaa(&VAA[..end], limits(VAA)).is_err());
    }

    assert!(
        parse_wormhole_vaa(&VAA[..minimum_vaa], limits(VAA))
            .unwrap()
            .payload
            .is_empty()
    );

    for end in 0..WORMHOLE_VERIFICATION_SIZE {
        assert!(parse_pyth_wormhole_message(&MESSAGE[..end], limits(MESSAGE)).is_err());
    }

    for end in 0..PRICE_FEED_MESSAGE_SIZE {
        assert!(parse_pyth_price_feed_message(&PRICE[..end], limits(PRICE)).is_err());
    }
}

#[test]
fn nested_allocations_share_one_explicit_item_budget() {
    let exact = Limits::new(ACCUMULATOR.len(), 1 + PROOF_HASHES);
    assert!(parse_accumulator_update_data(ACCUMULATOR, exact).is_ok());
    assert_eq!(
        parse_accumulator_update_data(ACCUMULATOR, Limits::new(ACCUMULATOR.len(), PROOF_HASHES)),
        Err(Error::LimitExceeded)
    );
    assert!(parse_wormhole_vaa(VAA, Limits::new(VAA.len(), SIGNATURES)).is_ok());
    assert_eq!(
        parse_wormhole_vaa(VAA, Limits::new(VAA.len(), SIGNATURES - 1)),
        Err(Error::LimitExceeded)
    );

    let empty = b"PNAU\x01\0\0\0\0\0\0";
    assert!(parse_accumulator_update_data(empty, Limits::new(empty.len(), 0)).is_ok());
}

#[test]
fn byte_limits_apply_even_when_all_payload_fields_are_borrowed() {
    assert_eq!(
        parse_accumulator_update_data(
            ACCUMULATOR,
            Limits::new(ACCUMULATOR.len() - 1, TEST_ITEM_LIMIT)
        ),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        parse_pyth_wormhole_message(MESSAGE, Limits::new(0, 0)),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        parse_pyth_price_feed_message(PRICE, Limits::new(0, 0)),
        Err(Error::LimitExceeded)
    );
}

#[test]
fn compatible_minor_versions_and_extensions_are_preserved() {
    let mut bytes = ACCUMULATOR.to_vec();
    bytes[5] = 1;
    bytes.extend_from_slice(&[99, 100]);
    let parsed = parse_accumulator_update_data(&bytes, limits(&bytes)).unwrap();
    assert!(is_accumulator_update_data(&bytes));
    assert_eq!(parsed.minor_version, 1);
    assert_eq!(parsed.extension, [99, 100]);

    let mut message = MESSAGE.to_vec();
    message.push(FUTURE_FIELD_BYTE);
    assert_eq!(
        parse_pyth_wormhole_message(&message, limits(&message))
            .unwrap()
            .raw_payload,
        message
    );

    let mut price = PRICE.to_vec();
    price.push(FUTURE_FIELD_BYTE);
    assert_eq!(
        parse_pyth_price_feed_message(&price, limits(&price)),
        parse_pyth_price_feed_message(PRICE, limits(PRICE))
    );
}

#[test]
fn unsupported_major_proof_and_message_variants_fail_closed() {
    let mut accumulator = ACCUMULATOR.to_vec();
    accumulator[4] = 2;
    assert!(!is_accumulator_update_data(&accumulator));
    assert_eq!(
        parse_accumulator_update_data(&accumulator, limits(&accumulator)),
        Err(Error::InvalidDiscriminator)
    );
    accumulator[4] = 1;
    accumulator[7] = 1;
    assert!(is_accumulator_update_data(&accumulator));
    assert_eq!(
        parse_accumulator_update_data(&accumulator, limits(&accumulator)),
        Err(Error::InvalidDiscriminator)
    );

    let mut message = MESSAGE.to_vec();
    message[4] = 1;
    assert_eq!(
        parse_pyth_wormhole_message(&message, limits(&message)),
        Err(Error::InvalidDiscriminator)
    );
    let mut price = PRICE.to_vec();
    price[0] = 1;
    assert_eq!(
        parse_pyth_price_feed_message(&price, limits(&price)),
        Err(Error::InvalidDiscriminator)
    );
}

#[test]
fn the_synthetic_dart_wormhole_envelope_is_not_the_official_wire_message() {
    let legacy = [1_u8; 22];
    assert_eq!(
        parse_pyth_wormhole_message(&legacy, limits(&legacy)),
        Err(Error::InvalidConstant {
            offset: 0,
            length: WORMHOLE_VERIFICATION_MAGIC.len()
        })
    );
}

#[test]
fn signature_reduction_preserves_body_payload_and_order() {
    let trimmed = trim_vaa_signatures(VAA, DEFAULT_REDUCED_GUARDIAN_SET_SIZE, limits(VAA)).unwrap();
    assert!(matches!(trimmed, Cow::Owned(_)));
    let parsed = parse_wormhole_vaa(&trimmed, limits(&trimmed)).unwrap();
    let original = parse_wormhole_vaa(VAA, limits(VAA)).unwrap();
    assert_eq!(
        parsed.signatures,
        original.signatures[..usize::from(DEFAULT_REDUCED_GUARDIAN_SET_SIZE)]
    );
    assert_eq!(parsed.payload, original.payload);
    assert_eq!(parsed.sequence, original.sequence);
    let original_body = VAA_HEADER_BYTES + SIGNATURES * VAA_SIGNATURE_SIZE;
    let reduced_body =
        VAA_HEADER_BYTES + usize::from(DEFAULT_REDUCED_GUARDIAN_SET_SIZE) * VAA_SIGNATURE_SIZE;
    assert_eq!(&trimmed[reduced_body..], &VAA[original_body..]);
}

#[test]
fn unchanged_signature_reduction_and_headers_keep_their_narrow_contract() {
    let malformed = [2, 0, 0, 0, 4, 0];
    assert_eq!(get_guardian_set_index(&malformed), Ok(4));
    assert_eq!(get_vaa_signature_count(&malformed), Ok(0));
    assert!(matches!(
        trim_vaa_signatures(
            &malformed,
            DEFAULT_REDUCED_GUARDIAN_SET_SIZE,
            Limits::new(malformed.len(), 0)
        )
        .unwrap(),
        Cow::Borrowed(_)
    ));
    assert_eq!(
        parse_wormhole_vaa(&malformed, limits(&malformed)),
        Err(Error::InvalidDiscriminator)
    );
    assert!(get_guardian_set_index(&malformed[..4]).is_err());
    assert!(get_vaa_signature_count(&malformed[..VAA_HEADER_BYTES - 1]).is_err());

    let truncated = [1, 0, 0, 0, 4, 1];
    assert_eq!(
        trim_vaa_signatures(&truncated, 0, limits(&truncated)),
        Err(Error::Truncated)
    );
}

#[test]
fn copied_signature_reduction_claims_every_output_byte() {
    let trimmed = trim_vaa_signatures(VAA, 0, limits(VAA)).unwrap();
    assert!(trim_vaa_signatures(VAA, 0, Limits::new(VAA.len(), trimmed.len())).is_ok());
    assert_eq!(
        trim_vaa_signatures(VAA, 0, Limits::new(VAA.len(), trimmed.len() - 1)),
        Err(Error::LimitExceeded)
    );
}
