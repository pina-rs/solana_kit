//! SPL Token plans: create-mint, idempotent ATA + checked mint, and ATA transfer.

use crate::util;
use solana_kit_associated_token_account::{
    AssociatedTokenInstruction, AssociatedTokenSeeds, CreateAccounts,
};
use solana_kit_client::parsed::{ParsedAccountData, TokenAccountState, TokenProgramAccount};
use solana_kit_core::{Address, Instruction, plans::Plan};
use solana_kit_program_integration::{
    CODEC_LIMITS, Error, IntegrationEnv, PARSED_LIMITS, expect_custom_error,
};
use solana_kit_system::SYSTEM_PROGRAM;
use solana_kit_token::{
    CreateMintConfig, CreateMintInput, MintCodec, MintToAtaConfig, MintToAtaDerivedInput, PROGRAM,
    TokenCodec, TransferToAtaConfig, TransferToAtaDerivedInput, create_mint_plan,
    mint_to_ata_derived_plan, transfer_to_ata_derived_plan,
};

const DECIMALS: u8 = 9;
const MINTED: u64 = 1_000_000;
const TRANSFERRED: u64 = 250_000;

pub async fn run(env: &mut IntegrationEnv) -> Result<(), Error> {
    let payer = env.payer_address();
    let mint = env.new_key().await?;
    let recipient = env.new_key().await?;
    let payer_ata = derived_ata(payer, mint.address());
    let recipient_ata = derived_ata(recipient.address(), mint.address());

    let mint_plan = create_mint_plan(
        CreateMintInput {
            payer,
            new_mint: mint.address(),
            decimals: DECIMALS,
            mint_authority: payer,
            freeze_authority: None,
            mint_account_lamports: Some(env.rent(solana_kit_token::MINT_ACCOUNT_SIZE).await?),
        },
        CreateMintConfig::default(),
        CODEC_LIMITS,
    )?;
    env.submit(&flatten(mint_plan), &[env.payer(), &mint])
        .await?;

    let mint_plan = mint_to_ata_derived_plan(
        MintToAtaDerivedInput {
            payer,
            owner: payer,
            mint: mint.address(),
            mint_authority: payer,
            amount: MINTED,
            decimals: DECIMALS,
        },
        MintToAtaConfig::default(),
        CODEC_LIMITS,
    )?;
    env.submit(&flatten(mint_plan), &[env.payer()]).await?;

    // The plan reuses CreateIdempotent, so a second run with zero amount must
    // succeed against the now-existing account.
    let idempotent_plan = mint_to_ata_derived_plan(
        MintToAtaDerivedInput {
            payer,
            owner: payer,
            mint: mint.address(),
            mint_authority: payer,
            amount: 0,
            decimals: DECIMALS,
        },
        MintToAtaConfig::default(),
        CODEC_LIMITS,
    )?;
    env.submit(&flatten(idempotent_plan), &[env.payer()])
        .await?;

    let transfer_plan = transfer_to_ata_derived_plan(
        TransferToAtaDerivedInput {
            payer,
            mint: mint.address(),
            authority: payer,
            recipient: recipient.address(),
            amount: TRANSFERRED,
            decimals: DECIMALS,
        },
        TransferToAtaConfig::default(),
        CODEC_LIMITS,
    )?;
    let transfer = flatten(transfer_plan);
    assert_derived_accounts(&transfer, payer_ata, recipient_ata, mint.address());
    let signature = env.submit(&transfer, &[env.payer()]).await?;

    let mint_account = env
        .account(mint.address())
        .await?
        .expect("mint account exists");
    let mint_state = mint_account.decode(&MintCodec, PROGRAM, CODEC_LIMITS)?;
    assert_eq!(mint_state.data.supply, MINTED);
    assert_eq!(mint_state.data.decimals, DECIMALS);
    assert_eq!(mint_state.data.mint_authority, Some(payer));

    let payer_state = decode_token(env, payer_ata).await?;
    let recipient_state = decode_token(env, recipient_ata).await?;
    assert_eq!(payer_state.amount, MINTED - TRANSFERRED);
    assert_eq!(recipient_state.amount, TRANSFERRED);
    assert_eq!(recipient_state.mint, mint.address());
    assert_eq!(recipient_state.owner, recipient.address());

    let parsed_mint = env
        .parsed_account(mint.address())
        .await?
        .expect("parsed mint exists");

    match parsed_mint.decode_typed(PARSED_LIMITS)? {
        ParsedAccountData::Token(TokenProgramAccount::Mint(info)) => {
            assert_eq!(info.supply, mint_state.data.supply);
            assert_eq!(info.decimals, mint_state.data.decimals);
            assert_eq!(info.mint_authority, mint_state.data.mint_authority);
            assert!(info.is_initialized);
        }
        other => panic!("expected a parsed Token mint, got {other:?}"),
    }

    let parsed_account = env
        .parsed_account(recipient_ata)
        .await?
        .expect("parsed token account exists");

    match parsed_account.decode_typed(PARSED_LIMITS)? {
        ParsedAccountData::Token(TokenProgramAccount::Account(info)) => {
            assert_eq!(info.token_amount.amount, recipient_state.amount);
            assert_eq!(info.owner, recipient_state.owner);
            assert_eq!(info.mint, recipient_state.mint);
            assert_eq!(info.state, TokenAccountState::Initialized);
        }
        other => panic!("expected a parsed Token account, got {other:?}"),
    }

    println!(
        "token: plans and jsonParsed projections confirmed {signature}; idempotent ATA re-created, minted {MINTED}, transferred {TRANSFERRED}; decoded balances {} / {MINTED}",
        payer_state.amount
    );

    expect_over_transfer_rejection(env, mint.address(), recipient.address()).await?;
    expect_duplicate_ata_rejection(env, payer_ata, payer, mint.address()).await
}

