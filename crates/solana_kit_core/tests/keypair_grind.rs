#![cfg(feature = "crypto")]

use core::{
    future::{Future, Ready, ready},
    pin::Pin,
    task::{Context, Poll, Waker},
};
use std::rc::Rc;

use solana_kit_core::{
    Address,
    crypto::{
        Entropy, GrindBudget, GrindError, GrindPattern, GrindPatternError, Keypair,
        constant_time_eq, grind_keypairs, grind_keypairs_with,
    },
};

const ATTEMPTS: usize = 8;
const TWO_MATCH_ATTEMPTS: usize = 10;
const ZERO_REQUEST_ATTEMPTS: usize = 16;
const EXHAUSTED_ATTEMPTS: usize = 4;
const OVERLONG_LENGTH: usize = 45;
const SOURCE_BATCH_SIZE: usize = 32;

const FIRST_TARGET: u8 = 3;
const SECOND_TARGET: u8 = 6;
const EARLY_TARGET: u8 = 1;
const FIRST_CANDIDATE: u8 = 0;
const LOCAL_TARGET: u8 = 2;

/// Fills each 32-byte seed with the next counter and records host operations.
struct SequenceEntropy {
    next: u8,
    fills: usize,
    fail_at: Option<usize>,
}

impl SequenceEntropy {
    const fn new(fail_at: Option<usize>) -> Self {
        Self {
            next: 0,
            fills: 0,
            fail_at,
        }
    }
}

impl Entropy for SequenceEntropy {
    type Error = &'static str;
    type Fill<'a> = Ready<Result<(), Self::Error>>;

    fn fill<'a>(&'a mut self, output: &'a mut [u8]) -> Self::Fill<'a> {
        let fill = self.fills;
        self.fills += 1;

        if self.fail_at == Some(fill) {
            return ready(Err("host entropy denied"));
        }

        output.fill(self.next);
        self.next += 1;
        ready(Ok(()))
    }
}

/// Completes a local future without imposing `Send` on it or its host.
fn complete<T>(future: impl Future<Output = T>) -> T {
    let mut future = Box::pin(future);
    let mut context = Context::from_waker(Waker::noop());

    match future.as_mut().poll(&mut context) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("the test host must complete immediately"),
    }
}

fn key(seed: u8) -> Keypair {
    Keypair::from_seed(&[seed; 32])
}

fn address(seed: u8) -> Address {
    key(seed).address()
}

fn exact(seed: u8) -> String {
    address(seed).to_string()
}

#[test]
fn prefix_grind_rejects_candidates_then_returns_the_exact_count() {
    let mut entropy = SequenceEntropy::new(None);
    let literal = exact(FIRST_TARGET);
    let pattern = GrindPattern::prefix(&literal).expect("a real address is valid base58");
    assert_eq!(pattern.literal(), literal);
    assert!(pattern.matches(address(FIRST_TARGET)));
    assert!(!pattern.matches(address(FIRST_CANDIDATE)));
    let keys = complete(grind_keypairs(
        &mut entropy,
        pattern,
        GrindBudget::new(1, ATTEMPTS),
    ))
    .expect("the fourth candidate matches");

    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0].signer().address(), address(3));
    assert_eq!(entropy.fills, 4);
    assert_eq!(keys[0].backup().expose_secret()[..32], [3; 32]);
}

#[test]
fn suffix_grind_matches_multiple_targets_in_generation_order() {
    let mut entropy = SequenceEntropy::new(None);
    let first_literal = exact(FIRST_TARGET);
    let second_literal = exact(SECOND_TARGET);
    let first = GrindPattern::suffix(&first_literal).expect("valid suffix");
    let second = GrindPattern::suffix(&second_literal).expect("valid suffix");
    let keys = complete(grind_keypairs_with(
        &mut entropy,
        GrindBudget::new(2, TWO_MATCH_ATTEMPTS),
        |candidate| first.matches(candidate) || second.matches(candidate),
    ))
    .expect("both targets occur inside the budget");

    assert_eq!(keys.len(), 2);
    assert_eq!(keys[0].signer().address(), address(3));
    assert_eq!(keys[1].signer().address(), address(6));
    assert_eq!(entropy.fills, 7);
}

#[test]
fn zero_amount_returns_empty_without_touching_entropy() {
    let mut entropy = SequenceEntropy::new(None);
    let pattern = GrindPattern::prefix("").expect("an empty literal matches every address");
    let budget = GrindBudget::new(0, ZERO_REQUEST_ATTEMPTS);
    assert_eq!(budget.amount(), 0);
    assert_eq!(budget.attempts(), ZERO_REQUEST_ATTEMPTS);
    let keys =
        complete(grind_keypairs(&mut entropy, pattern, budget)).expect("an empty request succeeds");

    assert!(keys.is_empty());
    assert_eq!(entropy.fills, 0);
}

#[test]
fn invalid_literals_reject_before_the_entropy_host_is_used() {
    let entropy = SequenceEntropy::new(None);

    for (literal, expected) in [("0", '0'), ("O", 'O'), ("I", 'I'), ("l", 'l'), ("é", 'é')] {
        let error = GrindPattern::prefix(literal).expect_err("no address contains this symbol");
        let GrindPatternError::InvalidCharacter { character } = error else {
            panic!("expected an invalid character, received {error:?}");
        };

        assert_eq!(character, expected);
    }

    let overlong = "1".repeat(OVERLONG_LENGTH);
    assert_eq!(
        GrindPattern::suffix(&overlong),
        Err(GrindPatternError::TooLong {
            length: OVERLONG_LENGTH
        })
    );
    assert_eq!(entropy.fills, 0);
}

