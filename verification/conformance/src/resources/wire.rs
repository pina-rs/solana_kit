//! Estimation compiles actual unsigned RPC wire and retains nonce position.

use super::*;
use crate::require;
use base64::{Engine, engine::general_purpose::STANDARD};
use solana_kit_client::{
    methods::Commitment,
    resources::{
        EstimateConfig, MAX_COMPUTE_UNIT_LIMIT, MAX_LOADED_ACCOUNTS_DATA_SIZE_LIMIT, estimate,
    },
};
use solana_kit_core::{
    AccountMeta, AccountRole,
    transaction::{DurableNonce, LookupTable, Message, Transaction, V1Config, Version},
};

pub(super) fn run() -> Result<(), &'static str> {
    let tables = [LookupTable {
        address: Address::from_bytes([8; 32]),
        addresses: vec![Address::from_bytes([2; 32])],
    }];

    for version in [
        VersionContext::Legacy,
        VersionContext::V0(&tables),
        VersionContext::V1(V1Config::default().with_priority_fee(u64::MAX)),
    ] {
        for nonce in [false, true] {
            simulate(version, nonce)?;
        }
    }

    Ok(())
}

fn simulate(version: VersionContext<'_>, use_nonce: bool) -> Result<(), &'static str> {
    let requests = RefCell::new(Vec::new());
    let client = client(&requests, result());
    let message = message(version, use_nonce)?;
    let original = message.clone();
    let limits = finish(estimate(
        &client,
        &message,
        EstimateConfig {
            commitment: Some(Commitment::Confirmed),
            min_context_slot: Some(SLOT),
        },
    ))?
    .map_err(|_| "wire simulation failed")?;
    require(
        limits.compute_unit_limit == COMPUTE_USAGE
            && limits.loaded_accounts_data_size_limit == Some(LOADED_USAGE),
        "wire simulation lost usage",
    )?;
    require(message == original, "estimation changed original draft")?;

    let requests = requests.borrow();
    require(
        requests.len() == 1,
        "estimator did not exchange exactly once",
    )?;
    let transaction = inspect_request(&requests[0], use_nonce)?;

    inspect_transaction(&transaction, version, use_nonce)
}

fn message(
    version: VersionContext<'_>,
    use_nonce: bool,
) -> Result<ResourceMessage<'_>, &'static str> {
    let mut message = draft(version)
        .with_compute_unit_limit(Some(EXPLICIT_COMPUTE))
        .map_err(|_| "explicit simulation compute setup failed")?;
    message.instructions.to_mut()[0].accounts.push(AccountMeta {
        address: Address::from_bytes([2; 32]),
        role: AccountRole::Writable,
    });

    if matches!(version, VersionContext::V1(_)) {
        message = message
            .with_loaded_accounts_data_size_limit(Some(EXPLICIT_LOADED))
            .map_err(|_| "explicit simulation loaded setup failed")?;
    }

    if use_nonce {
        let nonce = DurableNonce::new(BLOCKHASH, Address::from_bytes([3; 32]), PAYER);
        message.instructions = Cow::Owned(nonce.prepare_instructions(&message.instructions));
        message.lifetime = Lifetime::DurableNonce(nonce);
    }

    Ok(message)
}

fn inspect_request(request: &Value, use_nonce: bool) -> Result<Transaction, &'static str> {
    require(
        request["method"] == "simulateTransaction",
        "wrong simulation RPC",
    )?;
    let config = json!({
        "encoding": "base64", "commitment": "confirmed", "minContextSlot": SLOT,
        "sigVerify": false, "replaceRecentBlockhash": !use_nonce
    });
    require(
        request["params"][1] == config,
        "simulation node options changed",
    )?;
    let wire = STANDARD
        .decode(
            request["params"][0]
                .as_str()
                .ok_or("wire not base64 string")?,
        )
        .map_err(|_| "invalid base64 transaction")?;

    Transaction::decode(&wire).map_err(|_| "sent invalid transaction wire")
}

fn inspect_transaction(
    transaction: &Transaction,
    version: VersionContext<'_>,
    nonce: bool,
) -> Result<(), &'static str> {
    let message = transaction.message();
    require(
        transaction.signatures().iter().all(Option::is_none),
        "simulation wire was signed",
    )?;
    require(
        message.lifetime().token() == BLOCKHASH,
        "simulation changed lifetime token",
    )?;

    if nonce {
        require(
            message
                .nonce_advance()
                .map_err(|_| "nonce inspection failed")?
                .is_some(),
            "resource instruction preceded nonce advance",
        )?;
    }

    match version {
        VersionContext::V1(config) => inspect_v1(message, config)?,
        VersionContext::Legacy => inspect_legacy(message, Version::Legacy, nonce)?,
        VersionContext::V0(_) => inspect_legacy(message, Version::V0, nonce)?,
    }

    require(
        !matches!(version, VersionContext::V0(_)) || !message.lookups().is_empty(),
        "v0 lookup snapshot not used",
    )
}

fn inspect_v1(message: &Message, config: V1Config) -> Result<(), &'static str> {
    let actual = message.config().ok_or("v1 config absent")?;
    require(
        actual.compute_unit_limit() == Some(MAX_COMPUTE_UNIT_LIMIT)
            && actual.loaded_accounts_data_limit() == Some(MAX_LOADED_ACCOUNTS_DATA_SIZE_LIMIT),
        "v1 did not maximize both resource limits",
    )?;

    require(
        actual.priority_fee() == config.priority_fee(),
        "v1 priority fee changed",
    )
}

fn inspect_legacy(message: &Message, version: Version, nonce: bool) -> Result<(), &'static str> {
    require(
        message.version() == version && message.config().is_none(),
        "legacy/v0 wire gained inline config",
    )?;
    require(
        message
            .instructions()
            .last()
            .is_some_and(|instruction| instruction.data() == [2, 192, 92, 21, 0]),
        "legacy/v0 compute ceiling bytes differ",
    )?;
    let expected_instructions = 2 + usize::from(nonce);

    require(
        message.instructions().len() == expected_instructions,
        "legacy/v0 gained loaded instruction",
    )
}
