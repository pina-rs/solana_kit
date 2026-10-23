//! Independent public controls against the complete emitted Token-2022 data model.
extern crate alloc;

/// Actual immutable-IDL output selected explicitly by the verification runner.
pub mod emitted {
    include!(env!("TOKEN_LAYOUTS_GENERATED"));
}

#[cfg(test)]
mod tests {
    use super::emitted::*;
    use solana_kit_core::{
        Address,
        codecs::{Decoder, Encoder, Error, Limits, Reader},
    };

    const LIMITS: Limits = Limits::new(4096, 4096);

    fn mint(extensions: Vec<Extension>) -> Mint {
        Mint {
            mint_authority: None,
            supply: 7,
            decimals: 9,
            is_initialized: true,
            freeze_authority: None,
            extensions: Some(extensions),
        }
    }

    fn token(extensions: Vec<Extension>) -> Token {
        Token {
            mint: Address::from_bytes([0; 32]),
            owner: Address::from_bytes([9; 32]),
            amount: 7,
            delegate: None,
            state: AccountState::Initialized,
            is_native: None,
            delegated_amount: 0,
            close_authority: None,
            extensions: Some(extensions),
        }
    }

    fn fee() -> Extension {
        Extension::TransferFeeAmount(ExtensionTransferFeeAmountPayload { withheld_amount: 7 })
    }

    fn metadata(name: String, entries: Vec<(String, String)>) -> Extension {
        Extension::TokenMetadata(ExtensionTokenMetadataPayload {
            update_authority: None,
            mint: Address::from_bytes([7; 32]),
            name,
            symbol: String::new(),
            uri: String::new(),
            additional_metadata: entries,
        })
    }

    #[test]
    fn typed_mint_terminator_is_rejected_or_roundtrips_losslessly() {
        let value = mint(vec![Extension::Uninitialized]);

        if let Ok(bytes) = MintCodec.encode(&value, LIMITS.max_bytes) {
            assert_eq!(MintCodec.decode(&bytes, LIMITS).unwrap(), value);
        }
    }

    #[test]
    fn typed_token_terminator_cannot_silently_drop_later_extensions() {
        let value = token(vec![Extension::Uninitialized, fee()]);

        if let Ok(bytes) = TokenCodec.encode(&value, LIMITS.max_bytes) {
            assert_eq!(TokenCodec.decode(&bytes, LIMITS).unwrap(), value);
        }
    }

