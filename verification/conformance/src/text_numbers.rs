//! Portable numeric and text cases shared by every host.
use solana_kit_core::codecs::{
    Base16, BaseX, Decoder, Encoder, Error, Int256, Limits, Uint256, Utf8Config,
};
const LIMITS: Limits = Limits::new(128, 1024);

/// Executes pinned text-direction, leading-zero and odd-hex literals.
pub fn text() -> Result<(), &'static str> {
    let decimal = BaseX::base10()
        .encode("00065535", 5)
        .map_err(|_| "decimal encode")?;
    require(decimal == [0, 0, 0, 255, 255], "decimal leading zeros")?;
    let hex = Base16.encode("AbC", 2).map_err(|_| "hex encode")?;
    require(hex == [171, 12], "odd hex low nibble")?;
    let base58 = BaseX::base58()
        .decode(&[0, 4, 0], LIMITS)
        .map_err(|_| "base58 decode")?;
    require(base58 == "1Jf", "base58 literal")?;
    let text = Utf8Config::default()
        .decode(&[239, 187, 191, 65, 0], LIMITS)
        .map_err(|_| "UTF8 decode")?;
    require(text == "A\0", "UTF8 configured default")
}

/// Preserves exact signed and unsigned 256-bit boundaries without floats.
pub fn wide() -> Result<(), &'static str> {
    let unsigned: Uint256 =
        "115792089237316195423570985008687907853269984665640564039457584007913129639935"
            .parse()
            .map_err(|_| "u256 parse")?;
    require(unsigned == Uint256::MAX, "u256 max")?;
    let signed: Int256 =
        "-57896044618658097711785492504343953926634992332820282019728792003956564819968"
            .parse()
            .map_err(|_| "i256 parse")?;
    require(signed == Int256::MIN, "i256 min")
}

/// Rejects malformed alphabets, overflowing wide values, and insufficient output claims.
pub fn malformed() -> Result<(), &'static str> {
    require(
        Base16.encode("abg", 10) == Err(Error::InvalidValue),
        "invalid hex accepted",
    )?;
    require(
        Base16.decode(&[0], Limits::new(1, 1)) == Err(Error::LimitExceeded),
        "text budget ignored",
    )?;
    require(
        Uint256::from_str_radix("1", 1) == Err(Error::InvalidValue),
        "invalid radix accepted",
    )?;
    {
        use solana_kit_core::codecs::Base64;
        require(
            Base64.encode("A=Q", 1).as_deref() == Ok(&[1][..]),
            "tolerant base64",
        )?;
        require(
            Base64.encode("AB", 1) == Err(Error::InvalidValue),
            "base64 trailing bits accepted",
        )?;
    }
    Ok(())
}

/// Portable checks return explicit errors instead of panicking inside a host.
fn require(condition: bool, error: &'static str) -> Result<(), &'static str> {
    if !condition {
        return Err(error);
    }

    Ok(())
}
