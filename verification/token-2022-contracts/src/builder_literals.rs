//! Independent original Dart builder literals.
use solana_kit_core::{
    AccountMeta, AccountRole, Address,
    codecs::{Decoder, Limits},
};
use solana_kit_token_2022::*;
const LIMITS: Limits = Limits::new(100_000, 4096);
pub(crate) fn initialize_mint_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::InitializeMint(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializeMintAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                rent: Some(
                    "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                ),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            }
        ]
    );
}
pub(crate) fn initialize_account_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [1];

    let Token2022InstructionData::InitializeAccount(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializeAccountAccounts {
                account: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                owner: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse::<Address>()
                    .unwrap(),
                rent: Some(
                    "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                        .parse::<Address>()
                        .unwrap(),
                ),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            }
        ]
    );
}
pub(crate) fn initialize_multisig_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [2, 0];

    let Token2022InstructionData::InitializeMultisig(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializeMultisigAccounts {
                multisig: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                rent: Some(
                    "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                ),
                signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            }
        ]
    );
}
pub(crate) fn transfer_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [3, 0, 0, 0, 0, 0, 0, 0, 0];

    let Token2022InstructionData::Transfer(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            TransferAccounts {
                source: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                destination: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                authority: Authority {
                    address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn approve_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [4, 0, 0, 0, 0, 0, 0, 0, 0];

    let Token2022InstructionData::Approve(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            ApproveAccounts {
                source: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                delegate: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                owner: Authority {
                    address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn revoke_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [5];

    let Token2022InstructionData::Revoke(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            RevokeAccounts {
                source: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                owner: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn set_authority_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [6, 0, 0];

    let Token2022InstructionData::SetAuthority(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            SetAuthorityAccounts {
                owned: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                owner: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn mint_to_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [7, 0, 0, 0, 0, 0, 0, 0, 0];

    let Token2022InstructionData::MintTo(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            MintToAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                token: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                mint_authority: Authority {
                    address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn burn_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [8, 0, 0, 0, 0, 0, 0, 0, 0];

    let Token2022InstructionData::Burn(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            BurnAccounts {
                account: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                authority: Authority {
                    address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn close_account_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [9];

    let Token2022InstructionData::CloseAccount(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            CloseAccountAccounts {
                account: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                destination: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                owner: Authority {
                    address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn freeze_account_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [10];

    let Token2022InstructionData::FreezeAccount(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            FreezeAccountAccounts {
                account: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                owner: Authority {
                    address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn thaw_account_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [11];

    let Token2022InstructionData::ThawAccount(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            ThawAccountAccounts {
                account: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                owner: Authority {
                    address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn transfer_checked_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [12, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let Token2022InstructionData::TransferChecked(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            TransferCheckedAccounts {
                source: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                destination: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse::<Address>()
                    .unwrap(),
                authority: Authority {
                    address: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn approve_checked_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [13, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let Token2022InstructionData::ApproveChecked(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            ApproveCheckedAccounts {
                source: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                delegate: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse::<Address>()
                    .unwrap(),
                owner: Authority {
                    address: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn mint_to_checked_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [14, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let Token2022InstructionData::MintToChecked(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            MintToCheckedAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                token: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                mint_authority: Authority {
                    address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn burn_checked_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [15, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let Token2022InstructionData::BurnChecked(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            BurnCheckedAccounts {
                account: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                authority: Authority {
                    address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn initialize_account2_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0,
    ];

    let Token2022InstructionData::InitializeAccount2(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializeAccount2Accounts {
                account: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                rent: Some(
                    "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                        .parse::<Address>()
                        .unwrap(),
                ),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            }
        ]
    );
}
pub(crate) fn sync_native_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [17];

    let Token2022InstructionData::SyncNative(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            SyncNativeAccounts {
                account: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                rent: None,
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true)
        }]
    );
}
pub(crate) fn initialize_account3_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0,
    ];

    let Token2022InstructionData::InitializeAccount3(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializeAccount3Accounts {
                account: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            }
        ]
    );
}
pub(crate) fn initialize_multisig2_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [19, 0];

    let Token2022InstructionData::InitializeMultisig2(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializeMultisig2Accounts {
                multisig: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true)
        }]
    );
}
pub(crate) fn initialize_mint2_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::InitializeMint2(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializeMint2Accounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true)
        }]
    );
}
pub(crate) fn get_account_data_size_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [21];

    let Token2022InstructionData::GetAccountDataSize(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            GetAccountDataSizeAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, false)
        }]
    );
}
pub(crate) fn initialize_immutable_owner_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [22];

    let Token2022InstructionData::InitializeImmutableOwner(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializeImmutableOwnerAccounts {
                account: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true)
        }]
    );
}
pub(crate) fn amount_to_ui_amount_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [23, 0, 0, 0, 0, 0, 0, 0, 0];

    let Token2022InstructionData::AmountToUiAmount(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            AmountToUiAmountAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, false)
        }]
    );
}
pub(crate) fn ui_amount_to_amount_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [24];

    let Token2022InstructionData::UiAmountToAmount(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            UiAmountToAmountAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, false)
        }]
    );
}
pub(crate) fn initialize_mint_close_authority_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [25, 0];

    let Token2022InstructionData::InitializeMintCloseAuthority(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializeMintCloseAuthorityAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true)
        }]
    );
}
pub(crate) fn initialize_transfer_fee_config_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let Token2022InstructionData::InitializeTransferFeeConfig(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializeTransferFeeConfigAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true)
        }]
    );
}
pub(crate) fn transfer_checked_with_fee_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [26, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let Token2022InstructionData::TransferCheckedWithFee(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            TransferCheckedWithFeeAccounts {
                source: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                destination: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse::<Address>()
                    .unwrap(),
                authority: Authority {
                    address: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn withdraw_withheld_tokens_from_mint_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [26, 2];

    let Token2022InstructionData::WithdrawWithheldTokensFromMint(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            WithdrawWithheldTokensFromMintAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                fee_receiver: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                withdraw_withheld_authority: Authority {
                    address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn withdraw_withheld_tokens_from_accounts_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [26, 3, 0];

    let Token2022InstructionData::WithdrawWithheldTokensFromAccounts(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            WithdrawWithheldTokensFromAccountsAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                fee_receiver: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                withdraw_withheld_authority: Authority {
                    address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
                sources: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn harvest_withheld_tokens_to_mint_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [26, 4];

    let Token2022InstructionData::HarvestWithheldTokensToMint(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            HarvestWithheldTokensToMintAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                sources: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true)
        }]
    );
}
pub(crate) fn set_transfer_fee_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [26, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let Token2022InstructionData::SetTransferFee(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            SetTransferFeeAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                transfer_fee_config_authority: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn initialize_confidential_transfer_mint_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::InitializeConfidentialTransferMint(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializeConfidentialTransferMintAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true)
        }]
    );
}
pub(crate) fn update_confidential_transfer_mint_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        27, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::UpdateConfidentialTransferMint(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            UpdateConfidentialTransferMintAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                authority: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn configure_confidential_transfer_account_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        27, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::ConfigureConfidentialTransferAccount(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            ConfigureConfidentialTransferAccountAccounts {
                token: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                instructions_sysvar_or_context_state: Some(
                    "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                        .parse::<Address>()
                        .unwrap(),
                ),
                authority: Authority {
                    address: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn approve_confidential_transfer_account_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [27, 3];

    let Token2022InstructionData::ApproveConfidentialTransferAccount(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            ApproveConfidentialTransferAccountAccounts {
                token: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                authority: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn empty_confidential_transfer_account_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [27, 4, 0];

    let Token2022InstructionData::EmptyConfidentialTransferAccount(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            EmptyConfidentialTransferAccountAccounts {
                token: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                instructions_sysvar_or_context_state: Some(
                    "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                ),
                authority: Authority {
                    address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn confidential_deposit_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [27, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let Token2022InstructionData::ConfidentialDeposit(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            ConfidentialDepositAccounts {
                token: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                authority: Authority {
                    address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn confidential_withdraw_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        27, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::ConfidentialWithdraw(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            ConfidentialWithdrawAccounts {
                token: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                instructions_sysvar: None,
                equality_record: None,
                range_record: None,
                authority: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn confidential_transfer_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        27, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::ConfidentialTransfer(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            ConfidentialTransferAccounts {
                source_token: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                destination_token: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse::<Address>()
                    .unwrap(),
                instructions_sysvar: None,
                equality_record: None,
                ciphertext_validity_record: None,
                range_record: None,
                authority: Authority {
                    address: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn apply_confidential_pending_balance_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        27, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::ApplyConfidentialPendingBalance(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            ApplyConfidentialPendingBalanceAccounts {
                token: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                authority: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn enable_confidential_credits_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [27, 9];

    let Token2022InstructionData::EnableConfidentialCredits(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            EnableConfidentialCreditsAccounts {
                token: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                authority: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn disable_confidential_credits_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [27, 10];

    let Token2022InstructionData::DisableConfidentialCredits(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            DisableConfidentialCreditsAccounts {
                token: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                authority: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn enable_non_confidential_credits_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [27, 11];

    let Token2022InstructionData::EnableNonConfidentialCredits(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            EnableNonConfidentialCreditsAccounts {
                token: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                authority: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn disable_non_confidential_credits_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [27, 12];

    let Token2022InstructionData::DisableNonConfidentialCredits(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            DisableNonConfidentialCreditsAccounts {
                token: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                authority: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn confidential_transfer_with_fee_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        27, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::ConfidentialTransferWithFee(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            ConfidentialTransferWithFeeAccounts {
                source_token: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                destination_token: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse::<Address>()
                    .unwrap(),
                instructions_sysvar: None,
                equality_record: None,
                transfer_amount_ciphertext_validity_record: None,
                fee_sigma_record: None,
                fee_ciphertext_validity_record: None,
                range_record: None,
                authority: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn configure_confidential_transfer_account_with_registry_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [27, 14];

    let Token2022InstructionData::ConfigureConfidentialTransferAccountWithRegistry(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            ConfigureConfidentialTransferAccountWithRegistryAccounts {
                token: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                elgamal_registry: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse::<Address>()
                    .unwrap(),
                payer: None,
                system_program: None,
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            }
        ]
    );
}
pub(crate) fn initialize_default_account_state_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [28, 0, 0];

    let Token2022InstructionData::InitializeDefaultAccountState(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializeDefaultAccountStateAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true)
        }]
    );
}
pub(crate) fn update_default_account_state_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [28, 1, 0];

    let Token2022InstructionData::UpdateDefaultAccountState(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            UpdateDefaultAccountStateAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                freeze_authority: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn reallocate_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [29];

    let Token2022InstructionData::Reallocate(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            ReallocateAccounts {
                token: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                payer: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                system_program: Some(
                    "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                        .parse::<Address>()
                        .unwrap(),
                ),
                owner: Authority {
                    address: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, true)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn enable_memo_transfers_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [30, 0];

    let Token2022InstructionData::EnableMemoTransfers(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            EnableMemoTransfersAccounts {
                token: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                owner: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn disable_memo_transfers_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [30, 1];

    let Token2022InstructionData::DisableMemoTransfers(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            DisableMemoTransfersAccounts {
                token: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                owner: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn create_native_mint_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [31];

    let Token2022InstructionData::CreateNativeMint(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            CreateNativeMintAccounts {
                payer: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                native_mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                system_program: Some(
                    "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                        .parse::<Address>()
                        .unwrap(),
                ),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(true, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            }
        ]
    );
}
pub(crate) fn initialize_non_transferable_mint_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [32];

    let Token2022InstructionData::InitializeNonTransferableMint(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializeNonTransferableMintAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true)
        }]
    );
}
pub(crate) fn initialize_interest_bearing_mint_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::InitializeInterestBearingMint(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializeInterestBearingMintAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true)
        }]
    );
}
pub(crate) fn update_rate_interest_bearing_mint_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [33, 1, 0, 0];

    let Token2022InstructionData::UpdateRateInterestBearingMint(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            UpdateRateInterestBearingMintAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                rate_authority: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, true)
            }
        ]
    );
}
pub(crate) fn enable_cpi_guard_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [34, 0];

    let Token2022InstructionData::EnableCpiGuard(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            EnableCpiGuardAccounts {
                token: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                owner: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn disable_cpi_guard_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [34, 1];

    let Token2022InstructionData::DisableCpiGuard(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            DisableCpiGuardAccounts {
                token: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                owner: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn initialize_permanent_delegate_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0,
    ];

    let Token2022InstructionData::InitializePermanentDelegate(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializePermanentDelegateAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true)
        }]
    );
}
pub(crate) fn initialize_transfer_hook_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::InitializeTransferHook(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializeTransferHookAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true)
        }]
    );
}
pub(crate) fn update_transfer_hook_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        36, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    let Token2022InstructionData::UpdateTransferHook(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            UpdateTransferHookAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                authority: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn initialize_confidential_transfer_fee_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::InitializeConfidentialTransferFee(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializeConfidentialTransferFeeAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true)
        }]
    );
}
pub(crate) fn withdraw_withheld_tokens_from_mint_for_confidential_transfer_fee_matches_dart_literal()
 {
    let limits = LIMITS;
    let bytes = [
        37, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::WithdrawWithheldTokensFromMintForConfidentialTransferFee(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            WithdrawWithheldTokensFromMintForConfidentialTransferFeeAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                destination: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                instructions_sysvar_or_context_state:
                    "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                        .parse::<Address>()
                        .unwrap(),
                authority: Authority {
                    address: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn withdraw_withheld_tokens_from_accounts_for_confidential_transfer_fee_matches_dart_literal()
 {
    let limits = LIMITS;
    let bytes = [
        37, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::WithdrawWithheldTokensFromAccountsForConfidentialTransferFee(
        value,
    ) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            WithdrawWithheldTokensFromAccountsForConfidentialTransferFeeAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                destination: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                instructions_sysvar_or_context_state:
                    "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                        .parse::<Address>()
                        .unwrap(),
                authority: Authority {
                    address: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn harvest_withheld_tokens_to_mint_for_confidential_transfer_fee_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [37, 3];

    let Token2022InstructionData::HarvestWithheldTokensToMintForConfidentialTransferFee(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            HarvestWithheldTokensToMintForConfidentialTransferFeeAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                sources: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true)
        }]
    );
}
pub(crate) fn enable_harvest_to_mint_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [37, 4];

    let Token2022InstructionData::EnableHarvestToMint(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            EnableHarvestToMintAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                authority: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn disable_harvest_to_mint_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [37, 5];

    let Token2022InstructionData::DisableHarvestToMint(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            DisableHarvestToMintAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                authority: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn withdraw_excess_lamports_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [38];

    let Token2022InstructionData::WithdrawExcessLamports(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            WithdrawExcessLamportsAccounts {
                source: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                destination: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                authority: Authority {
                    address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn initialize_metadata_pointer_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::InitializeMetadataPointer(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializeMetadataPointerAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true)
        }]
    );
}
pub(crate) fn update_metadata_pointer_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        39, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    let Token2022InstructionData::UpdateMetadataPointer(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            UpdateMetadataPointerAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                metadata_pointer_authority: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn initialize_group_pointer_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::InitializeGroupPointer(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializeGroupPointerAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true)
        }]
    );
}
pub(crate) fn update_group_pointer_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        40, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    let Token2022InstructionData::UpdateGroupPointer(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            UpdateGroupPointerAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                group_pointer_authority: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn initialize_group_member_pointer_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::InitializeGroupMemberPointer(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializeGroupMemberPointerAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true)
        }]
    );
}
pub(crate) fn update_group_member_pointer_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        41, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    let Token2022InstructionData::UpdateGroupMemberPointer(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            UpdateGroupMemberPointerAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                group_member_pointer_authority: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn initialize_confidential_mint_burn_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::InitializeConfidentialMintBurn(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializeConfidentialMintBurnAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true)
        }]
    );
}
pub(crate) fn rotate_supply_elgamal_pubkey_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        42, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::RotateSupplyElgamalPubkey(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            RotateSupplyElgamalPubkeyAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                instructions_sysvar_or_context_state: Some(
                    "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                ),
                authority: Authority {
                    address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn update_confidential_mint_burn_decryptable_supply_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        42, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::UpdateConfidentialMintBurnDecryptableSupply(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            UpdateConfidentialMintBurnDecryptableSupplyAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                authority: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn confidential_mint_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        42, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::ConfidentialMint(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            ConfidentialMintAccounts {
                token: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                instructions_sysvar: None,
                equality_record: None,
                ciphertext_validity_record: None,
                range_record: None,
                authority: Authority {
                    address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn confidential_burn_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        42, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::ConfidentialBurn(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            ConfidentialBurnAccounts {
                token: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                instructions_sysvar: None,
                equality_record: None,
                ciphertext_validity_record: None,
                range_record: None,
                authority: Authority {
                    address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn apply_confidential_pending_burn_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [42, 5];

    let Token2022InstructionData::ApplyConfidentialPendingBurn(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            ApplyConfidentialPendingBurnAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                authority: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn initialize_scaled_ui_amount_mint_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::InitializeScaledUiAmountMint(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializeScaledUiAmountMintAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true)
        }]
    );
}
pub(crate) fn update_multiplier_scaled_ui_mint_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [43, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let Token2022InstructionData::UpdateMultiplierScaledUiMint(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            UpdateMultiplierScaledUiMintAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                authority: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, true)
            }
        ]
    );
}
pub(crate) fn initialize_pausable_config_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    let Token2022InstructionData::InitializePausableConfig(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializePausableConfigAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true)
        }]
    );
}
pub(crate) fn pause_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [44, 1];

    let Token2022InstructionData::Pause(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            PauseAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                authority: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn resume_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [44, 2];

    let Token2022InstructionData::Resume(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            ResumeAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                authority: Authority {
                    address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn initialize_token_metadata_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        210, 225, 30, 162, 88, 184, 77, 141, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::InitializeTokenMetadata(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializeTokenMetadataAccounts {
                metadata: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                update_authority: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                mint: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse::<Address>()
                    .unwrap(),
                mint_authority: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn update_token_metadata_field_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [221, 233, 49, 45, 181, 202, 220, 200, 0, 0, 0, 0, 0];

    let Token2022InstructionData::UpdateTokenMetadataField(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            UpdateTokenMetadataFieldAccounts {
                metadata: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                update_authority: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn remove_token_metadata_key_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [234, 18, 32, 56, 89, 141, 37, 181, 0, 0, 0, 0, 0];

    let Token2022InstructionData::RemoveTokenMetadataKey(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            RemoveTokenMetadataKeyAccounts {
                metadata: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                update_authority: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn update_token_metadata_update_authority_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        215, 228, 166, 228, 84, 100, 86, 123, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::UpdateTokenMetadataUpdateAuthority(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            UpdateTokenMetadataUpdateAuthorityAccounts {
                metadata: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                update_authority: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn emit_token_metadata_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [250, 166, 180, 250, 13, 12, 184, 70, 0, 0];

    let Token2022InstructionData::EmitTokenMetadata(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            EmitTokenMetadataAccounts {
                metadata: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, false)
        }]
    );
}
pub(crate) fn initialize_token_group_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        121, 113, 108, 39, 54, 51, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::InitializeTokenGroup(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializeTokenGroupAccounts {
                group: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                mint_authority: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn update_token_group_max_size_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [108, 37, 171, 143, 248, 30, 18, 110, 0, 0, 0, 0, 0, 0, 0, 0];

    let Token2022InstructionData::UpdateTokenGroupMaxSize(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            UpdateTokenGroupMaxSizeAccounts {
                group: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                update_authority: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn update_token_group_update_authority_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        161, 105, 88, 1, 237, 221, 216, 203, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::UpdateTokenGroupUpdateAuthority(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            UpdateTokenGroupUpdateAuthorityAccounts {
                group: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                update_authority: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn initialize_token_group_member_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [152, 32, 222, 176, 223, 237, 116, 134];

    let Token2022InstructionData::InitializeTokenGroupMember(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializeTokenGroupMemberAccounts {
                member: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                member_mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                member_mint_authority: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse::<Address>()
                    .unwrap(),
                group: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                    .parse::<Address>()
                    .unwrap(),
                group_update_authority: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, false)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            },
            AccountMeta {
                address: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn unwrap_lamports_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [45, 0];

    let Token2022InstructionData::UnwrapLamports(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            UnwrapLamportsAccounts {
                source: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                destination: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                authority: Authority {
                    address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn initialize_permissioned_burn_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    let Token2022InstructionData::InitializePermissionedBurn(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            InitializePermissionedBurnAccounts {
                mint: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![AccountMeta {
            address: "11111111111111111111111111111111".parse().unwrap(),
            role: AccountRole::from_privileges(false, true)
        }]
    );
}
pub(crate) fn permissioned_burn_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [46, 1, 0, 0, 0, 0, 0, 0, 0, 0];

    let Token2022InstructionData::PermissionedBurn(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            PermissionedBurnAccounts {
                account: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                permissioned_burn_authority: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse::<Address>()
                    .unwrap(),
                authority: Authority {
                    address: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            },
            AccountMeta {
                address: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn permissioned_burn_checked_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [46, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let Token2022InstructionData::PermissionedBurnChecked(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            PermissionedBurnCheckedAccounts {
                account: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                permissioned_burn_authority: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse::<Address>()
                    .unwrap(),
                authority: Authority {
                    address: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            },
            AccountMeta {
                address: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn permissioned_confidential_burn_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [
        46, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let Token2022InstructionData::PermissionedConfidentialBurn(value) =
        Token2022InstructionDataCodec
            .decode(&bytes, limits)
            .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            PermissionedConfidentialBurnAccounts {
                token: "11111111111111111111111111111111"
                    .parse::<Address>()
                    .unwrap(),
                mint: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse::<Address>()
                    .unwrap(),
                instructions_sysvar: None,
                equality_record: None,
                ciphertext_validity_record: None,
                range_record: None,
                permissioned_burn_authority: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse::<Address>()
                    .unwrap(),
                authority: Authority {
                    address: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                        .parse::<Address>()
                        .unwrap(),
                    is_signer: true,
                },
                multi_signers: &[],
            },
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(
        instruction.accounts,
        vec![
            AccountMeta {
                address: "11111111111111111111111111111111".parse().unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(false, true)
            },
            AccountMeta {
                address: "JEKNVnkbo3jma5nREBBJCDoXFVeKkD56V3xKrvRmWxFG"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            },
            AccountMeta {
                address: "9ecqKYcJhB1zrb1xqEkcF3neFKvmD8sxXEJzTejgdeHV"
                    .parse()
                    .unwrap(),
                role: AccountRole::from_privileges(true, false)
            }
        ]
    );
}
pub(crate) fn batch_matches_dart_literal() {
    let limits = LIMITS;
    let bytes = [255];

    let Token2022InstructionData::Batch(value) = Token2022InstructionDataCodec
        .decode(&bytes, limits)
        .unwrap()
    else {
        panic!("wrong variant")
    };

    let instruction = value
        .instruction_with_program(
            "11111111111111111111111111111111".parse().unwrap(),
            BatchAccounts {},
            limits,
        )
        .unwrap();
    assert_eq!(instruction.data, bytes);
    assert_eq!(instruction.accounts, vec![]);
}
