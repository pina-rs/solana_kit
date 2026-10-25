//! Public selection and byte-context contracts shared with portable conformance.

use alloc::vec::Vec;
use core::cell::Cell;
use solana_kit_core::codecs::{
    Bytes, Contextual, Decoder, Encoder, Error, Limits, Mapped, Number, Reader, Size, UnionDecoder,
    UnionEncoder,
};

pub fn untagged_selection() {
    let narrow = Mapped::new(
        Number::<u8>::LE,
        |value: &u32| u8::try_from(*value).map_err(|_| Error::NumberOutOfRange),
        |value: u8| Ok(u32::from(value)),
    );
    let wide = Mapped::new(
        Number::<u16>::LE,
        |value: &u32| u16::try_from(*value).map_err(|_| Error::NumberOutOfRange),
        |value: u16| Ok(u32::from(value)),
    );
    let encoder = UnionEncoder {
        variants: &[&narrow, &wide],
        select: |value: &u32| Ok(usize::from(*value > u32::from(u8::MAX))),
    };
    let decoder = UnionDecoder {
        variants: &[&narrow, &wide],
        select: |bytes: &[u8], offset: usize| {
            let previous = offset.checked_sub(1).ok_or(Error::InvalidValue)?;
            Ok(usize::from(*bytes.get(previous).ok_or(Error::Truncated)?))
        },
    };

    assert_eq!(encoder.encode(&7, 2).unwrap(), [7]);
    assert_eq!(encoder.encode(&0x1234, 2).unwrap(), [0x34, 0x12]);
    assert_eq!(encoder.encode_size(), Ok(Size::Variable { max: Some(2) }));
    assert_eq!(decoder.decode_size(), encoder.encode_size());
    assert_eq!(
        decoder.decode_prefix(&[1, 0x34, 0x12, 99], 1, Limits::new(4, 0)),
        Ok((0x1234, 3))
    );
    assert_eq!(
        decoder.decode_prefix(&[0, 7], 1, Limits::new(2, 0)),
        Ok((7, 2))
    );
    assert_eq!(
        decoder.decode_prefix(&[2, 7], 1, Limits::new(2, 0)),
        Err(Error::InvalidDiscriminator)
    );
    assert_eq!(
        decoder.decode_prefix(&[1, 7], 1, Limits::new(2, 0)),
        Err(Error::Truncated)
    );
}

pub fn contextual_decoding() {
    let calls = Cell::new(0);
    let mapped = Contextual::new(Number::<u16>::LE, |value, bytes: &[u8], offset| {
        calls.set(calls.get() + 1);
        Ok((value, bytes[0], offset))
    });

    assert_eq!(mapped.decode_size(), Ok(Size::Fixed(2)));
    assert_eq!(
        mapped.decode_prefix(&[99, 0x34, 0x12, 77], 1, Limits::new(4, 0)),
        Ok(((0x1234, 99, 1), 3))
    );
    assert_eq!(calls.get(), 1);
    assert_eq!(
        mapped.decode(&[0], Limits::new(1, 0)),
        Err(Error::Truncated)
    );
    assert_eq!(calls.get(), 1);

    let rejected = Contextual::new(
        Bytes,
        |_: Vec<u8>, _: &[u8], _: usize| -> Result<(), Error> { Err(Error::InvalidValue) },
    );
    let mut reader = Reader::new(&[4, 5], Limits::new(2, 2)).unwrap();
    assert_eq!(rejected.read(&mut reader), Err(Error::InvalidValue));
    assert_eq!(reader.position(), 2);
    assert_eq!(reader.claim(1), Err(Error::LimitExceeded));
}
