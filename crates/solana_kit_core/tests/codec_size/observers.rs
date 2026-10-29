use alloc::vec;
use core::{cell::Cell, fmt::Debug};
use solana_kit_core::codecs::{
    Checked, Decoder, Encoder, Error, Limits, Number, Paired, Reader, ShortU16, Size, Writer,
};

const LIMITS: Limits = Limits::new(64, 64);
const PREFIX: u8 = 88;
const SUFFIX: u8 = 99;

/// Both fixed and bounded-variable observers remain pairable without side effects.
pub fn metadata_and_callbacks() {
    checked_operations(Number::<u8>::LE, Size::Fixed(1), 7u8, 9u8, &[7]);
    checked_operations(
        ShortU16,
        Size::Variable { max: Some(3) },
        128u16,
        9u16,
        &[128, 1],
    );
}

/// Metadata and sizing are silent; callbacks keep their original byte-operation timing.
fn checked_operations<T, C>(codec: C, size: Size, value: T, rejected: T, wire: &[u8])
where
    T: Copy + Debug + PartialEq,
    C: Encoder<T> + Decoder<T>,
{
    let rejected_wire = codec.encode(&rejected, LIMITS.max_bytes).unwrap();
    let writes = Cell::new(0);
    let reads = Cell::new(0);
    let checked = Checked {
        item: codec,
        before_write: |value: &T| {
            writes.set(writes.get() + 1);
            reject(value, &rejected)
        },
        after_read: |value: &T| {
            reads.set(reads.get() + 1);
            reject(value, &rejected)
        },
    };

    assert_eq!(checked.encode_size(), Ok(size));
    assert_eq!(checked.decode_size(), Ok(size));
    assert_eq!(checked.encoded_len(&value), Ok(wire.len()));
    assert_eq!(checked.encoded_len(&rejected), Ok(rejected_wire.len()));

    let paired = Paired::try_new(&checked, &checked).unwrap();
    assert_eq!(paired.encode_size(), Ok(size));
    assert_eq!(paired.decode_size(), Ok(size));
    assert_eq!(paired.encoded_len(&value), Ok(wire.len()));
    assert_eq!((writes.get(), reads.get()), (0, 0));

    assert_eq!(paired.encode(&value, wire.len()).unwrap(), wire);
    assert_eq!(paired.decode(wire, LIMITS), Ok(value));
    assert_eq!((writes.get(), reads.get()), (1, 1));

    rejected_operations(&paired, rejected, &rejected_wire);
    assert_eq!((writes.get(), reads.get()), (2, 2));

    truncated_operations(&paired, value);
    assert_eq!((writes.get(), reads.get()), (3, 2));
}

/// Rejection precedes writes and follows successful reads at their original offsets.
fn rejected_operations<T, C>(codec: &C, rejected: T, wire: &[u8])
where
    T: Debug + PartialEq,
    C: Encoder<T> + Decoder<T>,
{
    let mut output = vec![SUFFIX; 4];
    let mut writer = Writer::new(&mut output);
    writer.put(&[PREFIX]).unwrap();
    assert_eq!(
        codec.write(&rejected, &mut writer),
        Err(Error::InvalidValue)
    );
    assert_eq!(writer.position(), 1);
    assert_eq!(output, [PREFIX, SUFFIX, SUFFIX, SUFFIX]);

    let mut input = vec![PREFIX];
    input.extend_from_slice(wire);
    input.push(SUFFIX);
    let mut reader = Reader::new(&input, LIMITS).unwrap();
    reader.take(1).unwrap();
    assert_eq!(codec.read(&mut reader), Err(Error::InvalidValue));
    assert_eq!(reader.position(), 1 + wire.len());
}

/// Failed inner operations still run the pre-write callback and skip the post-read callback.
fn truncated_operations<T, C>(codec: &C, value: T)
where
    T: Debug + PartialEq,
    C: Encoder<T> + Decoder<T>,
{
    let mut reader = Reader::new(&[], LIMITS).unwrap();
    assert_eq!(codec.read(&mut reader), Err(Error::Truncated));

    let mut output = [];
    let mut writer = Writer::new(&mut output);
    assert_eq!(codec.write(&value, &mut writer), Err(Error::Truncated));
    assert_eq!(writer.position(), 0);
}

/// A rejected value aborts the callback's surrounding operation.
fn reject<T: PartialEq>(value: &T, rejected: &T) -> Result<(), Error> {
    if value == rejected {
        return Err(Error::InvalidValue);
    }

    Ok(())
}
