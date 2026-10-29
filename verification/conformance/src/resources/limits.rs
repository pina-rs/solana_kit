//! Source ordering, inline config, and provisional placeholder contracts.

use super::*;
use crate::require;
use solana_kit_core::{
    AccountMeta, AccountRole,
    transaction::{HeapSize, V1Config},
};

pub(super) fn run() -> Result<(), &'static str> {
    legacy_edits()?;
    inline_edits()?;
    provisional_limits()
}

fn legacy_edits() -> Result<(), &'static str> {
    for version in [VersionContext::Legacy, VersionContext::V0(&[])] {
        let mut message = draft(version);
        let mut first = message
            .clone()
            .with_compute_unit_limit(Some(EXPLICIT_COMPUTE))
            .map_err(|_| "initial compute setter failed")?
            .instructions[1]
            .clone();
        first.data.push(99);
        first.accounts.push(AccountMeta {
            address: PAYER,
            role: AccountRole::Readonly,
        });
        let second = first.clone();
        message.instructions.to_mut().extend([
            Instruction {
                program_address: solana_kit_compute_budget::PROGRAM,
                accounts: vec![],
                data: vec![255; 5],
            },
            first.clone(),
            second.clone(),
        ]);
        let unchanged = message
            .clone()
            .with_compute_unit_limit(Some(EXPLICIT_COMPUTE))
            .map_err(|_| "same value setter failed")?;
        require(
            unchanged == message,
            "equal compute changed trailing data or accounts",
        )?;
        let changed = message
            .with_compute_unit_limit(Some(COMPUTE_USAGE))
            .map_err(|_| "replacement setter failed")?;
        require(
            changed.instructions.len() == 4 && changed.instructions[3] == second,
            "compute replacement changed duplicate or unknown tag",
        )?;
        let removed = changed
            .with_compute_unit_limit(None)
            .map_err(|_| "remove compute failed")?;
        require(
            removed.compute_unit_limit() == Some(EXPLICIT_COMPUTE),
            "compute removal did not expose second match",
        )?;
        loaded_edits(removed)?;
    }

    Ok(())
}

fn loaded_edits(message: ResourceMessage<'_>) -> Result<(), &'static str> {
    let mut message = message
        .with_loaded_accounts_data_size_limit(Some(EXPLICIT_LOADED))
        .map_err(|_| "append loaded limit failed")?;
    let mut first = message
        .instructions
        .last()
        .ok_or("loaded instruction absent")?
        .clone();
    first.data.push(99);
    *message
        .instructions
        .to_mut()
        .last_mut()
        .ok_or("loaded instruction absent")? = first;
    let second = message
        .instructions
        .last()
        .ok_or("loaded instruction absent")?
        .clone();
    message.instructions.to_mut().push(second.clone());
    let unchanged = message
        .clone()
        .with_loaded_accounts_data_size_limit(Some(EXPLICIT_LOADED))
        .map_err(|_| "equal loaded limit failed")?;
    require(
        unchanged == message,
        "equal loaded value changed original bytes",
    )?;
    let changed = message
        .with_loaded_accounts_data_size_limit(Some(LOADED_USAGE))
        .map_err(|_| "replace loaded limit failed")?;
    require(
        changed.instructions.last() == Some(&second),
        "loaded replacement changed duplicate",
    )?;
    let removed = changed
        .with_loaded_accounts_data_size_limit(None)
        .map_err(|_| "remove loaded limit failed")?;

    require(
        removed.loaded_accounts_data_size_limit() == Some(EXPLICIT_LOADED),
        "loaded removal did not expose second match",
    )
}

fn inline_edits() -> Result<(), &'static str> {
    let heap = HeapSize::new(65_536).map_err(|_| "heap fixture invalid")?;
    let config = V1Config::default()
        .with_priority_fee(u64::MAX)
        .with_heap_size(heap);
    let message = draft(VersionContext::V1(config))
        .with_compute_unit_limit(Some(EXPLICIT_COMPUTE))
        .map_err(|_| "inline compute set failed")?
        .with_loaded_accounts_data_size_limit(Some(EXPLICIT_LOADED))
        .map_err(|_| "inline loaded set failed")?;
    require(
        message.instructions.len() == 1,
        "inline resource edits added instructions",
    )?;
    let message = message
        .with_compute_unit_limit(None)
        .map_err(|_| "inline compute remove failed")?
        .with_loaded_accounts_data_size_limit(None)
        .map_err(|_| "inline loaded remove failed")?;
    let VersionContext::V1(actual) = message.version else {
        return Err("lost inline version");
    };
    require(
        actual == config,
        "inline removal changed priority fee or heap",
    )?;
    let compiled = message
        .compile()
        .map_err(|_| "inline clearing did not compile")?;

    require(
        compiled.config() == Some(config),
        "compiled inline clearing changed config",
    )
}

fn provisional_limits() -> Result<(), &'static str> {
    for version in [
        VersionContext::Legacy,
        VersionContext::V0(&[]),
        VersionContext::V1(V1Config::default()),
    ] {
        let filled = draft(version)
            .fill_provisory_resource_limits()
            .map_err(|_| "provisional fill failed")?;
        require(
            filled.compute_unit_limit() == Some(0),
            "compute placeholder absent",
        )?;
        require(
            filled.loaded_accounts_data_size_limit()
                == matches!(version, VersionContext::V1(_)).then_some(0),
            "loaded placeholder version mismatch",
        )?;
        let repeated = filled
            .clone()
            .fill_provisory_resource_limits()
            .map_err(|_| "repeated fill failed")?;
        require(filled == repeated, "provisional fill was not idempotent")?;
        let explicit = filled
            .with_compute_unit_limit(Some(EXPLICIT_COMPUTE))
            .map_err(|_| "explicit compute failed")?
            .with_loaded_accounts_data_size_limit(Some(EXPLICIT_LOADED))
            .map_err(|_| "explicit loaded failed")?;
        require(
            explicit
                .clone()
                .fill_provisory_resource_limits()
                .map_err(|_| "explicit fill failed")?
                == explicit,
            "fill changed explicit resource value",
        )?;
    }

    Ok(())
}