    #[test]
    fn transfer_fee_literal_and_every_shortened_full_frame() {
        let literal = [2, 0, 8, 0, 7, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(ExtensionCodec.encode(&fee(), 12).unwrap(), literal);
        assert_eq!(ExtensionCodec.decode(&literal, LIMITS).unwrap(), fee());

        for end in 0..literal.len() {
            assert!(ExtensionCodec.decode(&literal[..end], LIMITS).is_err());
        }

        for bytes in [
            vec![255, 255],
            vec![2, 0, 7, 0, 7, 0, 0, 0, 0, 0, 0],
            vec![2, 0, 9, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![2, 0, 255, 255],
        ] {
            assert!(ExtensionCodec.decode(&bytes, LIMITS).is_err());
        }
    }

    #[test]
    fn successful_and_failed_nested_transactions_preserve_the_contract() {
        let mut reader = Reader::new(&[10, 20, 30, 40], Limits::new(4, 5)).unwrap();
        let mut external = 0;
        let failed: Result<(), Error> = reader.transaction(|attempt| {
            external += 1;
            attempt.take(1)?;
            attempt.claim(2)?;
            attempt.transaction(|nested| {
                nested.take(1)?;
                nested.claim(1)
            })?;
            Err(Error::InvalidValue)
        });
        assert_eq!(failed, Err(Error::InvalidValue));
        assert_eq!(external, 1);
        assert_eq!(reader.position(), 0);

        reader
            .transaction(|attempt| {
                assert_eq!(attempt.take(2)?, &[10, 20]);
                attempt.claim(3)
            })
            .unwrap();
        assert_eq!(reader.position(), 2);
        assert_eq!(reader.claim(3), Err(Error::LimitExceeded));
        reader.claim(2).unwrap();
    }

    #[test]
    fn ordinary_failed_frame_still_consumes_bytes_and_claims() {
        let mut reader = Reader::new(&[10, 20, 30], Limits::new(3, 3)).unwrap();
        let failed: Result<(), Error> = reader.frame(2, |frame| {
            frame.claim(2)?;
            Err(Error::InvalidValue)
        });
        assert_eq!(failed, Err(Error::InvalidValue));
        assert_eq!(reader.position(), 2);
        assert_eq!(reader.take(2), Err(Error::Truncated));
        assert_eq!(reader.position(), 2);
        assert_eq!(reader.claim(2), Err(Error::LimitExceeded));
        reader.claim(1).unwrap();
    }

    #[test]
    fn canonical_account_tails_keep_termination_and_odd_capacity() {
        let value = mint(vec![fee()]);
        let canonical = MintCodec.encode(&value, LIMITS.max_bytes).unwrap();
        assert_eq!(canonical.len(), 178);
        assert_eq!(&canonical[82..165], &[0; 83]);
        assert_eq!(canonical[165], 1);
        assert_eq!(&canonical[166..], &[2, 0, 8, 0, 7, 0, 0, 0, 0, 0, 0, 0]);

        for tail in [vec![255], vec![0, 0], vec![0, 0, 255, 255, 255]] {
            let mut padded = canonical.clone();
            padded.extend(tail);
            assert_eq!(MintCodec.decode(&padded, LIMITS).unwrap(), value);
        }

        let value = token(vec![fee()]);
        let bytes = TokenCodec.encode(&value, LIMITS.max_bytes).unwrap();
        assert_eq!(bytes[165], 2);
        assert_eq!(TokenCodec.decode(&bytes, LIMITS).unwrap(), value);
    }

    #[test]
    fn ordered_metadata_entries_preserve_duplicate_keys_and_shared_budget() {
        let entries = vec![
            (String::from("key"), String::from("first")),
            (String::from("key"), String::from("second")),
        ];
        let value = metadata(String::new(), entries);
        let bytes = ExtensionCodec.encode(&value, LIMITS.max_bytes).unwrap();
        assert_eq!(ExtensionCodec.decode(&bytes, LIMITS).unwrap(), value);
        assert!(
            ExtensionCodec
                .decode(&bytes, Limits::new(LIMITS.max_bytes, 1))
                .is_err()
        );

        let mut hostile = vec![0; 84];
        hostile[..4].copy_from_slice(&[19, 0, 80, 0]);
        hostile[80..].copy_from_slice(&u32::MAX.to_le_bytes());
        let mut reader = Reader::new(&hostile, Limits::new(84, 2)).unwrap();
        assert!(ExtensionCodec.read(&mut reader).is_err());
        assert_eq!(reader.position(), 0);
        reader.claim(2).unwrap();
    }

    #[test]
    fn named_structural_codecs_do_not_implement_reserved_size_dispatch() {
        let extension = metadata("x".repeat(105), Vec::new());
        let mint_value = mint(vec![extension.clone()]);
        let token_value = token(vec![extension]);
        let mint_bytes = MintCodec.encode(&mint_value, LIMITS.max_bytes).unwrap();
        let token_bytes = TokenCodec.encode(&token_value, LIMITS.max_bytes).unwrap();
        assert_eq!(mint_bytes.len(), 355);
        assert_eq!(token_bytes.len(), 355);
        assert_eq!(MintCodec.decode(&mint_bytes, LIMITS).unwrap(), mint_value);
        assert_eq!(
            TokenCodec.decode(&token_bytes, LIMITS).unwrap(),
            token_value
        );
        assert!(MultisigCodec.decode(&mint_bytes, LIMITS).is_ok());
        assert!(MultisigCodec.decode(&token_bytes, LIMITS).is_ok());
    }
}
