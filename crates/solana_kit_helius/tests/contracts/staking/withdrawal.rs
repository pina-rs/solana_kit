//! Withdrawal observations use literal sysvar bytes and real shared-client RPC framing.

use base64::{Engine, engine::general_purpose::STANDARD};
use solana_kit_client::{
    Client, Reply, Request, Transport, TransportError,
    methods::{Commitment, ReadConfig},
};
use solana_kit_core::{Address, codecs::Limits, sysvars};
use solana_kit_helius::staking::{
    StakeAccountError, StakeWithdrawalQuery, get_withdrawable_amount,
};
use solana_kit_stake::{WithdrawalError, WithdrawalLimits, WithdrawalSigners};
use std::{
    cell::{Cell, RefCell},
    future::{Future, Ready, ready},
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, Waker},
};

const SLOT: u64 = 100;
const EPOCH: u64 = 2;
const MINIMUM_SLOT: u64 = 90;
const NOW: i64 = 50;
const CLUSTER_PENDING: u64 = 500;
const NON_INTEGER_THRESHOLD: f64 = 1.5;
const BURN_PERCENT: u8 = 50;

const RATE: u64 = 3;
const ALLOCATION: usize = 200;
const RESERVE: u64 = (ALLOCATION as u64 + 128) * RATE * 2;
const BALANCE: u64 = 9_007_199_254_740_993;
const WITHDRAWER: Address = Address::from_bytes([1; 32]);
const STAKE: Address = Address::from_bytes([2; 32]);
const BUDGET: usize = 65_536;

type Frames = Rc<RefCell<Vec<serde_json::Value>>>;

struct Rpc {
    result: RefCell<Option<String>>,
    frames: Frames,
}

impl Transport for Rpc {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let frame: serde_json::Value = serde_json::from_slice(&request.body).unwrap();
        let id = frame["id"].as_u64().unwrap();
        self.frames.borrow_mut().push(frame);
        let result = self
            .result
            .borrow_mut()
            .take()
            .expect("one bundled bank read");
        ready(Ok(Reply {
            status: 200,
            body: format!(r#"{{"jsonrpc":"2.0","id":{id},"result":{result}}}"#).into_bytes(),
        }))
    }
}

/// The original staking contract group executes these cases in every feature/runtime matrix.
pub(super) fn evaluation() {
    one_bank_and_exact_lamports();
    inactive_gap_and_delegated_principal();
    integer_rent_and_zero_data_accounts();
    inconsistent_observations();
    malformed_accounts_and_rent();
    cancellation_releases_the_single_exchange();
}

struct PendingRpc {
    started: Rc<Cell<usize>>,
    dropped: Rc<Cell<usize>>,
}

struct PendingRead(Rc<Cell<usize>>);

impl Future for PendingRead {
    type Output = Result<Reply, TransportError>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

impl Drop for PendingRead {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

impl Transport for PendingRpc {
    type Exchange<'a> = PendingRead;

