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

    /// Exercises the transaction module from the extracted archive with explicit wire bytes.
    #[test]
    fn immutable_transaction_preserves_exact_signed_bytes() {
        use solana_kit_core::{
            Instruction,
            crypto::Keypair,
            transaction::{Lifetime, Message, Transaction, Version},
        };

        let key = Keypair::from_seed(&[7; 32]);
        let program = Address::from([2; 32]);
        let blockhash = Hash::from([3; 32]);
        let lifetime = Lifetime::Blockhash {
            blockhash,
            last_valid_block_height: 42,
        };
        let instruction = Instruction {
            program_address: program,
            accounts: vec![],
            data: vec![],
        };
        let message = Message::compile(
            Version::Legacy,
            key.address(),
            lifetime,
            &[instruction],
            &[],
        )
        .expect("valid legacy message");
        let mut expected = vec![1, 0, 1, 2];
        expected.extend(key.address().as_bytes());
        expected.extend(program.as_bytes());
        expected.extend(blockhash.as_bytes());
        expected.extend([1, 1, 0, 0]);
        assert_eq!(message.bytes(), expected);

        let signed = Transaction::new(message).sign(&key).expect("owned signer");
        let packet = signed.encode(true).expect("complete signature slots");
        let received = Transaction::decode(&packet).expect("valid packet");
        assert_eq!(received.message().bytes(), expected);
        assert_eq!(
            received.message().lifetime(),
            Lifetime::Unresolved(blockhash)
        );
        assert_eq!(received.verify(), Ok(()));
        assert_eq!(received.encode(true), Ok(packet));
    }
}
