//! Transactional reads commit cursor and allocation claims together.
use solana_kit_core::codecs::{Error, Limits, Reader};

#[test]
fn failed_transaction_restores_cursor_and_nested_frame_claims() {
    let mut reader = Reader::new(&[1, 2, 3], Limits::new(3, 2)).unwrap();
    reader.take(1).unwrap();
    let result: Result<(), Error> = reader.transaction(|snapshot| {
        snapshot.frame(2, |inner| {
            inner.claim(2)?;
            inner.take(1)?;
            Err(Error::InvalidValue)
        })
    });
    assert_eq!(result, Err(Error::InvalidValue));
    assert_eq!(reader.position(), 1);
    assert_eq!(reader.remaining(), &[2, 3]);
    reader.claim(2).unwrap();
    assert_eq!(reader.claim(1), Err(Error::LimitExceeded));
}

#[test]
fn successful_transaction_commits_position_and_claims() {
    let mut reader = Reader::new(&[1, 2], Limits::new(2, 2)).unwrap();
    let value = reader
        .transaction(|snapshot| {
            snapshot.claim(1)?;
            snapshot.take(1)
        })
        .unwrap();
    assert_eq!(value, &[1]);
    assert_eq!(reader.position(), 1);
    reader.claim(1).unwrap();
    assert_eq!(reader.claim(1), Err(Error::LimitExceeded));
}

#[test]
fn ordinary_failed_frame_still_consumes_input_and_claims() {
    let mut reader = Reader::new(&[1, 2], Limits::new(2, 2)).unwrap();
    let result: Result<(), Error> = reader.frame(2, |inner| {
        inner.claim(2)?;
        Err(Error::InvalidValue)
    });
    assert_eq!(result, Err(Error::InvalidValue));
    assert_eq!(reader.position(), 2);
    assert_eq!(reader.claim(1), Err(Error::LimitExceeded));
}
