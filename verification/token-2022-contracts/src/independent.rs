//! Public boundary controls distinct from the original source fixtures.

use solana_kit_core::{
    AccountRole, Address,
    codecs::{Encoder, Error, Limits},
};
use solana_kit_token_2022::{accounts::*, *};

const MAX_SOURCES: usize = u8::MAX as usize;
const MAX_ACCOUNTS: usize = MAX_SOURCES + 5;
const BYTE_LIMIT: usize = 1000;

pub(crate) fn maximum_variable_tail_keeps_exact_roles_and_item_limit() {
    let source = Address::from_bytes([9; 32]);
    let sources = vec![source; MAX_SOURCES];
    let signers = [source; 2];
    let value = WithdrawWithheldTokensFromAccounts {
        num_token_accounts: u8::MAX,
    };
    let input = WithdrawWithheldTokensFromAccountsAccounts {
        mint: source,
        fee_receiver: source,
        withdraw_withheld_authority: Authority {
            address: source,
            is_signer: false,
        },
        multi_signers: &signers,
        sources: &sources,
    };
    let limits = Limits::new(BYTE_LIMIT, MAX_ACCOUNTS);
    let instruction = value.instruction(input, limits).unwrap();

    assert_eq!(instruction.accounts.len(), MAX_ACCOUNTS);
    assert_eq!(instruction.data, [26, 3, u8::MAX]);
    assert_eq!(instruction.accounts[3].role, AccountRole::ReadonlySigner);
    assert_eq!(instruction.accounts[4].role, AccountRole::ReadonlySigner);
    assert!(
        instruction.accounts[5..]
            .iter()
            .all(|meta| meta.role == AccountRole::Writable)
    );
    assert!(parse_instruction(&instruction, limits).is_ok());
    assert_eq!(
        value.instruction(input, Limits::new(BYTE_LIMIT, MAX_ACCOUNTS - 1)),
        Err(Error::LimitExceeded)
    );
    let mut wrong = instruction;
    wrong.accounts[5].role = AccountRole::ReadonlySigner;
    assert!(parse_instruction(&wrong, limits).is_err());
}

fn mint(initialized: bool) -> Mint {
    Mint {
        mint_authority: None,
        supply: u64::MAX,
        decimals: u8::MAX,
        is_initialized: initialized,
        freeze_authority: None,
        extensions: None,
    }
}

pub(crate) fn canonical_mint_checks_state_owner_and_exact_allocation() {
    let limits = Limits::new(BYTE_LIMIT, MAX_ACCOUNTS);
    let owner = Address::from_bytes([8; 32]);
    let value = Token2022Account::Mint(mint(true));
    let bytes = value.encode(82).unwrap();

    assert_eq!(bytes.len(), 82);
    assert!(value.encode(81).is_err());
    assert_eq!(
        decode_account_with_program(owner, owner, &bytes, limits).unwrap(),
        value
    );
    assert_eq!(
        decode_account(owner, &bytes, limits),
        Err(Error::InvalidValue)
    );
    assert_eq!(
        decode_account_with_program(owner, PROGRAM, &bytes, limits),
        Err(Error::InvalidValue)
    );
    let uninitialized = MintCodec.encode(&mint(false), BYTE_LIMIT).unwrap();

    assert_eq!(
        decode_account(PROGRAM, &uninitialized, limits),
        Err(Error::InvalidValue)
    );
    assert_eq!(
        Token2022Account::Mint(mint(false)).encode(BYTE_LIMIT),
        Err(Error::InvalidValue)
    );
}

pub(crate) fn multisig_threshold_edges_and_uninitialized_state_are_rejected() {
    for (m, n, initialized) in [(0, 0, true), (0, 1, true), (1, 12, true), (1, 1, false)] {
        let value = Multisig {
            m,
            n,
            is_initialized: initialized,
            signers: vec![Address::from_bytes([7; 32]); 11],
        };
        let bytes = MultisigCodec.encode(&value, BYTE_LIMIT).unwrap();

        assert_eq!(
            decode_account(PROGRAM, &bytes, Limits::new(BYTE_LIMIT, MAX_ACCOUNTS)),
            Err(Error::InvalidValue)
        );
        assert_eq!(
            Token2022Account::Multisig(value).encode(BYTE_LIMIT),
            Err(Error::InvalidValue)
        );
    }
}
