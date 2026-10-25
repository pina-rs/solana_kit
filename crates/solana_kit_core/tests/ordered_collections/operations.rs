use alloc::{rc::Rc, vec, vec::Vec};
use core::cell::Cell;
use solana_kit_core::{
    codecs::{
        Count, Decoder, Encoder, Error, Limits, Mapped, Number, OrderedMapCodec, OrderedSetCodec,
        Paired, Reader, Sequence, Size, Tuple, Writer,
    },
    collections::{OrderedMap, OrderedSet},
};

const LIMITS: Limits = Limits::new(64, 64);
const ENCODE: u8 = 1;
const DECODE: u8 = 2;
const PREFIX: u8 = 4;

/// An ordinary selected sequence with observable whole-operation overrides.
struct Declared<C> {
    inner: C,
    size: Result<Size, Error>,
    calls: Rc<Cell<u8>>,
}

impl<C: Encoder<[T]>, T> Encoder<[T]> for Declared<C> {
    fn encode_size(&self) -> Result<Size, Error> {
        self.size
    }

    fn encoded_len(&self, value: &[T]) -> Result<usize, Error> {
        self.inner.encoded_len(value)
    }

    fn write(&self, value: &[T], writer: &mut Writer<'_>) -> Result<(), Error> {
        self.inner.write(value, writer)
    }

    fn encode(&self, value: &[T], maximum: usize) -> Result<Vec<u8>, Error> {
        self.calls.set(self.calls.get() | ENCODE);
        self.inner.encode(value, maximum)
    }
}

impl<C: Decoder<Vec<T>>, T> Decoder<Vec<T>> for Declared<C> {
    fn decode_size(&self) -> Result<Size, Error> {
        self.size
    }

    fn read(&self, reader: &mut Reader<'_>) -> Result<Vec<T>, Error> {
        self.inner.read(reader)
    }

    fn decode(&self, bytes: &[u8], limits: Limits) -> Result<Vec<T>, Error> {
        self.calls.set(self.calls.get() | DECODE);
        self.inner.decode(bytes, limits)
    }

    fn decode_prefix(
        &self,
        bytes: &[u8],
        offset: usize,
        limits: Limits,
    ) -> Result<(Vec<T>, usize), Error> {
        self.calls.set(self.calls.get() | PREFIX);
        self.inner.decode_prefix(bytes, offset, limits)
    }
}

/// No Clone implementation exists for either encoded key or value.
#[derive(Debug, Eq, PartialEq)]
struct Owned(u8);

/// Selected declarations and whole-operation overrides survive the domain adapter.
pub fn borrowed_operations() {
    let item = Mapped::new(
        Number::<u8>::LE,
        |value: &Owned| Ok(value.0),
        |value| Ok(Owned(value)),
    );
    let map = OrderedMapCodec(Sequence {
        item: Tuple(item, item),
        count: Count::U32,
    });
    let value = OrderedMap::from_entries(vec![(Owned(2), Owned(20)), (Owned(1), Owned(10))]);
    let bytes = map.encode(&value, 64).unwrap();
    assert_eq!(bytes, [2, 0, 0, 0, 2, 20, 1, 10]);
    assert_eq!(
        map.decode(&bytes, LIMITS).unwrap().as_slice(),
        value.as_slice()
    );
    let set = OrderedSetCodec(Sequence {
        item,
        count: Count::U32,
    });
    let value = OrderedSet::from_values(vec![Owned(2), Owned(1)]);
    assert_eq!(set.encode(&value, 64).unwrap(), [2, 0, 0, 0, 2, 1]);

    map_declarations();
    set_declarations();
}

fn map_declarations() {
    let calls = Rc::new(Cell::new(0));
    let codec = OrderedMapCodec(Declared {
        inner: Sequence {
            item: Tuple(Number::<u8>::LE, Number::<u8>::LE),
            count: Count::Fixed(2),
        },
        size: Ok(Size::Fixed(4)),
        calls: Rc::clone(&calls),
    });
    let paired =
        Paired::<_, _, OrderedMap<u8, u8>, OrderedMap<u8, u8>>::try_new(&codec, &codec).unwrap();
    let value = OrderedMap::from_entries(vec![(2, 20), (1, 10)]);
    assert_eq!(paired.encode_size(), Ok(Size::Fixed(4)));
    assert_eq!(paired.decode_size(), Ok(Size::Fixed(4)));
    assert_eq!(paired.encode(&value, 64).unwrap(), [2, 20, 1, 10]);
    assert_eq!(paired.decode(&[2, 20, 1, 10], LIMITS).unwrap(), value);
    let (decoded, next) = paired
        .decode_prefix(&[77, 2, 20, 1, 10, 88], 1, LIMITS)
        .unwrap();
    assert_eq!(decoded, value);
    assert_eq!(next, 5);
    assert_eq!(calls.get(), ENCODE | DECODE | PREFIX);
}

fn set_declarations() {
    let calls = Rc::new(Cell::new(0));
    let codec = OrderedSetCodec(Declared {
        inner: Sequence {
            item: Number::<u8>::LE,
            count: Count::Fixed(2),
        },
        size: Ok(Size::Fixed(2)),
        calls: Rc::clone(&calls),
    });
    let paired = Paired::<_, _, OrderedSet<u8>, OrderedSet<u8>>::try_new(&codec, &codec).unwrap();
    let value = OrderedSet::from_values(vec![2, 1]);
    assert_eq!(paired.encode_size(), Ok(Size::Fixed(2)));
    assert_eq!(paired.decode_size(), Ok(Size::Fixed(2)));
    assert_eq!(paired.encode(&value, 64).unwrap(), [2, 1]);
    assert_eq!(paired.decode(&[2, 1], LIMITS).unwrap(), value);
    let (decoded, next) = paired.decode_prefix(&[77, 2, 1, 88], 1, LIMITS).unwrap();
    assert_eq!(decoded, value);
    assert_eq!(next, 3);
    assert_eq!(calls.get(), ENCODE | DECODE | PREFIX);

    let failed = OrderedSetCodec(Declared {
        inner: Sequence {
            item: Number::<u8>::LE,
            count: Count::U32,
        },
        size: Err(Error::Overflow),
        calls,
    });
    assert_eq!(
        Encoder::<OrderedSet<u8>>::encode_size(&failed),
        Err(Error::Overflow)
    );
    assert_eq!(
        Decoder::<OrderedSet<u8>>::decode_size(&failed),
        Err(Error::Overflow)
    );
}
