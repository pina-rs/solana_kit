//! A loader-valid Anchor artifact must still reject a noncanonical program ID.

use crate::util;
use solana_kit_anchor::{Coder, Idl, Limits, Value, Values};
use solana_kit_core::{AccountMeta, AccountRole, Address, Instruction};
use solana_kit_program_integration::{
    Error, IntegrationEnv, expect_custom_error, fixtures::ANCHOR_COMPATIBILITY,
};
use solana_kit_system::SYSTEM_PROGRAM;
use std::fs;

const LIMITS: Limits = Limits::new(1 << 20, 4_096, 64);
const MISMATCH_ID: Address = Address::from_bytes([0x5a; 32]);
const MISMATCH_ERROR: u32 = 4_100;

pub async fn run(env: &mut IntegrationEnv) -> Result<(), Error> {
    let idl = read_idl(env)?;
    let coder = Coder::new(&idl, LIMITS);
    let counter = env.new_key().await?;
    let initialize = initialize_instruction(env, &coder, counter.address())?;
    let error = env
        .submit(&[initialize], &[env.payer(), &counter])
        .await
        .expect_err("the deployed Anchor image rejects its mismatched program ID");

    expect_custom_error(&error, 0, MISMATCH_ERROR);
    let resolved = solana_kit_anchor::program_error(i64::from(MISMATCH_ERROR), None);
    assert_eq!(resolved.name, "DeclaredProgramIdMismatch");
    println!(
        "anchor_mismatch: executable image at {MISMATCH_ID} rejected initialize with 4100 ({})",
        resolved.name
    );

    Ok(())
}

fn read_idl(env: &IntegrationEnv) -> Result<Idl, Error> {
    let path = env
        .fixture_dir()
        .expect("override environment loaded a fixture directory")
        .join("anchor_compatibility-v0.1.0.json");
    let idl = Idl::parse(&fs::read_to_string(path)?, LIMITS)
        .map_err(|error| Error::Program(format!("anchor mismatch IDL: {error}")))?;

    assert_eq!(
        idl.address, ANCHOR_COMPATIBILITY.program_id,
        "the artifact still declares its canonical ID"
    );

    Ok(idl)
}

fn initialize_instruction(
    env: &IntegrationEnv,
    coder: &Coder<'_>,
    counter: Address,
) -> Result<Instruction, Error> {
    let mut values = Values::new();
    values.insert("initial_value".to_owned(), Value::Unsigned(1));
    values.insert("label".to_owned(), Value::String("mismatch".to_owned()));
    let data = coder
        .encode_instruction("initialize", &values)
        .map_err(|error| Error::Program(format!("anchor mismatch data: {error}")))?;

    Ok(Instruction {
        program_address: MISMATCH_ID,
        accounts: vec![
            meta(counter, AccountRole::WritableSigner),
            meta(env.payer_address(), AccountRole::WritableSigner),
            meta(SYSTEM_PROGRAM, AccountRole::Readonly),
        ],
        data,
    })
}

fn meta(address: Address, role: AccountRole) -> AccountMeta {
    AccountMeta { address, role }
}

#[tokio::test]
async fn rejects_a_declared_program_id_mismatch_after_deploy() {
    let mut env = util::start_with_overrides(&[(ANCHOR_COMPATIBILITY, MISMATCH_ID)]).await;
    let result = run(&mut env).await;
    let cleanup = env.close().await;

    util::finish(result, cleanup);
}
