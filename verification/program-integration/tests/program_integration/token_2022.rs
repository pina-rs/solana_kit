//! Token-2022 pre-initialized extensions, checked mint, fee-bearing transfer.

use crate::util;
use solana_kit_associated_token_account::AssociatedTokenSeeds;
use solana_kit_core::{Address, Instruction, plans::Plan};
use solana_kit_program_integration::{CODEC_LIMITS, Error, IntegrationEnv, expect_custom_error};
use solana_kit_system::{CreateAccount, CreateAccountAccounts};
use solana_kit_token::{
    MintToAtaConfig, MintToAtaDerivedInput, TransferToAtaConfig, TransferToAtaDerivedInput,
    mint_to_ata_derived_plan, transfer_to_ata_derived_plan,
};
use solana_kit_token_2022::{
    Extension, ExtensionInterestBearingConfigPayload, ExtensionTransferFeeConfigPayload,
    InitializeMint2, InitializeMint2Accounts, PROGRAM, TransferFee,
    accounts::{Token2022Account, decode_account_with_program, mint_size},
    extensions::pre_initialize_mint_extensions,
};

const DECIMALS: u8 = 9;
const MINTED: u64 = 1_000_000;
const TRANSFERRED: u64 = 100_000;
const BASIS_POINTS: u16 = 100;
const RATE: i16 = 500;
const MAXIMUM_FEE: u64 = 1_000_000;

pub async fn run(env: &mut IntegrationEnv) -> Result<(), Error> {
    let payer = env.payer_address();
    let mint = env.new_key().await?;
    let recipient = env.new_key().await?;
    let payer_ata = ata(payer, mint.address())?;
    let recipient_ata = ata(recipient.address(), mint.address())?;
    let extensions = extension_values(payer);
    let space = mint_size(Some(&extensions))? as u64;

    let create = CreateAccount {
        lamports: env.rent(space).await?,
        space,
        program_address: PROGRAM,
    }
    .instruction(
        CreateAccountAccounts {
            payer,
            new_account: mint.address(),
        },
        CODEC_LIMITS,
    )?;
    let pre_initialize =
        pre_initialize_mint_extensions(mint.address(), &extensions, Some(PROGRAM), CODEC_LIMITS)?;
    let initialize = InitializeMint2 {
        decimals: DECIMALS,
        mint_authority: payer,
        freeze_authority: None,
    }
    .instruction(
        InitializeMint2Accounts {
            mint: mint.address(),
        },
        CODEC_LIMITS,
    )?;

    let mut instructions = vec![create];
    instructions.extend(pre_initialize);
    instructions.push(initialize);
    let setup_signature = env.submit(&instructions, &[env.payer(), &mint]).await?;

    let mint_plan = mint_to_ata_derived_plan(
        MintToAtaDerivedInput {
            payer,
            owner: payer,
            mint: mint.address(),
            mint_authority: payer,
            amount: MINTED,
            decimals: DECIMALS,
        },
        token_2022_plan_config(),
        CODEC_LIMITS,
    )?;
    env.submit(&flatten(mint_plan), &[env.payer()]).await?;

    let transfer_plan = transfer_to_ata_derived_plan(
        TransferToAtaDerivedInput {
            payer,
            mint: mint.address(),
            authority: payer,
            recipient: recipient.address(),
            amount: TRANSFERRED,
            decimals: DECIMALS,
        },
        TransferToAtaConfig {
            token_program: Some(PROGRAM),
            ..TransferToAtaConfig::default()
        },
        CODEC_LIMITS,
    )?;
    let transfer_signature = env.submit(&flatten(transfer_plan), &[env.payer()]).await?;

    let mint_account = env
        .account(mint.address())
        .await?
        .expect("Token-2022 mint exists");
    let Token2022Account::Mint(mint_state) = decode_account_with_program(
        PROGRAM,
        mint_account.owner,
        &mint_account.data,
        CODEC_LIMITS,
    )?
    else {
        panic!("expected a decoded Token-2022 mint");
    };

    assert_eq!(mint_state.supply, MINTED);
    assert_eq!(mint_state.decimals, DECIMALS);
    assert_eq!(mint_account.space, space);
    assert_extension_values(&mint_state.extensions);

    let recipient_state = decode_token(env, recipient_ata).await?;
    assert_eq!(recipient_state.amount, TRANSFERRED - fee());
    assert_transfer_fee_amount(&recipient_state.extensions);
    println!(
        "token_2022: setup {setup_signature} and transfer {transfer_signature}; mint space {space}, recipient net {}, withheld {}",
        recipient_state.amount,
        fee()
    );

    expect_wrong_authority_rejection(env, mint.address(), payer_ata).await
}

