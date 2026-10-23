//! Account TLV entries reject capacity terminators through every encoder entry point.
use solana_kit_core::{
    Address,
    codecs::{Decoder, Encoder, Error, Limits, Writer},
};
use solana_kit_token_layouts_independent::emitted::*;

const LIMITS: Limits = Limits::new(4096, 4096);

fn fee() -> Extension {
    Extension::TransferFeeAmount(ExtensionTransferFeeAmountPayload { withheld_amount: 7 })
}

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

fn rejected<T>(codec: impl Encoder<T>, value: &T, base_size: usize) {
    let mut output = [255; LIMITS.max_bytes];
    let mut writer = Writer::new(&mut output);
    assert_eq!(codec.encoded_len(value), Err(Error::InvalidValue));
    assert_eq!(
        codec.encode(value, LIMITS.max_bytes),
        Err(Error::InvalidValue)
    );
    assert_eq!(codec.write(value, &mut writer), Err(Error::InvalidValue));
    assert_eq!(writer.position(), base_size);
    assert!(writer.bytes()[base_size..].iter().all(|byte| *byte == 255));
}

fn invalid_entries() -> [Vec<Extension>; 3] {
    [
        vec![Extension::Uninitialized, fee(), fee()],
        vec![fee(), Extension::Uninitialized, fee()],
        vec![fee(), fee(), Extension::Uninitialized],
    ]
}

#[test]
fn mint_rejects_terminator_at_beginning_middle_and_end() {
    for extensions in invalid_entries() {
        rejected(MintCodec, &mint(extensions), 82);
    }
}

#[test]
fn token_rejects_terminator_at_beginning_middle_and_end() {
    for extensions in invalid_entries() {
        rejected(TokenCodec, &token(extensions), 165);
    }
}

#[test]
fn standalone_zero_variant_keeps_its_two_byte_codec() {
    let value = Extension::Uninitialized;
    let mut output = [255; 2];
    let mut writer = Writer::new(&mut output);
    assert_eq!(ExtensionCodec.encoded_len(&value), Ok(2));
    assert_eq!(ExtensionCodec.encode(&value, 2).unwrap(), [0, 0]);
    ExtensionCodec.write(&value, &mut writer).unwrap();
    assert_eq!(writer.bytes(), &[0, 0]);
    assert_eq!(ExtensionCodec.decode(&[0, 0], LIMITS).unwrap(), value);
}

#[test]
fn empty_and_valid_account_collections_roundtrip() {
    for extensions in [Vec::new(), vec![fee()], vec![fee(), fee()]] {
        let mint = mint(extensions.clone());
        let token = token(extensions);
        let mint_bytes = MintCodec.encode(&mint, LIMITS.max_bytes).unwrap();
        let token_bytes = TokenCodec.encode(&token, LIMITS.max_bytes).unwrap();
        assert_eq!(MintCodec.decode(&mint_bytes, LIMITS).unwrap(), mint);
        assert_eq!(TokenCodec.decode(&token_bytes, LIMITS).unwrap(), token);
    }
}
