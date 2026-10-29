//! These assertions use the actual public renderer's output, not handwritten codecs.
use crate::numeric_contracts as n;
use solana_kit_core::codecs::{Decoder, Encoder, Limits};

const LIMITS: Limits = Limits::new(256, 256);
const U128_HIGH_BIT: u128 = 1 << (u128::BITS - 1);
const U16_PREFIXED_TRUE: [u8; 3] = [0, 1, 1];
const PING_TAG: [u8; 4] = [1, 2, 3, 4];
const PONG_TAG: [u8; 4] = [1, 2, 3, 5];

fn check<T, C>(codec: C, value: T, bytes: &[u8])
where
    T: PartialEq + core::fmt::Debug,
    C: Encoder<T> + Decoder<T>,
{
    assert_eq!(codec.encode(&value, bytes.len()).unwrap(), bytes);
    assert_eq!(codec.decode(bytes, LIMITS).unwrap(), value);

    for length in 0..bytes.len() {
        assert!(codec.decode(&bytes[..length], LIMITS).is_err());
    }
}

macro_rules! scalar {
    ($model:ident, $codec:ident, $value:expr, $bytes:expr) => {
        check(n::$codec, n::$model { value: $value }, &$bytes);
    };
}

fn integers() {
    for value in [i32::MIN, -1, 0, i32::MAX] {
        scalar!(I32Le, I32LeCodec, value, value.to_le_bytes());
        scalar!(I32Be, I32BeCodec, value, value.to_be_bytes());
    }

    for value in [u128::MIN, U128_HIGH_BIT, u128::MAX] {
        scalar!(U128Le, U128LeCodec, value, value.to_le_bytes());
        scalar!(U128Be, U128BeCodec, value, value.to_be_bytes());
    }

    for value in [i128::MIN, -1, 0, i128::MAX] {
        scalar!(I128Le, I128LeCodec, value, value.to_le_bytes());
        scalar!(I128Be, I128BeCodec, value, value.to_be_bytes());
    }
}

macro_rules! floating {
    ($model:ident, $codec:ident, $value:expr, $bytes:expr) => {{
        let bytes = $bytes;
        let value = n::$model { value: $value };
        assert_eq!(n::$codec.encode(&value, bytes.len()).unwrap(), bytes);
        assert_eq!(
            n::$codec.decode(&bytes, LIMITS).unwrap().value.to_bits(),
            value.value.to_bits()
        );

        for length in 0..bytes.len() {
            assert!(n::$codec.decode(&bytes[..length], LIMITS).is_err());
        }
    }};
}

fn floats() {
    for value in [
        f32::MIN,
        f32::MAX,
        f32::NEG_INFINITY,
        f32::INFINITY,
        -0.0,
        f32::from_bits(0x7fc0_1234),
    ] {
        floating!(F32Le, F32LeCodec, value, value.to_le_bytes());
        floating!(F32Be, F32BeCodec, value, value.to_be_bytes());
    }
}

fn prefixes() {
    scalar!(U16Be, U16BeCodec, 258, [1, 2]);
    scalar!(BoolArrayBe, BoolArrayBeCodec, vec![true], U16_PREFIXED_TRUE);
    scalar!(
        SizedBoolBe,
        SizedBoolBeCodec,
        true,
        [0, 0, 0, 0, 0, 0, 0, 1, 1]
    );
    scalar!(
        OptionalBoolBe,
        OptionalBoolBeCodec,
        Some(true),
        U16_PREFIXED_TRUE
    );
    scalar!(OptionalBoolBe, OptionalBoolBeCodec, None, [0, 0]);
    scalar!(
        FixedOptionalBoolBe,
        FixedOptionalBoolBeCodec,
        Some(true),
        [0, 0, 0, 1, 1]
    );
    scalar!(
        FixedOptionalBoolBe,
        FixedOptionalBoolBeCodec,
        None,
        [0, 0, 0, 0, 0]
    );
}

fn reject_prefixes() {
    assert!(n::BoolArrayBeCodec.decode(&[0, 2, 1], LIMITS).is_err());
    assert!(n::OptionalBoolBeCodec.decode(&[0, 2, 1], LIMITS).is_err());
    assert!(n::OptionalBoolBeCodec.decode(&[1, 0, 1], LIMITS).is_err());
    assert!(
        n::FixedOptionalBoolBeCodec
            .decode(&[0, 0, 0, 2, 1], LIMITS)
            .is_err()
    );
    assert!(
        n::SizedBoolBeCodec
            .decode(&[0, 0, 0, 0, 0, 0, 0, 2, 1], LIMITS)
            .is_err()
    );
}

fn instruction_tags() {
    check(n::PingCodec, n::Ping {}, &PING_TAG);
    check(n::PongCodec, n::Pong {}, &PONG_TAG);
    let ping = n::Ping {}.instruction(&[], LIMITS).unwrap();
    let pong = n::Pong {}.instruction(&[], LIMITS).unwrap();
    assert_eq!(ping.data, PING_TAG);
    assert_eq!(pong.data, PONG_TAG);
    assert_eq!(n::identify_instruction(&PING_TAG).unwrap(), PING_TAG);
    assert_eq!(n::identify_instruction(&PONG_TAG).unwrap(), PONG_TAG);

    assert!(matches!(
        n::parse_instruction(&ping, LIMITS).unwrap(),
        n::NumericContractsInstructionData::Ping(_)
    ));
    assert!(matches!(
        n::parse_instruction(&pong, LIMITS).unwrap(),
        n::NumericContractsInstructionData::Pong(_)
    ));
    assert!(n::Ping::parse(&pong, LIMITS).is_err());
    assert!(n::Pong::parse(&ping, LIMITS).is_err());
    reject_instruction_tags();
}

fn reject_instruction_tags() {
    for length in 0..PING_TAG.len() {
        assert!(n::identify_instruction(&PING_TAG[..length]).is_err());
        assert!(
            n::NumericContractsInstructionDataCodec
                .decode(&PING_TAG[..length], LIMITS)
                .is_err()
        );
    }

    for bytes in [[4, 3, 2, 1], [5, 3, 2, 1], [1, 2, 3, 6]] {
        assert!(n::identify_instruction(&bytes).is_err());
        assert!(
            n::NumericContractsInstructionDataCodec
                .decode(&bytes, LIMITS)
                .is_err()
        );
    }
}

pub(super) fn run() {
    integers();
    floats();
    prefixes();
    reject_prefixes();
    instruction_tags();
}
