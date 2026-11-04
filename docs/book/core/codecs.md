# Codecs

The `solana_kit_core::codecs` module turns Rust values into Solana wire bytes and back. Instruction data, account layouts, sysvars and transactions all use it. Codecs are small structs that implement two traits, `Encoder<T>` and `Decoder<T>`. You compose them by nesting. Every decode takes an explicit budget, so untrusted input cannot make you allocate without limit.

## The two traits

An `Encoder<T>` reports the exact encoded length and writes into a bounded buffer:

- `encoded_len(&value)` returns the exact byte size.
- `write(&value, &mut Writer)` writes at a cursor.
- `encode(&value, max_bytes)` allocates and returns a `Vec<u8>`. It fails with `Error::LimitExceeded` if the result would exceed `max_bytes`.

A `Decoder<T>` reads from a checked cursor:

- `decode(bytes, limits)` decodes exactly one value and rejects trailing bytes.
- `decode_prefix(bytes, offset, limits)` starts at an absolute offset. It returns the value and the next offset.
- `read(&mut Reader)` is the low-level cursor operation that composite codecs call.

```rust
use solana_kit_core::codecs::{Decoder, Encoder, Error, Limits, Number};

fn example() -> Result<(), Error> {
    let limits = Limits::new(64, 0);
    let bytes = Number::<u16>::LE.encode(&0xabcd, 2)?;
    assert_eq!(bytes, [0xcd, 0xab]);
    assert_eq!(Number::<u16>::BE.encode(&0xabcd, 2)?, [0xab, 0xcd]);
    assert_eq!(Number::<u16>::LE.decode(&bytes, limits)?, 0xabcd);

    // Full decoding rejects unread input; prefix decoding reports the next offset.
    assert_eq!(Number::<u8>::LE.decode(&[1, 2], limits), Err(Error::TrailingBytes));
    assert_eq!(
        Number::<u32>::LE.decode_prefix(&[99, 42, 0, 0, 0, 88], 1, limits)?,
        (42, 5)
    );
    Ok(())
}
```

## Budgets

`Limits::new(max_bytes, max_items)` sets two budgets for one whole decode, nested codecs included:

- `max_bytes` caps the input size and is checked before decoding starts.
- `max_items` caps the total output elements allocated by every nested decoder. Each sequence entry counts, and so does each allocated byte of `Bytes` or text.

A decoder checks the length a prefix claims against the remaining budget before it allocates. Here two one-byte vectors need four items: two entries plus two bytes.

```rust
use solana_kit_core::codecs::{Bytes, Count, Decoder, Error, Limits, Sequence, SizePrefix};

fn example() -> Result<(), Error> {
    let nested = Sequence { item: SizePrefix(Bytes), count: Count::U32 };
    let bytes = [2, 0, 0, 0, 1, 0, 0, 0, 42, 1, 0, 0, 0, 43];
    assert_eq!(nested.decode(&bytes, Limits::new(20, 3)), Err(Error::LimitExceeded));
    assert_eq!(nested.decode(&bytes, Limits::new(20, 4))?, [vec![42], vec![43]]);
    Ok(())
}
```

`max_items` is not a heap-byte quota for arbitrary types. If you write your own allocating decoder, call `Reader::claim` for what you allocate.

## Numbers

| Codec                                   | Wire format                                                                                                      |
| --------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| `Number::<T>::LE`, `Number::<T>::BE`    | Fixed-width integers `u8` through `u128` and `i8` through `i128`, plus `f32` and `f64`, in the chosen byte order |
| `Number::<Uint256>`, `Number::<Int256>` | 256-bit integers stored as exact wire bytes, with no arithmetic                                                  |
| `ShortU16`                              | Solana's compact u16. Redundant bytes are rejected as `Error::NonCanonical`                                      |
| `BigNumber`                             | Arbitrary-width integers (`bigint-codecs` feature)                                                               |

The Rust input type enforces numeric range. Floats keep NaN payloads, infinities and signed zero bit for bit. Use `ensure_number_range` to check an explicit interval.

## Bytes, booleans and strings

