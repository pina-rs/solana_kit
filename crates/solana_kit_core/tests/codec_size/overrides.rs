use alloc::{rc::Rc, vec::Vec};
use core::cell::Cell;
use solana_kit_core::codecs::{
    Decoder, Encoder, Error, Limits, Number, Paired, Reader, Size, Writer,
};

const LIMITS: Limits = Limits::new(8, 8);

#[derive(Default)]
struct Calls {
    encode: Cell<usize>,
    write: Cell<usize>,
    decode: Cell<usize>,
    prefix: Cell<usize>,
    read: Cell<usize>,
}

/// A declared byte codec with independent owned-operation policies.
struct Overrides(Rc<Calls>);

impl Encoder<u8> for Overrides {
    fn encode_size(&self) -> Result<Size, Error> {
        Ok(Size::Fixed(1))
    }

    fn encoded_len(&self, value: &u8) -> Result<usize, Error> {
        Number::<u8>::LE.encoded_len(value)
    }

    fn write(&self, value: &u8, writer: &mut Writer<'_>) -> Result<(), Error> {
        self.0.write.set(self.0.write.get() + 1);
        let start = writer.position();
        Number::<u8>::LE.write(value, writer)?;
        writer.seek(start)
    }

    fn encode(&self, value: &u8, max_bytes: usize) -> Result<Vec<u8>, Error> {
        self.0.encode.set(self.0.encode.get() + 1);
        Number::<u8>::LE.encode(value, max_bytes)
    }
}

impl Decoder<u8> for Overrides {
    fn decode_size(&self) -> Result<Size, Error> {
        Ok(Size::Fixed(1))
    }

    fn read(&self, reader: &mut Reader<'_>) -> Result<u8, Error> {
        self.0.read.set(self.0.read.get() + 1);
        Number::<u8>::LE.read(reader)
    }

    fn decode(&self, bytes: &[u8], limits: Limits) -> Result<u8, Error> {
        self.0.decode.set(self.0.decode.get() + 1);
        Number::<u8>::LE
            .decode_prefix(bytes, 0, limits)
            .map(|(value, _)| value)
    }

    fn decode_prefix(
        &self,
        bytes: &[u8],
        offset: usize,
        limits: Limits,
    ) -> Result<(u8, usize), Error> {
        self.0.prefix.set(self.0.prefix.get() + 1);
        let (value, _) = Number::<u8>::LE.decode_prefix(bytes, offset, limits)?;
        Ok((value, offset))
    }
}

/// Pairing preserves overrides, raw cursor behavior and selected failure policies.
pub fn complete_operations() {
    let calls = Rc::new(Calls::default());
    let paired =
        Paired::try_new(Overrides(Rc::clone(&calls)), Overrides(Rc::clone(&calls))).unwrap();
    assert_eq!(paired.encoded_len(&7), Ok(1));
    assert_eq!(paired.encode(&7, 1), Ok(alloc::vec![7]));
    assert_eq!(calls.encode.get(), 1);
    assert_eq!(calls.write.get(), 0);
    assert_eq!(paired.encode(&7, 0), Err(Error::LimitExceeded));
    assert_eq!(paired.decode(&[7, 88], LIMITS), Ok(7));
    assert_eq!(paired.decode(&[], LIMITS), Err(Error::Truncated));
    assert_eq!(calls.decode.get(), 2);
    assert_eq!(calls.read.get(), 0);
    assert_eq!(paired.decode_prefix(&[99, 7, 88], 1, LIMITS), Ok((7, 1)));
    assert_eq!(
        paired.decode_prefix(&[99], 1, LIMITS),
        Err(Error::Truncated)
    );
    assert_eq!(calls.prefix.get(), 2);
    assert_eq!(calls.read.get(), 0);
    let mut bytes = [99, 0, 88];
    let mut writer = Writer::new(&mut bytes);
    writer.seek(1).unwrap();
    paired.write(&7, &mut writer).unwrap();
    assert_eq!(writer.position(), 1);
    assert_eq!(calls.write.get(), 1);
    assert_eq!(bytes, [99, 7, 88]);
    let mut reader = Reader::new(&bytes, LIMITS).unwrap();
    reader.take(1).unwrap();
    assert_eq!(paired.read(&mut reader), Ok(7));
    assert_eq!(reader.position(), 2);
    assert_eq!(calls.read.get(), 1);

    borrowed_complete_operations();
}

/// Borrowing a codec preserves its complete-operation policies and callbacks.
pub fn borrowed_complete_operations() {
    let calls = Rc::new(Calls::default());
    let codec = Overrides(Rc::clone(&calls));
    let paired = Paired::try_new(&codec, &codec).unwrap();

    assert_eq!(paired.encode(&7, 1), Ok(alloc::vec![7]));
    assert_eq!(paired.encode(&7, 0), Err(Error::LimitExceeded));
    assert_eq!(calls.encode.get(), 2);
    assert_eq!(calls.write.get(), 0);

    assert_eq!(paired.decode(&[7, 88], LIMITS), Ok(7));
    assert_eq!(paired.decode(&[], LIMITS), Err(Error::Truncated));
    assert_eq!(calls.decode.get(), 2);
    assert_eq!(calls.read.get(), 0);

    assert_eq!(paired.decode_prefix(&[99, 7, 88], 1, LIMITS), Ok((7, 1)));
    assert_eq!(
        paired.decode_prefix(&[99], 1, LIMITS),
        Err(Error::Truncated)
    );
    assert_eq!(calls.prefix.get(), 2);
    assert_eq!(calls.read.get(), 0);
}
