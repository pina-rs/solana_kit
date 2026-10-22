//! Exact fixed-point vectors from pinned Dart tests and independent wire bytes.
use super::require;
use solana_kit_core::{
    codecs::{Decoder, Encoder, Endian, Limits},
    fixed_points::{BinaryShape, DecimalShape, Error, Rounding, Signedness},
};

/// Executes reference decimal strict parsing and signed binary tie cases.
pub(super) fn parsing() -> Result<(), &'static str> {
    let decimal =
        DecimalShape::new(Signedness::Signed, u16::BITS, 2).map_err(|_| "decimal shape")?;
    require(
        decimal.parse("1.000", Rounding::Strict) == Err(Error::PrecisionLoss),
        "strict excess-zero precision accepted",
    )?;
    let binary = BinaryShape::new(Signedness::Signed, u16::BITS, 4).map_err(|_| "binary shape")?;
    let value = binary
        .parse("-0.09375", Rounding::Nearest)
        .map_err(|_| "binary tie parse")?;
    require(
        value.to_string() == "-0.125",
        "negative tie must round away from zero",
    )
}

/// Checks exact negative division and a scale larger than storage width.
pub(super) fn arithmetic() -> Result<(), &'static str> {
    let shape = DecimalShape::new(Signedness::Signed, u16::BITS, 2).map_err(|_| "decimal shape")?;
    let value = shape
        .parse("-1", Rounding::Strict)
        .map_err(|_| "decimal parse")?;
    let result = value
        .checked_div_scalar(3, Rounding::Floor)
        .map_err(|_| "decimal divide")?;
    require(
        result.to_string() == "-0.34",
        "negative floor quotient mismatch",
    )?;
    let shape = BinaryShape::new(Signedness::Unsigned, u8::BITS, 12).map_err(|_| "binary shape")?;
    let value = shape
        .parse("0.062255859375", Rounding::Strict)
        .map_err(|_| "fine quantum parse")?;
    require(
        value.raw() == &u8::MAX.into(),
        "scale greater than width mismatch",
    )
}

/// Preserves signed two's-complement bytes and rejects truncation.
pub(super) fn codecs() -> Result<(), &'static str> {
    let shape = DecimalShape::new(Signedness::Signed, u16::BITS, 2).map_err(|_| "decimal shape")?;
    let codec = shape.codec(Endian::Big).map_err(|_| "decimal codec")?;
    let value = shape.raw(-2).map_err(|_| "decimal raw")?;
    let bytes = [u8::MAX, u8::MAX - 1];
    require(
        codec.encode(&value, 2).map_err(|_| "decimal encode")? == bytes,
        "signed fixed-point bytes mismatch",
    )?;
    require(
        codec
            .decode(&bytes, Limits::new(2, 2))
            .map_err(|_| "decimal decode")?
            == value,
        "signed fixed-point decode mismatch",
    )?;
    require(
        codec.decode(&bytes[..1], Limits::new(2, 2)).is_err(),
        "truncated fixed-point accepted",
    )
}

use alloc::string::ToString;

/// Appends the three fixed-point cases without growing the root suite builder.
pub(super) fn cases() -> [super::CaseResult; 3] {
    [
        super::case("fixed-point-exact-parsing", parsing()),
        super::case("fixed-point-exact-arithmetic", arithmetic()),
        super::case("fixed-point-exact-codecs", codecs()),
    ]
}
