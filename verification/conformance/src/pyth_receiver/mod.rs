//! Receiver bytes, fixed account privileges and optional PDA crypto on every host.
mod derivation;
use crate::{CaseResult, case, require};
use alloc::vec;
use solana_kit_core::{
    AccountRole, Address,
    codecs::{Error, Limits},
};
use solana_kit_pyth::*;

const VAA: &[u8] = include_bytes!("../../../../crates/solana_kit_pyth/tests/fixtures/vaa.bin");
const ATOMIC: &[u8] =
    include_bytes!("../../../../crates/solana_kit_pyth/tests/fixtures/receiver-atomic.bin");
const POSTED: &[u8] =
    include_bytes!("../../../../crates/solana_kit_pyth/tests/fixtures/receiver-posted.bin");
const ATOMIC_OVERRIDE: &[u8] = include_bytes!(
    "../../../../crates/solana_kit_pyth/tests/fixtures/receiver-atomic-override.bin"
);
const POSTED_OVERRIDE: &[u8] = include_bytes!(
    "../../../../crates/solana_kit_pyth/tests/fixtures/receiver-posted-override.bin"
);

const LIMITS: Limits = Limits::new(4096, 32);
const ACCOUNT_COUNT: usize = 7;
const PREFIX_BYTES: usize = 8;

const PROOF_BYTES: usize = 20;
const FIRST_PROOF: u8 = 0xa1;
const SECOND_PROOF: u8 = 0xa2;

pub(crate) fn cases() -> [CaseResult; 3] {
    [
        case("pyth-receiver-Borsh-and-account-privileges", wire()),
        case("pyth-receiver-PDAs-and-Anchor-domain", derivation::run()),
        case("pyth-receiver-budgets-and-caller-entropy", budgets()),
    ]
}

fn update() -> MerklePriceUpdate<'static> {
    MerklePriceUpdate {
        message: &[9, 8, 7],
        proof: vec![[FIRST_PROOF; PROOF_BYTES], [SECOND_PROOF; PROOF_BYTES]],
    }
}

fn accounts() -> ReceiverAccounts {
    let mut accounts = ReceiverAccounts::new(
        Address::from_bytes([1; 32]),
        Address::from_bytes([3; 32]),
        Address::from_bytes([4; 32]),
        Address::from_bytes([7; 32]),
    );
    accounts.program_address = Address::from_bytes([8; 32]);
    accounts.write_authority = Address::from_bytes([2; 32]);
    accounts
}

/// Complete parameter bytes and both instruction variants use independent Dart output.
fn wire() -> Result<(), &'static str> {
    let update = update();
    let atomic = PostUpdateAtomicParams::new(VAA, &update);
    let posted = PostUpdateParams::new(&update);
    require(
        atomic.encode(LIMITS).map_err(|_| "atomic params")? == ATOMIC[PREFIX_BYTES..],
        "atomic parameter bytes differ",
    )?;
    require(
        posted.encode(LIMITS).map_err(|_| "posted params")? == POSTED[PREFIX_BYTES..],
        "posted parameter bytes differ",
    )?;
    let atomic = PostUpdateAtomicParams {
        treasury_id: u8::MAX,
        ..atomic
    };
    let posted = PostUpdateParams {
        treasury_id: u8::MAX,
        ..posted
    };
    let variants = [
        (
            ReceiverInstruction::PostUpdateAtomic {
                accounts: accounts(),
                guardian_set: Address::from_bytes([5; 32]),
                params: atomic,
            },
            ATOMIC_OVERRIDE,
            Address::from_bytes([5; 32]),
        ),
        (
            ReceiverInstruction::PostUpdate {
                accounts: accounts(),
                encoded_vaa: Address::from_bytes([6; 32]),
                params: posted,
            },
            POSTED_OVERRIDE,
            Address::from_bytes([6; 32]),
        ),
    ];

    for (variant, expected, verification) in variants {
        check_instruction(variant, expected, verification)?;
    }

    Ok(())
}

/// Checks both data and the complete account list for one independent vector.
fn check_instruction(
    variant: ReceiverInstruction<'_>,
    expected: &[u8],
    verification: Address,
) -> Result<(), &'static str> {
    let instruction = variant
        .instruction(LIMITS)
        .map_err(|_| "receiver instruction")?;
    require(
        instruction.data == expected,
        "receiver instruction bytes differ",
    )?;
    require(
        instruction.program_address == Address::from_bytes([8; 32]),
        "receiver custom program lost",
    )?;
    require(
        instruction.accounts.len() == ACCOUNT_COUNT,
        "receiver account count differs",
    )?;
    let expected = expected_accounts(verification);
    require(
        instruction
            .accounts
            .iter()
            .map(|meta| (meta.address, meta.role))
            .eq(expected),
        "receiver account order or privilege differs",
    )?;

    Ok(())
}

/// Independent expected privilege array never calls the production metadata builder.
fn expected_accounts(verification: Address) -> [(Address, AccountRole); 7] {
    [
        (Address::from_bytes([1; 32]), AccountRole::WritableSigner),
        (verification, AccountRole::Readonly),
        (Address::from_bytes([3; 32]), AccountRole::Readonly),
        (Address::from_bytes([4; 32]), AccountRole::Writable),
        (Address::from_bytes([7; 32]), AccountRole::WritableSigner),
        (SYSTEM_PROGRAM, AccountRole::Readonly),
        (Address::from_bytes([2; 32]), AccountRole::ReadonlySigner),
    ]
}

/// Structural proof limits and bounded wire output reject before an instruction returns.
fn budgets() -> Result<(), &'static str> {
    let update = update();
    let params = PostUpdateAtomicParams::new(VAA, &update);
    let instruction = ReceiverInstruction::PostUpdateAtomic {
        accounts: accounts(),
        guardian_set: Address::from_bytes([5; 32]),
        params,
    };
    let items = update.proof.len() + ACCOUNT_COUNT;
    require(
        instruction.instruction(Limits::new(ATOMIC.len(), items - 1)) == Err(Error::LimitExceeded),
        "receiver item budget bypassed",
    )?;
    require(
        instruction.instruction(Limits::new(ATOMIC.len() - 1, items)) == Err(Error::LimitExceeded),
        "receiver byte budget bypassed",
    )?;
    require(
        instruction
            .instruction(Limits::new(ATOMIC.len(), items))
            .map_err(|_| "receiver exact budget")?
            .data
            == ATOMIC,
        "receiver exact budget changed bytes",
    )?;

    let mut calls = 0;
    let treasury = random_treasury_id::<&str>(|byte| {
        calls += 1;
        byte[0] = u8::MAX;
        Ok(())
    })
    .map_err(|_| "caller entropy")?;
    require(
        (treasury, calls) == (u8::MAX, 1),
        "receiver entropy provider contract differs",
    )?;
    require(
        random_treasury_id(|_| Err("entropy denied")) == Err("entropy denied"),
        "receiver entropy failure was lost",
    )
}
