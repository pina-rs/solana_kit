//! Independent literals for the source-defined Token instruction plans.
use solana_kit_associated_token_account::{AssociatedTokenInstruction, CreateAccounts};
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Hash, Instruction,
    codecs::{self, Limits},
    plans::Plan,
    transaction::{Lifetime, Message, Version},
};
use solana_kit_token::*;

const LIMITS: Limits = Limits::new(4096, 4096);
const EMPTY_BYTES: Limits = Limits::new(0, 16);
const SHORT_ATA_ITEMS: Limits = Limits::new(4096, 5);
const HEX_RADIX: u32 = 16;
const DECIMALS: u8 = 6;

const MINT_AMOUNT: u64 = 1_000_000;
const TRANSFER_AMOUNT: u64 = 1_234_567_890_123;

const PAYER: Address = Address::from_bytes([1; 32]);
const NEW_MINT: Address = Address::from_bytes([2; 32]);
const OWNER: Address = Address::from_bytes([3; 32]);
const MINT: Address = Address::from_bytes([4; 32]);
const AUTHORITY: Address = Address::from_bytes([5; 32]);
const SOURCE: Address = Address::from_bytes([6; 32]);

const CUSTOM_TOKEN: Address = Address::from_bytes([7; 32]);
const DESTINATION: Address = Address::from_bytes([8; 32]);
const CUSTOM_ATA: Address = Address::from_bytes([9; 32]);
const CUSTOM_SYSTEM: Address = Address::from_bytes([10; 32]);

const TOKEN_PROGRAM: Address = Address::from_bytes([
    6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172, 28, 180, 133, 237,
    95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169,
]);

const ASSOCIATED_TOKEN_PROGRAM: Address = Address::from_bytes([
    140, 151, 37, 143, 78, 36, 137, 241, 187, 61, 16, 41, 20, 142, 13, 131, 11, 90, 19, 153, 218,
    255, 16, 132, 4, 142, 123, 216, 219, 233, 248, 89,
]);

const SYSTEM_PROGRAM: Address = Address::from_bytes([0; 32]);

fn pinned(value: &str) -> Address {
    value.parse().expect("pinned address")
}

fn bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|index| u8::from_str_radix(&hex[index..index + 2], HEX_RADIX).unwrap())
        .collect()
}

fn sequence(plan: &Plan<Instruction>) -> (&[Plan<Instruction>], bool) {
    match plan {
        Plan::Sequential { plans, divisible } => (plans, *divisible),
        Plan::Single(_) | Plan::Parallel(_) => panic!("expected a sequential plan"),
    }
}

fn single(plan: &Plan<Instruction>) -> &Instruction {
    match plan {
        Plan::Single(instruction) => instruction,
        Plan::Sequential { .. } | Plan::Parallel(_) => panic!("expected one instruction"),
    }
}

fn create_mint_input() -> CreateMintInput {
    CreateMintInput {
        payer: PAYER,
        new_mint: NEW_MINT,
        decimals: DECIMALS,
        mint_authority: OWNER,
        freeze_authority: None,
        mint_account_lamports: None,
    }
}

fn mint_to_ata_input() -> MintToAtaInput {
    MintToAtaInput {
        payer: PAYER,
        ata: SOURCE,
        owner: OWNER,
        mint: MINT,
        mint_authority: AUTHORITY,
        amount: MINT_AMOUNT,
        decimals: DECIMALS,
    }
}

fn transfer_to_ata_input() -> TransferToAtaInput {
    TransferToAtaInput {
        payer: PAYER,
        mint: MINT,
        source: SOURCE,
        authority: AUTHORITY,
        destination: DESTINATION,
        recipient: OWNER,
        amount: TRANSFER_AMOUNT,
        decimals: DECIMALS,
    }
}

