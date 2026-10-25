//! UTF-16 symbols have exact reference positions, including surrogate halves.

use alloc::vec;
use solana_kit_core::codecs::{
    BaseX, Decoder, Encoder, Error, Limits, Utf16BaseX, Utf16BaseXReslice,
};

const DECIMAL_ALPHABET: [u16; 10] = [48, 49, 50, 51, 52, 53, 54, 55, 56, 57];
const BINARY_SCRATCH_DIGITS: usize = 8;
const BINARY_SINGLE_UNIT_BUDGET: usize = BINARY_SCRATCH_DIGITS + 1;

pub fn surrogate_symbols() {
    let pair = Utf16BaseX::new(&[0xd83d, 0xde00]).unwrap();
    assert_eq!(pair.encode(&[0xd83d, 0xde00], 2).unwrap(), [0, 1]);
    assert_eq!(
        pair.decode(&[0, 1], Limits::new(2, 10)),
        Ok(vec![0xd83d, 0xde00])
    );
    assert_eq!(pair.decode(&[1], Limits::new(1, 9)), Ok(vec![0xde00]));

    let repeated = Utf16BaseX::new(&[0xd83d, 0xde00, 0xd83d, 0xde01]).unwrap();
    assert_eq!(repeated.encode(&[0xde01], 1).unwrap(), [3]);
    assert_eq!(repeated.encode(&[0xd83d], 1).unwrap(), [0]);
    assert_eq!(repeated.decode(&[2], Limits::new(1, 6)), Ok(vec![0xd83d]));
    let decimal = Utf16BaseX::new(&DECIMAL_ALPHABET).unwrap();

    assert_eq!(
        decimal.encode(&[48, 48, 50, 53, 54], 4).unwrap(),
        [0, 0, 1, 0]
    );
    assert_eq!(
        decimal.decode(&[0, 0, 1, 0], Limits::new(4, 16)),
        Ok(vec![48, 48, 50, 53, 54])
    );
    assert_eq!(BaseX::base10().encode("00256", 4).unwrap(), [0, 0, 1, 0]);
}

pub fn unicode_limits() {
    assert!(Utf16BaseX::new(&[]).is_err());
    assert!(Utf16BaseX::new(&[0xd800]).is_err());

    let binary = Utf16BaseX::new(&[0xd800, 0xdc00]).unwrap();
    assert_eq!(binary.encode(&[65], 1), Err(Error::InvalidValue));
    assert_eq!(
        binary.encode(&vec![0xdc00; 4097], 8192),
        Err(Error::LimitExceeded)
    );

    assert_eq!(
        binary.decode(&[1], Limits::new(1, BINARY_SCRATCH_DIGITS)),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        binary.decode(&[1], Limits::new(1, BINARY_SINGLE_UNIT_BUDGET)),
        Ok(vec![0xdc00])
    );

    assert_eq!(
        binary.decode(&[0, 0], Limits::new(2, 2)),
        Ok(vec![0xd800, 0xd800])
    );
    assert_eq!(binary.decode(&[], Limits::new(0, 0)), Ok(vec![]));
    assert_eq!(
        binary.decode(&[1], Limits::new(0, 9)),
        Err(Error::LimitExceeded)
    );
}

pub fn surrogate_bit_reslicing() {
    let codec = Utf16BaseXReslice::new(&[0xd800, 0xdc00], 1).unwrap();
    let units = [
        0xdc00, 0xd800, 0xd800, 0xdc00, 0xd800, 0xd800, 0xdc00, 0xdc00,
    ];
    assert_eq!(codec.encode(&units, 1).unwrap(), [0x93]);
    assert_eq!(codec.decode(&[0x93], Limits::new(1, 8)).unwrap(), units);
    assert_eq!(codec.encode(&units[..7], 1).unwrap().as_slice(), b"");

    let alphabet = [
        0xd800, 0xd801, 0xd802, 0xd803, 0xd804, 0xd805, 0xd806, 0xd807,
    ];
    let triple = Utf16BaseXReslice::new(&alphabet, 3).unwrap();
    assert_eq!(
        triple.decode(&[1], Limits::new(1, 3)),
        Ok(vec![0xd800, 0xd800, 0xd802])
    );
    assert_eq!(
        triple.decode(&[1], Limits::new(1, 2)),
        Err(Error::LimitExceeded)
    );
    assert_eq!(triple.encode(&[65], 1), Err(Error::InvalidValue));

    assert!(Utf16BaseXReslice::new(&alphabet, 0).is_err());
    assert!(Utf16BaseXReslice::new(&alphabet, 17).is_err());
    assert!(Utf16BaseXReslice::new(&alphabet, 2).is_err());
}
