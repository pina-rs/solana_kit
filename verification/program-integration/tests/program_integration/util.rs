//! Shared start/finish helpers so every failure still reaps its owned node.

use solana_kit_program_integration::{Error, IntegrationEnv};

/// Starts the owned validator or fails loudly with its original cause.
pub async fn start() -> IntegrationEnv {
    IntegrationEnv::start()
        .await
        .expect("owned Agave 4.2 validator must start and fund its payer")
}

/// Starts the owned validator with exact pinned external artifacts.
pub async fn start_with(
    programs: &[solana_kit_program_integration::fixtures::ExternalProgram],
) -> IntegrationEnv {
    IntegrationEnv::start_with(programs)
        .await
        .expect("owned Agave 4.2 validator and pinned program artifacts must start")
}

/// Starts with exact local genesis accounts built from the fresh payer.
pub async fn start_with_accounts<F>(
    programs: &[solana_kit_program_integration::fixtures::ExternalProgram],
    accounts: F,
) -> IntegrationEnv
where
    F: FnOnce(
            solana_kit_core::Address,
        )
            -> Result<Vec<solana_kit_program_integration::fixtures::LocalAccount>, Error>
        + Send
        + 'static,
{
    IntegrationEnv::start_with_accounts(programs, accounts)
        .await
        .expect("owned Agave 4.2 validator, artifacts and local accounts must start")
}

/// Starts with verified artifacts deployed at caller-selected addresses.
pub async fn start_with_overrides(
    overrides: &[(
        solana_kit_program_integration::fixtures::ExternalProgram,
        solana_kit_core::Address,
    )],
) -> IntegrationEnv {
    IntegrationEnv::start_with_overrides(overrides)
        .await
        .expect("owned Agave 4.2 validator and overridden pinned program artifacts must start")
}

/// Reports the journey result and the independent cleanup result together.
pub fn finish(journey: Result<(), Error>, cleanup: Result<(), Error>) {
    match (journey, cleanup) {
        (Ok(()), Ok(())) => {}
        (Err(journey), Ok(())) => panic!("journey failed: {journey}"),
        (Ok(()), Err(cleanup)) => panic!("cleanup failed: {cleanup}"),
        (Err(journey), Err(cleanup)) => {
            panic!("journey failed: {journey}; cleanup also failed: {cleanup}")
        }
    }
}