#[test]
fn create_mint_account_is_an_indivisible_source_funded_pair() {
    let plan = create_mint_plan(create_mint_input(), CreateMintConfig::default(), LIMITS).unwrap();
    let (children, divisible) = sequence(&plan);

    assert!(!divisible);
    assert_eq!(children.len(), 2);
    assert_eq!(plan.flatten().len(), 2);

    let create = single(&children[0]);
    assert_eq!(create.program_address, SYSTEM_PROGRAM);
    assert_eq!(
        create.accounts,
        vec![
            AccountMeta {
                address: PAYER,
                role: AccountRole::WritableSigner,
            },
            AccountMeta {
                address: NEW_MINT,
                role: AccountRole::WritableSigner,
            },
        ]
    );
    assert_eq!(
        create.data,
        bytes(
            "00000000604d160000000000520000000000000006ddf6e1d765a193d9cbe146ceeb79ac1cb485ed5f5b37913a8cf5857eff00a9"
        )
    );

    let (value, accounts) = solana_kit_system::CreateAccount::parse(create, LIMITS).unwrap();
    assert_eq!(value.lamports, 1_461_600);
    assert_eq!(value.space, 82);
    assert_eq!(value.program_address, TOKEN_PROGRAM);
    assert_eq!(accounts.payer, PAYER);
    assert_eq!(accounts.new_account, NEW_MINT);
}

#[test]
fn create_mint_initialize_mint2_has_source_wire_values() {
    let plan = create_mint_plan(create_mint_input(), CreateMintConfig::default(), LIMITS).unwrap();
    let (children, _) = sequence(&plan);
    let initialize = single(&children[1]);

    assert_eq!(initialize.program_address, TOKEN_PROGRAM);
    assert_eq!(
        initialize.accounts,
        vec![AccountMeta {
            address: NEW_MINT,
            role: AccountRole::Writable,
        }]
    );
    assert_eq!(
        initialize.data,
        bytes("1406030303030303030303030303030303030303030303030303030303030303030300")
    );
    assert_eq!(
        InitializeMint2::parse_with_program(TOKEN_PROGRAM, initialize, LIMITS).unwrap(),
        InitializeMint2 {
            decimals: DECIMALS,
            mint_authority: OWNER,
            freeze_authority: None,
        }
    );
}

#[test]
fn create_mint_freeze_authority_and_caller_rent_are_exact() {
    let input = CreateMintInput {
        freeze_authority: Some(AUTHORITY),
        mint_account_lamports: Some(42),
        ..create_mint_input()
    };
    let plan = create_mint_plan(input, CreateMintConfig::default(), LIMITS).unwrap();
    let (children, _) = sequence(&plan);

    assert_eq!(
        single(&children[0]).data,
        bytes(
            "000000002a00000000000000520000000000000006ddf6e1d765a193d9cbe146ceeb79ac1cb485ed5f5b37913a8cf5857eff00a9"
        )
    );
    assert_eq!(
        single(&children[1]).data,
        bytes(
            "14060303030303030303030303030303030303030303030303030303030303030303010505050505050505050505050505050505050505050505050505050505050505"
        )
    );
}

#[test]
fn mint_to_ata_creates_the_associated_account_idempotently() {
    let plan = mint_to_ata_plan(mint_to_ata_input(), MintToAtaConfig::default(), LIMITS).unwrap();
    let (children, divisible) = sequence(&plan);

    assert!(divisible);
    assert_eq!(children.len(), 2);

    let create = single(&children[0]);
    assert_eq!(create.program_address, ASSOCIATED_TOKEN_PROGRAM);
    assert_eq!(create.data, vec![1]);
    assert_eq!(
        create.accounts,
        vec![
            AccountMeta {
                address: PAYER,
                role: AccountRole::WritableSigner,
            },
            AccountMeta {
                address: SOURCE,
                role: AccountRole::Writable,
            },
            AccountMeta {
                address: OWNER,
                role: AccountRole::Readonly,
            },
            AccountMeta {
                address: MINT,
                role: AccountRole::Readonly,
            },
            AccountMeta {
                address: SYSTEM_PROGRAM,
                role: AccountRole::Readonly,
            },
            AccountMeta {
                address: TOKEN_PROGRAM,
                role: AccountRole::Readonly,
            },
        ]
    );
    assert_eq!(
        solana_kit_associated_token_account::parse_instruction(create, LIMITS).unwrap(),
        AssociatedTokenInstruction::CreateIdempotent(CreateAccounts {
            payer: PAYER,
            associated_token: SOURCE,
            owner: OWNER,
            mint: MINT,
            system_program: SYSTEM_PROGRAM,
            token_program: TOKEN_PROGRAM,
        })
    );
}

