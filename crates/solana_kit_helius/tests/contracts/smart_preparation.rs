//! Unsigned smart preparation exercises actual RPC requests and compiled transactions.

mod cancellation;
mod failures;
mod success;

use serde_json::{Value, json};
use solana_kit_client::{
    Client, Limits, Reply, Request, Transport, TransportError, TransportFailureKind,
    methods::{BlockhashValue, Commitment, ReadConfig},
};
use solana_kit_core::{AccountMeta, AccountRole, Address, Hash, Instruction};
use solana_kit_helius::transactions::{SmartTransactionInput, SmartTransactionOptions};
use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    future::{Ready, ready},
    rc::Rc,
};

const SIMULATED_UNITS: u64 = 100_000;
const BUFFERED_UNITS: u32 = 110_001;
const RECOMMENDED_RATE: u64 = 2000;
const DEFAULT_TOTAL: u64 = 221;
const EXPLICIT_UNITS: u32 = 75_000;
const EXPLICIT_TOTAL: u64 = 150;
const SIMULATION_SLOT: u64 = 10;
const FINAL_SLOT: u64 = 20;
const MINIMUM_SLOT: u64 = 3;
const SIMULATION_EXPIRY: u64 = 77;
const FINAL_EXPIRY: u64 = 2000;
const PRIVATE_DETAIL: &str = "private-preparation-error";
const USER_DATA: &[u8] = &[11, 12];
const SECOND_DATA: &[u8] = &[13];
const BUDGET_ADDRESS: &str = "ComputeBudget111111111111111111111111111111";

/// Shared assertion outcome without a guest unwind on a behavioral mismatch.
pub type Outcome = Result<(), &'static str>;

fn require(condition: bool, reason: &'static str) -> Outcome {
    condition.then_some(()).ok_or(reason)
}

#[derive(Default)]
struct Observed {
    frames: RefCell<Vec<Value>>,
    violation: Cell<Option<&'static str>>,
}

type Requests = Rc<Observed>;

impl Observed {
    fn valid(&self) -> Outcome {
        self.violation.get().map_or(Ok(()), Err)
    }

    fn fail(&self, reason: &'static str) -> TransportError {
        self.violation.set(self.violation.get().or(Some(reason)));
        TransportError::new(TransportFailureKind::InvalidResponse)
    }

    fn record(&self, request: &Request) -> Result<u64, TransportError> {
        let frame: Value = serde_json::from_slice(&request.body)
            .map_err(|_| self.fail("actual RPC frame is invalid JSON"))?;
        let id = frame["id"]
            .as_u64()
            .ok_or_else(|| self.fail("actual correlation ID is missing"))?;
        self.frames.borrow_mut().push(frame);
        Ok(id)
    }
}

enum Step {
    Result(Value),
    Remote,
}

impl Step {
    fn reply(self, id: u64) -> Reply {
        let frame = match self {
            Self::Result(result) => json!({"jsonrpc":"2.0","id":id,"result":result}),
            Self::Remote => {
                json!({"jsonrpc":"2.0","id":id,"error":{"code":-1,"message":PRIVATE_DETAIL}})
            }
        };
        Reply {
            status: 200,
            body: frame.to_string().into_bytes(),
        }
    }
}

struct Rpc {
    replies: RefCell<VecDeque<Step>>,
    requests: Requests,
}

impl Transport for Rpc {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let result = (|| {
            let id = self.requests.record(&request)?;
            let reply = self
                .replies
                .borrow_mut()
                .pop_front()
                .ok_or_else(|| self.requests.fail("unexpected unqueued RPC exchange"))?;
            Ok(reply.reply(id))
        })();
        ready(result)
    }
}

fn client(replies: Vec<Step>) -> (Client<Rpc>, Requests) {
    let requests = Rc::new(Observed::default());
    let client = Client::new(
        Rpc {
            replies: RefCell::new(replies.into()),
            requests: Rc::clone(&requests),
        },
        Limits::default(),
    );
    (client, requests)
}

