//! Accepted strict Token assertions executed without host services.
macro_rules! assert_eq {
    ($left:expr, $right:expr $(, $rest:tt)*) => {
        crate::require($left == $right, "Token literal mismatch")?
    };
}
macro_rules! assert {
    ($value:expr $(, $rest:tt)*) => {
        crate::require($value, "Token parser rejected oracle")?
    };
}
use alloc::{vec, vec::Vec};
use solana_kit_core::{
    AccountMeta, AccountRole, Address,
    codecs::{Decoder, Encoder, Limits},
};
use solana_kit_token::*;
const LIMITS: Limits = Limits::new(4096, 4096);
fn address(value: u8) -> Address {
    Address::from_bytes([value; 32])
}
fn mint_bytes() -> Vec<u8> {
    vec![0; 82]
}

fn absent_fixed_options_ignore_nonzero_payload_and_encode_zero_padding() -> Result<(), &'static str>
{
    let mut bytes = mint_bytes();
    bytes[4..36].fill(255);
    bytes[50..82].fill(127);
    let value = MintCodec
        .decode(&bytes, LIMITS)
        .map_err(|_| "Token strict operation failed")?;
    assert_eq!(value.mint_authority, None);
    assert_eq!(value.freeze_authority, None);

    assert_eq!(
        MintCodec
            .encode(&value, 82)
            .map_err(|_| "Token strict operation failed")?,
        mint_bytes()
    );
    let mut token = vec![0; 165];

    token[76..108].fill(255);
    token[113..121].fill(255);
    token[133..165].fill(255);
    let decoded = TokenCodec
        .decode(&token, LIMITS)
        .map_err(|_| "Token strict operation failed")?;

    assert_eq!(decoded.delegate, None);
    assert_eq!(decoded.is_native, None);
    assert_eq!(decoded.close_authority, None);
    assert_eq!(
        TokenCodec
            .encode(&decoded, 165)
            .map_err(|_| "Token strict operation failed")?,
        vec![0; 165]
    );
    Ok(())
}

fn malformed_bool_option_enum_account_lengths_and_allocations_reject() -> Result<(), &'static str> {
    for offset in [0, 3, 45, 46, 49] {
        let mut bytes = mint_bytes();
        bytes[offset] = 2;
        assert!(
            MintCodec.decode(&bytes, LIMITS).is_err(),
            "mint offset {offset}"
        );
    }

    for offset in [72, 75, 108, 109, 112, 129, 132] {
        let mut bytes = vec![0; 165];
        bytes[offset] = 255;
        assert!(
            TokenCodec.decode(&bytes, LIMITS).is_err(),
            "token offset {offset}"
        );
    }

    let mut multisig = vec![0; 355];
    multisig[2] = 2;
    assert!(MultisigCodec.decode(&multisig, LIMITS).is_err());
    multisig[2] = 0;
    assert!(
        MultisigCodec
            .decode(&multisig, Limits::new(355, 10))
            .is_err()
    );
    assert!(
        MultisigCodec
            .decode(&multisig, Limits::new(355, 11))
            .is_ok()
    );

    for length in [0, 1, 81, 83, 164, 166, 354, 356] {
        assert!(identify_account(&vec![0; length]).is_err());
    }

    assert_eq!(
        identify_account(&mint_bytes()).map_err(|_| "Token strict operation failed")?,
        TokenAccountKind::Mint
    );
    Ok(())
}

/// Exercise tag domains and incomplete frames through public codecs.
fn instruction_tags_optional_payloads_and_batch_frames_fail_strictly() -> Result<(), &'static str> {
    for bytes in [
        vec![0, 7],
        vec![0, 7, 0],
        vec![6, 4, 0],
        vec![6, 0, 2],
        vec![45, 2],
        vec![255, 1],
        vec![255, 1, 4, 0, 0],
    ] {
        assert!(
            TokenInstructionDataCodec.decode(&bytes, LIMITS).is_err(),
            "{bytes:?}"
        );
    }

    for tag in 0u8..=255 {
        if !(0..=24).contains(&tag) && ![38, 45, 255].contains(&tag) {
            assert!(identify_instruction(&[tag]).is_err());
        }
    }

    let batch = Batch {
        data: vec![BatchDataItem {
            number_of_accounts: 255,
            instruction_data: vec![0; 256],
        }],
    };
    assert!(BatchCodec.encode(&batch, 4096).is_err());
    let valid = vec![255, 7, 3, 1, 2, 3];
    assert!(BatchCodec.decode(&valid, Limits::new(4096, 3)).is_err());

    assert!(BatchCodec.decode(&valid, Limits::new(4096, 4)).is_ok());
    assert!(UiAmountToAmountCodec.decode(&[24, 255], LIMITS).is_err());
    Ok(())
}