#[test]
fn mint_to_ata_mints_checked_after_creation() {
    let plan = mint_to_ata_plan(mint_to_ata_input(), MintToAtaConfig::default(), LIMITS).unwrap();
    let (children, _) = sequence(&plan);
    let mint_to = single(&children[1]);

    assert_eq!(mint_to.program_address, TOKEN_PROGRAM);
    assert_eq!(
        mint_to.accounts,
        vec![
            AccountMeta {
                address: MINT,
                role: AccountRole::Writable,
            },
            AccountMeta {
                address: SOURCE,
                role: AccountRole::Writable,
            },
            AccountMeta {
                address: AUTHORITY,
                role: AccountRole::ReadonlySigner,
            },
        ]
    );
    assert_eq!(mint_to.data, bytes("0e40420f000000000006"));
    assert_eq!(
        MintToChecked::parse_with_program(TOKEN_PROGRAM, mint_to, LIMITS).unwrap(),
        MintToChecked {
            amount: MINT_AMOUNT,
            decimals: DECIMALS,
        }
    );
}

#[test]
fn transfer_to_ata_creates_the_destination_idempotently() {
    let plan = transfer_to_ata_plan(
        transfer_to_ata_input(),
        TransferToAtaConfig::default(),
        LIMITS,
    )
    .unwrap();
    let (children, divisible) = sequence(&plan);

    assert!(divisible);
    assert_eq!(children.len(), 2);

    let create = single(&children[0]);
    assert_eq!(create.program_address, ASSOCIATED_TOKEN_PROGRAM);
    assert_eq!(create.data, vec![1]);
    assert_eq!(
        create.accounts,
        vec![
            AccountMeta {
                address: PAYER,
                role: AccountRole::WritableSigner,
            },
            AccountMeta {
                address: DESTINATION,
                role: AccountRole::Writable,
            },
            AccountMeta {
                address: OWNER,
                role: AccountRole::Readonly,
            },
            AccountMeta {
                address: MINT,
                role: AccountRole::Readonly,
            },
            AccountMeta {
                address: SYSTEM_PROGRAM,
                role: AccountRole::Readonly,
            },
            AccountMeta {
                address: TOKEN_PROGRAM,
                role: AccountRole::Readonly,
            },
        ]
    );
}

#[test]
fn transfer_to_ata_transfers_checked_after_creation() {
    let plan = transfer_to_ata_plan(
        transfer_to_ata_input(),
        TransferToAtaConfig::default(),
        LIMITS,
    )
    .unwrap();
    let (children, _) = sequence(&plan);
    let transfer = single(&children[1]);

    assert_eq!(transfer.program_address, TOKEN_PROGRAM);
    assert_eq!(
        transfer.accounts,
        vec![
            AccountMeta {
                address: SOURCE,
                role: AccountRole::Writable,
            },
            AccountMeta {
                address: MINT,
                role: AccountRole::Readonly,
            },
            AccountMeta {
                address: DESTINATION,
                role: AccountRole::Writable,
            },
            AccountMeta {
                address: AUTHORITY,
                role: AccountRole::ReadonlySigner,
            },
        ]
    );
    assert_eq!(transfer.data, bytes("0ccb04fb711f01000006"));
    assert_eq!(
        TransferChecked::parse_with_program(TOKEN_PROGRAM, transfer, LIMITS).unwrap(),
        TransferChecked {
            amount: TRANSFER_AMOUNT,
            decimals: DECIMALS,
        }
    );
}

#[test]
fn compiled_ata_plans_require_the_distinct_authority_signature() {
    let mint = mint_to_ata_plan(mint_to_ata_input(), MintToAtaConfig::default(), LIMITS).unwrap();
    let transfer = transfer_to_ata_plan(
        transfer_to_ata_input(),
        TransferToAtaConfig::default(),
        LIMITS,
    )
    .unwrap();

    for plan in [mint, transfer] {
        let instructions: Vec<_> = plan.flatten().into_iter().cloned().collect();
        let message = Message::compile(
            Version::Legacy,
            PAYER,
            Lifetime::Blockhash {
                blockhash: Hash::from_bytes([42; 32]),
                last_valid_block_height: 42,
            },
            &instructions,
            &[],
        )
        .unwrap();

        assert_eq!(message.signers(), [PAYER, AUTHORITY]);
    }
}

