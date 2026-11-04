//! Real loader v3 deploy and upgrade with pinned ELF bytes, plus the Anchor ID guard.

use crate::util;
use solana_kit_core::{
    Address, Instruction,
    codecs::Limits as CodecLimits,
    crypto::{Keypair, find_program_address},
    sysvars::{CLOCK_ADDRESS, RENT_ADDRESS},
};
use solana_kit_loader::{
    BUFFER_HEADER_SIZE, PROGRAM_DATA_HEADER_SIZE, UpgradeableState, decode_upgradeable_account,
    v3::{
        self, DeployWithMaxDataLenAccounts, InitializeBuffer, InitializeBufferAccounts,
        UpgradeAccounts,
    },
};
use solana_kit_program_integration::{
    CODEC_LIMITS, Error, IntegrationEnv,
    fixtures::{FixtureDirectory, NOOP},
};
use solana_kit_system::{CreateAccount, CreateAccountAccounts, SYSTEM_PROGRAM};

const WRITE_CHUNK: usize = 1_000;
const PROGRAM_DATA_LIMITS: CodecLimits = CodecLimits::new(4 << 20, 4 << 20);
const EXECUTABLE_WAIT: std::time::Duration = std::time::Duration::from_secs(30);
const EXECUTABLE_POLL: std::time::Duration = std::time::Duration::from_millis(100);
const PROGRAM_ACCOUNT_BYTES: u64 = 36;

pub async fn run(env: &mut IntegrationEnv, fixtures: &FixtureDirectory) -> Result<(), Error> {
    let loaded = fixtures.load(&[NOOP])?;
    let noop = &loaded[0];

    let program = env.new_key().await?;
    let program_data = deploy(env, &program, &noop.elf).await?;
    assert_executable(env, program.address()).await?;
    assert_program_image(env, program_data, &noop.elf).await?;
    invoke_noop(env, program.address()).await?;
    println!(
        "loader: deployed pinned {} ({} bytes) at {} with ProgramData {program_data}",
        noop.program.artifact,
        noop.elf.len(),
        program.address()
    );

    let upgraded_buffer = env.new_key().await?;
    create_buffer(env, &upgraded_buffer, noop.elf.len()).await?;
    let upgrade_accounts = UpgradeAccounts {
        program_data_account: program_data,
        program_account: program.address(),
        buffer_account: upgraded_buffer.address(),
        spill_account: env.payer_address(),
        rent_sysvar: RENT_ADDRESS,
        clock_sysvar: CLOCK_ADDRESS,
        authority: env.payer_address(),
    };
    let plan = upgrade_accounts.plan(&noop.elf, WRITE_CHUNK, CODEC_LIMITS)?;
    submit_plan(env, plan).await?;

    let state = decode_program_data(env, program_data).await?;
    assert!(matches!(
        state,
        UpgradeableState::ProgramData {
            authority: Some(authority),
            ..
        } if authority == env.payer_address()
    ));
    assert_program_image(env, program_data, &noop.elf).await?;
    invoke_noop(env, program.address()).await?;
    println!("loader: upgraded the same program with pinned bytes and re-invoked it");

    Ok(())
}

async fn deploy(env: &mut IntegrationEnv, program: &Keypair, elf: &[u8]) -> Result<Address, Error> {
    let program_data =
        find_program_address(&[program.address().as_bytes()], &v3::PROGRAM)?.address();
    let buffer = env.new_key().await?;
    create_buffer(env, &buffer, elf.len()).await?;

    let create_program = CreateAccount {
        lamports: env.rent(PROGRAM_ACCOUNT_BYTES).await?,
        space: PROGRAM_ACCOUNT_BYTES,
        program_address: v3::PROGRAM,
    }
    .instruction(
        CreateAccountAccounts {
            payer: env.payer_address(),
            new_account: program.address(),
        },
        CODEC_LIMITS,
    )?;
    env.submit(&[create_program], &[env.payer(), program])
        .await?;

    let accounts = DeployWithMaxDataLenAccounts {
        payer_account: env.payer_address(),
        program_data_account: program_data,
        program_account: program.address(),
        buffer_account: buffer.address(),
        rent_sysvar: RENT_ADDRESS,
        clock_sysvar: CLOCK_ADDRESS,
        system_program: SYSTEM_PROGRAM,
        authority: env.payer_address(),
    };
    let plan = accounts.plan(elf, Some(elf.len() as u64), WRITE_CHUNK, CODEC_LIMITS)?;
    println!(
        "loader: deploying {} bytes with {} chunks via {} instructions",
        elf.len(),
        elf.len().div_ceil(WRITE_CHUNK),
        plan.flatten().len()
    );
    submit_plan(env, plan).await?;

    Ok(program_data)
}