fn flatten(plan: Plan<Instruction>) -> Vec<Instruction> {
    plan.flatten().into_iter().cloned().collect()
}

fn derived_ata(owner: Address, mint: Address) -> Address {
    AssociatedTokenSeeds {
        owner,
        token_program: PROGRAM,
        mint,
    }
    .find()
    .expect("canonical ATA derivation always finds a bump")
    .address()
}

fn assert_derived_accounts(
    instructions: &[Instruction],
    payer_ata: Address,
    recipient_ata: Address,
    mint: Address,
) {
    assert_eq!(instructions.len(), 2);
    assert_eq!(
        instructions[0].accounts[1].address, recipient_ata,
        "CreateIdempotent must target the derived recipient ATA"
    );
    assert_eq!(
        instructions[1].accounts[0].address, payer_ata,
        "TransferChecked must use the authority's derived source ATA"
    );
    assert_eq!(instructions[1].accounts[1].address, mint);
    assert_eq!(instructions[1].accounts[2].address, recipient_ata);
}

async fn decode_token(
    env: &IntegrationEnv,
    address: Address,
) -> Result<solana_kit_token::Token, Error> {
    let account = env.account(address).await?.expect("token account exists");
    let state = account.decode(&TokenCodec, PROGRAM, CODEC_LIMITS)?;

    Ok(state.data)
}

async fn expect_over_transfer_rejection(
    env: &IntegrationEnv,
    mint: Address,
    recipient: Address,
) -> Result<(), Error> {
    let plan = transfer_to_ata_derived_plan(
        TransferToAtaDerivedInput {
            payer: env.payer_address(),
            mint,
            authority: env.payer_address(),
            recipient,
            amount: u64::MAX,
            decimals: DECIMALS,
        },
        TransferToAtaConfig::default(),
        CODEC_LIMITS,
    )?;
    let error = env
        .submit(&flatten(plan), &[env.payer()])
        .await
        .expect_err("u64::MAX tokens cannot be transferred");

    expect_custom_error(&error, 1, 1);
    println!("token: plan TransferChecked rejected an over-balance amount with custom error 1");

    Ok(())
}

async fn expect_duplicate_ata_rejection(
    env: &IntegrationEnv,
    existing: Address,
    owner: Address,
    mint: Address,
) -> Result<(), Error> {
    let duplicate = AssociatedTokenInstruction::Create(CreateAccounts {
        payer: env.payer_address(),
        associated_token: existing,
        owner,
        mint,
        system_program: SYSTEM_PROGRAM,
        token_program: PROGRAM,
    })
    .instruction(CODEC_LIMITS)?;
    let error = env
        .submit(&[duplicate], &[env.payer()])
        .await
        .expect_err("non-idempotent ATA creation cannot reuse an existing account");

    match error.instruction_failure() {
        Some((0, solana_kit_client::methods::submission::InstructionFailure::IllegalOwner)) => {
            println!("token: duplicate non-idempotent ATA create rejected with IllegalOwner")
        }
        other => panic!("expected ATA IllegalOwner at instruction 0, got {other:?} ({error:?})"),
    }

    Ok(())
}

#[tokio::test]
async fn executes_token_plans_and_rejects_invalid_amounts() {
    let mut env = util::start().await;
    let result = run(&mut env).await;
    let cleanup = env.close().await;

    util::finish(result, cleanup);
}
