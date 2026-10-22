const WIRE_MAX: usize = 100000;
const ITEM_MAX: usize = 200000;
const SMALL_WIRE_MAX: usize = 10000;
const SMALL_ITEM_MAX: usize = 20000;
const ENVELOPE_MAX: usize = 30000;
const LONG_CONTENT_MAX: usize = 65535;
const LONG_CONTENT_OVERFLOW: usize = 65536;
const UTF8_OVERFLOW_CHARACTERS: usize = 617;

const CONTENT_BYTES: usize = 5;

use solana_kit_core::{
    Address,
    codecs::{Error as CodecError, Limits},
    offchain::{ContentFormat, Envelope, Error, Message},
};

fn version_zero(format: ContentFormat, content: &str) -> Result<Message, Error> {
    Message::v0(
        Address::from_bytes([1; 32]),
        format,
        content,
        &[Address::from_bytes([2; 32])],
        WIRE_MAX,
    )
}

#[test]
fn content_byte_boundaries_are_independent_of_transaction_packets() {
    for format in [ContentFormat::RestrictedAscii, ContentFormat::Utf8] {
        assert!(version_zero(format, &"a".repeat(1232)).is_ok());
        assert_eq!(
            version_zero(format, &"a".repeat(1233)),
            Err(Error::InvalidContent)
        );
        assert_eq!(version_zero(format, ""), Err(Error::InvalidContent));
    }

    for invalid in ["\n", "\u{1f}", "\u{7f}", "é"] {
        assert_eq!(
            version_zero(ContentFormat::RestrictedAscii, invalid),
            Err(Error::InvalidContent)
        );
    }

    assert!(version_zero(ContentFormat::RestrictedAscii, " ~").is_ok());
    assert!(version_zero(ContentFormat::Utf8, &"é".repeat(616)).is_ok());
    assert_eq!(
        version_zero(ContentFormat::Utf8, &"é".repeat(UTF8_OVERFLOW_CHARACTERS)),
        Err(Error::InvalidContent)
    );
    let longest = version_zero(ContentFormat::Utf8Long, &"a".repeat(LONG_CONTENT_MAX)).unwrap();
    assert_eq!(longest.bytes().len(), 65620);
    assert_eq!(
        Message::decode(longest.bytes(), Limits::new(WIRE_MAX, ITEM_MAX)).unwrap(),
        longest
    );
    assert_eq!(
        version_zero(ContentFormat::Utf8Long, &"a".repeat(LONG_CONTENT_OVERFLOW)),
        Err(Error::InvalidContent)
    );
    let version_one = Message::v1(
        &"a".repeat(LONG_CONTENT_OVERFLOW),
        &[Address::from_bytes([2; 32])],
        WIRE_MAX,
    )
    .unwrap();

    assert_eq!(version_one.content().len(), LONG_CONTENT_OVERFLOW);
}

#[test]
fn caller_budgets_cover_wire_text_and_signatory_allocations() {
    let message = version_zero(ContentFormat::Utf8, "hello").unwrap();
    let size = message.bytes().len();
    assert_eq!(
        Message::decode(message.bytes(), Limits::new(size - 1, 1000)),
        Err(Error::Codec(CodecError::LimitExceeded))
    );
    assert_eq!(
        Message::decode(message.bytes(), Limits::new(size, size + CONTENT_BYTES)),
        Err(Error::Codec(CodecError::LimitExceeded))
    );
    assert!(Message::decode(message.bytes(), Limits::new(size, size + 6)).is_ok());
    assert!(Message::v1("hi", &[Address::from_bytes([1; 32])], 51).is_err());
    let unsigned = Envelope::unsigned(message).unwrap();
    let wire = unsigned.encode(false, 1000).unwrap();

    assert!(unsigned.encode(false, wire.len() - 1).is_err());
    assert!(Envelope::decode(&wire, Limits::new(wire.len(), 1)).is_err());
}

#[test]
fn maximum_signatory_count_and_v1_remainder_are_exact() {
    let addresses: Vec<_> = (0..255)
        .map(|index| Address::from_bytes([index; 32]))
        .collect();
    let message = Message::v1("hi", &addresses, SMALL_WIRE_MAX).unwrap();
    assert_eq!(message.signatories().len(), 255);
    let envelope = Envelope::unsigned(message.clone()).unwrap();
    let wire = envelope.encode(false, ENVELOPE_MAX).unwrap();
    assert_eq!(wire[0], 255);
    assert_eq!(
        Envelope::decode(&wire, Limits::new(ENVELOPE_MAX, ENVELOPE_MAX)).unwrap(),
        envelope
    );
    let mut appended = message.bytes().to_vec();

    appended.push(b'!');
    assert_eq!(
        Message::decode(&appended, Limits::new(SMALL_WIRE_MAX, SMALL_ITEM_MAX))
            .unwrap()
            .content(),
        "hi!"
    );
}
