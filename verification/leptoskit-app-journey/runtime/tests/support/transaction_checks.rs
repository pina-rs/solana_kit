//! Isolate the oracle's signature check from its exact-message comparison.
use super::*;
use ed25519_dalek::{Signer, SigningKey};

const PAYER_OFFSET: usize = 4;
const RECIPIENT_OFFSET: usize = 36;

fn allowed(key: &SigningKey) -> Transactions {
    let transactions = Transactions::default();
    transactions.allow(&bs58::encode(key.verifying_key().as_bytes()).into_string());
    transactions
}

fn request(key: &SigningKey, message: &[u8]) -> Value {
    let mut wire = vec![1];
    wire.extend_from_slice(&key.sign(message).to_bytes());
    wire.extend_from_slice(message);
    json!({"params":[STANDARD.encode(wire),{"encoding":"base64"}]})
}

#[test]
fn expected_message_needs_a_signature_from_the_allowed_payer() {
    let payer = SigningKey::from_bytes(&[7; 32]);
    let other = SigningKey::from_bytes(&[8; 32]);
    let transactions = allowed(&payer);
    let message = expected_message(payer.verifying_key().to_bytes());
    assert!(transactions.submit(&request(&other, &message)).is_err());
    assert_eq!(transactions.signature(), None);
    assert_eq!(transactions.balance(), BALANCE);

    assert!(transactions.submit(&request(&payer, &message)).is_ok());
    assert!(transactions.signature().is_some());
    assert_eq!(transactions.balance(), BALANCE + AMOUNT);
}

#[test]
fn a_valid_payer_signature_does_not_authorize_a_changed_transfer() {
    let payer = SigningKey::from_bytes(&[7; 32]);
    let original = expected_message(payer.verifying_key().to_bytes());
    let amount = original.len() - size_of::<u64>();

    for (field, offset) in [
        ("payer", PAYER_OFFSET),
        ("recipient", RECIPIENT_OFFSET),
        ("amount", amount),
    ] {
        let transactions = allowed(&payer);
        let mut message = original.clone();
        message[offset] ^= 1;
        let result = transactions.submit(&request(&payer, &message));
        assert!(result.is_err(), "changed {field} must be rejected");
        assert_eq!(transactions.signature(), None, "changed {field}");
        assert_eq!(transactions.balance(), BALANCE, "changed {field}");
    }
}
