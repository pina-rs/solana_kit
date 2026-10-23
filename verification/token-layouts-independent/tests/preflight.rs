//! Collection validation precedes payload and capacity errors.
use solana_kit_core::{
    Address,
    codecs::{Encoder, Error, Writer},
};
use solana_kit_token_layouts_independent::emitted::*;

/// A late terminator must be rejected before an oversized earlier payload is encoded.
#[test]
fn terminator_preflight_precedes_oversized_payload_and_output_capacity() {
    let oversized = Extension::TokenMetadata(ExtensionTokenMetadataPayload {
        update_authority: None,
        mint: Address::from_bytes([7; 32]),
        name: "x".repeat(65_536),
        symbol: String::new(),
        uri: String::new(),
        additional_metadata: Vec::new(),
    });
    let value = Mint {
        mint_authority: None,
        supply: 7,
        decimals: 9,
        is_initialized: true,
        freeze_authority: None,
        extensions: Some(vec![oversized, Extension::Uninitialized]),
    };
    let mut output = [255; 82];
    let mut writer = Writer::new(&mut output);
    assert_eq!(MintCodec.encoded_len(&value), Err(Error::InvalidValue));
    assert_eq!(MintCodec.encode(&value, 82), Err(Error::InvalidValue));
    assert_eq!(
        MintCodec.write(&value, &mut writer),
        Err(Error::InvalidValue)
    );
    assert_eq!(writer.position(), 82);
}
