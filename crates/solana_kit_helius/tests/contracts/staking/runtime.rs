//! One-call RPC and caller-signing controls run on every portable host.

use super::LIMITS;
use base64::{Engine, engine::general_purpose::STANDARD};
use solana_kit_client::{Client, Reply, Request, Transport, TransportError, plans::Signer};
use solana_kit_core::{
    Address, Hash, Signature,
    transaction::{self, Lifetime, Message, Transaction, Version},
};
use solana_kit_helius::staking::*;
use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    future::{Future, Ready, ready},
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, Waker},
};

const BLOCKHASH: &str = r#"{"context":{"slot":7},"value":{"blockhash":"11111111111111111111111111111111","lastValidBlockHeight":18446744073709551615}}"#;

#[derive(Clone, Copy)]
enum Mode {
    Success,
    Rejected,
    Missing,
    Changed,
    InvalidSignature,
}

#[derive(Debug, PartialEq)]
struct Rejected;

struct Backend {
    mode: Mode,
    calls: Rc<Cell<usize>>,
}

impl Signer for Backend {
    type Error = Rejected;
    type Sign<'a> = Ready<Result<Transaction, Rejected>>;

    fn sign(&self, message: Message) -> Self::Sign<'_> {
        self.calls.set(self.calls.get() + 1);

        if matches!(self.mode, Mode::Rejected) {
            return ready(Err(Rejected));
        }

        let message = if matches!(self.mode, Mode::Changed) {
            Message::compile(
                Version::V0,
                address(1),
                Lifetime::Blockhash {
                    blockhash: Hash::from_bytes([9; 32]),
                    last_valid_block_height: 1,
                },
                &[],
                &[],
            )
            .expect("different valid message")
        } else {
            message
        };
        let mut transaction = Transaction::new(message);

        if matches!(self.mode, Mode::Missing) {
            return ready(Ok(transaction));
        }

        for seed in [1, 2] {
            let owner = address(seed);

            if !transaction.message().signers().contains(&owner) {
                continue;
            }

            transaction = sign(
                transaction,
                owner,
                seed,
                matches!(self.mode, Mode::InvalidSignature),
            );
        }

        ready(Ok(transaction))
    }
}

fn address(seed: u8) -> Address {
    #[cfg(feature = "crypto")]
    {
        solana_kit_core::crypto::Keypair::from_seed(&[seed; 32]).address()
    }
    #[cfg(not(feature = "crypto"))]
    {
        Address::from_bytes([seed; 32])
    }
}

fn sign(transaction: Transaction, owner: Address, seed: u8, invalid: bool) -> Transaction {
    #[cfg(feature = "crypto")]
    if !invalid {
        return transaction
            .sign(&solana_kit_core::crypto::Keypair::from_seed(&[seed; 32]))
            .expect("fixture signing key");
    }

    #[cfg(not(feature = "crypto"))]
    let _ = (seed, invalid);

    transaction
        .with_signature(
            owner,
            transaction.message().bytes(),
            Signature::from_bytes([7; 64]),
        )
        .expect("nonzero structural signature")
}

struct Rpc {
    replies: RefCell<VecDeque<&'static str>>,
    requests: Rc<RefCell<Vec<serde_json::Value>>>,
}

impl Transport for Rpc {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;
    fn exchange(&self, request: Request) -> Self::Exchange<'_> {
        let value: serde_json::Value =
            serde_json::from_slice(&request.body).expect("actual RPC frame");
        let id = value["id"].as_u64().expect("correlated request ID");
        self.requests.borrow_mut().push(value);
        let result = self
            .replies
            .borrow_mut()
            .pop_front()
            .expect("one queued exchange");
        ready(Ok(Reply {
            status: 200,
            body: format!(r#"{{"jsonrpc":"2.0","id":{id},"result":{result}}}"#).into_bytes(),
        }))
    }
}

type Requests = Rc<RefCell<Vec<serde_json::Value>>>;

