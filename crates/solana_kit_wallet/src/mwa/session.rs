use super::{Error, SessionKey, SessionProperties};
use aes_gcm::{Aes128Gcm, KeyInit, Nonce, aead::AeadInPlace};
use alloc::{string::String, vec::Vec};
use zeroize::Zeroizing;

/// Encrypts exact UTF-8 bytes with sequence AAD and a caller-supplied random IV.
///
/// The caller must supply an unpredictable, fresh IV for every encryption under
/// this key, across both directions. Never reuse a key and IV pair.
pub fn encrypt_message(
    message: &str,
    sequence: u32,
    iv: &[u8; 12],
    key: &SessionKey,
) -> Result<Vec<u8>, Error> {
    let aad = sequence.to_be_bytes();
    let cipher = Aes128Gcm::new_from_slice(key.0.as_ref()).map_err(|_| Error::Authentication)?;
    let mut body = Zeroizing::new(message.as_bytes().to_vec());
    cipher
        .encrypt_in_place(Nonce::from_slice(iv), &aad, &mut *body)
        .map_err(|_| Error::Authentication)?;
    let mut frame = Vec::with_capacity(16 + body.len());
    frame.extend_from_slice(&aad);
    frame.extend_from_slice(iv);
    frame.extend_from_slice(&body);
    Ok(frame)
}

/// Authenticates a frame before exposing its UTF-8 plaintext or sequence.
/// Stateless parsing does not enforce replay protection. Use [`Session`].
pub fn decrypt_message(frame: &[u8], key: &SessionKey) -> Result<(u32, String), Error> {
    if frame.len() < 32 {
        return Err(Error::Authentication);
    }

    let cipher = Aes128Gcm::new_from_slice(key.0.as_ref()).map_err(|_| Error::Authentication)?;
    let mut body = Zeroizing::new(frame[16..].to_vec());
    cipher
        .decrypt_in_place(Nonce::from_slice(&frame[4..16]), &frame[..4], &mut *body)
        .map_err(|_| Error::Authentication)?;
    let text = core::str::from_utf8(&body)
        .map_err(|_| Error::Authentication)?
        .into();
    let sequence = u32::from_be_bytes([frame[0], frame[1], frame[2], frame[3]]);
    Ok((sequence, text))
}

/// One established session owns its key and independent directional counters.
///
/// Initial counter values are explicit because HELLO_RSP session properties may
/// already consume the first receive frame. Dropping or closing erases the key.
/// The owner supplies transport, deadlines, cancellation, and fresh random IVs.
pub struct Session {
    key: SessionKey,
    next_send: u64,
    next_receive: u64,
}

impl Session {
    /// Starts ordered framing at the supplied next sequence numbers.
    pub fn new(key: SessionKey, next_send: u32, next_receive: u32) -> Self {
        Self {
            key,
            next_send: next_send.into(),
            next_receive: next_receive.into(),
        }
    }

    /// Encrypts the next outbound frame and advances only after success.
    /// The IV must be fresh under this session key in both directions.
    pub fn encrypt(&mut self, message: &str, iv: &[u8; 12]) -> Result<Vec<u8>, Error> {
        let sequence = u32::try_from(self.next_send).map_err(|_| Error::Sequence)?;
        let frame = encrypt_message(message, sequence, iv, &self.key)?;
        self.next_send += 1;
        Ok(frame)
    }

    /// Accepts only the next authenticated frame. Replays and gaps are rejected.
    /// An authentication or ordering failure leaves receive state unchanged.
    pub fn decrypt(&mut self, frame: &[u8]) -> Result<String, Error> {
        let (sequence, plaintext) = decrypt_message(frame, &self.key)?;

        if u64::from(sequence) != self.next_receive {
            return Err(Error::Sequence);
        }

        self.next_receive += 1;
        Ok(plaintext)
    }

    /// Authenticates the next frame and parses negotiated properties.
    pub fn properties(&mut self, frame: &[u8]) -> Result<SessionProperties, Error> {
        SessionProperties::from_json(&self.decrypt(frame)?)
    }

    /// Consumes this session and immediately drops its key material.
    pub fn close(self) {}
}
