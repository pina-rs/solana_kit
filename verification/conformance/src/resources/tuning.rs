//! Tuning bypasses explicit limits and only updates applicable provisional fields.

use super::*;
use crate::require;
use solana_kit_client::resources::{
    EstimateConfig, MAX_COMPUTE_UNIT_LIMIT, ResourceError, ResourceLimitsEstimate, TuningError,
    estimate_and_set, estimate_and_set_with_client,
};
use solana_kit_core::transaction::V1Config;

pub(super) fn run() -> Result<(), &'static str> {
    explicit_bypass()?;
    partial_tuning()?;
    custom_estimator()
}

fn explicit_bypass() -> Result<(), &'static str> {
    for version in [
        VersionContext::Legacy,
        VersionContext::V0(&[]),
        VersionContext::V1(V1Config::default()),
    ] {
        let message = draft(version)
            .with_compute_unit_limit(Some(EXPLICIT_COMPUTE))
            .map_err(|_| "explicit compute setup failed")?
            .with_loaded_accounts_data_size_limit(Some(EXPLICIT_LOADED))
            .map_err(|_| "explicit loaded setup failed")?;
        let requests = RefCell::new(Vec::new());
        let client = client(&requests, json!(null));
        let unchanged = finish(estimate_and_set_with_client(
            &client,
            message.clone(),
            EstimateConfig::default(),
        ))?
        .map_err(|_| "explicit bypass failed")?;
        require(
            requests.borrow().is_empty() && unchanged == message,
            "explicit limits spent simulation or changed draft",
        )?;
    }

    for version in [VersionContext::Legacy, VersionContext::V0(&[])] {
        let message = draft(version)
            .with_compute_unit_limit(Some(EXPLICIT_COMPUTE))
            .map_err(|_| "explicit compute setup failed")?;
        let called = core::cell::Cell::new(false);
        let unchanged = finish(estimate_and_set(message.clone(), |_| {
            called.set(true);
            ready(Ok::<_, ()>(usage()))
        }))?
        .map_err(|_| "legacy compute bypass failed")?;
        require(
            !called.get() && unchanged == message,
            "legacy loaded absence prevented bypass",
        )?;
    }

    Ok(())
}

fn partial_tuning() -> Result<(), &'static str> {
    for version in [
        VersionContext::Legacy,
        VersionContext::V0(&[]),
        VersionContext::V1(V1Config::default()),
    ] {
        for initial in [None, Some(0), Some(MAX_COMPUTE_UNIT_LIMIT)] {
            let message = draft(version)
                .with_compute_unit_limit(initial)
                .map_err(|_| "provisional setup failed")?;
            let message = message
                .with_loaded_accounts_data_size_limit(Some(EXPLICIT_LOADED))
                .map_err(|_| "loaded setup failed")?;
            let updated = finish(estimate_and_set(message, |_| ready(Ok::<_, ()>(usage()))))?
                .map_err(|_| "provisional tuning failed")?;
            require(
                updated.compute_unit_limit() == Some(COMPUTE_USAGE),
                "provisional compute did not tune",
            )?;
            require(
                updated.loaded_accounts_data_size_limit() == Some(EXPLICIT_LOADED),
                "tuning changed explicit loaded limit",
            )?;
        }
    }

    let message = draft(VersionContext::V1(V1Config::default()))
        .with_compute_unit_limit(Some(EXPLICIT_COMPUTE))
        .map_err(|_| "partial setup failed")?
        .fill_provisory_resource_limits()
        .map_err(|_| "partial fill failed")?;
    let updated = finish(estimate_and_set(message, |_| ready(Ok::<_, ()>(usage()))))?
        .map_err(|_| "partial v1 tuning failed")?;
    require(
        updated.compute_unit_limit() == Some(EXPLICIT_COMPUTE)
            && updated.loaded_accounts_data_size_limit() == Some(LOADED_USAGE),
        "partial tuning replaced explicit compute or missed loaded",
    )?;
    never_add_legacy_loaded()
}

fn never_add_legacy_loaded() -> Result<(), &'static str> {
    for version in [VersionContext::Legacy, VersionContext::V0(&[])] {
        let updated = finish(estimate_and_set(draft(version), |_| {
            ready(Ok::<_, ()>(usage()))
        }))?
        .map_err(|_| "legacy tune failed")?;
        require(
            updated.compute_unit_limit() == Some(COMPUTE_USAGE)
                && updated.loaded_accounts_data_size_limit().is_none(),
            "legacy tuning added loaded bytes",
        )?;
    }

    let message = draft(VersionContext::V1(V1Config::default()))
        .fill_provisory_resource_limits()
        .map_err(|_| "omitted loaded fill failed")?;
    let updated = finish(estimate_and_set(message, |_| {
        ready(Ok::<_, ()>(ResourceLimitsEstimate {
            loaded_accounts_data_size_limit: None,
            ..usage()
        }))
    }))?
    .map_err(|_| "omitted custom loaded failed")?;

    require(
        updated.loaded_accounts_data_size_limit() == Some(0),
        "omitted custom loaded erased placeholder",
    )
}

fn custom_estimator() -> Result<(), &'static str> {
    let message = draft(VersionContext::V1(V1Config::default()));
    let failure = finish(estimate_and_set(message.clone(), |_| {
        ready(Err::<ResourceLimitsEstimate, _>(SLOT))
    }))?;
    require(
        matches!(failure, Err(TuningError::Estimation(SLOT))),
        "custom estimator cause erased",
    )?;
    let failure = finish(estimate_and_set(message, |_| {
        ready(Ok::<_, ()>(ResourceLimitsEstimate {
            compute_unit_limit: u32::MAX,
            ..usage()
        }))
    }))?;
    require(
        matches!(
            failure,
            Err(TuningError::Message(
                ResourceError::ComputeUnitLimitOutOfRange(u32::MAX)
            ))
        ),
        "oversized estimate silently applied",
    )?;

    for limit in [0, MAX_COMPUTE_UNIT_LIMIT] {
        require(
            draft(VersionContext::Legacy)
                .with_compute_unit_limit(Some(limit))
                .is_ok(),
            "valid compute boundary rejected",
        )?;
    }

    require(
        matches!(
            draft(VersionContext::Legacy).with_compute_unit_limit(Some(MAX_COMPUTE_UNIT_LIMIT + 1)),
            Err(ResourceError::ComputeUnitLimitOutOfRange(_))
        ),
        "out of range compute accepted",
    )
}

fn usage() -> ResourceLimitsEstimate {
    ResourceLimitsEstimate {
        compute_unit_limit: COMPUTE_USAGE,
        loaded_accounts_data_size_limit: Some(LOADED_USAGE),
    }
}
