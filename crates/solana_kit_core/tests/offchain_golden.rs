const MESSAGE_MAX: usize = 2000;
const ENVELOPE_MAX: usize = 3000;
const MESSAGE_ITEMS: usize = 5000;
const ENVELOPE_ITEMS: usize = 6000;
const HEX_RADIX: u32 = 16;

use solana_kit_core::{
    Address,
    codecs::Limits,
    offchain::{ContentFormat, Envelope, Format, Message},
};

const DOMAIN: &str = "testdomain111111111111111111111111111111111";

fn unhex(text: &str) -> Vec<u8> {
    assert_eq!(text.len() % 2, 0);
    text.as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| {
            let text = core::str::from_utf8(pair).unwrap();
            u8::from_str_radix(text, HEX_RADIX).unwrap()
        })
        .collect()
}

fn fixtures() -> Vec<Vec<&'static str>> {
    let rows: Vec<Vec<_>> = include_str!("fixtures/offchain/golden.tsv")
        .lines()
        .map(|line| line.split('\t').collect())
        .collect();
    assert_eq!(rows.len(), 4);
    assert!(rows.iter().all(|fields| fields.len() == 4));
    rows
}

#[test]
fn pinned_dart_message_and_unsigned_envelope_bytes_match() {
    for fields in fixtures() {
        let bytes = unhex(fields[1]);
        let message = Message::decode(&bytes, Limits::new(MESSAGE_MAX, MESSAGE_ITEMS)).unwrap();
        assert_eq!(message.bytes(), bytes);
        assert_eq!(message.signatories().len(), 2);

        let rebuilt = match message.format() {
            Format::V0 {
                application_domain,
                content_format,
            } => {
                assert_eq!(application_domain.to_string(), DOMAIN);
                Message::v0(
                    application_domain,
                    content_format,
                    message.content(),
                    message.signatories(),
                    MESSAGE_MAX,
                )
            }
            Format::V1 => Message::v1(message.content(), message.signatories(), MESSAGE_MAX),
        }
        .unwrap();
        assert_eq!(rebuilt, message);

        let envelope = Envelope::unsigned(message.clone()).unwrap();
        assert!(!envelope.is_complete());
        assert_eq!(envelope.signatures(), &[None, None]);
        assert_eq!(envelope.message(), &message);
        assert_eq!(
            envelope.encode(false, ENVELOPE_MAX).unwrap(),
            unhex(fields[2])
        );
        assert_eq!(
            Envelope::decode(&unhex(fields[2]), Limits::new(ENVELOPE_MAX, ENVELOPE_ITEMS)).unwrap(),
            envelope
        );

        let signed =
            Envelope::decode(&unhex(fields[3]), Limits::new(ENVELOPE_MAX, ENVELOPE_ITEMS)).unwrap();
        assert!(signed.is_complete());
        assert_eq!(signed.encode(true, ENVELOPE_MAX).unwrap(), unhex(fields[3]));
    }
}

#[test]
fn versions_keep_distinct_domain_and_format_contracts() {
    let a = Address::from_bytes([1; 32]);
    let b = Address::from_bytes([2; 32]);
    let message = Message::v0(a, ContentFormat::RestrictedAscii, "hello", &[b, a], 1000).unwrap();
    assert_eq!(message.signatories(), &[b, a]);
    let message = Message::v1("\u{feff}hi\0é", &[b, a], 1000).unwrap();
    assert_eq!(message.signatories(), &[a, b]);
    assert_eq!(message.content(), "\u{feff}hi\0é");
    assert_eq!(message.format(), Format::V1);

    assert!(DOMAIN.parse::<Address>().is_ok());
    assert!("bad domain".parse::<Address>().is_err());
}

#[cfg(feature = "crypto")]
#[test]
fn original_dart_signatures_match_and_tampering_fails() {
    use solana_kit_core::{Signature, crypto::Keypair, offchain::Error};
    let a = Keypair::from_seed(&[7; 32]);
    let b = Keypair::from_seed(&[9; 32]);

    for fields in fixtures() {
        let message =
            Message::decode(&unhex(fields[1]), Limits::new(MESSAGE_MAX, MESSAGE_ITEMS)).unwrap();
        let unsigned = Envelope::unsigned(message).unwrap();
        assert_eq!(unsigned.verify(), Err(Error::MissingSignature));
        assert_eq!(
            unsigned.encode(true, ENVELOPE_MAX),
            Err(Error::MissingSignature)
        );
        let partial = unsigned.sign(&a).unwrap();

        assert!(!partial.is_complete());

        let signed = partial.sign(&b).unwrap();
        assert!(signed.is_complete());
        assert_eq!(signed.sign(&a).unwrap(), signed);
        signed.verify().unwrap();
        let wire = signed.encode(true, ENVELOPE_MAX).unwrap();
        assert_eq!(wire, unhex(fields[3]));
        assert_eq!(signed.message().bytes(), unsigned.message().bytes());
        assert!(!unsigned.is_complete());

        assert_tampering_rejected(&wire);

        assert_eq!(
            signed.sign(&Keypair::from_seed(&[8; 32])),
            Err(Error::SignerMismatch)
        );
        let absent = signed
            .with_signature(a.address(), Signature::from_bytes([0; 64]))
            .unwrap();
        assert!(!absent.is_complete());
        assert_eq!(absent.verify(), Err(Error::MissingSignature));
    }
}

#[cfg(feature = "crypto")]
fn assert_tampering_rejected(wire: &[u8]) {
    use solana_kit_core::offchain::Error;
    let mut tampered = wire.to_vec();
    tampered[1] ^= 1;
    let decoded = Envelope::decode(&tampered, Limits::new(ENVELOPE_MAX, ENVELOPE_ITEMS)).unwrap();
    assert_eq!(decoded.verify(), Err(Error::InvalidSignature));
    let mut tampered = wire.to_vec();
    *tampered.last_mut().unwrap() ^= 1;
    assert_eq!(
        Envelope::decode(&tampered, Limits::new(ENVELOPE_MAX, ENVELOPE_ITEMS))
            .unwrap()
            .verify(),
        Err(Error::InvalidSignature)
    );
}
