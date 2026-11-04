//! Independent owned validators must coexist without sharing a gossip port.

use crate::util;
use solana_kit_program_integration::{Error, IntegrationEnv};

#[tokio::test]
async fn two_owned_validators_keep_distinct_identities_and_ledgers() {
    let first = util::start().await;

    let (result, second_cleanup) = match IntegrationEnv::start().await {
        Ok(second) => {
            let result = isolated(&first, &second).await;
            let cleanup = second.close().await;

            (result, cleanup)
        }
        Err(error) => (Err(error), Ok(())),
    };

    let first_cleanup = first.close().await;

    util::finish(result, combine_cleanup(first_cleanup, second_cleanup));
}

/// Challenge both clients while both children are alive, including cross-ledger absence.
async fn isolated(first: &IntegrationEnv, second: &IntegrationEnv) -> Result<(), Error> {
    if first.validator_identity == second.validator_identity
        || first.genesis == second.genesis
        || first.payer_address() == second.payer_address()
    {
        return Err(Error::Client("owned validators reused identity or genesis"));
    }

    if first.balance(first.payer_address()).await? == 0
        || second.balance(second.payer_address()).await? == 0
        || first.balance(second.payer_address()).await? != 0
        || second.balance(first.payer_address()).await? != 0
    {
        return Err(Error::Client("owned validator ledgers are not isolated"));
    }

    Ok(())
}

/// Neither cleanup failure may hide the other or the original journey result.
fn combine_cleanup(first: Result<(), Error>, second: Result<(), Error>) -> Result<(), Error> {
    match (first, second) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(first), Err(second)) => Err(Error::Program(format!(
            "first validator cleanup: {first}; second validator cleanup: {second}"
        ))),
        (Err(error), Ok(())) | (Ok(()), Err(error)) => Err(error),
    }
}