fn multisigner_roles() -> Result<(), &'static str> {
    let accounts = vec![
        AccountMeta {
            address: address(1),
            role: AccountRole::Writable,
        },
        AccountMeta {
            address: address(2),
            role: AccountRole::Writable,
        },
        AccountMeta {
            address: address(3),
            role: AccountRole::Readonly,
        },
        AccountMeta {
            address: address(4),
            role: AccountRole::ReadonlySigner,
        },
        AccountMeta {
            address: address(4),
            role: AccountRole::ReadonlySigner,
        },
    ];
    let value = Transfer { amount: u64::MAX };
    let instruction = value
        .instruction(&accounts, LIMITS)
        .map_err(|_| "Token strict operation failed")?;

    assert_eq!(instruction.accounts, accounts);
    assert_eq!(
        Transfer::parse(&instruction, LIMITS).map_err(|_| "Token strict operation failed")?,
        value
    );
    assert_eq!(instruction.accounts[2].role, AccountRole::Readonly);
    assert!(value.instruction(&accounts, Limits::new(9, 4)).is_err());

    for index in 0..accounts.len() {
        let mut bad = accounts.clone();
        bad[index].role = if index == 2 {
            AccountRole::Writable
        } else {
            AccountRole::Readonly
        };

        assert!(value.instruction(&bad, LIMITS).is_err(), "role {index}");
    }

    let mut foreign = instruction;
    foreign.program_address = address(99);
    assert!(Transfer::parse(&foreign, LIMITS).is_err());
    assert!(Transfer::parse_with_program(address(99), &foreign, LIMITS).is_ok());
    Ok(())
}

fn omitted_rent_and_initialization_remaining_roles_follow_the_source() -> Result<(), &'static str> {
    let account = AccountMeta {
        address: address(1),
        role: AccountRole::Writable,
    };
    assert!(SyncNative {}.instruction(&[account], LIMITS).is_ok());
    let signer = AccountMeta {
        address: address(2),
        role: AccountRole::Readonly,
    };
    let instruction = InitializeMultisig2 { m: 255 }
        .instruction(&[account, signer, signer], LIMITS)
        .map_err(|_| "Token strict operation failed")?;
    assert_eq!(instruction.accounts, [account, signer, signer]);
    let signer = AccountMeta {
        role: AccountRole::ReadonlySigner,
        ..signer
    };
    assert!(
        InitializeMultisig2 { m: 0 }
            .instruction(&[account, signer], LIMITS)
            .is_err()
    );
    Ok(())
}

fn all_error_codes_preserve_unknown_values_and_source_messages() -> Result<(), &'static str> {
    for row in
        include_str!("../../../crates/solana_kit_token/tests/fixtures/token-errors.tsv").lines()
    {
        let (code, message) = row.split_once('\t').ok_or("missing Token error field")?;
        let code = code
            .parse::<u32>()
            .map_err(|_| "Token strict operation failed")?;
        assert_eq!(
            TokenError::from_code(code).message().unwrap_or("-"),
            message
        );
    }

    for code in 0..20 {
        let error = TokenError::from_code(code);
        assert_eq!(error.code(), code);
        assert!(error.message().is_some());
    }

    for code in [20, 255, u32::MAX] {
        assert_eq!(TokenError::from_code(code), TokenError::Unknown(code));
        assert_eq!(TokenError::from_code(code).code(), code);
        assert_eq!(TokenError::from_code(code).message(), None);
    }

    assert_eq!(
        TokenError::UninitializedState.message(),
        Some("State is unititialized")
    );
    Ok(())
}
pub(crate) fn run() -> Result<(), &'static str> {
    absent_fixed_options_ignore_nonzero_payload_and_encode_zero_padding()?;
    malformed_bool_option_enum_account_lengths_and_allocations_reject()?;
    instruction_tags_optional_payloads_and_batch_frames_fail_strictly()?;
    multisigner_roles()?;
    omitted_rent_and_initialization_remaining_roles_follow_the_source()?;
    all_error_codes_preserve_unknown_values_and_source_messages()?;
    Ok(())
}
