//! Usage absence, capping, error order, and original boundary failures.

use super::*;
use crate::require;
use solana_kit_client::{
    Error,
    methods::submission::{KnownTransactionFailure, TransactionFailure},
    resources::{EstimateConfig, ResourceError, estimate},
};
use solana_kit_core::transaction::{Error as TransactionError, V1Config};

pub(super) fn run() -> Result<(), &'static str> {
    missing_metrics()?;
    transaction_failure()?;
    capped_compute()
}

fn missing_metrics() -> Result<(), &'static str> {
    for version in [
        VersionContext::Legacy,
        VersionContext::V0(&[]),
        VersionContext::V1(V1Config::default()),
    ] {
        for null in [false, true] {
            let requests = RefCell::new(Vec::new());
            let mut response = result();
            response["value"]["err"] = json!("AccountInUse");
            omit(&mut response, "unitsConsumed", null);
            omit(&mut response, "loadedAccountsDataSize", null);
            let client = client(&requests, response);
            let failure = finish(estimate(
                &client,
                &draft(version),
                EstimateConfig::default(),
            ))?;
            require(
                matches!(failure, Err(ResourceError::MissingComputeUsage)),
                "compute absence lost error precedence",
            )?;
            loaded_missing(version, null)?;
        }
    }

    Ok(())
}

fn loaded_missing(version: VersionContext<'_>, null: bool) -> Result<(), &'static str> {
    let requests = RefCell::new(Vec::new());
    let mut response = result();
    omit(&mut response, "loadedAccountsDataSize", null);
    let client = client(&requests, response.clone());
    let actual = finish(estimate(
        &client,
        &draft(version),
        EstimateConfig::default(),
    ))?;

    if matches!(version, VersionContext::V1(_)) {
        return require(
            matches!(actual, Err(ResourceError::MissingLoadedAccountsDataSize)),
            "v1 accepted absent loaded usage",
        );
    }

    let observed = actual.map_err(|_| "legacy loaded absence rejected")?;
    require(
        observed.loaded_accounts_data_size_limit.is_none(),
        "legacy loaded absence invented value",
    )?;
    response["value"]["err"] = json!("AccountInUse");
    let client = super::client(&requests, response);

    require(
        matches!(
            finish(estimate(
                &client,
                &draft(version),
                EstimateConfig::default()
            ))?,
            Err(ResourceError::Transaction(_))
        ),
        "legacy missing loaded hid transaction error",
    )
}

fn transaction_failure() -> Result<(), &'static str> {
    let requests = RefCell::new(Vec::new());
    let mut response = result();
    response["value"]["err"] = json!({"InsufficientFundsForRent": {"account_index": 1}});
    let client = client(&requests, response.clone());
    let failure = finish(estimate(
        &client,
        &draft(VersionContext::V1(V1Config::default())),
        EstimateConfig::default(),
    ))?;
    require(
        matches!(
            failure,
            Err(ResourceError::Transaction(TransactionFailure::Known(
                KnownTransactionFailure::InsufficientFundsForRent { account_index: 1 }
            )))
        ),
        "transaction failure cause was erased",
    )?;
    omit(&mut response, "loadedAccountsDataSize", false);
    let client = super::client(&requests, response);

    require(
        matches!(
            finish(estimate(
                &client,
                &draft(VersionContext::V1(V1Config::default())),
                EstimateConfig::default()
            ))?,
            Err(ResourceError::MissingLoadedAccountsDataSize)
        ),
        "v1 loaded absence lost transaction error precedence",
    )
}

fn capped_compute() -> Result<(), &'static str> {
    for units in [0, u64::from(u32::MAX), u64::from(u32::MAX) + 1, u64::MAX] {
        let requests = RefCell::new(Vec::new());
        let mut response = result();
        response["value"]["unitsConsumed"] = json!(units);
        let client = client(&requests, response);
        let estimate = finish(estimate(
            &client,
            &draft(VersionContext::V1(V1Config::default())),
            EstimateConfig::default(),
        ))?
        .map_err(|_| "valid u64 compute usage rejected")?;
        require(
            estimate.compute_unit_limit == u32::try_from(units).unwrap_or(u32::MAX),
            "compute usage wrapped instead of capping",
        )?;
    }

    Ok(())
}

fn omit(response: &mut Value, field: &str, null: bool) {
    if null {
        response["value"][field] = Value::Null;
    } else if let Some(value) = response["value"].as_object_mut() {
        value.remove(field);
    }
}

pub(super) fn failures() -> Result<(), &'static str> {
    let requests = RefCell::new(Vec::new());
    let client = client(&requests, json!({"unexpected":true}));
    let message = draft(VersionContext::Legacy);
    require(
        matches!(
            finish(estimate(&client, &message, EstimateConfig::default()))?,
            Err(ResourceError::Rpc(Error::Schema(_)))
        ),
        "invalid typed response was not preserved",
    )?;
    let mut unresolved = message.clone();
    unresolved.lifetime = Lifetime::Unresolved(BLOCKHASH);
    require(
        matches!(
            finish(estimate(&client, &unresolved, EstimateConfig::default()))?,
            Err(ResourceError::Compilation(
                TransactionError::UnresolvedLifetime
            ))
        ),
        "unresolved lifetime did not fail locally",
    )?;
    require(
        requests.borrow().len() == 1,
        "invalid local input exchanged RPC",
    )?;
    boundary_failure(message)
}

struct Failing;

impl Transport for Failing {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, _: Request) -> Self::Exchange<'_> {
        ready(Err(TransportError::new(TransportFailureKind::Deadline)))
    }
}

fn boundary_failure(message: ResourceMessage<'_>) -> Result<(), &'static str> {
    let client = Client::new(Failing, Limits::default());
    let failure = finish(estimate(&client, &message, EstimateConfig::default()))?;

    require(
        matches!(
            failure,
            Err(ResourceError::Rpc(Error::Transport(TransportError {
                kind: TransportFailureKind::Deadline,
                ..
            })))
        ),
        "transport failure cause was erased",
    )
}
