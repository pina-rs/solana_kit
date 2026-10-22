//! Exact 256-bit boundary fixtures, independent of arithmetic implementations.

use solana_kit_core::codecs::{Decoder, Encoder, Error, Int256, Limits, Number, Uint256};

const WIDTH: usize = 32;
const LIMITS: Limits = Limits::new(WIDTH, 0);

#[test]
fn unsigned_wide_numbers_preserve_all_bits_in_both_orders() {
    let mut little = [0; WIDTH];
    little[0] = 42;
    let mut big = [0; WIDTH];
    big[WIDTH - 1] = 42;
    let value = Uint256::from_le_bytes(little);

    assert_eq!(value.to_le_bytes(), little);
    assert_eq!(Number::<Uint256>::LE.encode(&value, WIDTH).unwrap(), little);
    assert_eq!(Number::<Uint256>::BE.encode(&value, WIDTH).unwrap(), big);
    assert_eq!(Number::<Uint256>::LE.decode(&little, LIMITS), Ok(value));
    assert_eq!(Number::<Uint256>::BE.decode(&big, LIMITS), Ok(value));

    let maximum = Uint256::from_le_bytes([255; WIDTH]);
    assert_eq!(
        Number::<Uint256>::LE.decode(&[255; WIDTH], LIMITS),
        Ok(maximum)
    );
    assert_eq!(
        Number::<Uint256>::LE.decode(&[0; WIDTH - 1], LIMITS),
        Err(Error::Truncated)
    );
}

#[test]
fn signed_wide_numbers_use_twos_complement_without_normalization() {
    let minus_one = Int256::from_le_bytes([255; WIDTH]);
    assert_eq!(minus_one.to_le_bytes(), [255; WIDTH]);
    assert_eq!(
        Number::<Int256>::LE.encode(&minus_one, WIDTH).unwrap(),
        [255; WIDTH]
    );
    assert_eq!(
        Number::<Int256>::BE.decode(&[255; WIDTH], LIMITS),
        Ok(minus_one)
    );

    let mut little = [0; WIDTH];
    little[WIDTH - 1] = 128;
    let minimum = Int256::from_le_bytes(little);
    let mut big = [0; WIDTH];
    big[0] = 128;

    assert_eq!(Number::<Int256>::BE.encode(&minimum, WIDTH).unwrap(), big);
    assert_eq!(Number::<Int256>::LE.decode(&little, LIMITS), Ok(minimum));
    assert_eq!(
        Number::<Int256>::LE.decode(&[0; WIDTH - 1], LIMITS),
        Err(Error::Truncated)
    );
}
