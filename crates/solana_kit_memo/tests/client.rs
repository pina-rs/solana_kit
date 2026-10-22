use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Error, Limits},
};
use solana_kit_memo::*;

const LIMITS: Limits = Limits::new(128, 128);

#[test]
fn raw_utf8_and_historical_extraction_preserve_literal_bytes() {
    let signers = [Address::from_bytes([9; 32]); 2];
    let memo = "gm 🌞 café\0";
    let expected = [
        103, 109, 32, 240, 159, 140, 158, 32, 99, 97, 102, 195, 169, 0,
    ];
    let invocation = add_memo(memo, &signers, LIMITS).unwrap();
    assert_eq!(invocation.data, expected);
    assert_eq!(
        invocation.program_address.to_string(),
        "Memo4c2pN8afCj432Lb7RMVKi9PbQnnW7ewFFaV3oAH"
    );
    assert_eq!(
        invocation.accounts,
        signers.map(|address| AccountMeta {
            address,
            role: AccountRole::ReadonlySigner
        })
    );
    assert_eq!(
        parse_instruction(&invocation, LIMITS).unwrap(),
        ParsedMemo {
            memo: memo.into(),
            signers: signers.to_vec()
        }
    );
    let mut instructions = vec![Instruction {
        program_address: Address::from_bytes([1; 32]),
        accounts: vec![],
        data: vec![255],
    }];

    for program in SUPPORTED_PROGRAMS {
        instructions.push(add_memo_with_program(program, memo, &[], LIMITS).unwrap());
    }

    let extracted = memos_from_instructions(&instructions, LIMITS).unwrap();
    assert_eq!(extracted.len(), 3);

    for (index, memo) in extracted.iter().enumerate() {
        assert_eq!(memo.index, index + 1);
        assert_eq!(memo.bytes, expected);
        assert_eq!(memo.program_address, SUPPORTED_PROGRAMS[index]);
    }

    assert_eq!(
        LEGACY_PROGRAM.to_string(),
        "Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo"
    );
    assert_eq!(
        LEGACY_PROGRAM_V3.to_string(),
        "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"
    );
}

#[test]
fn malformed_text_bom_nulls_metadata_and_budgets_are_explicit() {
    assert_eq!(
        decode_memo(&[239, 187, 191, 65, 0, 66], LIMITS).unwrap(),
        "A\0B"
    );
    assert_eq!(
        decode_memo(&[239, 187, 191, 239, 187, 191], LIMITS).unwrap(),
        "\u{feff}"
    );
    assert_eq!(decode_memo(&[], Limits::new(0, 0)).unwrap(), "");

    for malformed in [&[195, 40][..], &[255][..], &[240, 159][..]] {
        assert_eq!(decode_memo(malformed, LIMITS), Err(Error::InvalidUtf8));
    }

    assert_eq!(
        add_memo("ab", &[], Limits::new(1, 2)),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        decode_memo(b"ab", Limits::new(2, 1)),
        Err(Error::LimitExceeded)
    );
    assert!(add_memo_with_program(Address::from_bytes([1; 32]), "", &[], LIMITS).is_err());
    let mut invocation = add_memo("a", &[Address::from_bytes([9; 32])], LIMITS).unwrap();
    assert_eq!(
        parse_instruction(&invocation, Limits::new(1, 1)),
        Err(Error::LimitExceeded)
    );
    invocation.accounts[0].role = AccountRole::WritableSigner;
    assert!(parse_instruction(&invocation, LIMITS).is_ok());
    invocation.accounts[0].role = AccountRole::Readonly;
    assert_eq!(
        parse_instruction(&invocation, LIMITS),
        Err(Error::InvalidValue)
    );
    assert_eq!(
        memos_from_instructions(&[invocation.clone()], LIMITS).unwrap()[0].memo,
        "a"
    );
    invocation.program_address = Address::from_bytes([1; 32]);
    assert_eq!(
        parse_instruction(&invocation, LIMITS),
        Err(Error::InvalidValue)
    );
}
