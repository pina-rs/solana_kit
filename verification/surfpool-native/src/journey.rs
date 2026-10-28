//! Actual typed requests against the challenge-bound live child.

use crate::{
    Error, Result,
    owner::{Owner, transport},
    require,
};
use solana_kit_client::{
    Client, Limits, TransportFailureKind,
    methods::{
        AccountInfo, Balance, Commitment, ReadConfig,
        node::{GenesisHash, GetEpochInfo},
    },
};
use solana_kit_core::{Address, accounts::MaybeAccount};
use solana_kit_surfpool::{
    Surfnet,
    builders::SetAccount,
    cheatcodes::{GetSurfnetInfo, PauseClock, ResumeClock, TimeTravel, WriteProgram},
    hex::HexBytes,
    types::TimeTarget,
};
use std::time::{Duration, Instant};

const STARTUP: Duration = Duration::from_secs(30);
const POLL: Duration = Duration::from_millis(100);
const TEST_BALANCE: u64 = 20_000_000;
const RAW_PROGRAM_BYTES: usize = 8 * 1024 * 1024;
const PROBE_WIRE_BYTES: usize = 18 * 1024 * 1024;
const PAYLOAD_TOO_LARGE: u16 = 413;
const DATA: &[u8] = &[1, 3, 5, 7];

/// Exercises the real supported surface, then returns ownership to awaited close.
pub(crate) async fn run(owner: &mut Owner) -> Result<()> {
    ready(owner).await?;
    owner.verified = true;
    println!("PASS fresh genesis and startup ready while owned child lives");
    account(owner).await?;
    println!("PASS validated funded payer and exact account readback");
    clock(owner).await?;
    println!("PASS pause, time travel, normal epoch read and resume");

    if owner.config.body_probe {
        body_cap(owner).await?;
    }

    owner.alive()
}

/// An unrelated healthy server cannot satisfy a fresh genesis challenge.
async fn ready(owner: &Owner) -> Result<()> {
    let deadline = Instant::now() + STARTUP;

    loop {
        owner.alive()?;

        if probe(owner).await? {
            owner.alive()?;

            return Ok(());
        }

        if Instant::now() >= deadline {
            return Err(Error::StartupDeadline);
        }

        tokio::time::sleep(POLL).await;
    }
}

/// Only connection/deadline failures are startup-pending; other protocol errors propagate.
async fn probe(owner: &Owner) -> Result<bool> {
    let genesis = match owner.surfnet.rpc().call(GenesisHash).await {
        Ok(genesis) => genesis,
        Err(solana_kit_client::Error::Transport(error))
            if matches!(
                error.kind,
                TransportFailureKind::Io | TransportFailureKind::Deadline
            ) =>
        {
            return Ok(false);
        }
        Err(error) => return Err(error.into()),
    };

    require(
        genesis == owner.config.genesis,
        "fresh genesis differs from the owned child challenge",
    )?;
    let info = owner.surfnet.execute(GetSurfnetInfo).await?;
    startup(&info.value)
}

/// Inspects the actual 1.7 status projection, including planning and task failures.
fn startup(info: &serde_json::Value) -> Result<bool> {
    let status = info
        .get("startup")
        .ok_or(Error::Check("missing startup object"))?;
    let phase = status
        .get("phase")
        .and_then(serde_json::Value::as_str)
        .ok_or(Error::Check("missing Surfnet startup phase"))?;
    require(phase != "failed", "Surfnet startup reported failure")?;
    require(
        status.get("error").is_some_and(serde_json::Value::is_null),
        "Surfnet startup has an error",
    )?;
    let tasks = status
        .get("tasks")
        .and_then(serde_json::Value::as_array)
        .ok_or(Error::Check("missing startup tasks"))?;
    require(
        !tasks.iter().any(|task| {
            task.get("state").and_then(serde_json::Value::as_str) == Some("failed")
                || task.get("error").is_some_and(|error| !error.is_null())
        }),
        "Surfnet startup task failed",
    )?;

    if phase == "ready" {
        require(
            status
                .get("planSealed")
                .and_then(serde_json::Value::as_bool)
                == Some(true),
            "ready startup plan is not sealed",
        )?;
    }

    Ok(phase == "ready")
}

