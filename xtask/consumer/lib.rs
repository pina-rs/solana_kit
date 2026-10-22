//! Independent external tests of the packaged public API.
#![forbid(unsafe_code)]

#[cfg(test)]
mod tests {
    use solana_kit_core::{Address, Hash};

    /// Uses a fixed external base58 vector rather than only round-tripping output.
    #[test]
    fn address_golden_and_rejected_input() {
        let bytes = core::array::from_fn(|index| u8::try_from(index + 1).expect("32 bytes"));
        let address = Address::from(bytes);

        assert_eq!(
            address.to_string(),
            "4wBqpZM9xaSheZzJSMawUKKwhdpChKbZ5eu5ky4Vigw"
        );
        assert_eq!(address.to_string().parse::<Address>(), Ok(address));
        assert!("0".parse::<Address>().is_err());
        assert!("1".repeat(45).parse::<Address>().is_err());
    }

    /// Hash values preserve exact bytes and reject incorrect fixed lengths.
    #[test]
    fn fixed_hash_boundary() {
        let hash = Hash::from([7; 32]);

        assert_eq!(hash.to_string().parse::<Hash>(), Ok(hash));
        assert!(Hash::try_from(&[7; 31][..]).is_err());
    }

    /// Checks deterministic signing and rejection through only the public package.
    #[test]
    fn crypto_signing_rejects_changed_message() {
        use solana_kit_core::crypto::{Keypair, verify};

        let key = Keypair::from_seed(&[7; 32]);
        let signature = key.sign(b"downstream package");

        assert_eq!(
            verify(&key.address(), b"downstream package", &signature),
            Ok(())
        );
        assert!(verify(&key.address(), b"changed message", &signature).is_err());
    }
}
