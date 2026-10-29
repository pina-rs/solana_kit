//! Source metadata, call order, lookup ownership and actual final wire agree.

use super::*;
use base64::{Engine, engine::general_purpose::STANDARD};
use futures::executor::block_on;
use solana_kit_core::transaction::{Lifetime, LookupTable, Transaction, Version};
use solana_kit_helius::transactions::{
    PreparedSmartTransaction, ResolvedPriorityFee, create_smart_transaction,
};

const EXPLICIT_PAYER_SEED: u8 = 11;

pub(super) fn transactions() -> Outcome {
    let lifetime = simulation_lifetime();
    let signers = [address(1), address(2)];

    for explicit in [None, Some(EXPLICIT_UNITS)] {
        let (rpc, requests) = client(replies());
        let mut input = input(&lifetime, &signers)?;
        input.compute_unit_limit = explicit;
        let result = block_on(create_smart_transaction(&rpc, input, options()))
            .map_err(|_| "valid unsigned smart transaction rejected")?;
        requests.valid()?;

        let (units, total) = explicit.map_or((BUFFERED_UNITS, DEFAULT_TOTAL), |units| {
            (units, EXPLICIT_TOTAL)
        });
        require(
            result.compute_units.get() == units,
            "resolved compute limit changed",
        )?;
        require(
            result.priority_fee
                == ResolvedPriorityFee {
                    rate: RECOMMENDED_RATE,
                    lamports: total,
                },
            "recommended smart priority fee changed",
        )?;
        require(
            result.account_keys == [address(1), address(3), address(2), address(4)],
            "source-ordered smart fee account identities changed",
        )?;

        assert_requests(&requests.frames.borrow(), &result.account_keys)?;
        assert_final(&result)?;
    }

    payer_and_lookup_ownership()?;
    caps_and_override()
}

fn assert_requests(requests: &[Value], keys: &[Address]) -> Outcome {
    assert_methods(
        requests,
        &[
            "simulateTransaction",
            "getPriorityFeeEstimate",
            "getLatestBlockhash",
        ],
    )?;
    assert_draft(requests.first().ok_or("simulation request missing")?)?;
    let keys: Vec<_> = keys.iter().map(ToString::to_string).collect();
    require(
        requests.get(1).ok_or("fee request missing")?["params"]
            == json!({"accountKeys":keys,"options":{"recommended":true}}),
        "recommended account-key fee query changed",
    )?;
    require(
        requests.get(2).ok_or("blockhash request missing")?["params"]
            == json!([{"commitment":"confirmed","minContextSlot":MINIMUM_SLOT}]),
        "final blockhash read policy changed",
    )
}

fn assert_methods(requests: &[Value], expected: &[&str]) -> Outcome {
    require(
        methods(requests)?.as_slice() == expected,
        "smart RPC observation order changed",
    )
}

#[cfg(test)]
pub(super) fn wrong_order_case() -> Outcome {
    let lifetime = simulation_lifetime();
    let signers = [address(1), address(2)];
    let (rpc, requests) = client(replies());
    block_on(create_smart_transaction(
        &rpc,
        input(&lifetime, &signers)?,
        options(),
    ))
    .map_err(|_| "smart order control failed before comparison")?;
    requests.valid()?;
    assert_methods(
        &requests.frames.borrow(),
        &[
            "getLatestBlockhash",
            "getPriorityFeeEstimate",
            "simulateTransaction",
        ],
    )
}

fn methods(requests: &[Value]) -> Result<Vec<&str>, &'static str> {
    requests
        .iter()
        .map(|request| {
            request["method"]
                .as_str()
                .ok_or("actual smart RPC name missing")
        })
        .collect()
}

fn assert_draft(request: &Value) -> Outcome {
    require(
        request["params"][1]
            == json!({"encoding":"base64","commitment":"confirmed",
        "minContextSlot":MINIMUM_SLOT,"replaceRecentBlockhash":true,"sigVerify":false}),
        "unsigned simulation policy changed",
    )?;
    let wire = STANDARD
        .decode(
            request["params"][0]
                .as_str()
                .ok_or("simulation base64 draft missing")?,
        )
        .map_err(|_| "actual simulation draft base64 rejected")?;
    let transaction =
        Transaction::decode(&wire).map_err(|_| "unsigned simulation draft wire rejected")?;
    let message = transaction.message();
    require(
        message.lifetime().token() == simulation_lifetime().blockhash,
        "simulation lifetime changed",
    )?;
    require(
        message.instructions().len() == 2,
        "caller budget instructions were not stripped",
    )?;
    require(
        transaction.signatures().iter().all(Option::is_none),
        "simulation acquired a signature",
    )?;

    require(
        message
            .instructions()
            .first()
            .ok_or("first simulation instruction missing")?
            .data()
            == USER_DATA,
        "first simulation instruction data changed",
    )?;
    require(
        message
            .instructions()
            .get(1)
            .ok_or("second simulation instruction missing")?
            .data()
            == SECOND_DATA,
        "second simulation instruction data changed",
    )?;
    let programs: Vec<_> = message
        .instructions()
        .iter()
        .map(|instruction| {
            message
                .static_accounts()
                .get(usize::from(instruction.program_index()))
                .copied()
                .ok_or("simulation program index out of range")
        })
        .collect::<Result<_, _>>()?;
    require(
        programs == [address(8), address(9)],
        "simulation user program order changed",
    )
}