#[test]
fn exhaustion_reports_found_matches_without_returning_partial_results() {
    let mut entropy = SequenceEntropy::new(None);
    let first_literal = exact(EARLY_TARGET);
    let second_literal = exact(SECOND_TARGET);
    let first = GrindPattern::prefix(&first_literal).expect("valid prefix");
    let second = GrindPattern::prefix(&second_literal).expect("valid prefix");
    let error = complete(grind_keypairs_with(
        &mut entropy,
        GrindBudget::new(2, EXHAUSTED_ATTEMPTS),
        |candidate| first.matches(candidate) || second.matches(candidate),
    ))
    .expect_err("the second target is outside the budget");

    assert_eq!(
        error,
        GrindError::Exhausted {
            attempts: 4,
            found: 1
        }
    );
    assert_eq!(entropy.fills, 4);
}

#[test]
fn entropy_failure_before_a_match_stops_immediately() {
    let mut entropy = SequenceEntropy::new(Some(0));
    let literal = exact(FIRST_CANDIDATE);
    let pattern = GrindPattern::prefix(&literal).expect("valid prefix");
    let error = complete(grind_keypairs(
        &mut entropy,
        pattern,
        GrindBudget::new(1, ATTEMPTS),
    ))
    .expect_err("the host fails before generating a candidate");

    assert_eq!(error, GrindError::Entropy("host entropy denied"));
    assert_eq!(entropy.fills, 1);
}

#[test]
fn entropy_failure_after_a_match_discards_the_retained_key() {
    let mut entropy = SequenceEntropy::new(Some(1));
    let literal = exact(FIRST_CANDIDATE);
    let pattern = GrindPattern::prefix(&literal).expect("valid prefix");
    let error = complete(grind_keypairs(
        &mut entropy,
        pattern,
        GrindBudget::new(2, ATTEMPTS),
    ))
    .expect_err("the second candidate cannot be generated");

    assert_eq!(error, GrindError::Entropy("host entropy denied"));
    assert_eq!(entropy.fills, 2);
}

#[test]
fn empty_literal_matches_every_generated_address() {
    let mut entropy = SequenceEntropy::new(None);
    let pattern = GrindPattern::suffix("").expect("an empty literal matches every address");
    let keys = complete(grind_keypairs(
        &mut entropy,
        pattern,
        GrindBudget::new(2, 2),
    ))
    .expect("both candidates match");

    assert_eq!(keys.len(), 2);
    assert_eq!(entropy.fills, 2);
}

#[test]
fn synchronous_entropy_yields_between_batches_and_can_be_cancelled() {
    let mut entropy = SequenceEntropy::new(None);
    let mut future = Box::pin(grind_keypairs_with(
        &mut entropy,
        GrindBudget::new(1, SOURCE_BATCH_SIZE + 1),
        |_| false,
    ));
    let mut context = Context::from_waker(Waker::noop());

    assert!(future.as_mut().poll(&mut context).is_pending());
    drop(future);
    assert_eq!(entropy.fills, SOURCE_BATCH_SIZE);
}

#[test]
fn unrepresentable_result_capacity_rejects_without_entropy() {
    let mut entropy = SequenceEntropy::new(None);
    let pattern = GrindPattern::prefix("").expect("valid empty literal");
    let result = complete(grind_keypairs(
        &mut entropy,
        pattern,
        GrindBudget::new(usize::MAX, usize::MAX),
    ));

    assert_eq!(
        result.expect_err("capacity exceeds the allocation limit"),
        GrindError::Allocation
    );
    assert_eq!(entropy.fills, 0);
}

#[test]
fn grinds_accept_a_local_non_send_host_without_executor_timers() {
    let mut entropy = LocalEntropy { next: 0 };
    let literal = exact(LOCAL_TARGET);
    let pattern = GrindPattern::prefix(&literal).expect("valid prefix");
    let keys = complete(grind_keypairs(
        &mut entropy,
        pattern,
        GrindBudget::new(1, 3),
    ))
    .expect("the third local candidate matches");

    assert_eq!(keys[0].signer().address(), address(2));
}

#[test]
fn audited_constant_time_equality_handles_every_boundary_shape() {
    assert!(constant_time_eq(&[], &[]));
    assert!(!constant_time_eq(&[], &[0]));
    assert!(constant_time_eq(b"solana-kit", b"solana-kit"));
    assert!(!constant_time_eq(b"solana-kit", b"solana-kit."));
    assert!(!constant_time_eq(b"solana-kit", b"Solana-kit"));
    assert!(!constant_time_eq(&[0; 31], &[0; 32]));
    assert!(!constant_time_eq(&[0; 64], &[1; 64]));
}

/// A host whose fill future retains `Rc` state, so grinding must not require `Send`.
struct LocalEntropy {
    next: u8,
}

struct LocalFill {
    _local: Rc<()>,
}

impl Future for LocalFill {
    type Output = Result<(), &'static str>;

    fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(Ok(()))
    }
}

impl Entropy for LocalEntropy {
    type Error = &'static str;
    type Fill<'a> = LocalFill;

    fn fill<'a>(&'a mut self, output: &'a mut [u8]) -> Self::Fill<'a> {
        output.fill(self.next);
        self.next += 1;

        LocalFill {
            _local: Rc::new(()),
        }
    }
}
