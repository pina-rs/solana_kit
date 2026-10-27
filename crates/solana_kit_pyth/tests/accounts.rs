use solana_kit_core::{
    Address,
    codecs::{Error, Limits},
};
use solana_kit_pyth::*;

const LEGACY: &[u8] = include_bytes!("fixtures/legacy-price.bin");
const FULL: &[u8] = include_bytes!("fixtures/price-update-full.bin");
const PARTIAL: &[u8] = include_bytes!("fixtures/price-update-partial.bin");
const AGGREGATE_OFFSET: usize = 208;
const EMA_PRICE_OFFSET: usize = 48;
const LEGACY_ACCOUNT_SIZE: u32 = 432;
const FIXTURE_EXPONENT: i32 = -8;
const FIXTURE_SIGNATURES: u8 = 5;
const FRESH_CONTEXT_SLOT: u64 = 1025;
const STALE_CONTEXT_SLOT: u64 = 1026;

fn limits(bytes: &[u8]) -> Limits {
    Limits::new(bytes.len(), 2)
}

#[test]
fn legacy_reference_header_and_rational_fields() {
    let account = decode_pyth_price_account(LEGACY, None, limits(LEGACY)).unwrap();
    assert_eq!(
        (account.magic, account.version, account.account_type),
        (PYTH_MAGIC, 2, PythAccountType::Price)
    );
    assert_eq!(
        (account.size, account.price_type, account.exponent),
        (LEGACY_ACCOUNT_SIZE, PythPriceType::Price, FIXTURE_EXPONENT)
    );
    assert_eq!((account.num_component_prices, account.num_quoters), (2, 2));
    assert_eq!((account.last_slot, account.valid_slot), (100, 120));

    assert_eq!(
        account.ema_price,
        PythEma {
            value_component: 555,
            numerator: 999,
            denominator: 11
        }
    );
    assert_eq!(
        account.ema_confidence,
        PythEma {
            value_component: 6,
            numerator: 77,
            denominator: 7
        }
    );
}

#[test]
fn legacy_publication_metadata_and_linked_accounts() {
    let account = decode_pyth_price_account(LEGACY, None, limits(LEGACY)).unwrap();
    assert_eq!(
        (
            account.timestamp,
            account.min_publishers,
            account.message_sent,
            account.max_latency,
            account.feed_index
        ),
        (1_700_000_000, 3, 0, 5, 777)
    );
    assert!(account.flags.accumulator_v2 && account.flags.message_buffer_cleared);
    assert_eq!(account.product_account_key, Address::from_bytes([0xcd; 32]));
    assert_eq!(
        account.next_price_account_key,
        Some(Address::from_bytes([0xab; 32]))
    );
    assert_eq!(
        (
            account.previous_slot,
            account.previous_price_component,
            account.previous_confidence_component,
            account.previous_timestamp
        ),
        (119, 1_200_000_000, 9, 1_699_999_000)
    );
    assert_eq!(
        (
            account.aggregate.price_component,
            account.aggregate.confidence_component,
            account.aggregate.publish_slot
        ),
        (1_234_000_000, 60_000, 1000)
    );
    assert!(account.extension.is_empty());
}

#[test]
fn publisher_keys_and_both_samples_keep_encoded_order() {
    let account = decode_pyth_price_account(LEGACY, None, limits(LEGACY)).unwrap();
    assert_eq!(account.price_components.len(), 2);

    for (component, publisher) in account.price_components.iter().zip([0xee, 0xef]) {
        assert_eq!(component.publisher, Address::from_bytes([publisher; 32]));
        assert_eq!(
            component.aggregate,
            PythPriceInfo {
                price_component: 1,
                confidence_component: 2,
                status: PythPriceStatus::Trading,
                corporate_action: 0,
                publish_slot: 995,
            }
        );
        assert_eq!(
            component.latest,
            PythPriceInfo {
                price_component: 3,
                confidence_component: 4,
                status: PythPriceStatus::Trading,
                corporate_action: 0,
                publish_slot: 999,
            }
        );
    }
}

#[test]
fn legacy_unsigned_extremes_zero_next_and_borrowed_extension() {
    let mut bytes = LEGACY.to_vec();
    bytes[32..40].copy_from_slice(&u64::MAX.to_le_bytes());
    bytes[144..176].fill(0);
    bytes[208..216].copy_from_slice(&i64::MIN.to_le_bytes());
    bytes[216..224].copy_from_slice(&u64::MAX.to_le_bytes());
    bytes[228..232].copy_from_slice(&u32::MAX.to_le_bytes());
    bytes.extend_from_slice(&[77, 88]);

    let account = decode_pyth_price_account(&bytes, None, limits(&bytes)).unwrap();
    assert_eq!(account.last_slot, u64::MAX);
    assert_eq!(account.next_price_account_key, None);
    assert_eq!(account.aggregate.price_component, i64::MIN);
    assert_eq!(account.aggregate.confidence_component, u64::MAX);
    assert_eq!(account.aggregate.corporate_action, u32::MAX);

    assert_eq!(account.extension, [77, 88]);
    assert_eq!(account.extension.as_ptr(), bytes[LEGACY.len()..].as_ptr());
}

