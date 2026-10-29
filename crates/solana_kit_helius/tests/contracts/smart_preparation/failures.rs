//! Failures stop at their owning phase and never become a fabricated prepared transaction.

use super::*;
use futures::executor::block_on;
use solana_kit_helius::transactions::{
    ComputeUnitBufferError, ComputeUnitsError, PriorityFeeError, SmartTransactionBuildError,
    SmartTransactionError, SmartTransactionStage, create_smart_transaction,
};

const OVERSIZED_DATA: usize = 4096;

fn failure(replies: Vec<Step>) -> Result<(SmartTransactionError, usize), &'static str> {
    let lifetime = simulation_lifetime();
    let signers = [address(1)];
    let (rpc, requests) = client(replies);
    let error = block_on(create_smart_transaction(
        &rpc,
        input(&lifetime, &signers)?,
        options(),
    ))
    .err()
    .ok_or("failed smart preparation became success")?;
    requests.valid()?;
    let count = requests.frames.borrow().len();
    Ok((error, count))
}

fn preflight(
    input: SmartTransactionInput<'_>,
    options: SmartTransactionOptions<'_>,
) -> Result<SmartTransactionError, &'static str> {
    let (rpc, requests) = client(Vec::new());
    let error = block_on(create_smart_transaction(&rpc, input, options))
        .err()
        .ok_or("invalid caller input became success")?;
    requests.valid()?;
    require(
        requests.frames.borrow().is_empty(),
        "invalid caller input reached RPC",
    )?;
    Ok(error)
}

pub(super) fn boundaries() -> Outcome {
    invalid_inputs()?;
    construction()?;
    rpc_stages()?;
    simulation_and_fee()?;
    refreshed_lifetime()
}

fn invalid_inputs() -> Outcome {
    let lifetime = simulation_lifetime();
    let signers = [address(1)];
    let mut zero = input(&lifetime, &[])?;
    zero.compute_unit_limit = Some(0);
    require(
        matches!(
            preflight(zero, options())?,
            SmartTransactionError::NonPositiveComputeUnitLimit
        ),
        "zero compute limit was not rejected before RPC",
    )?;
    require(
        matches!(
            preflight(input(&lifetime, &[])?, options())?,
            SmartTransactionError::MissingFeePayer
        ),
        "missing payer was not rejected before RPC",
    )?;
    let mut invalid_buffer = options();
    invalid_buffer.compute_buffer.buffer_percent = f64::NAN;
    require(
        matches!(
            preflight(input(&lifetime, &signers)?, invalid_buffer)?,
            SmartTransactionError::Buffer(ComputeUnitBufferError::NonFinitePercent)
        ),
        "nonfinite buffer percent was not rejected before RPC",
    )?;

    let (rpc, requests) = client(replies());
    invalid_buffer.compute_buffer.buffer_percent = f64::MAX;
    let result = block_on(create_smart_transaction(
        &rpc,
        input(&lifetime, &signers)?,
        invalid_buffer,
    ));
    requests.valid()?;
    require(
        matches!(
            result,
            Err(SmartTransactionError::Buffer(
                ComputeUnitBufferError::NonFiniteResult
            ))
        ),
        "nonfinite buffered result was not rejected",
    )?;
    require(
        requests.frames.borrow().len() == 1,
        "failed compute buffer admitted another RPC",
    )
}

fn construction() -> Outcome {
    let lifetime = simulation_lifetime();
    let signers = [address(1)];

    for invalid in [
        Instruction {
            program_address: address(8),
            accounts: Vec::new(),
            data: vec![0; OVERSIZED_DATA],
        },
        Instruction {
            program_address: address(8),
            accounts: vec![AccountMeta {
                address: address(8),
                role: AccountRole::Writable,
            }],
            data: Vec::new(),
        },
        Instruction {
            program_address: address(8),
            accounts: vec![AccountMeta {
                address: budget_address()?,
                role: AccountRole::Writable,
            }],
            data: Vec::new(),
        },
    ] {
        let mut input = input(&lifetime, &signers)?;
        input.instructions = vec![invalid];
        require(
            matches!(
                preflight(input, options())?,
                SmartTransactionError::Build(SmartTransactionBuildError::Transaction(_))
            ),
            "invalid final-layout construction reached RPC or succeeded",
        )?;
    }
    Ok(())
}