fn client(replies: &[&'static str]) -> (Client<Rpc>, Requests) {
    let requests = Rc::new(RefCell::new(Vec::new()));
    (
        Client::new(
            Rpc {
                replies: RefCell::new(replies.iter().copied().collect()),
                requests: Rc::clone(&requests),
            },
            solana_kit_client::Limits::default(),
        ),
        requests,
    )
}

fn options() -> PreparationOptions {
    PreparationOptions {
        commitment: Some(solana_kit_client::methods::Commitment::Confirmed),
        instruction_limits: LIMITS,
    }
}

fn request() -> StakeRequest {
    StakeRequest {
        owner: address(1),
        stake_account: address(2),
        lamports: 9_007_199_254_740_993,
    }
}

fn backend(mode: Mode) -> Backend {
    Backend {
        mode,
        calls: Rc::new(Cell::new(0)),
    }
}

pub(super) fn transactions() {
    let (rpc, reads) = client(&["2"]);
    let result = futures::executor::block_on(get_stake_instructions(&rpc, request(), options()))
        .expect("instruction-only preparation");
    assert_eq!(result.instructions.len(), 3);
    assert_eq!(reads.borrow().len(), 1);

    stake_transaction();
    transitions();
}

fn stake_transaction() {
    let (rpc, requests) = client(&["2", BLOCKHASH]);
    let signer = backend(Mode::Success);
    let result = futures::executor::block_on(create_stake_transaction(
        &rpc,
        &signer,
        request(),
        options(),
    ))
    .expect("prepared caller-signed stake transaction");
    assert_eq!(signer.calls.get(), 1);
    assert_eq!(result.stake_account, address(2));
    assert_eq!(result.transaction.message().version(), Version::V0);
    assert_eq!(result.transaction.message().instructions().len(), 3);
    assert_eq!(result.transaction.message().signers().len(), 2);
    let encoded =
        serialize_transaction(&result.transaction, LIMITS).expect("canonical Base64 wire");
    let bytes = STANDARD
        .decode(encoded)
        .expect("independent receiver decoding");
    assert_eq!(
        Transaction::decode(&bytes)
            .expect("strict wire transaction")
            .message()
            .bytes(),
        result.transaction.message().bytes()
    );
    assert!(
        serialize_transaction(
            &result.transaction,
            solana_kit_core::codecs::Limits::new(0, 0)
        )
        .is_err()
    );
    let requests = requests.borrow();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0]["method"], "getMinimumBalanceForRentExemption");
    assert_eq!(
        requests[0]["params"],
        serde_json::json!([200, {"commitment":"confirmed"}])
    );
    assert_eq!(requests[1]["method"], "getLatestBlockhash");
    assert_eq!(
        requests[1]["params"],
        serde_json::json!([{"commitment":"confirmed"}])
    );
}

fn transitions() {
    let signer = backend(Mode::Success);
    let (rpc, reads) = client(&[BLOCKHASH, BLOCKHASH]);
    let unstake = futures::executor::block_on(create_unstake_transaction(
        &rpc,
        &signer,
        UnstakeRequest {
            owner: address(1),
            stake_account: address(2),
        },
        options(),
    ))
    .expect("deactivation transaction");
    let withdrawal = futures::executor::block_on(create_withdraw_transaction(
        &rpc,
        &signer,
        WithdrawalRequest {
            authority: address(1),
            stake_account: address(2),
            destination: address(3),
            lamports: u64::MAX,
        },
        options(),
    ))
    .expect("withdrawal transaction");
    assert_eq!(unstake.message().instructions()[0].data(), [5, 0, 0, 0]);
    assert_eq!(
        withdrawal.message().instructions()[0].data(),
        [&[4, 0, 0, 0][..], &u64::MAX.to_le_bytes()].concat()
    );
    assert_eq!(reads.borrow().len(), 2);
}

pub(super) fn failures() {
    funding_failures();
    signer_failures();
    signature_policy();
}