fn assert_final(result: &PreparedSmartTransaction) -> Outcome {
    let message = result.transaction.message();
    require(
        message.version() == Version::V0,
        "final smart message is not v0",
    )?;
    require(
        message.lifetime()
            == Lifetime::Blockhash {
                blockhash: Hash::from_bytes([7; 32]),
                last_valid_block_height: FINAL_EXPIRY,
            },
        "fresh blockhash or real expiry changed",
    )?;
    require(
        result.simulation.value == SIMULATED_UNITS,
        "raw compute observation changed",
    )?;
    require(
        result.simulation.context.slot == SIMULATION_SLOT,
        "simulation context changed",
    )?;
    require(
        result.blockhash_context.slot == FINAL_SLOT,
        "blockhash context changed",
    )?;

    require(
        message.instructions().len() == 4,
        "final smart instruction count changed",
    )?;
    let [price, limit, first, second] = message.instructions() else {
        return Err("price, limit and ordered user instructions missing");
    };
    require(
        message
            .static_accounts()
            .get(usize::from(price.program_index()))
            .copied()
            == Some(budget_address()?),
        "final price program changed",
    )?;
    require(
        message
            .static_accounts()
            .get(usize::from(limit.program_index()))
            .copied()
            == Some(budget_address()?),
        "final limit program changed",
    )?;
    require(
        price.data()
            == [&[3][..], &result.priority_fee.rate.to_le_bytes()]
                .concat()
                .as_slice(),
        "final price instruction bytes changed",
    )?;
    require(
        limit.data()
            == [&[2][..], &result.compute_units.get().to_le_bytes()]
                .concat()
                .as_slice(),
        "final limit instruction bytes changed",
    )?;
    require(
        first.data() == USER_DATA,
        "final first user instruction changed",
    )?;
    require(
        second.data() == SECOND_DATA,
        "final second user instruction changed",
    )?;

    assert_unsigned(&result.transaction)
}

fn assert_unsigned(transaction: &Transaction) -> Outcome {
    require(
        transaction.signatures().len() == 2,
        "final signer slots changed",
    )?;
    require(
        transaction.signatures().iter().all(Option::is_none),
        "prepared smart transaction was signed",
    )?;
    require(
        transaction.encode(true).is_err(),
        "unsigned transaction encoded as fully signed",
    )?;
    let wire = transaction
        .encode(false)
        .map_err(|_| "canonical unsigned final wire rejected")?;
    require(
        Transaction::decode(&wire)
            .map_err(|_| "receiver rejected unsigned smart wire")?
            .message()
            .bytes()
            == transaction.message().bytes(),
        "unsigned receiver message bytes changed",
    )
}

fn payer_and_lookup_ownership() -> Outcome {
    let lifetime = simulation_lifetime();
    let tables = [LookupTable {
        address: address(10),
        addresses: vec![address(4)],
    }];
    let (rpc, requests) = client(replies());
    let mut input = input(&lifetime, &[])?;
    input.fee_payer = Some(address(EXPLICIT_PAYER_SEED));
    input.lookup_tables = &tables;
    let result = block_on(create_smart_transaction(&rpc, input, options()))
        .map_err(|_| "explicit payer and caller lookup snapshot rejected")?;
    requests.valid()?;

    require(
        result
            .transaction
            .message()
            .static_accounts()
            .first()
            .copied()
            == Some(address(EXPLICIT_PAYER_SEED)),
        "explicit payer changed",
    )?;
    require(
        result.account_keys
            == [
                address(EXPLICIT_PAYER_SEED),
                address(3),
                address(2),
                address(1),
                address(4),
            ],
        "explicit payer fee account order changed",
    )?;
    require(
        result.transaction.message().lookups().len() == 1,
        "caller lookup snapshot not reused",
    )?;
    require(
        result
            .transaction
            .message()
            .lookups()
            .first()
            .ok_or("lookup descriptor missing")?
            .address()
            == address(10),
        "lookup table identity changed",
    )?;

    assert_requests(&requests.frames.borrow(), &result.account_keys)?;
    assert_final(&result)
}

fn caps_and_override() -> Outcome {
    let lifetime = simulation_lifetime();
    let signers = [address(1)];
    let cap = "220"
        .parse()
        .map_err(|_| "exact whole lamport cap rejected")?;
    let (rpc, requests) = client(replies());
    let mut options = options();
    options.lamports_cap = Some(&cap);
    let result = block_on(create_smart_transaction(
        &rpc,
        input(&lifetime, &signers)?,
        options,
    ))
    .map_err(|_| "capped smart preparation rejected")?;
    requests.valid()?;

    require(
        result.priority_fee
            == ResolvedPriorityFee {
                rate: 1999,
                lamports: 220,
            },
        "whole-lamport capped rate changed",
    )?;
    assert_final(&result)?;

    let (rpc, requests) = client(replies());
    let mut input = input(&lifetime, &signers)?;
    input.compute_unit_limit = Some(EXPLICIT_UNITS);
    options.compute_buffer.buffer_percent = f64::NAN;
    let result = block_on(create_smart_transaction(&rpc, input, options))
        .map_err(|_| "explicit limit evaluated unused NaN buffer")?;
    requests.valid()?;
    require(
        result.compute_units.get() == EXPLICIT_UNITS,
        "explicit compute limit changed",
    )?;
    require(
        requests.frames.borrow().len() == 3,
        "explicit limit RPC count changed",
    )
}
