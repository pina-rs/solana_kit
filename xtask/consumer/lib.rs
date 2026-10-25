//! Independent external tests of the packaged public API.
#![forbid(unsafe_code)]

extern crate alloc;

#[cfg(test)]
#[path = "codec_overrides.rs"]
mod override_contract;

#[cfg(test)]
mod tests {
    use solana_kit_core::{Address, Hash};

    /// Archive exports retain insertion order and normalize repeated keys.
    #[test]
    fn ordered_maps_are_available_from_the_archive() {
        use solana_kit_core::collections::OrderedMap;
        let value = OrderedMap::from_entries(vec![(2_u8, 5_u8), (1, 7), (2, 9)]);
        assert_eq!(value.as_slice(), &[(2_u8, 9_u8), (1_u8, 7_u8)]);
    }

    /// The packaged optional feature keeps original Keccak padding.
    #[test]
    fn keccak_is_available_from_the_archive() {
        use solana_kit_core::keccak::keccak256;
        let expected = [
            0x4e, 0x03, 0x65, 0x7a, 0xea, 0x45, 0xa9, 0x4f, 0xc7, 0xd4, 0x7b, 0xa8, 0x26, 0xc8,
            0xd6, 0x67, 0xc0, 0xd1, 0xe6, 0xe3, 0x3a, 0x64, 0xa0, 0x36, 0xec, 0x44, 0xf5, 0x8f,
            0xa1, 0x2d, 0x6c, 0x45,
        ];
        assert_eq!(keccak256(&[b"ab", b"c"]).to_bytes(), expected);
    }

    /// The registry dependency preserves owned and borrowed complete-operation policies.
    #[test]
    fn borrowed_codec_operation_policies() {
        crate::override_contract::complete_operations();
    }

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

    /// The registry consumer receives checked pairing and public size arithmetic.
    #[test]
    fn codec_metadata_and_pair_validation() {
        use solana_kit_core::codecs::{Decoder, Encoder, Error, Limits, Number, Paired, Size};

        let pair = Paired::<_, _, u16, u16>::try_new(Number::<u16>::LE, Number::<u16>::LE)
            .expect("matching metadata");
        assert_eq!(pair.encode_size(), Ok(Size::Fixed(2)));
        assert_eq!(pair.decode_size(), Ok(Size::Fixed(2)));
        assert_eq!(pair.encode(&513, 2), Ok(vec![1, 2]));
        assert_eq!(pair.decode(&[1, 2], Limits::new(2, 0)), Ok(513));
        assert!(Paired::<_, _, u8, u32>::try_new(Number::<u8>::LE, Number::<u32>::LE).is_err());
        assert_eq!(
            Size::checked_sum(&[Size::Fixed(usize::MAX), Size::Fixed(1)]),
            Err(Error::Overflow)
        );
    }

    /// The packaged key import rejects a forged public half before signing.
    #[test]
    fn imported_keypair_checks_both_halves() {
        use solana_kit_core::crypto::Keypair;

        let original = Keypair::from_seed(&[7; 32]);
        let mut bytes = [7; 64];
        bytes[32..].copy_from_slice(original.address().as_bytes());
        let imported = Keypair::from_bytes(&bytes).expect("matching key halves");
        assert_eq!(imported.address(), original.address());
        assert_eq!(imported.sign(b"registry"), original.sign(b"registry"));
        bytes[63] ^= 1;
        assert!(Keypair::from_bytes(&bytes).is_err());
        assert!(Keypair::from_bytes(&bytes[..63]).is_err());
    }

    /// The archive retains optional exact arithmetic and its independent wire layout.
    #[test]
    fn fixed_points_preserve_exact_rounding_and_bytes() {
        use solana_kit_core::{
            codecs::{Decoder, Encoder, Endian, Limits},
            fixed_points::{DecimalShape, Rounding, Signedness},
        };

        let shape = DecimalShape::new(Signedness::Signed, 16, 2).expect("bounded shape");
        let value = shape.parse("-1", Rounding::Strict).expect("exact decimal");
        let rounded = value
            .checked_div_scalar(3, Rounding::Floor)
            .expect("floor quotient");
        let codec = shape.codec(Endian::Little).expect("byte-aligned shape");

        assert_eq!(rounded.to_string(), "-0.34");
        assert_eq!(codec.encode(&rounded, 2), Ok(vec![222, 255]));
        assert_eq!(codec.decode(&[222, 255], Limits::new(2, 2)), Ok(rounded));
        assert!(codec.decode(&[222], Limits::new(2, 2)).is_err());
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
