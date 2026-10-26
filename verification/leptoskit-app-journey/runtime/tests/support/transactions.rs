//! Independent legacy/System wire facts, followed by explicit protocol fixtures.
use base64::{Engine, engine::general_purpose::STANDARD};
use ed25519_dalek::{Signature, VerifyingKey};
use serde_json::{Value, json};
use std::{io, sync::Mutex};

const BALANCE: u64 = 42;
const AMOUNT: u64 = 1_000_000;
const SIGNATURE_BYTES: usize = 64;
const WIRE_BYTES: usize = 215;

#[derive(Default)]
pub(super) struct Transactions {
    payer: Mutex<Option<[u8; 32]>>,
    signature: Mutex<Option<String>>,
}

impl Transactions {
    pub(super) fn allow(&self, account: &str) {
        let bytes = bs58::decode(account).into_vec().unwrap();
        *self.payer.lock().unwrap() = Some(bytes.try_into().expect("exact public key"));
    }

    pub(super) fn signature(&self) -> Option<String> {
        self.signature.lock().unwrap().clone()
    }

    pub(super) fn balance(&self) -> u64 {
        BALANCE
            + if self.signature().is_some() {
                AMOUNT
            } else {
                0
            }
    }

    /// An acknowledgement follows only a complete independent wire and signature
    /// check. No SDK decoder or builder supplies this oracle's expected message.
    pub(super) fn submit(&self, request: &Value) -> io::Result<Value> {
        let payer = self
            .payer
            .lock()
            .unwrap()
            .ok_or_else(|| io::Error::other("submissions disabled"))?;
        let text = request["params"][0]
            .as_str()
            .ok_or_else(|| io::Error::other("wire missing"))?;
        let wire = STANDARD.decode(text).map_err(io::Error::other)?;

        if wire.len() != WIRE_BYTES || wire[0] != 1 || request["params"][1]["encoding"] != "base64"
        {
            return Err(io::Error::other("unexpected transaction framing"));
        }

        let signature =
            Signature::from_slice(&wire[1..1 + SIGNATURE_BYTES]).map_err(io::Error::other)?;
        let message = &wire[1 + SIGNATURE_BYTES..];

        if message != expected_message(payer) {
            return Err(io::Error::other(
                "message differs from independent wire oracle",
            ));
        }

        VerifyingKey::from_bytes(&payer)
            .map_err(io::Error::other)?
            .verify_strict(message, &signature)
            .map_err(io::Error::other)?;
        let encoded = bs58::encode(signature.to_bytes()).into_string();
        let mut stored = self.signature.lock().unwrap();

        if stored.is_some() {
            return Err(io::Error::other("duplicate transaction submission"));
        }

        *stored = Some(encoded.clone());
        Ok(json!(encoded))
    }

    pub(super) fn statuses(&self, request: &Value) -> io::Result<Value> {
        let signature = self
            .signature()
            .ok_or_else(|| io::Error::other("no submitted transaction"))?;

        if request["params"][0] != json!([signature]) {
            return Err(io::Error::other("foreign status query"));
        }

        Ok(json!({"context":{"slot":10},"value":[{
            "slot":10,"confirmations":1,"err":null,"confirmationStatus":"confirmed","status":{"Ok":null}
        }]}))
    }
}

/// One writable signer, one writable recipient, the readonly System program,
/// a zero fixture blockhash and System's tag-two twelve-byte transfer payload.
fn expected_message(payer: [u8; 32]) -> Vec<u8> {
    let mut message = vec![1, 0, 1, 3];
    message.extend_from_slice(&payer);
    message.extend_from_slice(&[98; 32]);
    message.extend_from_slice(&[0; 32]);
    message.extend_from_slice(&[0; 32]);
    message.extend_from_slice(&[1, 2, 2, 0, 1, 12]);
    message.extend_from_slice(&2_u32.to_le_bytes());
    message.extend_from_slice(&AMOUNT.to_le_bytes());
    message
}
