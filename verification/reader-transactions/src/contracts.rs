//! Transactional reads commit cursor and allocation claims together.
use solana_kit_core::codecs::{Error, Limits, Reader};

pub(super) fn run() -> usize {
    let cases = [
        failed_transaction_restores_cursor_and_nested_frame_claims,
        successful_transaction_commits_position_and_claims,
        ordinary_failed_frame_still_consumes_input_and_claims,
        successful_and_failed_nested_transactions_preserve_the_contract,
        ordinary_failed_frame_still_consumes_bytes_and_claims,
    ];

    for case in cases {
        case();
    }

    cases.len()
}

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
fn successful_and_failed_nested_transactions_preserve_the_contract() {
    let mut reader = Reader::new(&[10, 20, 30, 40], Limits::new(4, 5)).unwrap();
    let mut external = 0;
    let failed: Result<(), Error> = reader.transaction(|attempt| {
        external += 1;
        attempt.take(1)?;
        attempt.claim(2)?;
        attempt.transaction(|nested| {
            nested.take(1)?;
            nested.claim(1)
        })?;
        Err(Error::InvalidValue)
    });
    assert_eq!(failed, Err(Error::InvalidValue));
    assert_eq!(external, 1);
    assert_eq!(reader.position(), 0);

    reader
        .transaction(|attempt| {
            assert_eq!(attempt.take(2)?, &[10, 20]);
            attempt.claim(3)
        })
        .unwrap();
    assert_eq!(reader.position(), 2);
    assert_eq!(reader.claim(3), Err(Error::LimitExceeded));
    reader.claim(2).unwrap();
}
fn ordinary_failed_frame_still_consumes_bytes_and_claims() {
    let mut reader = Reader::new(&[10, 20, 30], Limits::new(3, 3)).unwrap();
    let failed: Result<(), Error> = reader.frame(2, |frame| {
        frame.claim(2)?;
        Err(Error::InvalidValue)
    });
    assert_eq!(failed, Err(Error::InvalidValue));
    assert_eq!(reader.position(), 2);
    assert_eq!(reader.take(2), Err(Error::Truncated));
    assert_eq!(reader.position(), 2);
    assert_eq!(reader.claim(2), Err(Error::LimitExceeded));
    reader.claim(1).unwrap();
}
