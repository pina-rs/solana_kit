//! Accepted portable off-chain message cases reused by the shared suite.
extern crate alloc;

const VERSION_OFFSET: usize = 16;
const COUNT_OFFSET: usize = 17;
const CONTENT_LAST_OFFSET: usize = 54;
const ENVELOPE_MAX: usize = 2000;
const MESSAGE_ITEMS: usize = 3000;
const ENVELOPE_ITEMS: usize = 4000;

use solana_kit_core::{
    Address,
    codecs::Limits,
    crypto::Keypair,
    offchain::{ContentFormat, Envelope, Error, Message},
};

/// Executes V0/V1 message and unsigned envelope byte preservation.
pub fn valid() -> Result<(), &'static str> {
    let address = Address::from_bytes([1; 32]);

    for message in [
        Message::v0(address, ContentFormat::Utf8, "hello é", &[address], 1000),
        Message::v1("hello é", &[address], 1000),
    ] {
        let message = message.map_err(|_| "construction failed")?;
        let decoded = Message::decode(message.bytes(), Limits::new(1000, MESSAGE_ITEMS))
            .map_err(|_| "decode failed")?;

        if message != decoded {
            return Err("message bytes changed");
        }

        let envelope = Envelope::unsigned(message).map_err(|_| "unsigned failed")?;
        let bytes = envelope
            .encode(false, ENVELOPE_MAX)
            .map_err(|_| "encoding failed")?;
        let decoded = Envelope::decode(&bytes, Limits::new(ENVELOPE_MAX, ENVELOPE_ITEMS))
            .map_err(|_| "envelope decode failed")?;

        if decoded != envelope {
            return Err("envelope bytes changed");
        }
    }

    Ok(())
}

/// Executes invalid domain, UTF-8, count, and budget controls.
pub fn malformed() -> Result<(), &'static str> {
    let address = Address::from_bytes([1; 32]);
    let message = Message::v1("hello", &[address], 1000).map_err(|_| "construction failed")?;

    for offset in [1, VERSION_OFFSET, COUNT_OFFSET, CONTENT_LAST_OFFSET] {
        let mut bytes = message.bytes().to_vec();
        bytes[offset] = 255;

        if Message::decode(&bytes, Limits::new(1000, MESSAGE_ITEMS)).is_ok() {
            return Err("malformed bytes accepted");
        }
    }

    if Message::decode(message.bytes(), Limits::new(1, 1)).is_ok() {
        return Err("input budget ignored");
    }

    Ok(())
}

/// Executes deterministic signing and a cryptographic tamper negative control.
pub fn tamper() -> Result<(), &'static str> {
    let key = Keypair::from_seed(&[7; 32]);
    let message =
        Message::v1("authorize", &[key.address()], 1000).map_err(|_| "construction failed")?;
    let envelope = Envelope::unsigned(message).map_err(|_| "unsigned failed")?;
    let signed = envelope.sign(&key).map_err(|_| "signing failed")?;
    signed.verify().map_err(|_| "valid signature rejected")?;
    let mut bytes = signed
        .encode(true, ENVELOPE_MAX)
        .map_err(|_| "encoding failed")?;
    bytes[1] ^= 1;

    let tampered = Envelope::decode(&bytes, Limits::new(ENVELOPE_MAX, ENVELOPE_ITEMS))
        .map_err(|_| "tampered envelope decode failed")?;

    if tampered.verify() != Err(Error::InvalidSignature) {
        return Err("tampered signature accepted");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn isolated_portable_functions_execute() {
        super::valid().unwrap();
        super::malformed().unwrap();
        super::tamper().unwrap();
    }
}
