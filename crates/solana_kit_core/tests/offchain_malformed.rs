const MESSAGE_MAX: usize = 10000;
const ITEM_MAX: usize = 30000;
const ENVELOPE_MAX: usize = 2000;
const ENVELOPE_ITEMS: usize = 5000;
const VERSION_OFFSET: usize = 16;
const FORMAT_OFFSET: usize = 49;
const COUNT_OFFSET: usize = 50;
const LENGTH_OFFSET: usize = 83;

use solana_kit_core::{
    Address, Signature,
    codecs::{Error as CodecError, Limits},
    offchain::{ContentFormat, Envelope, Error, Message},
};

fn message() -> Message {
    Message::v0(
        Address::from_bytes([1; 32]),
        ContentFormat::Utf8,
        "hello",
        &[Address::from_bytes([2; 32])],
        1000,
    )
    .unwrap()
}

fn decode(bytes: &[u8]) -> Result<Message, Error> {
    Message::decode(bytes, Limits::new(MESSAGE_MAX, ITEM_MAX))
}

#[test]
fn strict_v0_decoder_rejects_every_truncation_and_trailing_data() {
    let original = message();

    for length in 0..original.bytes().len() {
        assert!(
            decode(&original.bytes()[..length]).is_err(),
            "length {length}"
        );
    }

    let mut trailing = original.bytes().to_vec();
    trailing.push(0);
    assert_eq!(
        decode(&trailing),
        Err(Error::Codec(CodecError::TrailingBytes))
    );

    for (offset, value) in [
        (0, 0),
        (VERSION_OFFSET, 2),
        (FORMAT_OFFSET, 3),
        (COUNT_OFFSET, 0),
        (LENGTH_OFFSET, 255),
        (LENGTH_OFFSET + 1, 255),
    ] {
        let mut invalid = original.bytes().to_vec();
        invalid[offset] = value;
        assert!(decode(&invalid).is_err(), "offset {offset}");
    }

    let mut invalid = original.bytes().to_vec();
    *invalid.last_mut().unwrap() = 255;
    assert_eq!(decode(&invalid), Err(Error::Codec(CodecError::InvalidUtf8)));
}

#[test]
fn v1_requires_nonempty_unique_sorted_signatories_and_nonempty_utf8() {
    let a = Address::from_bytes([1; 32]);
    let b = Address::from_bytes([2; 32]);
    assert_eq!(Message::v1("hi", &[], 1000), Err(Error::InvalidSignatories));
    assert_eq!(
        Message::v1("hi", &[a, a], 1000),
        Err(Error::InvalidSignatories)
    );
    assert_eq!(Message::v1("", &[a], 1000), Err(Error::InvalidContent));
    assert_eq!(
        Message::v1("hi", &[a; 256], 100000),
        Err(Error::InvalidSignatories)
    );
    let message = Message::v1("hi", &[a, b], 1000).unwrap();
    let mut duplicate = message.bytes().to_vec();

    duplicate[50..82].copy_from_slice(a.as_bytes());
    assert_eq!(decode(&duplicate), Err(Error::InvalidSignatories));
    let mut unsorted = message.bytes().to_vec();
    unsorted[18..50].copy_from_slice(b.as_bytes());
    unsorted[50..82].copy_from_slice(a.as_bytes());
    assert_eq!(decode(&unsorted), Err(Error::InvalidSignatories));
    assert_eq!(decode(&message.bytes()[..82]), Err(Error::InvalidContent));
    let mut invalid = message.bytes().to_vec();
    *invalid.last_mut().unwrap() = 255;
    assert_eq!(decode(&invalid), Err(Error::Codec(CodecError::InvalidUtf8)));
}

#[test]
fn envelopes_validate_the_message_counts_and_slot_identity() {
    let unsigned = Envelope::unsigned(message()).unwrap();
    let wire = unsigned.encode(false, 1000).unwrap();

    for length in 0..wire.len() {
        assert!(
            Envelope::decode(&wire[..length], Limits::new(ENVELOPE_MAX, ENVELOPE_ITEMS)).is_err()
        );
    }

    let mut malformed = wire;
    malformed[65] = 0;
    assert!(Envelope::decode(&malformed, Limits::new(ENVELOPE_MAX, ENVELOPE_ITEMS)).is_err());
    let mut missing = vec![0];
    missing.extend_from_slice(unsigned.message().bytes());
    assert_eq!(
        Envelope::decode(&missing, Limits::new(ENVELOPE_MAX, ENVELOPE_ITEMS)),
        Err(Error::SignerMismatch)
    );
    assert_eq!(
        unsigned.with_signature(Address::from_bytes([3; 32]), Signature::from_bytes([1; 64])),
        Err(Error::SignerMismatch)
    );
    let duplicate = Message::v0(
        Address::from_bytes([1; 32]),
        ContentFormat::Utf8,
        "hi",
        &[Address::from_bytes([2; 32]); 2],
        1000,
    )
    .unwrap();

    assert_eq!(duplicate.signatories().len(), 2);
    assert_eq!(
        Envelope::unsigned(duplicate),
        Err(Error::InvalidSignatories)
    );
}
