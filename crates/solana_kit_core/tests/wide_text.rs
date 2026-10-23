use solana_kit_core::codecs::{Decoder, Encoder, Endian, Error, Int256, Limits, Number, Uint256};

const WIDE_BYTES: usize = 32;
const HEX_DIGITS: usize = 64;
const HEX_RADIX: u32 = 16;
const OVERFLOW_BITS: usize = 257;
const OVERFLOW_INPUT: usize = 4097;
#[cfg(feature = "fixed-points")]
const BYTE_MULTIPLIER: u32 = 31;
#[cfg(feature = "fixed-points")]
const INDEX_MULTIPLIER: u32 = 71;
#[cfg(feature = "fixed-points")]
const MAX_RADIX: u32 = 36;

#[test]
fn actual_dart_wide_text_values() {
    let mut count = 0;

    for line in include_str!("fixtures/numeric-dart.tsv")
        .lines()
        .filter(|line| line.starts_with("wide\t"))
    {
        let fields: Vec<_> = line.split('\t').collect();
        let bytes: Vec<u8> = fields[1].split(',').map(|s| s.parse().unwrap()).collect();
        let array: [u8; 32] = bytes.clone().try_into().unwrap();
        let unsigned: Uint256 = fields[2].parse().unwrap();
        let signed: Int256 = fields[3].parse().unwrap();

        assert_eq!(unsigned.to_le_bytes(), array);
        assert_eq!(signed.to_le_bytes(), array);
        assert_eq!(unsigned.to_string(), fields[2]);
        assert_eq!(signed.to_string(), fields[3]);
        assert_eq!(
            Number::<Uint256>::default()
                .encode(&unsigned, WIDE_BYTES)
                .unwrap(),
            bytes
        );
        let mut big = array;
        big.reverse();
        assert_eq!(
            Number::<Int256>::new(Endian::Big)
                .encode(&signed, 32)
                .unwrap(),
            big
        );

        assert_eq!(Uint256::from_be_bytes(big), unsigned);
        assert_eq!(Int256::from_be_bytes(big), signed);
        assert_eq!(
            Number::<Int256>::BE
                .decode(&big, Limits::new(WIDE_BYTES, 0))
                .unwrap(),
            signed
        );
        count += 1;
    }
    assert_eq!(count, 512);
}

#[test]
fn wide_exact_boundaries_and_invalid_syntax() {
    let unsigned_max =
        "115792089237316195423570985008687907853269984665640564039457584007913129639935";
    let signed_min =
        "-57896044618658097711785492504343953926634992332820282019728792003956564819968";
    let signed_max =
        "57896044618658097711785492504343953926634992332820282019728792003956564819967";
    assert_eq!(unsigned_max.parse::<Uint256>().unwrap(), Uint256::MAX);
    assert_eq!(signed_min.parse::<Int256>().unwrap(), Int256::MIN);
    assert_eq!(signed_max.parse::<Int256>().unwrap(), Int256::MAX);
    assert_eq!(
        Uint256::from_str_radix(&"f".repeat(HEX_DIGITS), HEX_RADIX).unwrap(),
        Uint256::MAX
    );
    assert_eq!(
        Uint256::from_str_radix(&"1".repeat(OVERFLOW_BITS), 2),
        Err(Error::Overflow)
    );

    assert_eq!(
        Int256::from_str_radix(&("8".to_owned() + &"0".repeat(HEX_DIGITS - 1)), HEX_RADIX),
        Err(Error::Overflow)
    );

    for text in ["", "-", "+", "--1", "1.0", " 1", "0x10", "１２", "1\n"] {
        assert_eq!(text.parse::<Int256>(), Err(Error::InvalidValue));
    }

    assert_eq!("-0".parse::<Uint256>(), Err(Error::InvalidValue));
    assert_eq!("-0".parse::<Int256>().unwrap().to_le_bytes(), [0; 32]);
    assert_eq!(Uint256::from_str_radix("0", 1), Err(Error::InvalidValue));
    assert_eq!(
        Uint256::from_str_radix(&"0".repeat(OVERFLOW_INPUT), 10),
        Err(Error::LimitExceeded)
    );
    assert_eq!(Number::<u32>::default().endian(), Endian::Little);
}

#[cfg(feature = "fixed-points")]
#[test]
fn independent_bigint_radix_oracle() {
    use solana_kit_core::fixed_points::BigInt;

    for n in 0..256u32 {
        let bytes: [u8; 32] = core::array::from_fn(|i| {
            n.wrapping_mul(BYTE_MULTIPLIER)
                .wrapping_add(u32::try_from(i).unwrap() * INDEX_MULTIPLIER)
                .to_le_bytes()[0]
        });
        let mut positive = bytes.to_vec();
        positive.push(0);
        let unsigned = BigInt::from_signed_bytes_le(&positive);
        let signed = BigInt::from_signed_bytes_le(&bytes);

        for radix in 2..=MAX_RADIX {
            assert_eq!(
                Uint256::from_str_radix(&unsigned.to_str_radix(radix), radix)
                    .unwrap()
                    .to_le_bytes(),
                bytes
            );

            assert_eq!(
                Int256::from_str_radix(&signed.to_str_radix(radix), radix)
                    .unwrap()
                    .to_le_bytes(),
                bytes
            );
        }
    }
}

#[test]
fn explicit_inclusive_range_checks_do_not_coerce_values() {
    use solana_kit_core::codecs::ensure_number_range;
    assert_eq!(ensure_number_range(&1, &1, &3), Ok(()));
    assert_eq!(ensure_number_range(&3, &1, &3), Ok(()));
    assert_eq!(
        ensure_number_range(&0, &1, &3),
        Err(Error::NumberOutOfRange)
    );
    assert_eq!(
        ensure_number_range(&4, &1, &3),
        Err(Error::NumberOutOfRange)
    );
    assert_eq!(ensure_number_range(&f64::NAN, &1.0, &3.0), Ok(()));
    assert_eq!(
        "115792089237316195423570985008687907853269984665640564039457584007913129639936"
            .parse::<Uint256>(),
        Err(Error::Overflow)
    );
    assert_eq!(
        "-57896044618658097711785492504343953926634992332820282019728792003956564819969"
            .parse::<Int256>(),
        Err(Error::Overflow)
    );
}