fn address(seed: u8) -> Address {
    Address::from_bytes([seed; 32])
}

fn budget_address() -> Result<Address, &'static str> {
    BUDGET_ADDRESS
        .parse()
        .map_err(|_| "pinned Compute Budget identity rejected")
}

fn simulation_lifetime() -> BlockhashValue {
    BlockhashValue {
        blockhash: Hash::from_bytes([5; 32]),
        last_valid_block_height: SIMULATION_EXPIRY,
    }
}

fn options<'a>() -> SmartTransactionOptions<'a> {
    SmartTransactionOptions {
        read: ReadConfig {
            commitment: Some(Commitment::Confirmed),
            min_context_slot: Some(MINIMUM_SLOT),
        },
        ..Default::default()
    }
}

fn input<'a>(
    lifetime: &'a BlockhashValue,
    signers: &'a [Address],
) -> Result<SmartTransactionInput<'a>, &'static str> {
    Ok(SmartTransactionInput {
        instructions: instructions()?,
        fee_payer: None,
        signers,
        simulation_lifetime: lifetime,
        lookup_tables: &[],
        compute_unit_limit: None,
    })
}

fn instructions() -> Result<Vec<Instruction>, &'static str> {
    let accounts = [
        (address(3), AccountRole::Readonly),
        (address(2), AccountRole::ReadonlySigner),
        (address(1), AccountRole::Readonly),
        (address(3), AccountRole::Writable),
        (address(4), AccountRole::Readonly),
    ]
    .into_iter()
    .map(|(address, role)| AccountMeta { address, role })
    .collect();
    Ok(vec![
        Instruction {
            program_address: address(8),
            accounts,
            data: USER_DATA.to_vec(),
        },
        Instruction {
            program_address: budget_address()?,
            accounts: vec![AccountMeta {
                address: address(99),
                role: AccountRole::WritableSigner,
            }],
            data: vec![u8::MAX],
        },
        Instruction {
            program_address: address(9),
            accounts: vec![AccountMeta {
                address: address(4),
                role: AccountRole::Writable,
            }],
            data: SECOND_DATA.to_vec(),
        },
    ])
}

fn simulation() -> Value {
    json!({"context":{"slot":SIMULATION_SLOT},"value":{
        "err":null,"fee":null,"accounts":null,"loadedAddresses":null,"logs":null,
        "preBalances":null,"postBalances":null,"preTokenBalances":null,"postTokenBalances":null,
        "returnData":null,"unitsConsumed":SIMULATED_UNITS,
        "replacementBlockhash":{"blockhash":Hash::from_bytes([6;32]).to_string(),"lastValidBlockHeight":SIMULATION_EXPIRY}
    }})
}

fn fee() -> Value {
    json!({"priorityFeeEstimate":RECOMMENDED_RATE})
}

fn blockhash() -> Value {
    json!({"context":{"slot":FINAL_SLOT},"value":{
        "blockhash":Hash::from_bytes([7;32]).to_string(),"lastValidBlockHeight":FINAL_EXPIRY
    }})
}

fn replies() -> Vec<Step> {
    vec![
        Step::Result(simulation()),
        Step::Result(fee()),
        Step::Result(blockhash()),
    ]
}

/// All deterministic preparation, failure and cancellation cases share every portable host.
pub fn preparation() -> Outcome {
    success::transactions()?;
    failures::boundaries()?;
    cancellation::owned_exchanges()
}

/// Changes only the independent expected call-order oracle for capture failure testing.
#[cfg(test)]
pub fn wrong_order_case() -> Outcome {
    success::wrong_order_case()
}

#[cfg(test)]
mod tests {
    #[test]
    fn wrong_order_is_a_reported_failure() -> super::Outcome {
        super::require(
            super::wrong_order_case() == Err("smart RPC observation order changed"),
            "changed smart order oracle did not report its mismatch",
        )
    }
}