#[test]
fn derived_mint_plan_matches_the_explicit_pinned_ata() {
    let derived = mint_to_ata_derived_plan(
        MintToAtaDerivedInput {
            payer: PAYER,
            owner: OWNER,
            mint: MINT,
            mint_authority: AUTHORITY,
            amount: MINT_AMOUNT,
            decimals: DECIMALS,
        },
        MintToAtaConfig::default(),
        LIMITS,
    )
    .unwrap();
    let minted_ata = pinned("CxX9kqQpBX4PczGhFTxU4WmHjknEgpNtUAR9LbJCSw8u");
    let explicit = mint_to_ata_plan(
        MintToAtaInput {
            ata: minted_ata,
            ..mint_to_ata_input()
        },
        MintToAtaConfig::default(),
        LIMITS,
    )
    .unwrap();

    assert_eq!(derived, explicit);
    let (children, _) = sequence(&derived);
    assert_eq!(single(&children[0]).accounts[1].address, minted_ata);
}

#[test]
fn derived_transfer_plan_matches_the_explicit_pinned_atas() {
    let derived = transfer_to_ata_derived_plan(
        TransferToAtaDerivedInput {
            payer: PAYER,
            mint: MINT,
            authority: AUTHORITY,
            recipient: OWNER,
            amount: TRANSFER_AMOUNT,
            decimals: DECIMALS,
        },
        TransferToAtaConfig::default(),
        LIMITS,
    )
    .unwrap();
    let destination = pinned("CxX9kqQpBX4PczGhFTxU4WmHjknEgpNtUAR9LbJCSw8u");
    let source = pinned("ExdJGUaW8C3EL2TDp4poEekZhYBUhmt8WJa7d8T1kD7d");
    let explicit = transfer_to_ata_plan(
        TransferToAtaInput {
            source,
            destination,
            ..transfer_to_ata_input()
        },
        TransferToAtaConfig::default(),
        LIMITS,
    )
    .unwrap();

    assert_eq!(derived, explicit);
}

#[test]
fn create_mint_custom_programs_apply_to_identity_and_owner() {
    let plan = create_mint_plan(
        create_mint_input(),
        CreateMintConfig {
            token_program: Some(CUSTOM_TOKEN),
            system_program: Some(CUSTOM_SYSTEM),
        },
        LIMITS,
    )
    .unwrap();
    let (children, _) = sequence(&plan);
    let create = single(&children[0]);

    assert_eq!(create.program_address, CUSTOM_SYSTEM);
    assert_eq!(&create.data[20..52], CUSTOM_TOKEN.as_bytes());
    assert_eq!(single(&children[1]).program_address, CUSTOM_TOKEN);
}

#[test]
fn ata_plans_apply_custom_programs_to_identity_and_metadata() {
    let mint_plan = mint_to_ata_plan(
        mint_to_ata_input(),
        MintToAtaConfig {
            token_program: Some(CUSTOM_TOKEN),
            associated_token_program: Some(CUSTOM_ATA),
            system_program: Some(CUSTOM_SYSTEM),
        },
        LIMITS,
    )
    .unwrap();
    let (children, _) = sequence(&mint_plan);
    let create = single(&children[0]);

    assert_eq!(create.program_address, CUSTOM_ATA);
    assert_eq!(
        create
            .accounts
            .iter()
            .map(|account| account.address)
            .collect::<Vec<_>>(),
        [PAYER, SOURCE, OWNER, MINT, CUSTOM_SYSTEM, CUSTOM_TOKEN]
    );
    assert_eq!(single(&children[1]).program_address, CUSTOM_TOKEN);

    let transfer_plan = transfer_to_ata_plan(
        transfer_to_ata_input(),
        TransferToAtaConfig {
            token_program: Some(CUSTOM_TOKEN),
            associated_token_program: Some(CUSTOM_ATA),
            system_program: Some(CUSTOM_SYSTEM),
        },
        LIMITS,
    )
    .unwrap();
    let (children, _) = sequence(&transfer_plan);
    let create = single(&children[0]);

    assert_eq!(create.program_address, CUSTOM_ATA);
    assert_eq!(
        create
            .accounts
            .iter()
            .map(|account| account.address)
            .collect::<Vec<_>>(),
        [PAYER, DESTINATION, OWNER, MINT, CUSTOM_SYSTEM, CUSTOM_TOKEN]
    );
    assert_eq!(single(&children[1]).program_address, CUSTOM_TOKEN);
}

