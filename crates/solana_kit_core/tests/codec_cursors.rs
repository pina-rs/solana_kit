//! Observable cursor state and custom codec contract failures.

use solana_kit_core::codecs::{Bytes, Decoder, Encoder, Error, Limits, Reader, Utf8, Writer};

const LIMITS: Limits = Limits::new(8, 2);

/// A deliberately inconsistent caller implementation for contract checks.
struct Underwriter;

impl Encoder<()> for Underwriter {
    fn encoded_len(&self, _: &()) -> Result<usize, Error> {
        Ok(1)
    }

    fn write(&self, _: &(), _: &mut Writer<'_>) -> Result<(), Error> {
        Ok(())
    }
}

#[test]
fn incomplete_custom_writes_are_rejected() {
    assert_eq!(Underwriter.encode(&(), 1), Err(Error::InvalidValue));
    let mut bytes = [0; 2];
    let mut writer = Writer::new(&mut bytes);
    assert_eq!(writer.frame(2, |_| Ok(())), Err(Error::InvalidValue));
    assert_eq!(writer.position(), 0);

    assert_eq!(writer.frame(3, |_| Ok(())), Err(Error::Truncated));
    writer.frame(2, |inner| inner.put(&[1, 2])).unwrap();
    assert_eq!(writer.position(), 2);
    assert_eq!(writer.fill(1, 0), Err(Error::Truncated));
    assert_eq!(bytes, [1, 2]);
}

#[test]
fn failed_frames_keep_the_shared_allocation_budget() {
    let mut reader = Reader::new(&[1, 2, 3], LIMITS).unwrap();
    let result = reader.frame(2, |inner| {
        inner.claim(2)?;
        inner.take(1)?;
        Ok(())
    });

    assert_eq!(result, Err(Error::TrailingBytes));
    assert_eq!(reader.position(), 2);
    assert_eq!(reader.remaining(), [3]);
    assert_eq!(reader.claim(1), Err(Error::LimitExceeded));
    reader.take(1).unwrap();
    assert_eq!(reader.finish(), Ok(()));
}

#[test]
fn strict_text_preserves_bom_and_nulls() {
    let text = "\u{feff}\0";
    let bytes = [0xef, 0xbb, 0xbf, 0];
    assert_eq!(Utf8.encode(text, 4).unwrap(), bytes);
    assert_eq!(Utf8.decode(&bytes, Limits::new(4, 4)).unwrap(), text);
}

#[test]
fn raw_bytes_use_exact_prefix_offsets() {
    let bytes = [1, 2, 3];
    assert_eq!(Bytes.encode(&bytes, 3).unwrap(), bytes);
    assert_eq!(Bytes.decode_prefix(&bytes, 1, LIMITS), Ok((vec![2, 3], 3)));
    assert_eq!(Bytes.decode(&[], LIMITS), Ok(vec![]));
}