fn extension_values(payer: Address) -> Vec<Extension> {
    let fee = TransferFee {
        epoch: 0,
        maximum_fee: MAXIMUM_FEE,
        transfer_fee_basis_points: BASIS_POINTS,
    };

    vec![
        Extension::TransferFeeConfig(ExtensionTransferFeeConfigPayload {
            transfer_fee_config_authority: payer,
            withdraw_withheld_authority: payer,
            withheld_amount: 0,
            older_transfer_fee: fee.clone(),
            newer_transfer_fee: fee,
        }),
        Extension::InterestBearingConfig(ExtensionInterestBearingConfigPayload {
            rate_authority: payer,
            initialization_timestamp: 0,
            pre_update_average_rate: RATE,
            last_update_timestamp: 0,
            current_rate: RATE,
        }),
    ]
}

fn ata(owner: Address, mint: Address) -> Result<Address, Error> {
    Ok(AssociatedTokenSeeds {
        owner,
        token_program: PROGRAM,
        mint,
    }
    .find()?
    .address())
}

fn token_2022_plan_config() -> MintToAtaConfig {
    MintToAtaConfig {
        token_program: Some(PROGRAM),
        ..MintToAtaConfig::default()
    }
}

fn flatten(plan: Plan<Instruction>) -> Vec<Instruction> {
    plan.flatten().into_iter().cloned().collect()
}

async fn decode_token(
    env: &IntegrationEnv,
    address: Address,
) -> Result<solana_kit_token_2022::Token, Error> {
    let account = env.account(address).await?.expect("token account exists");
    let Token2022Account::Token(state) =
        decode_account_with_program(PROGRAM, account.owner, &account.data, CODEC_LIMITS)?
    else {
        panic!("expected a decoded Token-2022 token account");
    };

    Ok(state)
}

fn assert_extension_values(extensions: &Option<Vec<Extension>>) {
    let extensions = extensions.as_ref().expect("mint extensions are present");
    let fee = extensions
        .iter()
        .find_map(|extension| match extension {
            Extension::TransferFeeConfig(value) => Some(value),
            _ => None,
        })
        .expect("transfer fee config is present");
    let interest = extensions
        .iter()
        .find_map(|extension| match extension {
            Extension::InterestBearingConfig(value) => Some(value),
            _ => None,
        })
        .expect("interest-bearing config is present");

    assert_eq!(
        fee.newer_transfer_fee.transfer_fee_basis_points,
        BASIS_POINTS
    );
    assert_eq!(fee.newer_transfer_fee.maximum_fee, MAXIMUM_FEE);
    assert_eq!(interest.current_rate, RATE);
    assert_eq!(extensions.len(), 2);
}

fn assert_transfer_fee_amount(extensions: &Option<Vec<Extension>>) {
    let extensions = extensions.as_ref().expect("token extensions are present");
    let withheld = extensions
        .iter()
        .find_map(|extension| match extension {
            Extension::TransferFeeAmount(value) => Some(value.withheld_amount),
            _ => None,
        })
        .expect("transfer fee amount is present");

    assert_eq!(withheld, fee());
}

const fn fee() -> u64 {
    TRANSFERRED * BASIS_POINTS as u64 / 10_000
}

async fn expect_wrong_authority_rejection(
    env: &mut IntegrationEnv,
    mint: Address,
    token: Address,
) -> Result<(), Error> {
    let intruder = env.new_key().await?;
    let plan = mint_to_ata_derived_plan(
        MintToAtaDerivedInput {
            payer: env.payer_address(),
            owner: env.payer_address(),
            mint,
            mint_authority: intruder.address(),
            amount: 1,
            decimals: DECIMALS,
        },
        token_2022_plan_config(),
        CODEC_LIMITS,
    )?;
    let instructions = flatten(plan);
    assert_eq!(
        instructions[1].accounts[1].address, token,
        "plan must mint to the owner's derived ATA"
    );
    let error = env
        .submit(&instructions, &[env.payer(), &intruder])
        .await
        .expect_err("a foreign mint authority cannot mint");

    expect_custom_error(&error, 1, 4);
    println!(
        "token_2022: plan mint with a foreign authority rejected with custom error 4 (OwnerMismatch)"
    );

    Ok(())
}

#[tokio::test]
async fn pre_initializes_extensions_and_applies_a_transfer_fee() {
    let mut env = util::start().await;
    let result = run(&mut env).await;
    let cleanup = env.close().await;

    util::finish(result, cleanup);
}