| Codec                                                                                  | Behavior                                                                                     |
| -------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- |
| `Bool`                                                                                 | One byte. Any value other than 0 or 1 fails with `Error::InvalidBoolean { value }`           |
| `FixedBytes::<N>`                                                                      | Exactly `N` bytes as `[u8; N]`                                                               |
| `Bytes`, `Utf8`                                                                        | All remaining bytes. Use them inside a frame or as the last field                            |
| `Utf8Config`                                                                           | UTF-8 with explicit strict/lossy, BOM and null policies                                      |
| `Base16`, `BaseX::base10()`, `BaseX::base58()`, `BaseX::new(alphabet)`, `BaseXReslice` | Text alphabets. The encoder takes text and produces bytes; the decoder renders bytes as text |
| `Base64`                                                                               | Base64 text (`text-codecs` feature)                                                          |
| `Constant(&[u8])`                                                                      | A fixed byte pattern, checked on decode                                                      |

`Utf8` is strict, and it keeps a leading BOM and any null characters. `remove_null_characters` and `pad_null_characters` help with fixed-width text fields.

```rust
use solana_kit_core::codecs::{Base16, BaseX, Decoder, Encoder, Error, Limits};

fn example() -> Result<(), Error> {
    let limits = Limits::new(64, 64);
    assert_eq!(Base16.encode("0aff", 2)?, [0x0a, 0xff]);
    assert_eq!(Base16.decode(&[0x0a, 0xff], limits)?, "0aff");
    assert_eq!(BaseX::base58().encode("11", 8)?, [0, 0]);
    Ok(())
}
```

## Framing

A frame tells a codec where its payload ends.

| Wrapper                                                                      | Frame                                                                                                                                                               |
| ---------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `SizePrefix(item)`, `SizePrefix64(item)`                                     | u32 or u64 little-endian byte length. The payload must be consumed exactly                                                                                          |
| `ByteSizePrefix { item, prefix, consumption }`                               | Byte length written by any u64-valued `prefix` codec. `FrameConsumption::Exact` requires every byte to be read. `FrameConsumption::Skip` advances past unread bytes |
| `FixedSize { item, size, decoded_size, allow_truncation, max_encoded_size }` | Fixed-width field with zero padding and optional truncation                                                                                                         |
| `Padded { left, item, right }`                                               | Zero padding around a value                                                                                                                                         |
| `Sentinel::new(item, sentinel, max)`                                         | Payload ending at a delimiter                                                                                                                                       |
| `Affix { prefix, item, suffix }`                                             | Constant bytes on either side                                                                                                                                       |
| `Reverse`, `Resized`, `Offset`                                               | Byte reversal, a changed allocation extent, and checked repositioning                                                                                               |

`SizePrefix(Utf8)` is the Borsh string layout:

```rust
use solana_kit_core::codecs::{Decoder, Encoder, Error, Limits, SizePrefix, Utf8};

fn example() -> Result<(), Error> {
    let text = SizePrefix(Utf8);
    let bytes = text.encode("hé", 7)?;
    assert_eq!(bytes, [3, 0, 0, 0, b'h', 0xc3, 0xa9]);
    assert_eq!(text.decode(&bytes, Limits::new(1024, 128))?, "hé");
    Ok(())
}
```

## Composite codecs

| Codec                                                          | Shape                                                                                                                                                               |
| -------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Sequence { item, count }`                                     | Homogeneous `Vec<T>`. `Count::U32` writes a Borsh-style u32 prefix, `Count::Fixed(n)` has no prefix, and `Count::Remainder` reads to the end of the enclosing frame |
| `Counted { prefix, item }`                                     | `Vec<T>` with any count codec as the prefix, such as a mapped `u8`                                                                                                  |
| `Tuple(a, b)`                                                  | A pair. Nest tuples for more fields                                                                                                                                 |
| `Mapped::new(item, into_wire, from_wire)`                      | Fallible conversion between your type and a wire codec                                                                                                              |
| `OptionCodec::u8`, `::u32`, `::c_option`, `::remainder`        | Tagged, C-layout and trailing options                                                                                                                               |
| `Choice(a, b)` with `Either`                                   | Two variants with a one-byte tag, 0 or 1                                                                                                                            |
| `Literal { variants, index }`                                  | A value from a fixed list, stored by index                                                                                                                          |
| `Dependent(head, f)`                                           | A second codec chosen from the first decoded value                                                                                                                  |
| `BitArray`, `Unit`, `Nullable`, `UnionEncoder`/`UnionDecoder`  | Bit flags, the zero-byte unit and other specialized layouts                                                                                                         |
| `OrderedMapCodec`, `OrderedSetCodec`                           | Insertion-ordered maps and sets                                                                                                                                     |
| `RecordEncoder`/`RecordDecoder`, `TupleEncoder`/`TupleDecoder` | Runtime schemas over borrowed codec objects                                                                                                                         |

To encode a struct, map it to and from nested tuples:

```rust
use solana_kit_core::codecs::{Decoder, Encoder, Error, Limits, Mapped, Number, Tuple};

