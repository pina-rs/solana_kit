use solana_kit_core::{
    Address,
    codecs::{Decoder, Encoder, Error, Limits},
};
use solana_kit_token_2022::{accounts::*, *};
const ACCOUNT_BYTES: usize = 100_000;
const ACCOUNT_ITEMS: usize = 4096;
const WITHHELD_AMOUNT: u64 = 7;
fn limits() -> Limits {
    Limits::new(ACCOUNT_BYTES, ACCOUNT_ITEMS)
}
fn mint(extensions: Option<Vec<Extension>>) -> Mint {
    Mint {
        mint_authority: None,
        supply: u64::MAX,
        decimals: 9,
        is_initialized: true,
        freeze_authority: None,
        extensions,
    }
}
fn token(extensions: Option<Vec<Extension>>) -> Token {
    Token {
        mint: Address::from_bytes([1; 32]),
        owner: Address::from_bytes([2; 32]),
        amount: u64::MAX,
        delegate: None,
        state: AccountState::Initialized,
        is_native: None,
        delegated_amount: 0,
        close_authority: None,
        extensions,
    }
}
fn metadata() -> Extension {
    Extension::TokenMetadata(ExtensionTokenMetadataPayload {
        update_authority: None,
        mint: Address::from_bytes([1; 32]),
        name: "x".repeat(105),
        symbol: String::new(),
        uri: String::new(),
        additional_metadata: vec![],
    })
}
fn fee() -> Extension {
    Extension::TransferFeeAmount(ExtensionTransferFeeAmountPayload {
        withheld_amount: WITHHELD_AMOUNT,
    })
}
pub(crate) fn canonical_355_padding_owner_and_account_type_are_checked() {
    let raw = mint(Some(vec![metadata()]));
    assert_eq!(MintCodec.encode(&raw, 1000).unwrap().len(), 355);
    let value = Token2022Account::Mint(raw);
    let bytes = value.encode(1000).unwrap();
    assert_eq!(bytes.len(), 357);
    assert_eq!(decode_account(PROGRAM, &bytes, limits()).unwrap(), value);
    assert!(value.encode(356).is_err());

    assert!(decode_account(Address::from_bytes([9; 32]), &bytes, limits()).is_err());
    let mut bad = bytes;
    bad[165] = 2;
    assert!(decode_account(PROGRAM, &bad, limits()).is_err());
}
pub(crate) fn affinity_duplicate_and_terminator_entries_are_rejected() {
    assert!(
        Token2022Account::Mint(mint(Some(vec![fee()])))
            .encode(1000)
            .is_err()
    );
    assert!(
        Token2022Account::Token(token(Some(vec![fee(), fee()])))
            .encode(1000)
            .is_err()
    );
    assert!(
        Token2022Account::Token(token(Some(vec![Extension::Uninitialized])))
            .encode(1000)
            .is_err()
    );
}
pub(crate) fn canonical_fee_frame_and_every_truncation_are_literal() {
    let value = Token2022Account::Token(token(Some(vec![fee()])));
    let bytes = value.encode(1000).unwrap();
    let literal_tail = [2, 2, 0, 8, 0, 7, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(&bytes[165..], &literal_tail);
    assert_eq!(decode_account(PROGRAM, &bytes, limits()).unwrap(), value);
    let frame = [2, 0, 8, 0, 7, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(ExtensionCodec.encode(&fee(), 100).unwrap(), frame);

    for end in 1..12 {
        assert!(ExtensionCodec.decode(&frame[..end], limits()).is_err());
    }

    assert_eq!(
        ExtensionCodec
            .encode(&Extension::Uninitialized, 100)
            .unwrap(),
        [0, 0]
    );
}
pub(crate) fn initialized_multisig_and_base_accounts_dispatch() {
    for value in [
        Token2022Account::Mint(mint(None)),
        Token2022Account::Token(token(None)),
        Token2022Account::Multisig(Multisig {
            m: 2,
            n: 3,
            is_initialized: true,
            signers: vec![Address::from_bytes([7; 32]); 11],
        }),
    ] {
        let bytes = value.encode(1000).unwrap();
        assert_eq!(decode_account(PROGRAM, &bytes, limits()).unwrap(), value);
    }
    let bad = Token2022Account::Multisig(Multisig {
        m: 4,
        n: 3,
        is_initialized: true,
        signers: vec![Address::from_bytes([7; 32]); 11],
    });
    assert_eq!(bad.encode(1000), Err(Error::InvalidValue));
}
pub(crate) fn public_extension_sequence_reuses_canonical_termination_and_shared_budget() {
    let mut bytes = ExtensionsCodec.encode(&vec![fee()], 100).unwrap();
    bytes.extend_from_slice(&[0, 0, 255, 255, 255]);
    assert_eq!(
        ExtensionsCodec.decode(&bytes, limits()).unwrap(),
        vec![fee()]
    );
    assert!(
        ExtensionsCodec
            .encode(&vec![Extension::Uninitialized], 100)
            .is_err()
    );
    assert!(ExtensionsCodec.decode(&bytes, Limits::new(100, 0)).is_err());
    assert_eq!(mint_size(None).unwrap(), 82);
    assert_eq!(token_size(None).unwrap(), 165);
    assert_eq!(mint_size(Some(&[])).unwrap(), 166);
}