    fn exchange(&self, _: Request) -> Self::Exchange<'_> {
        self.started.set(self.started.get() + 1);
        PendingRead(Rc::clone(&self.dropped))
    }
}

/// Dropping an actual suspended public call retires its one caller-owned exchange.
fn cancellation_releases_the_single_exchange() {
    let started = Rc::new(Cell::new(0));
    let dropped = Rc::new(Cell::new(0));
    let transport = PendingRpc {
        started: Rc::clone(&started),
        dropped: Rc::clone(&dropped),
    };
    let client = Client::new(transport, solana_kit_client::Limits::default());
    let mut future = Box::pin(get_withdrawable_amount(&client, STAKE, signers(), query()));
    assert_eq!(started.get(), 0);
    assert!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    assert_eq!(started.get(), 1);
    assert_eq!(dropped.get(), 0);
    drop(future);
    assert_eq!(dropped.get(), 1);
    assert_eq!(started.get(), 1);
}

/// No provider route or second bank read is involved, and all amounts remain u64.
fn one_bank_and_exact_lamports() {
    let (client, frames) = rpc_client(bundle(), SLOT);
    let result =
        futures::executor::block_on(get_withdrawable_amount(&client, STAKE, signers(), query()))
            .unwrap();
    assert_eq!(result.context.slot, SLOT);
    assert_eq!(result.value.max_partial_lamports, Some(BALANCE - RESERVE));
    assert_eq!(result.value.close_lamports, Some(BALANCE));

    assert!(!result.value.permits(BALANCE - 1));
    assert!(result.value.permits(BALANCE));
    assert_eq!(frames.borrow().len(), 1);
    let frames = frames.borrow();
    assert_eq!(frames[0]["method"], "getMultipleAccounts");
    assert_eq!(
        frames[0]["params"],
        serde_json::json!([
        [
            STAKE.to_string(),
            sysvars::CLOCK_ADDRESS.to_string(),
            sysvars::RENT_ADDRESS.to_string(),
            sysvars::STAKE_HISTORY_ADDRESS.to_string()
        ],
        {"encoding":"base64","commitment":"confirmed","minContextSlot":MINIMUM_SLOT}
        ])
    );
}

/// Complete closure is separate from partial withdrawal, and cooldown uses real history.
fn inactive_gap_and_delegated_principal() {
    let mut values = bundle();
    let mut bytes = stake_bytes(ALLOCATION);
    bytes[..4].copy_from_slice(&2_u32.to_le_bytes());
    bytes[156..164].copy_from_slice(&100_u64.to_le_bytes());
    bytes[164..172].copy_from_slice(&0_u64.to_le_bytes());
    bytes[172..180].copy_from_slice(&1_u64.to_le_bytes());

    values[0] = account(solana_kit_stake::PROGRAM, BALANCE, bytes.clone());
    let limits = evaluate(values).unwrap();
    assert_eq!(limits.max_partial_lamports, Some(BALANCE - RESERVE - 15));
    assert_eq!(limits.close_lamports, None);

    bytes[164..172].copy_from_slice(&EPOCH.to_le_bytes());
    bytes[172..180].copy_from_slice(&u64::MAX.to_le_bytes());
    let mut values = bundle();
    values[0] = account(solana_kit_stake::PROGRAM, BALANCE, bytes);
    let limits = evaluate(values).unwrap();
    assert_eq!(limits.max_partial_lamports, Some(BALANCE - RESERVE - 100));
    assert_eq!(limits.close_lamports, None);
}

/// Integer thresholds avoid rounding a large odd reserve through f64.
fn integer_rent_and_zero_data_accounts() {
    const LARGE_ALLOCATION: usize = 16_385;
    const LARGE_RATE: u64 = 1_000_000_000_001;
    const EXACT_RESERVE: u64 = 16_513_000_000_016_513;
    let mut values = bundle();
    values[0] = account(
        solana_kit_stake::PROGRAM,
        u64::MAX,
        stake_bytes(LARGE_ALLOCATION),
    );
    values[2] = account(sysvars::OWNER_ADDRESS, 1, rent_bytes(LARGE_RATE, 1.0));
    assert_eq!(
        evaluate(values).unwrap().max_partial_lamports,
        Some(u64::MAX - EXACT_RESERVE)
    );

    let mut values = bundle();
    values[0] = account(solana_kit_stake::PROGRAM, BALANCE, Vec::new());
    assert!(matches!(
        evaluate(values.clone()),
        Err(StakeAccountError::Withdrawal(
            WithdrawalError::AuthorityMismatch
        ))
    ));
    let (client, _) = rpc_client(values, SLOT);
    let limits = futures::executor::block_on(get_withdrawable_amount(
        &client,
        STAKE,
        WithdrawalSigners {
            withdrawer: STAKE,
            custodian: None,
        },
        query(),
    ))
    .unwrap();
    assert_eq!(limits.value.max_partial_lamports, Some(BALANCE - 1));
    assert_eq!(limits.value.close_lamports, Some(BALANCE));
}

/// A missing or reordered epoch cannot silently invoke the pure calculation's fallback.
fn inconsistent_observations() {
    for epochs in [vec![0], vec![1, 1], vec![2, 0]] {
        let mut values = bundle();
        values[3] = account(sysvars::OWNER_ADDRESS, 1, history_bytes(&epochs));
        assert!(matches!(
            evaluate(values),
            Err(StakeAccountError::IncoherentSnapshot)
        ));
    }

    let (client, _) = rpc_client(bundle(), SLOT - 1);
    assert!(matches!(
        futures::executor::block_on(get_withdrawable_amount(&client, STAKE, signers(), query())),
        Err(StakeAccountError::IncoherentSnapshot)
    ));
    let (client, _) = rpc_client(bundle(), 1);
    assert!(matches!(
        futures::executor::block_on(get_withdrawable_amount(&client, STAKE, signers(), query())),
        Err(StakeAccountError::StaleContext)
    ));
}

/// Missing sysvars, executable data, malformed layouts and invalid rent remain failures.
fn malformed_accounts_and_rent() {
    for index in 0..4 {
        let mut values = bundle();
        values[index] = serde_json::Value::Null;
        assert!(matches!(
            evaluate(values),
            Err(StakeAccountError::Decode(_))
        ));
    }

    for index in 1..4 {
        let mut values = bundle();
        values[index]["owner"] = serde_json::json!(STAKE.to_string());
        assert!(matches!(
            evaluate(values),
            Err(StakeAccountError::Decode(_))
        ));
    }

    let mut executable = bundle();
    executable[1]["executable"] = serde_json::json!(true);
    assert!(matches!(
        evaluate(executable),
        Err(StakeAccountError::Executable(_))
    ));
    let mut truncated = bundle();
    truncated[1] = account(sysvars::OWNER_ADDRESS, 1, vec![0; 39]);
    assert!(matches!(
        evaluate(truncated),
        Err(StakeAccountError::Decode(_))
    ));

    for (rate, threshold) in [
        (RATE, NON_INTEGER_THRESHOLD),
        (RATE, f64::NAN),
        (u64::MAX, 1.0),
        (u64::MAX, 2.0),
    ] {
        let mut values = bundle();
        values[2] = account(sysvars::OWNER_ADDRESS, 1, rent_bytes(rate, threshold));
        assert!(matches!(
            evaluate(values),
            Err(StakeAccountError::InvalidRent)
        ));
    }
}

/// The shared client validates its response frame and address order before interpretation.
fn rpc_client(values: Vec<serde_json::Value>, slot: u64) -> (Client<Rpc>, Frames) {
    let frames = Rc::new(RefCell::new(Vec::new()));
    let result = serde_json::json!({"context":{"slot":slot},"value":values}).to_string();
    (
        Client::new(
            Rpc {
                result: RefCell::new(Some(result)),
                frames: Rc::clone(&frames),
            },
            solana_kit_client::Limits {
                bytes: BUDGET,
                depth: 32,
            },
        ),
        frames,
    )
}

/// Error cases retain the same public RPC path and require exactly one exchange.
fn evaluate(values: Vec<serde_json::Value>) -> Result<WithdrawalLimits, StakeAccountError> {
    let (client, frames) = rpc_client(values, SLOT);
    let result =
        futures::executor::block_on(get_withdrawable_amount(&client, STAKE, signers(), query()));
    assert_eq!(frames.borrow().len(), 1);
    result.map(|result| result.value)
}

/// Current integer rates are explicit rather than inferred from an account's epoch.
fn query() -> StakeWithdrawalQuery {
    StakeWithdrawalQuery {
        read: ReadConfig {
            commitment: Some(Commitment::Confirmed),
            min_context_slot: Some(MINIMUM_SLOT),
        },
        decode_limits: Limits::new(BUDGET, BUDGET),
        new_rate_activation_epoch: Some(0),
    }
}

/// Caller identities are proposals; the read performs no signing or submission.
fn signers() -> WithdrawalSigners {
    WithdrawalSigners {
        withdrawer: WITHDRAWER,
        custodian: None,
    }
}

/// Each value is a standard complete-base64 account in the requested bank.
fn account(owner: Address, lamports: u64, bytes: Vec<u8>) -> serde_json::Value {
    serde_json::json!({
        "owner":owner.to_string(),
        "lamports":lamports,
        "executable":false,
        "space":bytes.len(),
        "rentEpoch":u64::MAX,
        "data":[STANDARD.encode(bytes),"base64"]
    })
}

/// Stored rent is deliberately stale; assessment must use the bundled current Rent.
fn stake_bytes(space: usize) -> Vec<u8> {
    let mut bytes = vec![0; space];
    bytes[..4].copy_from_slice(&1_u32.to_le_bytes());
    bytes[4..12].copy_from_slice(&u64::MAX.to_le_bytes());
    bytes[44..76].copy_from_slice(WITHDRAWER.as_bytes());
    bytes
}

/// Clock uses the source layout's signed timestamps and exact bank slot.
fn bundle() -> Vec<serde_json::Value> {
    let clock = [
        &SLOT.to_le_bytes()[..],
        &0_i64.to_le_bytes(),
        &EPOCH.to_le_bytes(),
        &EPOCH.to_le_bytes(),
        &NOW.to_le_bytes(),
    ]
    .concat();
    vec![
        account(solana_kit_stake::PROGRAM, BALANCE, stake_bytes(ALLOCATION)),
        account(sysvars::OWNER_ADDRESS, 1, clock),
        account(sysvars::OWNER_ADDRESS, 1, rent_bytes(RATE, 2.0)),
        account(sysvars::OWNER_ADDRESS, 1, history_bytes(&[1, 0])),
    ]
}

/// The wire field remains eight float bits, including unsupported configuration values.
fn rent_bytes(rate: u64, threshold: f64) -> Vec<u8> {
    [
        &rate.to_le_bytes()[..],
        &threshold.to_bits().to_le_bytes(),
        &[BURN_PERCENT],
    ]
    .concat()
}

/// Each epoch has independently calculable quarter and nine-percent stake allowances.
fn history_bytes(epochs: &[u64]) -> Vec<u8> {
    let mut bytes = (epochs.len() as u64).to_le_bytes().to_vec();

    for epoch in epochs {
        for value in [*epoch, 1_000, CLUSTER_PENDING, CLUSTER_PENDING] {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
    }

    bytes
}
