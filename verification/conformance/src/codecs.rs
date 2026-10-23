//! Pinned Dart numeric/constant vectors and independently specified Borsh layouts.

use solana_kit_core::codecs::{
    Affix, Bool, Count, Decoder, Encoder, Error, Limits, Number, OptionCodec, Sequence, ShortU16,
    SizePrefix, SizePrefix64, Utf8,
};

use super::require;

const LIMITS: Limits = Limits::new(128, 32);
const EXAMPLE: u32 = 42;
const U64_EXAMPLE: u64 = 0xffff_ffff;
const SHORT_MAX: u16 = 65535;

/// Executes immutable u32/u64 byte fixtures from the pinned Dart tests.
pub(super) fn numbers() -> Result<(), &'static str> {
    let encoded = Number::<u32>::LE
        .encode(&EXAMPLE, 4)
        .map_err(|_| "u32 encode failed")?;
    require(encoded == [42, 0, 0, 0], "u32 byte fixture mismatch")?;
    let decoded = Number::<u64>::BE
        .decode(&[0, 0, 0, 0, 255, 255, 255, 255], LIMITS)
        .map_err(|_| "u64 decode failed")?;
    require(decoded == U64_EXAMPLE, "u64 byte fixture mismatch")
}

/// Checks canonical and malformed compact integers on each host.
pub(super) fn short_u16() -> Result<(), &'static str> {
    require(
        ShortU16.decode(&[255, 255, 3], LIMITS) == Ok(SHORT_MAX),
        "short-u16 fixture mismatch",
    )?;
    require(
        ShortU16
            .encode(&SHORT_MAX, 3)
            .map_err(|_| "short-u16 encode failed")?
            == [255, 255, 3],
        "short-u16 encode mismatch",
    )?;
    require(
        ShortU16.decode(&[128, 0], LIMITS) == Err(Error::NonCanonical),
        "overlong compact integer accepted",
    )?;
    require(
        ShortU16.decode(&[255, 255, 4], LIMITS) == Err(Error::Overflow),
        "overflowing compact integer accepted",
    )
}

/// Preserves Borsh byte lengths and distinguishes bincode's u64 prefix.
pub(super) fn strings() -> Result<(), &'static str> {
    let borsh = [3, 0, 0, 0, b'h', 0xc3, 0xa9];
    let bincode = [3, 0, 0, 0, 0, 0, 0, 0, b'h', 0xc3, 0xa9];
    require(
        SizePrefix(Utf8)
            .encode("hé", borsh.len())
            .map_err(|_| "Borsh string encode failed")?
            == borsh,
        "Borsh string byte fixture mismatch",
    )?;
    require(
        SizePrefix64(Utf8)
            .decode(&bincode, LIMITS)
            .map_err(|_| "bincode string decode failed")?
            == "hé",
        "bincode string fixture mismatch",
    )?;
    require(
        SizePrefix(Utf8).decode(&[1, 0, 0, 0, 255], LIMITS) == Err(Error::InvalidUtf8),
        "invalid UTF-8 accepted",
    )
}

/// Counts allocations before decoding elements, including zero-width elements.
pub(super) fn bounded_sequences() -> Result<(), &'static str> {
    let codec = Sequence {
        item: Number::<u16>::LE,
        count: Count::U32,
    };
    let bytes = [2, 0, 0, 0, 1, 0, 255, 0];
    let values = codec
        .decode(&bytes, LIMITS)
        .map_err(|_| "vector decode failed")?;
    require(values == [1, 255], "vector byte fixture mismatch")?;
    require(
        codec.decode(&[255; 4], LIMITS) == Err(Error::LimitExceeded),
        "malicious vector length accepted",
    )?;
    require(
        codec.decode(&[], LIMITS) == Err(Error::Truncated),
        "missing vector length accepted",
    )
}

/// Differentiates option layouts and rejects noncanonical presence flags.
pub(super) fn options() -> Result<(), &'static str> {
    let item = Number::<u32>::LE;
    let tagged = OptionCodec::u8(item);
    require(
        tagged.decode(&[1, 42, 0, 0, 0], LIMITS) == Ok(Some(EXAMPLE)),
        "u8 option fixture mismatch",
    )?;
    require(
        tagged.decode(&[2, 0, 0, 0, 0], LIMITS) == Err(Error::InvalidDiscriminator),
        "invalid option tag accepted",
    )?;
    require(
        OptionCodec::c_option(item, 4).decode(&[0, 0, 0, 0, 99, 99, 99, 99], LIMITS) == Ok(None),
        "C option stale absent payload rejected",
    )?;
    require(
        OptionCodec::remainder(item).decode(&[], LIMITS) == Ok(None),
        "remainder option fixture mismatch",
    )
}

/// Validates exact constant framing, full decoding, and prefix offsets.
pub(super) fn framing() -> Result<(), &'static str> {
    let codec = Affix {
        prefix: &[1, 2, 3],
        item: Bool,
        suffix: &[4, 5],
    };
    require(
        codec.decode(&[1, 2, 3, 1, 4, 5], LIMITS) == Ok(true),
        "constant frame fixture mismatch",
    )?;
    require(
        codec.decode(&[1, 2, 3, 1, 4, 6], LIMITS)
            == Err(Error::InvalidConstant {
                offset: 4,
                length: 2,
            }),
        "forged suffix accepted",
    )?;
    require(
        Number::<u32>::LE.decode(&[42, 0, 0, 0, 99], LIMITS) == Err(Error::TrailingBytes),
        "trailing bytes accepted",
    )?;
    let prefix = Number::<u32>::LE.decode_prefix(&[99, 42, 0, 0, 0, 88], 1, LIMITS);
    require(prefix == Ok((EXAMPLE, 5)), "prefix cursor offset mismatch")
}
