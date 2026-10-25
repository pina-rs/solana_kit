//! Literal arbitrary-width unsigned and two's-complement integer boundaries.

use solana_kit_core::codecs::{
    BigInt, BigNumber, BigUint, Decoder, Encoder, Endian, Error, Limits, Reader, Size,
};

const SIGNED24_MINIMUM: i32 = -8_388_608;
const SIGNED24_MAXIMUM: i32 = 8_388_607;
const BELOW_SIGNED_BYTE: i16 = -129;
const ABOVE_SIGNED_BYTE: i16 = 128;

pub fn arbitrary_width_literals() {
    let unsigned = BigNumber::<BigUint>::new(3, Endian::Little).unwrap();
    let signed = BigNumber::<BigInt>::new(3, Endian::Big).unwrap();
    assert_eq!(
        unsigned.encode(&BigUint::from(0x123456u32), 3).unwrap(),
        [0x56, 0x34, 0x12]
    );
    assert_eq!(
        unsigned.decode(&[0x56, 0x34, 0x12], Limits::new(3, 3)),
        Ok(BigUint::from(0x123456u32))
    );
    assert_eq!(unsigned.encode_size(), Ok(Size::Fixed(3)));
    assert_eq!(unsigned.decode_size(), Ok(Size::Fixed(3)));

    assert_eq!(
        signed.encode(&BigInt::from(-2), 3).unwrap(),
        [255, 255, 254]
    );
    assert_eq!(
        signed.decode(&[128, 0, 0], Limits::new(3, 3)),
        Ok(BigInt::from(SIGNED24_MINIMUM))
    );
    assert_eq!(
        signed.decode(&[127, 255, 255], Limits::new(3, 3)),
        Ok(BigInt::from(SIGNED24_MAXIMUM))
    );

    let wide = BigNumber::<BigUint>::new(33, Endian::Little).unwrap();
    let mut literal = [0; 33];
    literal[32] = 1;
    let value = BigUint::from(1u8) << 256usize;

    assert_eq!(wide.encode(&value, 33).unwrap(), literal);
    assert_eq!(wide.decode(&literal, Limits::new(33, 33)), Ok(value));

    let signed_le = BigNumber::<BigInt>::new(2, Endian::Little).unwrap();
    assert_eq!(
        signed_le
            .encode(&BigInt::from(BELOW_SIGNED_BYTE), 2)
            .unwrap(),
        [127, 255]
    );
    assert_eq!(
        signed_le.decode(&[127, 255], Limits::new(2, 2)),
        Ok(BigInt::from(BELOW_SIGNED_BYTE))
    );
}

pub fn arbitrary_width_rejections() {
    assert!(BigNumber::<BigInt>::new(0, Endian::Little).is_err());
    assert!(BigNumber::<BigUint>::new(usize::MAX, Endian::Little).is_err());

    let unsigned = BigNumber::<BigUint>::new(1, Endian::Big).unwrap();
    let signed = BigNumber::<BigInt>::new(1, Endian::Little).unwrap();
    assert_eq!(
        unsigned.encode(&BigUint::from(256u16), 1),
        Err(Error::NumberOutOfRange)
    );
    assert_eq!(
        signed.encode(&BigInt::from(ABOVE_SIGNED_BYTE), 1),
        Err(Error::NumberOutOfRange)
    );
    assert_eq!(
        signed.encode(&BigInt::from(BELOW_SIGNED_BYTE), 1),
        Err(Error::NumberOutOfRange)
    );
    assert_eq!(signed.encode(&BigInt::from(i8::MIN), 1).unwrap(), [128]);

    assert_eq!(
        unsigned.decode(&[], Limits::new(1, 1)),
        Err(Error::Truncated)
    );
    assert_eq!(
        unsigned.decode(&[1], Limits::new(1, 0)),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        unsigned.decode(&[1, 2], Limits::new(2, 1)),
        Err(Error::TrailingBytes)
    );

    let mut reader = Reader::new(&[1, 2], Limits::new(2, 1)).unwrap();
    assert_eq!(unsigned.read(&mut reader), Ok(BigUint::from(1u8)));
    assert_eq!(reader.claim(1), Err(Error::LimitExceeded));
}