#[test]
fn effective_staleness_never_rewrites_the_recorded_sample() {
    for (slot, expected) in [
        (None, PythPriceStatus::Trading),
        (Some(900), PythPriceStatus::Trading),
        (Some(FRESH_CONTEXT_SLOT), PythPriceStatus::Trading),
        (Some(STALE_CONTEXT_SLOT), PythPriceStatus::Unknown),
        (Some(u64::MAX), PythPriceStatus::Unknown),
    ] {
        let account = decode_pyth_price_account(LEGACY, slot, limits(LEGACY)).unwrap();
        assert_eq!(account.status, expected);
        assert_eq!(account.aggregate.status, PythPriceStatus::Trading);
    }

    let mut bytes = LEGACY.to_vec();
    bytes[224..228].copy_from_slice(&2_u32.to_le_bytes());
    let account = decode_pyth_price_account(&bytes, Some(u64::MAX), limits(&bytes)).unwrap();
    assert_eq!(account.status, PythPriceStatus::Halted);
    bytes[224..228].copy_from_slice(&1_u32.to_le_bytes());
    bytes[232..240].copy_from_slice(&u64::MAX.to_le_bytes());
    let account = decode_pyth_price_account(&bytes, Some(u64::MAX), limits(&bytes)).unwrap();
    assert_eq!(account.status, PythPriceStatus::Trading);
}

#[test]
fn every_truncated_legacy_prefix_and_wrong_header_is_rejected() {
    for end in 0..LEGACY.len() {
        assert!(decode_pyth_price_account(&LEGACY[..end], None, limits(LEGACY)).is_err());
    }

    for offset in [0, 4, 8] {
        let mut bytes = LEGACY.to_vec();
        bytes[offset] ^= 0xff;
        assert_eq!(
            decode_pyth_price_account(&bytes, None, limits(&bytes)),
            Err(Error::InvalidDiscriminator)
        );
    }
}

#[test]
fn component_claims_and_input_lengths_are_bounded_before_reservation() {
    for maximum in [0, 1] {
        assert_eq!(
            decode_pyth_price_account(LEGACY, None, Limits::new(LEGACY.len(), maximum)),
            Err(Error::LimitExceeded)
        );
    }

    assert_eq!(
        decode_pyth_price_account(LEGACY, None, Limits::new(LEGACY.len() - 1, 2)),
        Err(Error::LimitExceeded)
    );
    let mut bytes = LEGACY.to_vec();
    bytes[24..28].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(matches!(
        decode_pyth_price_account(&bytes, None, Limits::new(bytes.len(), usize::MAX)),
        Err(Error::Truncated | Error::Overflow)
    ));
    bytes[24..28].fill(0);
    let account = decode_pyth_price_account(&bytes, None, Limits::new(bytes.len(), 0)).unwrap();
    assert!(account.price_components.is_empty());
    assert_eq!(account.extension, &bytes[PYTH_PRICE_ACCOUNT_HEADER_SIZE..]);
}

#[test]
fn standalone_samples_use_checked_offsets_and_signed_rational_components() {
    let account = decode_pyth_price_account(LEGACY, None, limits(LEGACY)).unwrap();
    assert_eq!(
        PythPriceInfo::from_bytes(LEGACY, AGGREGATE_OFFSET, limits(LEGACY)),
        Ok(account.aggregate)
    );
    assert_eq!(
        PythEma::from_bytes(LEGACY, EMA_PRICE_OFFSET, limits(LEGACY)),
        Ok(account.ema_price)
    );
    assert_eq!(
        PythPriceInfo::from_bytes(LEGACY, usize::MAX, limits(LEGACY)),
        Err(Error::Truncated)
    );
    assert_eq!(
        PythEma::from_bytes(LEGACY, LEGACY.len() - 1, limits(LEGACY)),
        Err(Error::Truncated)
    );

    let mut rational = [0; 24];
    rational[..8].copy_from_slice(&i64::MIN.to_le_bytes());
    rational[8..16].copy_from_slice(&(-42_i64).to_le_bytes());
    assert_eq!(
        PythEma::from_bytes(&rational, 0, limits(&rational)),
        Ok(PythEma {
            value_component: i64::MIN,
            numerator: -42,
            denominator: 0
        })
    );
}

