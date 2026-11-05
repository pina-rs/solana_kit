use solana_kit_core::{Address, crypto::is_on_curve};

/// Exact number of compressed-point classifications performed by [`classification`].
pub const CLASSIFICATIONS: usize = 7;

/// Builds a compressed Edwards y encoding with an explicit low and high byte.
const fn encoding(low: u8, fill: u8, high: u8) -> [u8; 32] {
    let mut bytes = [fill; 32];
    bytes[0] = low;
    bytes[31] = high;
    bytes
}

/// The Edwards basepoint encoding from RFC 8032 section 5.1.4.
const BASEPOINT: [u8; 32] = encoding(0x58, 0x66, 0x66);
/// The neutral element y = 1 with a clear sign bit.
const IDENTITY: [u8; 32] = encoding(1, 0, 0);
/// The y = p - 1 encoding with a clear sign bit.
const MINUS_ONE: [u8; 32] = encoding(0xec, 0xff, 0x7f);
/// y = 0 is a valid point because sqrt(-1) exists modulo p.
const ZERO: [u8; 32] = [0; 32];
/// y = 2 has no matching x coordinate.
const TWO: [u8; 32] = encoding(2, 0, 0);
/// The neutral element encoding with the sign bit set.
const SIGNED_IDENTITY: [u8; 32] = encoding(1, 0, 0x80);
/// The y = p - 1 encoding with the sign bit set.
const SIGNED_MINUS_ONE: [u8; 32] = encoding(0xec, 0xff, 0xff);

/// Compares one fixed encoding against the public curve predicate.
fn matches(bytes: [u8; 32], expected: bool) -> bool {
    is_on_curve(&Address::from_bytes(bytes)) == expected
}

/// Negative control: flipping one expected classification must fail the same comparison.
pub fn rejects_wrong_expected() -> Result<(), &'static str> {
    if matches(BASEPOINT, false) {
        return Err("mutated curve expectation was not rejected");
    }

    Ok(())
}

/// Exact public predicate results for standard and runtime-compatible encodings.
pub fn classification() -> Result<(), &'static str> {
    let cases = [
        (BASEPOINT, true, "basepoint was not on curve"),
        (IDENTITY, true, "identity was not on curve"),
        (MINUS_ONE, true, "y = p - 1 was not on curve"),
        (ZERO, true, "y = 0 was not on curve"),
        (TWO, false, "y = 2 was accepted on curve"),
        (
            SIGNED_IDENTITY,
            true,
            "runtime zero-x sign edge changed at identity",
        ),
        (
            SIGNED_MINUS_ONE,
            true,
            "runtime zero-x sign edge changed at y = p - 1",
        ),
    ];

    if cases.len() != CLASSIFICATIONS {
        return Err("curve classification ledger changed");
    }

    for (bytes, expected, explanation) in cases {
        if !matches(bytes, expected) {
            return Err(explanation);
        }
    }

    Ok(())
}