#[derive(Debug, PartialEq)]
struct Transfer {
    discriminator: u32,
    lamports: u64,
}

fn example() -> Result<(), Error> {
    let layout = Mapped::new(
        Tuple(Number::<u32>::LE, Number::<u64>::LE),
        |value: &Transfer| Ok((value.discriminator, value.lamports)),
        |(discriminator, lamports)| Ok(Transfer { discriminator, lamports }),
    );
    let value = Transfer { discriminator: 2, lamports: 1_000_000 };
    let bytes = layout.encode(&value, 12)?;
    assert_eq!(layout.decode(&bytes, Limits::new(12, 0))?, value);
    Ok(())
}
```

Options and choices validate their tags strictly:

```rust
use solana_kit_core::codecs::{
    Choice, Decoder, Either, Encoder, Error, Limits, Number, OptionCodec, Unit,
};

fn example() -> Result<(), Error> {
    let limits = Limits::new(64, 0);
    let item = Number::<u16>::LE;
    assert_eq!(OptionCodec::u8(item).encode(&Some(42), 3)?, [1, 42, 0]);
    assert_eq!(OptionCodec::c_option(item, 2).encode(&None, 6)?, [0; 6]);
    assert_eq!(OptionCodec::u8(item).decode(&[2, 0, 0], limits), Err(Error::InvalidDiscriminator));

    let choice = Choice(Unit, Number::<u16>::LE);
    assert_eq!(choice.decode(&[1, 2, 3], limits)?, Either::Second(770));
    Ok(())
}
```

Generated program clients use the same building blocks for their instruction and account layouts. See [Program clients](../programs/index.md).

## Errors

All codecs return one `Copy` error type, `codecs::Error`:

| Variant                                                               | Meaning                                                           |
| --------------------------------------------------------------------- | ----------------------------------------------------------------- |
| `Truncated`                                                           | Input or output ended early                                       |
| `Overflow`                                                            | A length or offset calculation overflowed                         |
| `LimitExceeded`                                                       | A byte or element budget was exceeded, detected before allocating |
| `TrailingBytes`                                                       | Full decode or an exact frame left bytes unread                   |
| `NonCanonical`                                                        | A compact integer used redundant bytes                            |
| `InvalidDiscriminator`                                                | An option or choice tag, or a sentinel, did not match             |
| `InvalidBoolean { value }`, `InvalidConstant { offset, length }`      | A bad boolean byte, or a constant mismatch at an offset           |
| `NumberOutOfRange`, `InvalidUtf8`, `InvalidValue`, `AllocationFailed` | Range, text, contract and allocation failures                     |

A failed operation can leave its cursor advanced or its output partly written. Do not reuse a cursor after an error unless you use `Reader::transaction`, which rolls back position and claims on failure. Codec errors also implement `errors::ErrorFacts`; see [Errors](errors.md).

## Status

> [!NOTE]
> The codec layer is a verified slice. It does not yet cover the whole reference package. The [core codec receipt](https://github.com/pina-rs/solana_kit/tree/main/docs/verification/core-codecs.md) and the [numeric and string receipt](https://github.com/pina-rs/solana_kit/tree/main/docs/parity/numeric-string-codecs.md) list the supported configurations and the partial factories.
