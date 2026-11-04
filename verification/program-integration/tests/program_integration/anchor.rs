//! Real Anchor program: IDL-encoded initialize, increment and constraint error.

use crate::util;
use solana_kit_anchor::{Coder, Idl, Limits, Value, Values, program_error};
use solana_kit_core::{AccountMeta, AccountRole, Address, Instruction, crypto::Keypair};
use solana_kit_program_integration::{
    Error, IntegrationEnv, expect_custom_error, fixtures::ANCHOR_COMPATIBILITY,
};
use solana_kit_system::SYSTEM_PROGRAM;
use std::{fs, path::PathBuf};

const LIMITS: Limits = Limits::new(1 << 20, 4_096, 64);
const INITIAL_VALUE: u64 = 41;
const INCREMENT: u64 = 5;
const CONSTRAINT_ERROR: u32 = 2001;

pub async fn run(env: &mut IntegrationEnv) -> Result<(), Error> {
    let idl = read_idl(env)?;
    let coder = Coder::new(&idl, LIMITS);
    let payer = env.payer_address();
    let counter = initialize_counter(env, &coder, payer).await?;
    increment_counter(env, &coder, counter.address(), payer).await?;

    expect_constraint_rejection(env, &coder, &idl, counter.address(), payer).await
}

fn read_idl(env: &IntegrationEnv) -> Result<Idl, Error> {
    let path = idl_path(env);
    let idl = anchor(Idl::parse(&fs::read_to_string(path)?, LIMITS))?;

    assert_eq!(
        idl.address, ANCHOR_COMPATIBILITY.program_id,
        "IDL and deployment addresses must agree"
    );

    Ok(idl)
}

async fn initialize_counter(
    env: &mut IntegrationEnv,
    coder: &Coder<'_>,
    payer: Address,
) -> Result<Keypair, Error> {
    let counter = env.new_key().await?;
    let initialize = Instruction {
        program_address: ANCHOR_COMPATIBILITY.address(),
        accounts: vec![
            meta(counter.address(), AccountRole::WritableSigner),
            meta(payer, AccountRole::WritableSigner),
            meta(SYSTEM_PROGRAM, AccountRole::Readonly),
        ],
        data: anchor(coder.encode_instruction("initialize", &initialize_values()))?,
    };
    let signature = env.submit(&[initialize], &[env.payer(), &counter]).await?;

    let created = decode_counter(env, coder, counter.address()).await?;
    assert_eq!(field(&created, "authority"), &Value::Address(payer));
    assert_eq!(
        field(&created, "value"),
        &Value::Unsigned(INITIAL_VALUE.into())
    );
    assert_eq!(
        field(&created, "label"),
        &Value::String("surfpool".to_owned())
    );

    let events = anchor(coder.decode_event_logs(&env.logs(signature).await?))?;
    assert_eq!(events.len(), 1, "exactly one event must be eligible");
    assert_eq!(events[0].name, "CounterChanged");
    println!(
        "anchor: initialize {signature} decoded Counter value={INITIAL_VALUE}; one CounterChanged event"
    );

    Ok(counter)
}

async fn increment_counter(
    env: &IntegrationEnv,
    coder: &Coder<'_>,
    counter: Address,
    payer: Address,
) -> Result<(), Error> {
    let increment = Instruction {
        program_address: ANCHOR_COMPATIBILITY.address(),
        accounts: vec![
            meta(counter, AccountRole::Writable),
            meta(payer, AccountRole::ReadonlySigner),
        ],
        data: anchor(coder.encode_instruction("increment", &increment_values(INCREMENT)))?,
    };
    let signature = env.submit(&[increment], &[env.payer()]).await?;

    let updated = decode_counter(env, coder, counter).await?;
    assert_eq!(
        field(&updated, "value"),
        &Value::Unsigned((INITIAL_VALUE + INCREMENT).into())
    );
    let events = anchor(coder.decode_event_logs(&env.logs(signature).await?))?;
    assert_eq!(events.len(), 1);
    assert_eq!(
        field(&events[0].data, "previous_value"),
        &Value::Unsigned(INITIAL_VALUE.into())
    );
    println!(
        "anchor: increment {signature} moved the decoded value to {}",
        INITIAL_VALUE + INCREMENT
    );

    Ok(())
}