/// Validated payer funding and exact binary account bytes use the shared Kit client.
async fn account(owner: &Owner) -> Result<()> {
    owner.alive()?;
    let payer = owner.payer.address();
    let initial = owner.surfnet.rpc().call(Balance(payer, None)).await?;
    require(initial.value > 0, "validated test payer was not funded")?;
    owner
        .surfnet
        .execute(
            SetAccount::new(payer)
                .with_lamports(TEST_BALANCE)
                .with_data(DATA.to_vec()),
        )
        .await?;
    let snapshot = owner
        .surfnet
        .rpc()
        .call(AccountInfo(payer, ReadConfig::default()))
        .await?;
    let MaybeAccount::Present(snapshot) = snapshot.value else {
        return Err(Error::Check("mutated payer account missing"));
    };

    require(
        snapshot.lamports == TEST_BALANCE && snapshot.data == DATA,
        "exact account lamports/data readback",
    )?;
    owner.alive()
}

/// Pauses, advances and checks the clock through actual typed cheatcodes and RPC.
async fn clock(owner: &Owner) -> Result<()> {
    owner.alive()?;
    let paused = owner.surfnet.execute(PauseClock).await?;
    let absolute = u64::try_from(paused.value.absolute_slot)
        .map_err(|_| Error::Check("negative clock slot"))?;
    let target = absolute
        .checked_add(10)
        .ok_or(Error::Check("clock slot overflow"))?;
    let travelled = owner
        .surfnet
        .execute(TimeTravel(TimeTarget::AbsoluteSlot(target)))
        .await?;
    require(
        travelled.value.absolute_slot == i128::from(target),
        "time travel confirmation slot",
    )?;
    // A paused open slot has not finalized. Read the processed bank explicitly.
    let epoch = owner
        .surfnet
        .rpc()
        .call(GetEpochInfo(Some(ReadConfig {
            commitment: Some(Commitment::Processed),
            ..ReadConfig::default()
        })))
        .await?;
    require(
        epoch.absolute_slot == target,
        "paused clock absolute slot readback",
    )?;
    owner.surfnet.execute(ResumeClock).await?;

    owner.alive()
}

/// Optional destructive-test address remains absent when the real server rejects the body.
async fn body_cap(owner: &Owner) -> Result<()> {
    owner.alive()?;
    let program = Address::from_bytes([3; 32]);
    absent(owner, program).await?;
    let limits = Limits {
        bytes: PROBE_WIRE_BYTES,
        ..Limits::default()
    };
    let surfnet = Surfnet::new(Client::new(transport(&owner.config.endpoint())?, limits));
    let request = WriteProgram {
        program_id: program,
        data: HexBytes(vec![0; RAW_PROGRAM_BYTES]),
        offset: 0,
        authority: Some(owner.payer.address()),
    };

    let error = match surfnet.execute(request).await {
        Err(error) => error,
        Ok(_) => return Err(Error::Check("oversized body unexpectedly accepted")),
    };

    if error.status_code() != Some(PAYLOAD_TOO_LARGE) {
        return Err(error.into());
    }

    absent(owner, program).await?;
    println!("PASS actual HTTP 413 for 8 MiB raw program; account remains absent");
    owner.alive()
}

/// Checks both before and after the failed write without accepting a decode fallback.
async fn absent(owner: &Owner, address: Address) -> Result<()> {
    let snapshot = owner
        .surfnet
        .rpc()
        .call(AccountInfo(address, ReadConfig::default()))
        .await?;
    require(
        matches!(snapshot.value, MaybeAccount::Missing(_)),
        "body probe account must remain absent",
    )
}
