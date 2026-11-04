//! Stake initialization, delegation to an owned vote fixture and deactivation.

use crate::util;
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    plans::{InstructionLeaf, InstructionPlan},
    sysvars::{CLOCK_ADDRESS, RENT_ADDRESS},
};
use solana_kit_program_integration::{CODEC_LIMITS, Error, IntegrationEnv, expect_custom_error};
use solana_kit_stake::{
    Authorized, Deactivate, DeactivateAccounts, PROGRAM, StakeStateV2, create_stake_account_plan,
    decode_account, delegate_stake_plan,
};
use solana_kit_system::{CreateAccount, CreateAccountAccounts};

const VOTE_PROGRAM: Address = Address::from_bytes([
    7, 97, 72, 29, 53, 116, 116, 187, 124, 77, 118, 36, 235, 211, 189, 179, 216, 53, 94, 115, 209,
    16, 67, 252, 13, 163, 83, 128, 0, 0, 0, 0,
]);
const VOTE_STATE_SIZE: u64 = 3_762;
const STAKE_STATE_SIZE: u64 = 200;
const DELEGATED_LAMPORTS: u64 = 5_000_000_000;
const COMMISSION: u8 = 0;

pub async fn run(env: &mut IntegrationEnv) -> Result<(), Error> {
    let payer = env.payer_address();
    let vote = env.new_key().await?;
    create_vote_fixture(env, &vote).await?;

    let stake = env.new_key().await?;
    let stake_rent = env.rent(STAKE_STATE_SIZE).await?;
    let plan = create_stake_account_plan(
        payer,
        stake.address(),
        stake_rent + DELEGATED_LAMPORTS,
        Authorized {
            staker: payer,
            withdrawer: payer,
        },
        None,
        CODEC_LIMITS,
    )?;
    let create = instructions(plan)?;
    env.submit(&create, &[env.payer(), &stake]).await?;

    let initialized = decode_stake(env, stake.address()).await?;
    let StakeStateV2::Initialized(meta) = initialized.state else {
        panic!("expected an initialized stake account");
    };
    assert_eq!(meta.authorized.staker, payer);
    assert_eq!(meta.authorized.withdrawer, payer);
    assert_eq!(meta.rent_exempt_reserve, stake_rent);

    let delegate_plan = delegate_stake_plan(stake.address(), vote.address(), payer, CODEC_LIMITS)?;
    let delegate = instructions(delegate_plan)?;
    let delegate_signature = env.submit(&delegate, &[env.payer()]).await?;

    let delegated = decode_stake(env, stake.address()).await?;
    let StakeStateV2::Stake(payload) = delegated.state else {
        panic!("expected a delegated stake account");
    };
    assert_eq!(payload.item1.delegation.voter_pubkey, vote.address());
    assert_eq!(payload.item1.delegation.stake, DELEGATED_LAMPORTS);
    assert_eq!(payload.item1.delegation.deactivation_epoch.value, u64::MAX);
    println!(
        "stake: {delegate_signature} delegated {DELEGATED_LAMPORTS} lamports to vote {}",
        vote.address()
    );

    let deactivate = DeactivateAccounts {
        stake: stake.address(),
        clock_sysvar: CLOCK_ADDRESS,
        stake_authority: payer,
    }
    .instruction(PROGRAM, &Deactivate {}, &[], CODEC_LIMITS)?;
    let deactivate_signature = env.submit(&[deactivate], &[env.payer()]).await?;

    let deactivated = decode_stake(env, stake.address()).await?;
    let StakeStateV2::Stake(payload) = deactivated.state else {
        panic!("expected a deactivating stake account");
    };
    assert_ne!(payload.item1.delegation.deactivation_epoch.value, u64::MAX);
    println!(
        "stake: {deactivate_signature} deactivated at epoch {}",
        payload.item1.delegation.deactivation_epoch.value
    );

    expect_second_deactivation_rejection(env, stake.address()).await
}

async fn create_vote_fixture(
    env: &IntegrationEnv,
    vote: &solana_kit_core::crypto::Keypair,
) -> Result<(), Error> {
    let payer = env.payer_address();
    let create = CreateAccount {
        lamports: env.rent(VOTE_STATE_SIZE).await?,
        space: VOTE_STATE_SIZE,
        program_address: VOTE_PROGRAM,
    }
    .instruction(
        CreateAccountAccounts {
            payer,
            new_account: vote.address(),
        },
        CODEC_LIMITS,
    )?;
    let initialize = Instruction {
        program_address: VOTE_PROGRAM,
        accounts: vec![
            AccountMeta {
                address: vote.address(),
                role: AccountRole::Writable,
            },
            AccountMeta {
                address: RENT_ADDRESS,
                role: AccountRole::Readonly,
            },
            AccountMeta {
                address: CLOCK_ADDRESS,
                role: AccountRole::Readonly,
            },
            AccountMeta {
                address: payer,
                role: AccountRole::ReadonlySigner,
            },
        ],
        data: vote_initialize_data(payer, payer, payer),
    };
    env.submit(&[create, initialize], &[env.payer(), vote])
        .await?;

    let account = env.account(vote.address()).await?.expect("vote exists");
    assert_eq!(account.owner, VOTE_PROGRAM);
    assert_eq!(account.space, VOTE_STATE_SIZE);
    println!(
        "stake: created a {VOTE_STATE_SIZE}-byte vote fixture at {}",
        vote.address()
    );

    Ok(())
}

fn vote_initialize_data(node: Address, voter: Address, withdrawer: Address) -> Vec<u8> {
    let mut data = Vec::with_capacity(4 + 32 + 32 + 32 + 1);
    data.extend_from_slice(&0_u32.to_le_bytes());
    data.extend_from_slice(node.as_bytes());
    data.extend_from_slice(voter.as_bytes());
    data.extend_from_slice(withdrawer.as_bytes());
    data.push(COMMISSION);

    data
}

fn instructions(plan: InstructionPlan) -> Result<Vec<Instruction>, Error> {
    plan.flatten()
        .into_iter()
        .map(|leaf| match leaf {
            InstructionLeaf::Instruction(instruction) => Ok(instruction.clone()),
            InstructionLeaf::Packer(_) => Err(Error::Client(
                "stake plan unexpectedly contained a packer leaf",
            )),
        })
        .collect()
}

async fn decode_stake(
    env: &IntegrationEnv,
    address: Address,
) -> Result<solana_kit_stake::StakeStateAccount, Error> {
    let account = env.account(address).await?.expect("stake account exists");
    let decoded = decode_account(account, CODEC_LIMITS)?;

    Ok(decoded.data)
}

async fn expect_second_deactivation_rejection(
    env: &IntegrationEnv,
    stake: Address,
) -> Result<(), Error> {
    let deactivate = DeactivateAccounts {
        stake,
        clock_sysvar: CLOCK_ADDRESS,
        stake_authority: env.payer_address(),
    }
    .instruction(PROGRAM, &Deactivate {}, &[], CODEC_LIMITS)?;
    let error = env
        .submit(&[deactivate], &[env.payer()])
        .await
        .expect_err("an already deactivating stake account cannot deactivate twice");

    expect_custom_error(&error, 0, 2);
    println!("stake: second deactivation rejected with custom error 2 (AlreadyDeactivated)");

    Ok(())
}

#[tokio::test]
async fn creates_delegates_and_deactivates_stake_with_a_vote_fixture() {
    let mut env = util::start().await;
    let result = run(&mut env).await;
    let cleanup = env.close().await;

    util::finish(result, cleanup);
}