#[test]
fn legacy_unknown_values_and_known_flags_match_reference_mapping() {
    for value in [0, 6, u32::MAX] {
        assert_eq!(PythAccountType::from_value(value), PythAccountType::Unknown);
        assert_eq!(PythPriceStatus::from_value(value), PythPriceStatus::Unknown);
        assert_eq!(PythPriceType::from_value(value), PythPriceType::Unknown);
    }

    assert_eq!(PythAccountType::from_value(5), PythAccountType::Permission);
    assert_eq!(PythPriceStatus::from_value(4), PythPriceStatus::Ignored);
    assert_eq!(PythCorpAction::NoCorpAct as u32, 0);

    for (bits, accumulator_v2, message_buffer_cleared) in [
        (0, false, false),
        (1, true, false),
        (2, false, true),
        (3, true, true),
        (0xfc, false, false),
        (0xff, true, true),
    ] {
        let flags = PythPriceFlags::from_bits(bits);
        assert_eq!(flags.accumulator_v2, accumulator_v2);
        assert_eq!(flags.message_buffer_cleared, message_buffer_cleared);
    }
}

#[test]
fn full_and_partial_receiver_variants_decode_the_same_exact_price_fields() {
    assert_eq!(FULL.len(), PRICE_UPDATE_V2_FULL_SIZE);
    assert_eq!(PARTIAL.len(), PRICE_UPDATE_V2_PARTIAL_SIZE);
    let full = decode_price_update_v2_account(FULL, Limits::new(FULL.len(), 0)).unwrap();
    let partial = decode_price_update_v2_account(PARTIAL, Limits::new(PARTIAL.len(), 0)).unwrap();
    assert_eq!(full.write_authority, Address::from_bytes([0x33; 32]));
    assert_eq!(full.price_message, partial.price_message);

    assert_eq!(
        full.price_message,
        PythPriceFeedMessage {
            feed_id: [0x11; 32],
            price: 3_000_123_456_789,
            confidence: u64::MAX,
            exponent: FIXTURE_EXPONENT,
            publish_time: 1_700_000_000,
            prev_publish_time: 1_699_999_900,
            ema_price: -2_999_000_000,
            ema_conf: u64::MAX,
        }
    );

    assert_eq!(full.verification_level, PythVerificationLevel::Full);
    assert_eq!(
        partial.verification_level,
        PythVerificationLevel::Partial {
            num_signatures: FIXTURE_SIGNATURES
        }
    );
    assert_eq!((full.posted_slot, partial.posted_slot), (4242, 4242));
    assert_eq!(full.feed_id_hex().unwrap(), "11".repeat(32));
    assert!(full.extension.is_empty() && partial.extension.is_empty());
}

#[test]
fn receiver_padding_and_extensions_borrow_the_original_account() {
    for original in [FULL, PARTIAL] {
        let mut bytes = original.to_vec();
        bytes.extend_from_slice(&[0, 33, 44]);
        let account = decode_price_update_v2_account(&bytes, Limits::new(bytes.len(), 0)).unwrap();
        assert_eq!(account.extension, [0, 33, 44]);
        assert_eq!(account.extension.as_ptr(), bytes[original.len()..].as_ptr());
        assert_eq!(
            decode_price_update_v2_account(&bytes, Limits::new(bytes.len() - 1, 0)),
            Err(Error::LimitExceeded)
        );
    }
}

#[test]
fn malformed_receiver_prefixes_tags_and_verification_variants_fail() {
    for original in [FULL, PARTIAL] {
        for end in 0..original.len() {
            assert!(decode_price_update_v2_account(&original[..end], limits(original)).is_err());
        }
    }

    let mut bytes = PARTIAL.to_vec();
    bytes[0] ^= 1;
    assert!(matches!(
        decode_price_update_v2_account(&bytes, limits(&bytes)),
        Err(Error::InvalidConstant {
            offset: 0,
            length: 8
        })
    ));
    bytes[0] ^= 1;
    bytes[40] = 2;
    assert_eq!(
        decode_price_update_v2_account(&bytes, limits(&bytes)),
        Err(Error::InvalidDiscriminator)
    );
}

#[test]
fn full_verification_orders_above_every_partial_count() {
    let full = PythVerificationLevel::Full;
    assert!(full.is_full());
    assert_eq!(full.num_signatures(), None);

    for count in 0..=u8::MAX {
        let partial = PythVerificationLevel::Partial {
            num_signatures: count,
        };
        assert!(!partial.is_full());
        assert_eq!(partial.num_signatures(), Some(count));
        assert!(full > partial);
        assert!(partial >= PythVerificationLevel::Partial { num_signatures: 0 });
    }
}
