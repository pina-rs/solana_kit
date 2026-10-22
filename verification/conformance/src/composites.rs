//! Isolated portable client codec cases awaiting shared registry integration.

use solana_kit_core::codecs::{
    BitArray, Bytes, Choice, Decoder, Either, Encoder, Error, Limits, Number, Padded, Sentinel,
    Unit,
};

const BIT_COUNT: usize = 16;

/// Exercises deterministic typed records and zero-width union payloads.
pub fn composition() -> Result<(), &'static str> {
    let codec = Padded {
        left: 1,
        item: Choice(Unit, Number::<u16>::LE),
        right: 1,
    };
    let value = Either::Second(u16::from_le_bytes([1, 2]));
    let expected = [0, 1, 1, 2, 0];
    let encoded = codec
        .encode(&value, expected.len())
        .map_err(|_| "composition write")?;

    if encoded != expected {
        return Err("composition bytes");
    }

    if codec
        .decode(&expected, Limits::new(5, 0))
        .map_err(|_| "composition read")?
        != value
    {
        return Err("composition value");
    }

    Ok(())
}

/// Exercises backward bits with the first bit in the final byte.
pub fn bits() -> Result<(), &'static str> {
    let codec = BitArray {
        bytes: 2,
        backward: true,
    };
    let encoded = codec
        .encode(&[true, false, true], 2)
        .map_err(|_| "bits write")?;

    if encoded != [0, 5] {
        return Err("bits bytes");
    }

    let decoded = codec
        .decode(&encoded, Limits::new(2, BIT_COUNT))
        .map_err(|_| "bits read")?;

    if decoded.len() != BIT_COUNT || decoded[..3] != [true, false, true] {
        return Err("bits value");
    }

    Ok(())
}

/// Rejects shared allocation exhaustion, malformed tags and delimiter collisions.
pub fn malformed() -> Result<(), &'static str> {
    let bits = BitArray {
        bytes: 2,
        backward: false,
    };

    if bits.decode(&[0, 0], Limits::new(2, 1)) != Err(Error::LimitExceeded) {
        return Err("bits claim");
    }

    if Choice(Unit, Unit).decode(&[2], Limits::new(1, 0)) != Err(Error::InvalidDiscriminator) {
        return Err("choice tag");
    }

    let sentinel = Sentinel::new(Bytes, &[1, 1], 3).map_err(|_| "sentinel config")?;

    if sentinel.encode(&[1][..], 3) != Err(Error::InvalidValue) {
        return Err("sentinel overlap");
    }

    Ok(())
}