fn rpc_stages() -> Outcome {
    for index in 0..3 {
        let mut replies = replies();
        replies[index] = Step::Remote;
        let (error, count) = failure(replies)?;
        require(
            count == index + 1,
            "failed smart RPC admitted a later phase",
        )?;
        require(
            !format!("{error:?} {error}").contains(PRIVATE_DETAIL),
            "smart error diagnostics leaked provider text",
        )?;
        require(
            std::error::Error::source(&error).is_some(),
            "smart RPC error lost its source",
        )?;

        let matched = match index {
            0 => matches!(
                error,
                SmartTransactionError::Simulation(ComputeUnitsError::Rpc(_))
            ),
            1 => matches!(
                error,
                SmartTransactionError::Rpc {
                    stage: SmartTransactionStage::FeeEstimate,
                    ..
                }
            ),
            2 => matches!(
                error,
                SmartTransactionError::Rpc {
                    stage: SmartTransactionStage::Blockhash,
                    ..
                }
            ),
            _ => return Err("closed RPC phase list changed"),
        };
        require(matched, "smart RPC error phase/type changed")?;
    }
    Ok(())
}

fn simulation_and_fee() -> Outcome {
    let mut failed = simulation();
    failed["value"]["err"] = json!({"FutureExecution":PRIVATE_DETAIL});
    let (error, count) = failure(vec![Step::Result(failed)])?;
    require(
        matches!(
            error,
            SmartTransactionError::Simulation(ComputeUnitsError::Execution { .. })
        ),
        "simulation execution error was not retained",
    )?;
    require(count == 1, "failed simulation admitted fee RPC")?;

    for missing in [json!({}), json!({"priorityFeeEstimate":null})] {
        let (error, count) = failure(vec![Step::Result(simulation()), Step::Result(missing)])?;
        require(
            matches!(error, SmartTransactionError::MissingFeeEstimate),
            "missing or null fee estimate was not rejected",
        )?;
        require(count == 2, "missing fee estimate admitted blockhash RPC")?;
    }

    let (error, count) = failure(vec![
        Step::Result(simulation()),
        Step::Result(json!({"priorityFeeEstimate":PRIVATE_DETAIL})),
    ])?;
    require(
        matches!(
            error,
            SmartTransactionError::Rpc {
                stage: SmartTransactionStage::FeeEstimate,
                ..
            }
        ),
        "malformed fee estimate lost its RPC phase",
    )?;
    require(count == 2, "malformed fee estimate admitted blockhash RPC")?;
    let huge: Value = serde_json::from_str(r#"{"priorityFeeEstimate":1e99999999999999999999}"#)
        .map_err(|_| "exact large fee fixture rejected")?;
    let (error, count) = failure(vec![Step::Result(simulation()), Step::Result(huge)])?;
    require(
        matches!(
            error,
            SmartTransactionError::Fee(PriorityFeeError::RateOverflow)
        ),
        "overflowing fee rate was not rejected exactly",
    )?;
    require(count == 2, "overflowing fee admitted blockhash RPC")
}

fn refreshed_lifetime() -> Outcome {
    let hash = Hash::from_bytes([7; 32]).to_string();
    let values = [
        json!({"blockhash":&hash,"lastValidBlockHeight":null}),
        json!({"blockhash":&hash,"lastValidBlockHeight":"not-a-height"}),
        json!({"blockhash":&hash}),
    ];

    for value in values {
        let mut malformed = blockhash();
        malformed["value"] = value;
        let (error, count) = failure(vec![
            Step::Result(simulation()),
            Step::Result(fee()),
            Step::Result(malformed),
        ])?;
        require(
            matches!(
                error,
                SmartTransactionError::Rpc {
                    stage: SmartTransactionStage::Blockhash,
                    ..
                }
            ),
            "invalid final expiry was not rejected at blockhash phase",
        )?;
        require(count == 3, "invalid final expiry changed RPC count")?;
    }

    let mut stale = blockhash();
    stale["context"]["slot"] = json!(0);
    let (error, count) = failure(vec![
        Step::Result(simulation()),
        Step::Result(fee()),
        Step::Result(stale),
    ])?;
    require(
        matches!(error, SmartTransactionError::StaleBlockhashContext { context, minimum_slot }
        if context.slot == 0 && minimum_slot == MINIMUM_SLOT),
        "stale final blockhash context was not rejected",
    )?;
    require(count == 3, "stale blockhash changed RPC count")
}
