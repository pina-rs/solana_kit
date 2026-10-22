//! Portable execution of the accepted literal Token constructor and account matrix.
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
use super::token_support as support;
use alloc::{
    string::{String, ToString},
    vec,
};
use solana_kit_core::{
    AccountMeta, AccountRole, Address,
    codecs::{Decoder, Encoder, Limits},
};
use solana_kit_token::*;
const LIMITS: Limits = Limits::new(4096, 4096);
const NONZERO_BYTE: u8 = 7;
fn initialize_mint_case0() -> Result<(), &'static str> {
    let value = InitializeMint {
        decimals: 0,
        mint_authority: "11111111111111111111111111111111"
            .parse()
            .map_err(|_| "Token operation failed")?,
        freeze_authority: None,
    };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("initializeMint-0", &instruction)?;
    assert_eq!(
        InitializeMint::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn initialize_account_case0() -> Result<(), &'static str> {
    let value = InitializeAccount {};
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("initializeAccount-0", &instruction)?;
    assert_eq!(
        InitializeAccount::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn initialize_multisig_case0() -> Result<(), &'static str> {
    let value = InitializeMultisig { m: 0 };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("initializeMultisig-0", &instruction)?;
    assert_eq!(
        InitializeMultisig::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn transfer_case0() -> Result<(), &'static str> {
    let value = Transfer { amount: 0 };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("transfer-0", &instruction)?;
    assert_eq!(
        Transfer::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn approve_case0() -> Result<(), &'static str> {
    let value = Approve { amount: 0 };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("approve-0", &instruction)?;
    assert_eq!(
        Approve::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn revoke_case0() -> Result<(), &'static str> {
    let value = Revoke {};
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("revoke-0", &instruction)?;
    assert_eq!(
        Revoke::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn set_authority_case0() -> Result<(), &'static str> {
    let value = SetAuthority {
        authority_type: AuthorityType::MintTokens,
        new_authority: None,
    };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("setAuthority-0", &instruction)?;
    assert_eq!(
        SetAuthority::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn mint_to_case0() -> Result<(), &'static str> {
    let value = MintTo { amount: 0 };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("mintTo-0", &instruction)?;
    assert_eq!(
        MintTo::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn burn_case0() -> Result<(), &'static str> {
    let value = Burn { amount: 0 };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("burn-0", &instruction)?;
    assert_eq!(
        Burn::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn close_account_case0() -> Result<(), &'static str> {
    let value = CloseAccount {};
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("closeAccount-0", &instruction)?;
    assert_eq!(
        CloseAccount::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn freeze_account_case0() -> Result<(), &'static str> {
    let value = FreezeAccount {};
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("freezeAccount-0", &instruction)?;
    assert_eq!(
        FreezeAccount::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn thaw_account_case0() -> Result<(), &'static str> {
    let value = ThawAccount {};
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("thawAccount-0", &instruction)?;
    assert_eq!(
        ThawAccount::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn transfer_checked_case0() -> Result<(), &'static str> {
    let value = TransferChecked {
        amount: 0,
        decimals: 0,
    };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("transferChecked-0", &instruction)?;
    assert_eq!(
        TransferChecked::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn approve_checked_case0() -> Result<(), &'static str> {
    let value = ApproveChecked {
        amount: 0,
        decimals: 0,
    };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("approveChecked-0", &instruction)?;
    assert_eq!(
        ApproveChecked::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn mint_to_checked_case0() -> Result<(), &'static str> {
    let value = MintToChecked {
        amount: 0,
        decimals: 0,
    };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("mintToChecked-0", &instruction)?;
    assert_eq!(
        MintToChecked::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn burn_checked_case0() -> Result<(), &'static str> {
    let value = BurnChecked {
        amount: 0,
        decimals: 0,
    };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("burnChecked-0", &instruction)?;
    assert_eq!(
        BurnChecked::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn initialize_account2_case0() -> Result<(), &'static str> {
    let value = InitializeAccount2 {
        owner: "11111111111111111111111111111111"
            .parse()
            .map_err(|_| "Token operation failed")?,
    };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("initializeAccount2-0", &instruction)?;
    assert_eq!(
        InitializeAccount2::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn sync_native_case0() -> Result<(), &'static str> {
    let value = SyncNative {};
    let accounts = vec![AccountMeta {
        address: "11111111111111111111111111111111"
            .parse()
            .map_err(|_| "Token operation failed")?,
        role: AccountRole::from_privileges(false, true),
    }];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("syncNative-0", &instruction)?;
    assert_eq!(
        SyncNative::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn initialize_account3_case0() -> Result<(), &'static str> {
    let value = InitializeAccount3 {
        owner: "11111111111111111111111111111111"
            .parse()
            .map_err(|_| "Token operation failed")?,
    };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("initializeAccount3-0", &instruction)?;
    assert_eq!(
        InitializeAccount3::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn initialize_multisig2_case0() -> Result<(), &'static str> {
    let value = InitializeMultisig2 { m: 0 };
    let accounts = vec![AccountMeta {
        address: "11111111111111111111111111111111"
            .parse()
            .map_err(|_| "Token operation failed")?,
        role: AccountRole::from_privileges(false, true),
    }];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("initializeMultisig2-0", &instruction)?;
    assert_eq!(
        InitializeMultisig2::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn initialize_mint2_case0() -> Result<(), &'static str> {
    let value = InitializeMint2 {
        decimals: 0,
        mint_authority: "11111111111111111111111111111111"
            .parse()
            .map_err(|_| "Token operation failed")?,
        freeze_authority: None,
    };
    let accounts = vec![AccountMeta {
        address: "11111111111111111111111111111111"
            .parse()
            .map_err(|_| "Token operation failed")?,
        role: AccountRole::from_privileges(false, true),
    }];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("initializeMint2-0", &instruction)?;
    assert_eq!(
        InitializeMint2::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn get_account_data_size_case0() -> Result<(), &'static str> {
    let value = GetAccountDataSize {};
    let accounts = vec![AccountMeta {
        address: "11111111111111111111111111111111"
            .parse()
            .map_err(|_| "Token operation failed")?,
        role: AccountRole::from_privileges(false, false),
    }];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("getAccountDataSize-0", &instruction)?;
    assert_eq!(
        GetAccountDataSize::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn initialize_immutable_owner_case0() -> Result<(), &'static str> {
    let value = InitializeImmutableOwner {};
    let accounts = vec![AccountMeta {
        address: "11111111111111111111111111111111"
            .parse()
            .map_err(|_| "Token operation failed")?,
        role: AccountRole::from_privileges(false, true),
    }];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("initializeImmutableOwner-0", &instruction)?;
    assert_eq!(
        InitializeImmutableOwner::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn amount_to_ui_amount_case0() -> Result<(), &'static str> {
    let value = AmountToUiAmount { amount: 0 };
    let accounts = vec![AccountMeta {
        address: "11111111111111111111111111111111"
            .parse()
            .map_err(|_| "Token operation failed")?,
        role: AccountRole::from_privileges(false, false),
    }];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("amountToUiAmount-0", &instruction)?;
    assert_eq!(
        AmountToUiAmount::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn ui_amount_to_amount_case0() -> Result<(), &'static str> {
    let value = UiAmountToAmount {
        ui_amount: String::new(),
    };
    let accounts = vec![AccountMeta {
        address: "11111111111111111111111111111111"
            .parse()
            .map_err(|_| "Token operation failed")?,
        role: AccountRole::from_privileges(false, false),
    }];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("uiAmountToAmount-0", &instruction)?;
    assert_eq!(
        UiAmountToAmount::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn withdraw_excess_lamports_case0() -> Result<(), &'static str> {
    let value = WithdrawExcessLamports {};
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("withdrawExcessLamports-0", &instruction)?;
    assert_eq!(
        WithdrawExcessLamports::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn unwrap_lamports_case0() -> Result<(), &'static str> {
    let value = UnwrapLamports { amount: None };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("unwrapLamports-0", &instruction)?;
    assert_eq!(
        UnwrapLamports::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn batch_case0() -> Result<(), &'static str> {
    let value = Batch { data: vec![] };
    let accounts = vec![];
    let program: Address = "11111111111111111111111111111111"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("batch-0", &instruction)?;
    assert_eq!(
        Batch::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn account_mint_case0() -> Result<(), &'static str> {
    let value = Mint {
        mint_authority: None,
        supply: 0,
        decimals: 0,
        is_initialized: false,
        freeze_authority: None,
    };
    let bytes = MintCodec
        .encode(&value, 4096)
        .map_err(|_| "Token operation failed")?;
    support::bytes("account-mint-0", &bytes)?;
    assert_eq!(
        MintCodec
            .decode(&bytes, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    support::fixed_negatives(&MintCodec, &bytes)?;
    Ok(())
}
fn account_token_case0() -> Result<(), &'static str> {
    let value = Token {
        mint: "11111111111111111111111111111111"
            .parse()
            .map_err(|_| "Token operation failed")?,
        owner: "11111111111111111111111111111111"
            .parse()
            .map_err(|_| "Token operation failed")?,
        amount: 0,
        delegate: None,
        state: AccountState::Uninitialized,
        is_native: None,
        delegated_amount: 0,
        close_authority: None,
    };
    let bytes = TokenCodec
        .encode(&value, 4096)
        .map_err(|_| "Token operation failed")?;
    support::bytes("account-token-0", &bytes)?;
    assert_eq!(
        TokenCodec
            .decode(&bytes, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    support::fixed_negatives(&TokenCodec, &bytes)?;
    Ok(())
}
fn account_multisig_case0() -> Result<(), &'static str> {
    let value = Multisig {
        m: 0,
        n: 0,
        is_initialized: false,
        signers: vec![
            "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
        ],
    };
    let bytes = MultisigCodec
        .encode(&value, 4096)
        .map_err(|_| "Token operation failed")?;
    support::bytes("account-multisig-0", &bytes)?;
    assert_eq!(
        MultisigCodec
            .decode(&bytes, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    support::fixed_negatives(&MultisigCodec, &bytes)?;
    Ok(())
}
fn initialize_mint_case1() -> Result<(), &'static str> {
    let value = InitializeMint {
        decimals: NONZERO_BYTE,
        mint_authority: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
            .parse()
            .map_err(|_| "Token operation failed")?,
        freeze_authority: Some(
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
        ),
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("initializeMint-1", &instruction)?;
    assert_eq!(
        InitializeMint::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn initialize_account_case1() -> Result<(), &'static str> {
    let value = InitializeAccount {};
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("initializeAccount-1", &instruction)?;
    assert_eq!(
        InitializeAccount::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn initialize_multisig_case1() -> Result<(), &'static str> {
    let value = InitializeMultisig { m: NONZERO_BYTE };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("initializeMultisig-1", &instruction)?;
    assert_eq!(
        InitializeMultisig::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn transfer_case1() -> Result<(), &'static str> {
    let value = Transfer {
        amount: u64::from(NONZERO_BYTE),
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("transfer-1", &instruction)?;
    assert_eq!(
        Transfer::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn approve_case1() -> Result<(), &'static str> {
    let value = Approve {
        amount: u64::from(NONZERO_BYTE),
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("approve-1", &instruction)?;
    assert_eq!(
        Approve::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn revoke_case1() -> Result<(), &'static str> {
    let value = Revoke {};
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("revoke-1", &instruction)?;
    assert_eq!(
        Revoke::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn set_authority_case1() -> Result<(), &'static str> {
    let value = SetAuthority {
        authority_type: AuthorityType::FreezeAccount,
        new_authority: Some(
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
        ),
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("setAuthority-1", &instruction)?;
    assert_eq!(
        SetAuthority::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn mint_to_case1() -> Result<(), &'static str> {
    let value = MintTo {
        amount: u64::from(NONZERO_BYTE),
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("mintTo-1", &instruction)?;
    assert_eq!(
        MintTo::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn burn_case1() -> Result<(), &'static str> {
    let value = Burn {
        amount: u64::from(NONZERO_BYTE),
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("burn-1", &instruction)?;
    assert_eq!(
        Burn::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn close_account_case1() -> Result<(), &'static str> {
    let value = CloseAccount {};
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("closeAccount-1", &instruction)?;
    assert_eq!(
        CloseAccount::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn freeze_account_case1() -> Result<(), &'static str> {
    let value = FreezeAccount {};
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("freezeAccount-1", &instruction)?;
    assert_eq!(
        FreezeAccount::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn thaw_account_case1() -> Result<(), &'static str> {
    let value = ThawAccount {};
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("thawAccount-1", &instruction)?;
    assert_eq!(
        ThawAccount::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn transfer_checked_case1() -> Result<(), &'static str> {
    let value = TransferChecked {
        amount: u64::from(NONZERO_BYTE),
        decimals: NONZERO_BYTE,
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("transferChecked-1", &instruction)?;
    assert_eq!(
        TransferChecked::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn approve_checked_case1() -> Result<(), &'static str> {
    let value = ApproveChecked {
        amount: u64::from(NONZERO_BYTE),
        decimals: NONZERO_BYTE,
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("approveChecked-1", &instruction)?;
    assert_eq!(
        ApproveChecked::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn mint_to_checked_case1() -> Result<(), &'static str> {
    let value = MintToChecked {
        amount: u64::from(NONZERO_BYTE),
        decimals: NONZERO_BYTE,
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("mintToChecked-1", &instruction)?;
    assert_eq!(
        MintToChecked::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn burn_checked_case1() -> Result<(), &'static str> {
    let value = BurnChecked {
        amount: u64::from(NONZERO_BYTE),
        decimals: NONZERO_BYTE,
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("burnChecked-1", &instruction)?;
    assert_eq!(
        BurnChecked::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn initialize_account2_case1() -> Result<(), &'static str> {
    let value = InitializeAccount2 {
        owner: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
            .parse()
            .map_err(|_| "Token operation failed")?,
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("initializeAccount2-1", &instruction)?;
    assert_eq!(
        InitializeAccount2::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn sync_native_case1() -> Result<(), &'static str> {
    let value = SyncNative {};
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("syncNative-1", &instruction)?;
    assert_eq!(
        SyncNative::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn initialize_account3_case1() -> Result<(), &'static str> {
    let value = InitializeAccount3 {
        owner: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
            .parse()
            .map_err(|_| "Token operation failed")?,
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("initializeAccount3-1", &instruction)?;
    assert_eq!(
        InitializeAccount3::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn initialize_multisig2_case1() -> Result<(), &'static str> {
    let value = InitializeMultisig2 { m: NONZERO_BYTE };
    let accounts = vec![AccountMeta {
        address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
            .parse()
            .map_err(|_| "Token operation failed")?,
        role: AccountRole::from_privileges(false, true),
    }];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("initializeMultisig2-1", &instruction)?;
    assert_eq!(
        InitializeMultisig2::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn initialize_mint2_case1() -> Result<(), &'static str> {
    let value = InitializeMint2 {
        decimals: NONZERO_BYTE,
        mint_authority: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
            .parse()
            .map_err(|_| "Token operation failed")?,
        freeze_authority: Some(
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
        ),
    };
    let accounts = vec![AccountMeta {
        address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
            .parse()
            .map_err(|_| "Token operation failed")?,
        role: AccountRole::from_privileges(false, true),
    }];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("initializeMint2-1", &instruction)?;
    assert_eq!(
        InitializeMint2::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn get_account_data_size_case1() -> Result<(), &'static str> {
    let value = GetAccountDataSize {};
    let accounts = vec![AccountMeta {
        address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
            .parse()
            .map_err(|_| "Token operation failed")?,
        role: AccountRole::from_privileges(false, false),
    }];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("getAccountDataSize-1", &instruction)?;
    assert_eq!(
        GetAccountDataSize::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn initialize_immutable_owner_case1() -> Result<(), &'static str> {
    let value = InitializeImmutableOwner {};
    let accounts = vec![AccountMeta {
        address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
            .parse()
            .map_err(|_| "Token operation failed")?,
        role: AccountRole::from_privileges(false, true),
    }];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("initializeImmutableOwner-1", &instruction)?;
    assert_eq!(
        InitializeImmutableOwner::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn amount_to_ui_amount_case1() -> Result<(), &'static str> {
    let value = AmountToUiAmount {
        amount: u64::from(NONZERO_BYTE),
    };
    let accounts = vec![AccountMeta {
        address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
            .parse()
            .map_err(|_| "Token operation failed")?,
        role: AccountRole::from_privileges(false, false),
    }];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("amountToUiAmount-1", &instruction)?;
    assert_eq!(
        AmountToUiAmount::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn ui_amount_to_amount_case1() -> Result<(), &'static str> {
    let value = UiAmountToAmount {
        ui_amount: "42.🦀\0".to_string(),
    };
    let accounts = vec![AccountMeta {
        address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
            .parse()
            .map_err(|_| "Token operation failed")?,
        role: AccountRole::from_privileges(false, false),
    }];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("uiAmountToAmount-1", &instruction)?;
    assert_eq!(
        UiAmountToAmount::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn withdraw_excess_lamports_case1() -> Result<(), &'static str> {
    let value = WithdrawExcessLamports {};
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("withdrawExcessLamports-1", &instruction)?;
    assert_eq!(
        WithdrawExcessLamports::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn unwrap_lamports_case1() -> Result<(), &'static str> {
    let value = UnwrapLamports {
        amount: Some(u64::from(NONZERO_BYTE)),
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("unwrapLamports-1", &instruction)?;
    assert_eq!(
        UnwrapLamports::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn batch_case1() -> Result<(), &'static str> {
    let value = Batch {
        data: vec![BatchDataItem {
            number_of_accounts: NONZERO_BYTE,
            instruction_data: vec![u8::MAX; 3],
        }],
    };
    let accounts = vec![];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("batch-1", &instruction)?;
    assert_eq!(
        Batch::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn account_mint_case1() -> Result<(), &'static str> {
    let value = Mint {
        mint_authority: Some(
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
        ),
        supply: u64::from(NONZERO_BYTE),
        decimals: NONZERO_BYTE,
        is_initialized: true,
        freeze_authority: Some(
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
        ),
    };
    let bytes = MintCodec
        .encode(&value, 4096)
        .map_err(|_| "Token operation failed")?;
    support::bytes("account-mint-1", &bytes)?;
    assert_eq!(
        MintCodec
            .decode(&bytes, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    support::fixed_negatives(&MintCodec, &bytes)?;
    Ok(())
}
fn account_token_case1() -> Result<(), &'static str> {
    let value = Token {
        mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
            .parse()
            .map_err(|_| "Token operation failed")?,
        owner: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
            .parse()
            .map_err(|_| "Token operation failed")?,
        amount: u64::from(NONZERO_BYTE),
        delegate: Some(
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
        ),
        state: AccountState::Initialized,
        is_native: Some(u64::from(NONZERO_BYTE)),
        delegated_amount: u64::from(NONZERO_BYTE),
        close_authority: Some(
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
        ),
    };
    let bytes = TokenCodec
        .encode(&value, 4096)
        .map_err(|_| "Token operation failed")?;
    support::bytes("account-token-1", &bytes)?;
    assert_eq!(
        TokenCodec
            .decode(&bytes, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    support::fixed_negatives(&TokenCodec, &bytes)?;
    Ok(())
}
fn account_multisig_case1() -> Result<(), &'static str> {
    let value = Multisig {
        m: NONZERO_BYTE,
        n: NONZERO_BYTE,
        is_initialized: true,
        signers: vec![
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
        ],
    };
    let bytes = MultisigCodec
        .encode(&value, 4096)
        .map_err(|_| "Token operation failed")?;
    support::bytes("account-multisig-1", &bytes)?;
    assert_eq!(
        MultisigCodec
            .decode(&bytes, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    support::fixed_negatives(&MultisigCodec, &bytes)?;
    Ok(())
}
fn initialize_mint_case2() -> Result<(), &'static str> {
    let value = InitializeMint {
        decimals: u8::MAX,
        mint_authority: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
            .parse()
            .map_err(|_| "Token operation failed")?,
        freeze_authority: Some(
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
        ),
    };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("initializeMint-2", &instruction)?;
    assert_eq!(
        InitializeMint::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn initialize_account_case2() -> Result<(), &'static str> {
    let value = InitializeAccount {};
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("initializeAccount-2", &instruction)?;
    assert_eq!(
        InitializeAccount::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn initialize_multisig_case2() -> Result<(), &'static str> {
    let value = InitializeMultisig { m: u8::MAX };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("initializeMultisig-2", &instruction)?;
    assert_eq!(
        InitializeMultisig::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn transfer_case2() -> Result<(), &'static str> {
    let value = Transfer { amount: u64::MAX };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("transfer-2", &instruction)?;
    assert_eq!(
        Transfer::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn approve_case2() -> Result<(), &'static str> {
    let value = Approve { amount: u64::MAX };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("approve-2", &instruction)?;
    assert_eq!(
        Approve::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn revoke_case2() -> Result<(), &'static str> {
    let value = Revoke {};
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("revoke-2", &instruction)?;
    assert_eq!(
        Revoke::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn set_authority_case2() -> Result<(), &'static str> {
    let value = SetAuthority {
        authority_type: AuthorityType::CloseAccount,
        new_authority: Some(
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
        ),
    };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("setAuthority-2", &instruction)?;
    assert_eq!(
        SetAuthority::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn mint_to_case2() -> Result<(), &'static str> {
    let value = MintTo { amount: u64::MAX };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("mintTo-2", &instruction)?;
    assert_eq!(
        MintTo::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn burn_case2() -> Result<(), &'static str> {
    let value = Burn { amount: u64::MAX };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("burn-2", &instruction)?;
    assert_eq!(
        Burn::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn close_account_case2() -> Result<(), &'static str> {
    let value = CloseAccount {};
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("closeAccount-2", &instruction)?;
    assert_eq!(
        CloseAccount::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn freeze_account_case2() -> Result<(), &'static str> {
    let value = FreezeAccount {};
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("freezeAccount-2", &instruction)?;
    assert_eq!(
        FreezeAccount::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn thaw_account_case2() -> Result<(), &'static str> {
    let value = ThawAccount {};
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("thawAccount-2", &instruction)?;
    assert_eq!(
        ThawAccount::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn transfer_checked_case2() -> Result<(), &'static str> {
    let value = TransferChecked {
        amount: u64::MAX,
        decimals: u8::MAX,
    };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("transferChecked-2", &instruction)?;
    assert_eq!(
        TransferChecked::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn approve_checked_case2() -> Result<(), &'static str> {
    let value = ApproveChecked {
        amount: u64::MAX,
        decimals: u8::MAX,
    };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("approveChecked-2", &instruction)?;
    assert_eq!(
        ApproveChecked::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn mint_to_checked_case2() -> Result<(), &'static str> {
    let value = MintToChecked {
        amount: u64::MAX,
        decimals: u8::MAX,
    };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("mintToChecked-2", &instruction)?;
    assert_eq!(
        MintToChecked::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn burn_checked_case2() -> Result<(), &'static str> {
    let value = BurnChecked {
        amount: u64::MAX,
        decimals: u8::MAX,
    };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("burnChecked-2", &instruction)?;
    assert_eq!(
        BurnChecked::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn initialize_account2_case2() -> Result<(), &'static str> {
    let value = InitializeAccount2 {
        owner: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
            .parse()
            .map_err(|_| "Token operation failed")?,
    };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("initializeAccount2-2", &instruction)?;
    assert_eq!(
        InitializeAccount2::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn sync_native_case2() -> Result<(), &'static str> {
    let value = SyncNative {};
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("syncNative-2", &instruction)?;
    assert_eq!(
        SyncNative::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn initialize_account3_case2() -> Result<(), &'static str> {
    let value = InitializeAccount3 {
        owner: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
            .parse()
            .map_err(|_| "Token operation failed")?,
    };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("initializeAccount3-2", &instruction)?;
    assert_eq!(
        InitializeAccount3::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn initialize_multisig2_case2() -> Result<(), &'static str> {
    let value = InitializeMultisig2 { m: u8::MAX };
    let accounts = vec![AccountMeta {
        address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
            .parse()
            .map_err(|_| "Token operation failed")?,
        role: AccountRole::from_privileges(false, true),
    }];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("initializeMultisig2-2", &instruction)?;
    assert_eq!(
        InitializeMultisig2::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn initialize_mint2_case2() -> Result<(), &'static str> {
    let value = InitializeMint2 {
        decimals: u8::MAX,
        mint_authority: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
            .parse()
            .map_err(|_| "Token operation failed")?,
        freeze_authority: Some(
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
        ),
    };
    let accounts = vec![AccountMeta {
        address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
            .parse()
            .map_err(|_| "Token operation failed")?,
        role: AccountRole::from_privileges(false, true),
    }];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("initializeMint2-2", &instruction)?;
    assert_eq!(
        InitializeMint2::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn get_account_data_size_case2() -> Result<(), &'static str> {
    let value = GetAccountDataSize {};
    let accounts = vec![AccountMeta {
        address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
            .parse()
            .map_err(|_| "Token operation failed")?,
        role: AccountRole::from_privileges(false, false),
    }];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("getAccountDataSize-2", &instruction)?;
    assert_eq!(
        GetAccountDataSize::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn initialize_immutable_owner_case2() -> Result<(), &'static str> {
    let value = InitializeImmutableOwner {};
    let accounts = vec![AccountMeta {
        address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
            .parse()
            .map_err(|_| "Token operation failed")?,
        role: AccountRole::from_privileges(false, true),
    }];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("initializeImmutableOwner-2", &instruction)?;
    assert_eq!(
        InitializeImmutableOwner::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn amount_to_ui_amount_case2() -> Result<(), &'static str> {
    let value = AmountToUiAmount { amount: u64::MAX };
    let accounts = vec![AccountMeta {
        address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
            .parse()
            .map_err(|_| "Token operation failed")?,
        role: AccountRole::from_privileges(false, false),
    }];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("amountToUiAmount-2", &instruction)?;
    assert_eq!(
        AmountToUiAmount::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn ui_amount_to_amount_case2() -> Result<(), &'static str> {
    let value = UiAmountToAmount {
        ui_amount: "\u{feff}18446744073709551615.000000001".to_string(),
    };
    let accounts = vec![AccountMeta {
        address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
            .parse()
            .map_err(|_| "Token operation failed")?,
        role: AccountRole::from_privileges(false, false),
    }];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("uiAmountToAmount-2", &instruction)?;
    assert_eq!(
        UiAmountToAmount::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn withdraw_excess_lamports_case2() -> Result<(), &'static str> {
    let value = WithdrawExcessLamports {};
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("withdrawExcessLamports-2", &instruction)?;
    assert_eq!(
        WithdrawExcessLamports::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn unwrap_lamports_case2() -> Result<(), &'static str> {
    let value = UnwrapLamports {
        amount: Some(u64::MAX),
    };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .map_err(|_| "Token operation failed")?,
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("unwrapLamports-2", &instruction)?;
    assert_eq!(
        UnwrapLamports::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn batch_case2() -> Result<(), &'static str> {
    let value = Batch {
        data: vec![
            BatchDataItem {
                number_of_accounts: u8::MAX,
                instruction_data: vec![u8::MAX; 255],
            },
            BatchDataItem {
                number_of_accounts: u8::MAX,
                instruction_data: vec![u8::MAX; 255],
            },
        ],
    };
    let accounts = vec![];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .map_err(|_| "Token operation failed")?;
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .map_err(|_| "Token operation failed")?;
    support::instruction("batch-2", &instruction)?;
    assert_eq!(
        Batch::parse_with_program(program, &instruction, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
    Ok(())
}
fn account_mint_case2() -> Result<(), &'static str> {
    let value = Mint {
        mint_authority: Some(
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
        ),
        supply: u64::MAX,
        decimals: u8::MAX,
        is_initialized: true,
        freeze_authority: Some(
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
        ),
    };
    let bytes = MintCodec
        .encode(&value, 4096)
        .map_err(|_| "Token operation failed")?;
    support::bytes("account-mint-2", &bytes)?;
    assert_eq!(
        MintCodec
            .decode(&bytes, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    support::fixed_negatives(&MintCodec, &bytes)?;
    Ok(())
}
fn account_token_case2() -> Result<(), &'static str> {
    let value = Token {
        mint: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
            .parse()
            .map_err(|_| "Token operation failed")?,
        owner: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
            .parse()
            .map_err(|_| "Token operation failed")?,
        amount: u64::MAX,
        delegate: Some(
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
        ),
        state: AccountState::Frozen,
        is_native: Some(u64::MAX),
        delegated_amount: u64::MAX,
        close_authority: Some(
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
        ),
    };
    let bytes = TokenCodec
        .encode(&value, 4096)
        .map_err(|_| "Token operation failed")?;
    support::bytes("account-token-2", &bytes)?;
    assert_eq!(
        TokenCodec
            .decode(&bytes, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    support::fixed_negatives(&TokenCodec, &bytes)?;
    Ok(())
}
fn account_multisig_case2() -> Result<(), &'static str> {
    let value = Multisig {
        m: u8::MAX,
        n: u8::MAX,
        is_initialized: true,
        signers: vec![
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .map_err(|_| "Token operation failed")?,
        ],
    };
    let bytes = MultisigCodec
        .encode(&value, 4096)
        .map_err(|_| "Token operation failed")?;
    support::bytes("account-multisig-2", &bytes)?;
    assert_eq!(
        MultisigCodec
            .decode(&bytes, LIMITS)
            .map_err(|_| "Token operation failed")?,
        value
    );
    support::fixed_negatives(&MultisigCodec, &bytes)?;
    Ok(())
}
pub(crate) fn run() -> Result<(), &'static str> {
    initialize_mint_case0()?;
    initialize_account_case0()?;
    initialize_multisig_case0()?;
    transfer_case0()?;
    approve_case0()?;
    revoke_case0()?;
    set_authority_case0()?;
    mint_to_case0()?;
    burn_case0()?;
    close_account_case0()?;
    freeze_account_case0()?;
    thaw_account_case0()?;
    transfer_checked_case0()?;
    approve_checked_case0()?;
    mint_to_checked_case0()?;
    burn_checked_case0()?;
    initialize_account2_case0()?;
    sync_native_case0()?;
    initialize_account3_case0()?;
    initialize_multisig2_case0()?;
    initialize_mint2_case0()?;
    get_account_data_size_case0()?;
    initialize_immutable_owner_case0()?;
    amount_to_ui_amount_case0()?;
    ui_amount_to_amount_case0()?;
    withdraw_excess_lamports_case0()?;
    unwrap_lamports_case0()?;
    batch_case0()?;
    account_mint_case0()?;
    account_token_case0()?;
    account_multisig_case0()?;
    initialize_mint_case1()?;
    initialize_account_case1()?;
    initialize_multisig_case1()?;
    transfer_case1()?;
    approve_case1()?;
    revoke_case1()?;
    set_authority_case1()?;
    mint_to_case1()?;
    burn_case1()?;
    close_account_case1()?;
    freeze_account_case1()?;
    thaw_account_case1()?;
    transfer_checked_case1()?;
    approve_checked_case1()?;
    mint_to_checked_case1()?;
    burn_checked_case1()?;
    initialize_account2_case1()?;
    sync_native_case1()?;
    initialize_account3_case1()?;
    initialize_multisig2_case1()?;
    initialize_mint2_case1()?;
    get_account_data_size_case1()?;
    initialize_immutable_owner_case1()?;
    amount_to_ui_amount_case1()?;
    ui_amount_to_amount_case1()?;
    withdraw_excess_lamports_case1()?;
    unwrap_lamports_case1()?;
    batch_case1()?;
    account_mint_case1()?;
    account_token_case1()?;
    account_multisig_case1()?;
    initialize_mint_case2()?;
    initialize_account_case2()?;
    initialize_multisig_case2()?;
    transfer_case2()?;
    approve_case2()?;
    revoke_case2()?;
    set_authority_case2()?;
    mint_to_case2()?;
    burn_case2()?;
    close_account_case2()?;
    freeze_account_case2()?;
    thaw_account_case2()?;
    transfer_checked_case2()?;
    approve_checked_case2()?;
    mint_to_checked_case2()?;
    burn_checked_case2()?;
    initialize_account2_case2()?;
    sync_native_case2()?;
    initialize_account3_case2()?;
    initialize_multisig2_case2()?;
    initialize_mint2_case2()?;
    get_account_data_size_case2()?;
    initialize_immutable_owner_case2()?;
    amount_to_ui_amount_case2()?;
    ui_amount_to_amount_case2()?;
    withdraw_excess_lamports_case2()?;
    unwrap_lamports_case2()?;
    batch_case2()?;
    account_mint_case2()?;
    account_token_case2()?;
    account_multisig_case2()?;
    Ok(())
}