async fn create_buffer(
    env: &mut IntegrationEnv,
    buffer: &Keypair,
    payload: usize,
) -> Result<(), Error> {
    let space = (BUFFER_HEADER_SIZE + payload) as u64;
    let create = CreateAccount {
        lamports: env.rent(space).await?,
        space,
        program_address: v3::PROGRAM,
    }
    .instruction(
        CreateAccountAccounts {
            payer: env.payer_address(),
            new_account: buffer.address(),
        },
        CODEC_LIMITS,
    )?;
    let initialize = InitializeBufferAccounts {
        source_account: buffer.address(),
        buffer_authority: env.payer_address(),
    }
    .instruction(v3::PROGRAM, &InitializeBuffer {}, &[], CODEC_LIMITS)?;
    env.submit(&[create, initialize], &[env.payer(), buffer])
        .await?;

    Ok(())
}

async fn submit_plan(
    env: &IntegrationEnv,
    plan: solana_kit_core::plans::InstructionPlan,
) -> Result<(), Error> {
    for (index, leaf) in plan.flatten().into_iter().enumerate() {
        let solana_kit_core::plans::InstructionLeaf::Instruction(instruction) = leaf else {
            return Err(Error::Client("loader plan contained a packer leaf"));
        };
        env.submit(std::slice::from_ref(instruction), &[env.payer()])
            .await
            .map_err(|error| {
                Error::Program(format!(
                    "loader instruction {index} of {} failed: {error}",
                    plan.flatten().len()
                ))
            })?;
    }

    Ok(())
}

// Confirmed status can precede the account RPC cache observing executability.
async fn assert_executable(env: &IntegrationEnv, program: Address) -> Result<(), Error> {
    let deadline = std::time::Instant::now() + EXECUTABLE_WAIT;

    let account = loop {
        if let Some(account) = env.account(program).await?
            && account.executable
        {
            break account;
        }

        if std::time::Instant::now() >= deadline {
            return Err(Error::Client("deployed program never became executable"));
        }

        tokio::time::sleep(EXECUTABLE_POLL).await;
    };

    assert_eq!(account.owner, v3::PROGRAM);
    let decoded = decode_upgradeable_account(account, CODEC_LIMITS)?;
    assert!(matches!(decoded.data, UpgradeableState::Program { .. }));

    Ok(())
}

async fn assert_program_image(
    env: &IntegrationEnv,
    program_data: Address,
    elf: &[u8],
) -> Result<(), Error> {
    let account = env
        .account(program_data)
        .await?
        .expect("ProgramData exists");
    assert_eq!(account.owner, v3::PROGRAM);
    assert_eq!(account.data.get(PROGRAM_DATA_HEADER_SIZE..), Some(elf));
    Ok(())
}

async fn decode_program_data(
    env: &IntegrationEnv,
    program_data: Address,
) -> Result<UpgradeableState, Error> {
    let account = env
        .account(program_data)
        .await?
        .expect("program data exists");
    let decoded = decode_upgradeable_account(account, PROGRAM_DATA_LIMITS)?;

    Ok(decoded.data)
}

async fn invoke_noop(env: &IntegrationEnv, program: Address) -> Result<(), Error> {
    let instruction = Instruction {
        program_address: program,
        accounts: Vec::new(),
        data: vec![1, 2, 3],
    };
    env.submit(&[instruction], &[env.payer()]).await?;

    Ok(())
}

#[tokio::test]
async fn deploys_upgrades_and_invokes_a_pinned_elf() {
    let mut env = util::start().await;

    let result = match FixtureDirectory::resolve() {
        Ok(fixtures) => run(&mut env, &fixtures).await,
        Err(error) => Err(error.into()),
    };

    let cleanup = env.close().await;

    util::finish(result, cleanup);
}