#[test]
fn derived_plans_use_custom_programs_in_the_pinned_seed_order() {
    let plan = mint_to_ata_derived_plan(
        MintToAtaDerivedInput {
            payer: PAYER,
            owner: OWNER,
            mint: MINT,
            mint_authority: AUTHORITY,
            amount: MINT_AMOUNT,
            decimals: DECIMALS,
        },
        MintToAtaConfig {
            token_program: Some(CUSTOM_TOKEN),
            associated_token_program: Some(CUSTOM_ATA),
            system_program: Some(CUSTOM_SYSTEM),
        },
        LIMITS,
    )
    .unwrap();
    let (children, _) = sequence(&plan);
    let minted_ata = pinned("AWuSAwMmNtEny1hf4BitXHcJQBHBr2G8QsM15t2D37Sj");
    assert_eq!(single(&children[0]).accounts[1].address, minted_ata);

    let plan = transfer_to_ata_derived_plan(
        TransferToAtaDerivedInput {
            payer: PAYER,
            mint: MINT,
            authority: AUTHORITY,
            recipient: OWNER,
            amount: TRANSFER_AMOUNT,
            decimals: DECIMALS,
        },
        TransferToAtaConfig {
            token_program: Some(CUSTOM_TOKEN),
            associated_token_program: Some(CUSTOM_ATA),
            system_program: Some(CUSTOM_SYSTEM),
        },
        LIMITS,
    )
    .unwrap();
    let (children, _) = sequence(&plan);
    assert_eq!(
        single(&children[1]).accounts[0].address,
        pinned("GGcnfFWVGymcLkXVb8BgWUPVRGPkmoAqhyCebaP6Z89s")
    );
    assert_eq!(single(&children[1]).accounts[2].address, minted_ata);
}

#[test]
fn zero_amounts_and_zero_decimals_are_preserved() {
    let plan = mint_to_ata_plan(
        MintToAtaInput {
            amount: 0,
            decimals: 0,
            ..mint_to_ata_input()
        },
        MintToAtaConfig::default(),
        LIMITS,
    )
    .unwrap();
    let (children, _) = sequence(&plan);
    assert_eq!(single(&children[1]).data, bytes("0e000000000000000000"));

    let plan = transfer_to_ata_plan(
        TransferToAtaInput {
            amount: 0,
            decimals: 0,
            ..transfer_to_ata_input()
        },
        TransferToAtaConfig::default(),
        LIMITS,
    )
    .unwrap();
    let (children, _) = sequence(&plan);
    assert_eq!(single(&children[1]).data, bytes("0c000000000000000000"));
}

#[test]
fn caller_limits_propagate_without_partial_plans() {
    assert_eq!(
        create_mint_plan(
            create_mint_input(),
            CreateMintConfig::default(),
            EMPTY_BYTES
        ),
        Err(PlanError::System(solana_kit_system::ClientError::Codec(
            codecs::Error::LimitExceeded
        )))
    );
    assert_eq!(
        mint_to_ata_plan(mint_to_ata_input(), MintToAtaConfig::default(), EMPTY_BYTES),
        Err(PlanError::Codec(codecs::Error::LimitExceeded))
    );
    assert_eq!(
        mint_to_ata_plan(
            mint_to_ata_input(),
            MintToAtaConfig::default(),
            SHORT_ATA_ITEMS
        ),
        Err(PlanError::Codec(codecs::Error::LimitExceeded))
    );
    assert_eq!(
        transfer_to_ata_plan(
            transfer_to_ata_input(),
            TransferToAtaConfig::default(),
            SHORT_ATA_ITEMS
        ),
        Err(PlanError::Codec(codecs::Error::LimitExceeded))
    );
}
