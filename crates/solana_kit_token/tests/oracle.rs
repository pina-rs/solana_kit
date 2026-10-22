//! Executed original Dart constructor and account byte oracles.
use solana_kit_core::{
    AccountMeta, AccountRole, Address,
    codecs::{Decoder, Encoder, Limits},
};
use solana_kit_token::*;
mod support;
const LIMITS: Limits = Limits::new(4096, 4096);
const NONZERO_BYTE: u8 = 7;
#[test]
fn initialize_mint_case0() {
    let value = InitializeMint {
        decimals: 0,
        mint_authority: "11111111111111111111111111111111".parse().unwrap(),
        freeze_authority: None,
    };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("initializeMint-0", &instruction);
    assert_eq!(
        InitializeMint::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn initialize_account_case0() {
    let value = InitializeAccount {};
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("initializeAccount-0", &instruction);
    assert_eq!(
        InitializeAccount::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn initialize_multisig_case0() {
    let value = InitializeMultisig { m: 0 };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("initializeMultisig-0", &instruction);
    assert_eq!(
        InitializeMultisig::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn transfer_case0() {
    let value = Transfer { amount: 0 };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("transfer-0", &instruction);
    assert_eq!(
        Transfer::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn approve_case0() {
    let value = Approve { amount: 0 };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("approve-0", &instruction);
    assert_eq!(
        Approve::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn revoke_case0() {
    let value = Revoke {};
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("revoke-0", &instruction);
    assert_eq!(
        Revoke::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn set_authority_case0() {
    let value = SetAuthority {
        authority_type: AuthorityType::MintTokens,
        new_authority: None,
    };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("setAuthority-0", &instruction);
    assert_eq!(
        SetAuthority::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn mint_to_case0() {
    let value = MintTo { amount: 0 };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("mintTo-0", &instruction);
    assert_eq!(
        MintTo::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn burn_case0() {
    let value = Burn { amount: 0 };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("burn-0", &instruction);
    assert_eq!(
        Burn::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn close_account_case0() {
    let value = CloseAccount {};
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("closeAccount-0", &instruction);
    assert_eq!(
        CloseAccount::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn freeze_account_case0() {
    let value = FreezeAccount {};
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("freezeAccount-0", &instruction);
    assert_eq!(
        FreezeAccount::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn thaw_account_case0() {
    let value = ThawAccount {};
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("thawAccount-0", &instruction);
    assert_eq!(
        ThawAccount::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn transfer_checked_case0() {
    let value = TransferChecked {
        amount: 0,
        decimals: 0,
    };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("transferChecked-0", &instruction);
    assert_eq!(
        TransferChecked::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn approve_checked_case0() {
    let value = ApproveChecked {
        amount: 0,
        decimals: 0,
    };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("approveChecked-0", &instruction);
    assert_eq!(
        ApproveChecked::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn mint_to_checked_case0() {
    let value = MintToChecked {
        amount: 0,
        decimals: 0,
    };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("mintToChecked-0", &instruction);
    assert_eq!(
        MintToChecked::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn burn_checked_case0() {
    let value = BurnChecked {
        amount: 0,
        decimals: 0,
    };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("burnChecked-0", &instruction);
    assert_eq!(
        BurnChecked::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn initialize_account2_case0() {
    let value = InitializeAccount2 {
        owner: "11111111111111111111111111111111".parse().unwrap(),
    };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("initializeAccount2-0", &instruction);
    assert_eq!(
        InitializeAccount2::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn sync_native_case0() {
    let value = SyncNative {};
    let accounts = vec![AccountMeta {
        address: "11111111111111111111111111111111".parse().unwrap(),
        role: AccountRole::from_privileges(false, true),
    }];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("syncNative-0", &instruction);
    assert_eq!(
        SyncNative::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn initialize_account3_case0() {
    let value = InitializeAccount3 {
        owner: "11111111111111111111111111111111".parse().unwrap(),
    };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("initializeAccount3-0", &instruction);
    assert_eq!(
        InitializeAccount3::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn initialize_multisig2_case0() {
    let value = InitializeMultisig2 { m: 0 };
    let accounts = vec![AccountMeta {
        address: "11111111111111111111111111111111".parse().unwrap(),
        role: AccountRole::from_privileges(false, true),
    }];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("initializeMultisig2-0", &instruction);
    assert_eq!(
        InitializeMultisig2::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn initialize_mint2_case0() {
    let value = InitializeMint2 {
        decimals: 0,
        mint_authority: "11111111111111111111111111111111".parse().unwrap(),
        freeze_authority: None,
    };
    let accounts = vec![AccountMeta {
        address: "11111111111111111111111111111111".parse().unwrap(),
        role: AccountRole::from_privileges(false, true),
    }];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("initializeMint2-0", &instruction);
    assert_eq!(
        InitializeMint2::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn get_account_data_size_case0() {
    let value = GetAccountDataSize {};
    let accounts = vec![AccountMeta {
        address: "11111111111111111111111111111111".parse().unwrap(),
        role: AccountRole::from_privileges(false, false),
    }];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("getAccountDataSize-0", &instruction);
    assert_eq!(
        GetAccountDataSize::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn initialize_immutable_owner_case0() {
    let value = InitializeImmutableOwner {};
    let accounts = vec![AccountMeta {
        address: "11111111111111111111111111111111".parse().unwrap(),
        role: AccountRole::from_privileges(false, true),
    }];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("initializeImmutableOwner-0", &instruction);
    assert_eq!(
        InitializeImmutableOwner::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn amount_to_ui_amount_case0() {
    let value = AmountToUiAmount { amount: 0 };
    let accounts = vec![AccountMeta {
        address: "11111111111111111111111111111111".parse().unwrap(),
        role: AccountRole::from_privileges(false, false),
    }];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("amountToUiAmount-0", &instruction);
    assert_eq!(
        AmountToUiAmount::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn ui_amount_to_amount_case0() {
    let value = UiAmountToAmount {
        ui_amount: String::new(),
    };
    let accounts = vec![AccountMeta {
        address: "11111111111111111111111111111111".parse().unwrap(),
        role: AccountRole::from_privileges(false, false),
    }];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("uiAmountToAmount-0", &instruction);
    assert_eq!(
        UiAmountToAmount::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn withdraw_excess_lamports_case0() {
    let value = WithdrawExcessLamports {};
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("withdrawExcessLamports-0", &instruction);
    assert_eq!(
        WithdrawExcessLamports::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn unwrap_lamports_case0() {
    let value = UnwrapLamports { amount: None };
    let accounts = vec![
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("unwrapLamports-0", &instruction);
    assert_eq!(
        UnwrapLamports::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn batch_case0() {
    let value = Batch { data: vec![] };
    let accounts = vec![];
    let program: Address = "11111111111111111111111111111111".parse().unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("batch-0", &instruction);
    assert_eq!(
        Batch::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn account_mint_case0() {
    let value = Mint {
        mint_authority: None,
        supply: 0,
        decimals: 0,
        is_initialized: false,
        freeze_authority: None,
    };
    let bytes = MintCodec.encode(&value, 4096).unwrap();
    support::bytes("account-mint-0", &bytes);
    assert_eq!(MintCodec.decode(&bytes, LIMITS).unwrap(), value);
    support::fixed_negatives(&MintCodec, &bytes);
}
#[test]
fn account_token_case0() {
    let value = Token {
        mint: "11111111111111111111111111111111".parse().unwrap(),
        owner: "11111111111111111111111111111111".parse().unwrap(),
        amount: 0,
        delegate: None,
        state: AccountState::Uninitialized,
        is_native: None,
        delegated_amount: 0,
        close_authority: None,
    };
    let bytes = TokenCodec.encode(&value, 4096).unwrap();
    support::bytes("account-token-0", &bytes);
    assert_eq!(TokenCodec.decode(&bytes, LIMITS).unwrap(), value);
    support::fixed_negatives(&TokenCodec, &bytes);
}
#[test]
fn account_multisig_case0() {
    let value = Multisig {
        m: 0,
        n: 0,
        is_initialized: false,
        signers: vec![
            "11111111111111111111111111111111".parse().unwrap(),
            "11111111111111111111111111111111".parse().unwrap(),
            "11111111111111111111111111111111".parse().unwrap(),
            "11111111111111111111111111111111".parse().unwrap(),
            "11111111111111111111111111111111".parse().unwrap(),
            "11111111111111111111111111111111".parse().unwrap(),
            "11111111111111111111111111111111".parse().unwrap(),
            "11111111111111111111111111111111".parse().unwrap(),
            "11111111111111111111111111111111".parse().unwrap(),
            "11111111111111111111111111111111".parse().unwrap(),
            "11111111111111111111111111111111".parse().unwrap(),
        ],
    };
    let bytes = MultisigCodec.encode(&value, 4096).unwrap();
    support::bytes("account-multisig-0", &bytes);
    assert_eq!(MultisigCodec.decode(&bytes, LIMITS).unwrap(), value);
    support::fixed_negatives(&MultisigCodec, &bytes);
}
#[test]
fn initialize_mint_case1() {
    let value = InitializeMint {
        decimals: NONZERO_BYTE,
        mint_authority: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
            .parse()
            .unwrap(),
        freeze_authority: Some(
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
        ),
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("initializeMint-1", &instruction);
    assert_eq!(
        InitializeMint::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn initialize_account_case1() {
    let value = InitializeAccount {};
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("initializeAccount-1", &instruction);
    assert_eq!(
        InitializeAccount::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn initialize_multisig_case1() {
    let value = InitializeMultisig { m: NONZERO_BYTE };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("initializeMultisig-1", &instruction);
    assert_eq!(
        InitializeMultisig::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn transfer_case1() {
    let value = Transfer {
        amount: u64::from(NONZERO_BYTE),
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("transfer-1", &instruction);
    assert_eq!(
        Transfer::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn approve_case1() {
    let value = Approve {
        amount: u64::from(NONZERO_BYTE),
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("approve-1", &instruction);
    assert_eq!(
        Approve::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn revoke_case1() {
    let value = Revoke {};
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("revoke-1", &instruction);
    assert_eq!(
        Revoke::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn set_authority_case1() {
    let value = SetAuthority {
        authority_type: AuthorityType::FreezeAccount,
        new_authority: Some(
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
        ),
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("setAuthority-1", &instruction);
    assert_eq!(
        SetAuthority::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn mint_to_case1() {
    let value = MintTo {
        amount: u64::from(NONZERO_BYTE),
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("mintTo-1", &instruction);
    assert_eq!(
        MintTo::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn burn_case1() {
    let value = Burn {
        amount: u64::from(NONZERO_BYTE),
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("burn-1", &instruction);
    assert_eq!(
        Burn::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn close_account_case1() {
    let value = CloseAccount {};
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("closeAccount-1", &instruction);
    assert_eq!(
        CloseAccount::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn freeze_account_case1() {
    let value = FreezeAccount {};
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("freezeAccount-1", &instruction);
    assert_eq!(
        FreezeAccount::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn thaw_account_case1() {
    let value = ThawAccount {};
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("thawAccount-1", &instruction);
    assert_eq!(
        ThawAccount::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn transfer_checked_case1() {
    let value = TransferChecked {
        amount: u64::from(NONZERO_BYTE),
        decimals: NONZERO_BYTE,
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("transferChecked-1", &instruction);
    assert_eq!(
        TransferChecked::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn approve_checked_case1() {
    let value = ApproveChecked {
        amount: u64::from(NONZERO_BYTE),
        decimals: NONZERO_BYTE,
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("approveChecked-1", &instruction);
    assert_eq!(
        ApproveChecked::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn mint_to_checked_case1() {
    let value = MintToChecked {
        amount: u64::from(NONZERO_BYTE),
        decimals: NONZERO_BYTE,
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("mintToChecked-1", &instruction);
    assert_eq!(
        MintToChecked::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn burn_checked_case1() {
    let value = BurnChecked {
        amount: u64::from(NONZERO_BYTE),
        decimals: NONZERO_BYTE,
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("burnChecked-1", &instruction);
    assert_eq!(
        BurnChecked::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn initialize_account2_case1() {
    let value = InitializeAccount2 {
        owner: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
            .parse()
            .unwrap(),
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("initializeAccount2-1", &instruction);
    assert_eq!(
        InitializeAccount2::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn sync_native_case1() {
    let value = SyncNative {};
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("syncNative-1", &instruction);
    assert_eq!(
        SyncNative::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn initialize_account3_case1() {
    let value = InitializeAccount3 {
        owner: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
            .parse()
            .unwrap(),
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("initializeAccount3-1", &instruction);
    assert_eq!(
        InitializeAccount3::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn initialize_multisig2_case1() {
    let value = InitializeMultisig2 { m: NONZERO_BYTE };
    let accounts = vec![AccountMeta {
        address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
            .parse()
            .unwrap(),
        role: AccountRole::from_privileges(false, true),
    }];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("initializeMultisig2-1", &instruction);
    assert_eq!(
        InitializeMultisig2::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn initialize_mint2_case1() {
    let value = InitializeMint2 {
        decimals: NONZERO_BYTE,
        mint_authority: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
            .parse()
            .unwrap(),
        freeze_authority: Some(
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
        ),
    };
    let accounts = vec![AccountMeta {
        address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
            .parse()
            .unwrap(),
        role: AccountRole::from_privileges(false, true),
    }];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("initializeMint2-1", &instruction);
    assert_eq!(
        InitializeMint2::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn get_account_data_size_case1() {
    let value = GetAccountDataSize {};
    let accounts = vec![AccountMeta {
        address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
            .parse()
            .unwrap(),
        role: AccountRole::from_privileges(false, false),
    }];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("getAccountDataSize-1", &instruction);
    assert_eq!(
        GetAccountDataSize::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn initialize_immutable_owner_case1() {
    let value = InitializeImmutableOwner {};
    let accounts = vec![AccountMeta {
        address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
            .parse()
            .unwrap(),
        role: AccountRole::from_privileges(false, true),
    }];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("initializeImmutableOwner-1", &instruction);
    assert_eq!(
        InitializeImmutableOwner::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn amount_to_ui_amount_case1() {
    let value = AmountToUiAmount {
        amount: u64::from(NONZERO_BYTE),
    };
    let accounts = vec![AccountMeta {
        address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
            .parse()
            .unwrap(),
        role: AccountRole::from_privileges(false, false),
    }];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("amountToUiAmount-1", &instruction);
    assert_eq!(
        AmountToUiAmount::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn ui_amount_to_amount_case1() {
    let value = UiAmountToAmount {
        ui_amount: "42.🦀\0".to_owned(),
    };
    let accounts = vec![AccountMeta {
        address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
            .parse()
            .unwrap(),
        role: AccountRole::from_privileges(false, false),
    }];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("uiAmountToAmount-1", &instruction);
    assert_eq!(
        UiAmountToAmount::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn withdraw_excess_lamports_case1() {
    let value = WithdrawExcessLamports {};
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("withdrawExcessLamports-1", &instruction);
    assert_eq!(
        WithdrawExcessLamports::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn unwrap_lamports_case1() {
    let value = UnwrapLamports {
        amount: Some(u64::from(NONZERO_BYTE)),
    };
    let accounts = vec![
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("unwrapLamports-1", &instruction);
    assert_eq!(
        UnwrapLamports::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn batch_case1() {
    let value = Batch {
        data: vec![BatchDataItem {
            number_of_accounts: NONZERO_BYTE,
            instruction_data: vec![u8::MAX; 3],
        }],
    };
    let accounts = vec![];
    let program: Address = "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("batch-1", &instruction);
    assert_eq!(
        Batch::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn account_mint_case1() {
    let value = Mint {
        mint_authority: Some(
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
        ),
        supply: u64::from(NONZERO_BYTE),
        decimals: NONZERO_BYTE,
        is_initialized: true,
        freeze_authority: Some(
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
        ),
    };
    let bytes = MintCodec.encode(&value, 4096).unwrap();
    support::bytes("account-mint-1", &bytes);
    assert_eq!(MintCodec.decode(&bytes, LIMITS).unwrap(), value);
    support::fixed_negatives(&MintCodec, &bytes);
}
#[test]
fn account_token_case1() {
    let value = Token {
        mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
            .parse()
            .unwrap(),
        owner: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
            .parse()
            .unwrap(),
        amount: u64::from(NONZERO_BYTE),
        delegate: Some(
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
        ),
        state: AccountState::Initialized,
        is_native: Some(u64::from(NONZERO_BYTE)),
        delegated_amount: u64::from(NONZERO_BYTE),
        close_authority: Some(
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
        ),
    };
    let bytes = TokenCodec.encode(&value, 4096).unwrap();
    support::bytes("account-token-1", &bytes);
    assert_eq!(TokenCodec.decode(&bytes, LIMITS).unwrap(), value);
    support::fixed_negatives(&TokenCodec, &bytes);
}
#[test]
fn account_multisig_case1() {
    let value = Multisig {
        m: NONZERO_BYTE,
        n: NONZERO_BYTE,
        is_initialized: true,
        signers: vec![
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
        ],
    };
    let bytes = MultisigCodec.encode(&value, 4096).unwrap();
    support::bytes("account-multisig-1", &bytes);
    assert_eq!(MultisigCodec.decode(&bytes, LIMITS).unwrap(), value);
    support::fixed_negatives(&MultisigCodec, &bytes);
}
#[test]
fn initialize_mint_case2() {
    let value = InitializeMint {
        decimals: u8::MAX,
        mint_authority: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
            .parse()
            .unwrap(),
        freeze_authority: Some(
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
        ),
    };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("initializeMint-2", &instruction);
    assert_eq!(
        InitializeMint::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn initialize_account_case2() {
    let value = InitializeAccount {};
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("initializeAccount-2", &instruction);
    assert_eq!(
        InitializeAccount::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn initialize_multisig_case2() {
    let value = InitializeMultisig { m: u8::MAX };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("initializeMultisig-2", &instruction);
    assert_eq!(
        InitializeMultisig::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn transfer_case2() {
    let value = Transfer { amount: u64::MAX };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("transfer-2", &instruction);
    assert_eq!(
        Transfer::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn approve_case2() {
    let value = Approve { amount: u64::MAX };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("approve-2", &instruction);
    assert_eq!(
        Approve::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn revoke_case2() {
    let value = Revoke {};
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("revoke-2", &instruction);
    assert_eq!(
        Revoke::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn set_authority_case2() {
    let value = SetAuthority {
        authority_type: AuthorityType::CloseAccount,
        new_authority: Some(
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
        ),
    };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("setAuthority-2", &instruction);
    assert_eq!(
        SetAuthority::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn mint_to_case2() {
    let value = MintTo { amount: u64::MAX };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("mintTo-2", &instruction);
    assert_eq!(
        MintTo::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn burn_case2() {
    let value = Burn { amount: u64::MAX };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("burn-2", &instruction);
    assert_eq!(
        Burn::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn close_account_case2() {
    let value = CloseAccount {};
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("closeAccount-2", &instruction);
    assert_eq!(
        CloseAccount::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn freeze_account_case2() {
    let value = FreezeAccount {};
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("freezeAccount-2", &instruction);
    assert_eq!(
        FreezeAccount::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn thaw_account_case2() {
    let value = ThawAccount {};
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("thawAccount-2", &instruction);
    assert_eq!(
        ThawAccount::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn transfer_checked_case2() {
    let value = TransferChecked {
        amount: u64::MAX,
        decimals: u8::MAX,
    };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("transferChecked-2", &instruction);
    assert_eq!(
        TransferChecked::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn approve_checked_case2() {
    let value = ApproveChecked {
        amount: u64::MAX,
        decimals: u8::MAX,
    };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("approveChecked-2", &instruction);
    assert_eq!(
        ApproveChecked::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn mint_to_checked_case2() {
    let value = MintToChecked {
        amount: u64::MAX,
        decimals: u8::MAX,
    };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("mintToChecked-2", &instruction);
    assert_eq!(
        MintToChecked::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn burn_checked_case2() {
    let value = BurnChecked {
        amount: u64::MAX,
        decimals: u8::MAX,
    };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("burnChecked-2", &instruction);
    assert_eq!(
        BurnChecked::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn initialize_account2_case2() {
    let value = InitializeAccount2 {
        owner: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
            .parse()
            .unwrap(),
    };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("initializeAccount2-2", &instruction);
    assert_eq!(
        InitializeAccount2::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn sync_native_case2() {
    let value = SyncNative {};
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("syncNative-2", &instruction);
    assert_eq!(
        SyncNative::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn initialize_account3_case2() {
    let value = InitializeAccount3 {
        owner: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
            .parse()
            .unwrap(),
    };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("initializeAccount3-2", &instruction);
    assert_eq!(
        InitializeAccount3::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn initialize_multisig2_case2() {
    let value = InitializeMultisig2 { m: u8::MAX };
    let accounts = vec![AccountMeta {
        address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
            .parse()
            .unwrap(),
        role: AccountRole::from_privileges(false, true),
    }];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("initializeMultisig2-2", &instruction);
    assert_eq!(
        InitializeMultisig2::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn initialize_mint2_case2() {
    let value = InitializeMint2 {
        decimals: u8::MAX,
        mint_authority: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
            .parse()
            .unwrap(),
        freeze_authority: Some(
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
        ),
    };
    let accounts = vec![AccountMeta {
        address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
            .parse()
            .unwrap(),
        role: AccountRole::from_privileges(false, true),
    }];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("initializeMint2-2", &instruction);
    assert_eq!(
        InitializeMint2::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn get_account_data_size_case2() {
    let value = GetAccountDataSize {};
    let accounts = vec![AccountMeta {
        address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
            .parse()
            .unwrap(),
        role: AccountRole::from_privileges(false, false),
    }];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("getAccountDataSize-2", &instruction);
    assert_eq!(
        GetAccountDataSize::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn initialize_immutable_owner_case2() {
    let value = InitializeImmutableOwner {};
    let accounts = vec![AccountMeta {
        address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
            .parse()
            .unwrap(),
        role: AccountRole::from_privileges(false, true),
    }];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("initializeImmutableOwner-2", &instruction);
    assert_eq!(
        InitializeImmutableOwner::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn amount_to_ui_amount_case2() {
    let value = AmountToUiAmount { amount: u64::MAX };
    let accounts = vec![AccountMeta {
        address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
            .parse()
            .unwrap(),
        role: AccountRole::from_privileges(false, false),
    }];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("amountToUiAmount-2", &instruction);
    assert_eq!(
        AmountToUiAmount::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn ui_amount_to_amount_case2() {
    let value = UiAmountToAmount {
        ui_amount: "\u{feff}18446744073709551615.000000001".to_owned(),
    };
    let accounts = vec![AccountMeta {
        address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
            .parse()
            .unwrap(),
        role: AccountRole::from_privileges(false, false),
    }];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("uiAmountToAmount-2", &instruction);
    assert_eq!(
        UiAmountToAmount::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn withdraw_excess_lamports_case2() {
    let value = WithdrawExcessLamports {};
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("withdrawExcessLamports-2", &instruction);
    assert_eq!(
        WithdrawExcessLamports::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn unwrap_lamports_case2() {
    let value = UnwrapLamports {
        amount: Some(u64::MAX),
    };
    let accounts = vec![
        AccountMeta {
            address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true),
        },
        AccountMeta {
            address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                .parse()
                .unwrap(),
            role: AccountRole::from_privileges(true, false),
        },
    ];
    let program: Address = "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
        .parse()
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("unwrapLamports-2", &instruction);
    assert_eq!(
        UnwrapLamports::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn batch_case2() {
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
        .unwrap();
    let instruction = value
        .instruction_with_program(program, &accounts, LIMITS)
        .unwrap();
    support::instruction("batch-2", &instruction);
    assert_eq!(
        Batch::parse_with_program(program, &instruction, LIMITS).unwrap(),
        value
    );
    assert!(parse_instruction_with_program(program, &instruction, LIMITS).is_ok());
}
#[test]
fn account_mint_case2() {
    let value = Mint {
        mint_authority: Some(
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
        ),
        supply: u64::MAX,
        decimals: u8::MAX,
        is_initialized: true,
        freeze_authority: Some(
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
        ),
    };
    let bytes = MintCodec.encode(&value, 4096).unwrap();
    support::bytes("account-mint-2", &bytes);
    assert_eq!(MintCodec.decode(&bytes, LIMITS).unwrap(), value);
    support::fixed_negatives(&MintCodec, &bytes);
}
#[test]
fn account_token_case2() {
    let value = Token {
        mint: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
            .parse()
            .unwrap(),
        owner: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
            .parse()
            .unwrap(),
        amount: u64::MAX,
        delegate: Some(
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
        ),
        state: AccountState::Frozen,
        is_native: Some(u64::MAX),
        delegated_amount: u64::MAX,
        close_authority: Some(
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
        ),
    };
    let bytes = TokenCodec.encode(&value, 4096).unwrap();
    support::bytes("account-token-2", &bytes);
    assert_eq!(TokenCodec.decode(&bytes, LIMITS).unwrap(), value);
    support::fixed_negatives(&TokenCodec, &bytes);
}
#[test]
fn account_multisig_case2() {
    let value = Multisig {
        m: u8::MAX,
        n: u8::MAX,
        is_initialized: true,
        signers: vec![
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
            "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                .parse()
                .unwrap(),
        ],
    };
    let bytes = MultisigCodec.encode(&value, 4096).unwrap();
    support::bytes("account-multisig-2", &bytes);
    assert_eq!(MultisigCodec.decode(&bytes, LIMITS).unwrap(), value);
    support::fixed_negatives(&MultisigCodec, &bytes);
}