fn anchor<T>(result: Result<T, solana_kit_anchor::Error>) -> Result<T, Error> {
    result.map_err(|error| Error::Program(format!("anchor: {error}")))
}

fn idl_path(env: &IntegrationEnv) -> PathBuf {
    env.fixture_dir()
        .expect("Anchor environment loaded a fixture directory")
        .join("anchor_compatibility-v0.1.0.json")
}

fn initialize_values() -> Values {
    let mut values = Values::new();
    values.insert(
        "initial_value".to_owned(),
        Value::Unsigned(INITIAL_VALUE.into()),
    );
    values.insert("label".to_owned(), Value::String("surfpool".to_owned()));

    values
}

fn increment_values(amount: u64) -> Values {
    let mut values = Values::new();
    values.insert("amount".to_owned(), Value::Unsigned(amount.into()));

    values
}

fn meta(address: Address, role: AccountRole) -> AccountMeta {
    AccountMeta { address, role }
}

async fn decode_counter(
    env: &IntegrationEnv,
    coder: &Coder<'_>,
    address: Address,
) -> Result<Value, Error> {
    let account = env.account(address).await?.expect("counter exists");
    let decoded = anchor(coder.decode_account("Counter", &account.data))?;

    Ok(decoded.data)
}

fn field<'a>(value: &'a Value, name: &str) -> &'a Value {
    let Value::Struct(fields) = value else {
        panic!("expected a decoded struct, got {value:?}");
    };

    fields
        .get(name)
        .unwrap_or_else(|| panic!("decoded struct has no {name} field"))
}

async fn expect_constraint_rejection(
    env: &mut IntegrationEnv,
    coder: &Coder<'_>,
    idl: &Idl,
    counter: Address,
    authority: Address,
) -> Result<(), Error> {
    let attacker = env.new_key().await?;
    let increment = Instruction {
        program_address: ANCHOR_COMPATIBILITY.address(),
        accounts: vec![
            meta(counter, AccountRole::Writable),
            meta(attacker.address(), AccountRole::ReadonlySigner),
        ],
        data: anchor(coder.encode_instruction("increment", &increment_values(1)))?,
    };
    let error = env
        .submit(&[increment], &[env.payer(), &attacker])
        .await
        .expect_err("a foreign authority must violate the has_one constraint");

    expect_custom_error(&error, 0, CONSTRAINT_ERROR);
    let resolved = program_error(i64::from(CONSTRAINT_ERROR), Some(idl));
    assert_eq!(resolved.name, "ConstraintHasOne");

    let unchanged = decode_counter(env, coder, counter).await?;
    assert_eq!(
        field(&unchanged, "value"),
        &Value::Unsigned((INITIAL_VALUE + INCREMENT).into())
    );
    assert_eq!(
        stored_authority(&unchanged),
        authority,
        "failed transaction preserved the stored authority"
    );
    println!(
        "anchor: foreign authority rejected with {} ({}) and state stayed unchanged",
        resolved.code, resolved.name
    );

    Ok(())
}

fn stored_authority(value: &Value) -> Address {
    match field(value, "authority") {
        Value::Address(address) => *address,
        other => panic!("expected an authority address, got {other:?}"),
    }
}

#[tokio::test]
async fn executes_anchor_idl_instructions_and_rejects_a_foreign_authority() {
    let mut env = util::start_with(&[ANCHOR_COMPATIBILITY]).await;
    let result = run(&mut env).await;
    let cleanup = env.close().await;

    util::finish(result, cleanup);
}