fn funding_failures() {
    let (rpc, reads) = client(&["1"]);
    let signer = backend(Mode::Success);
    let mut overflowing = request();
    overflowing.lamports = u64::MAX;
    let result = futures::executor::block_on(create_stake_transaction(
        &rpc,
        &signer,
        overflowing,
        options(),
    ));
    assert!(matches!(
        result,
        Err(PreparationError::Build(StakingBuildError::LamportOverflow))
    ));
    assert_eq!(reads.borrow().len(), 1);
    assert_eq!(signer.calls.get(), 0);

    let (rpc, reads) = client(&["\"invalid rent\""]);
    let result = futures::executor::block_on(get_stake_instructions(&rpc, request(), options()));
    assert!(matches!(result, Err(PreparationError::Rpc(_))));
    assert_eq!(reads.borrow().len(), 1);

    let (rpc, reads) = client(&["1", "null"]);
    let result = futures::executor::block_on(create_stake_transaction(
        &rpc,
        &signer,
        request(),
        options(),
    ));
    assert!(matches!(result, Err(PreparationError::Rpc(_))));
    assert_eq!(reads.borrow().len(), 2);
    assert_eq!(signer.calls.get(), 0);
}

fn signer_failures() {
    for mode in [Mode::Rejected, Mode::Missing, Mode::Changed] {
        let (rpc, reads) = client(&[BLOCKHASH]);
        let signer = backend(mode);
        let result = futures::executor::block_on(create_unstake_transaction(
            &rpc,
            &signer,
            UnstakeRequest {
                owner: address(1),
                stake_account: address(2),
            },
            options(),
        ));

        match mode {
            Mode::Rejected => assert!(matches!(result, Err(PreparationError::Signer(Rejected)))),
            Mode::Missing => assert!(matches!(
                result,
                Err(PreparationError::Build(StakingBuildError::Transaction(
                    transaction::Error::MissingSignature
                )))
            )),
            Mode::Changed => assert!(matches!(
                result,
                Err(PreparationError::Build(StakingBuildError::Transaction(
                    transaction::Error::ChangedMessage
                )))
            )),
            _ => unreachable!("closed negative control set"),
        }

        assert_eq!(reads.borrow().len(), 1);
        assert_eq!(signer.calls.get(), 1);
    }
}

fn signature_policy() {
    let (rpc, _) = client(&[BLOCKHASH]);
    let result = futures::executor::block_on(create_unstake_transaction(
        &rpc,
        &backend(Mode::InvalidSignature),
        UnstakeRequest {
            owner: address(1),
            stake_account: address(2),
        },
        options(),
    ));
    #[cfg(feature = "crypto")]
    assert!(matches!(
        result,
        Err(PreparationError::Build(StakingBuildError::Transaction(
            transaction::Error::InvalidSignature
        )))
    ));
    #[cfg(not(feature = "crypto"))]
    assert!(
        result.is_ok(),
        "crypto-disabled preparation checks structural signature custody"
    );
}

struct PendingSigner {
    dropped: Rc<Cell<usize>>,
}
struct PendingSign(Rc<Cell<usize>>);
impl Future for PendingSign {
    type Output = Result<Transaction, Rejected>;
    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}
impl Drop for PendingSign {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}
impl Signer for PendingSigner {
    type Error = Rejected;
    type Sign<'a> = PendingSign;
    fn sign(&self, _: Message) -> Self::Sign<'_> {
        PendingSign(Rc::clone(&self.dropped))
    }
}

pub(super) fn cancellation() {
    let (rpc, reads) = client(&[BLOCKHASH]);
    let dropped = Rc::new(Cell::new(0));
    let signer = PendingSigner {
        dropped: Rc::clone(&dropped),
    };
    let mut future = Box::pin(create_unstake_transaction(
        &rpc,
        &signer,
        UnstakeRequest {
            owner: address(1),
            stake_account: address(2),
        },
        options(),
    ));
    assert!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(Waker::noop()))
            .is_pending()
    );
    assert_eq!(reads.borrow().len(), 1);
    drop(future);
    assert_eq!(dropped.get(), 1);
}
