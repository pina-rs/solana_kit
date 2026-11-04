#![cfg(feature = "crypto")]

use core::{
    future::{Future, Ready, ready},
    task::{Context, Poll, Waker},
};

use solana_kit_core::{
    Address,
    crypto::{Entropy, GrindBudget, GrindError, GrindPattern, Keypair, verify},
};
use solana_kit_signers::{grind_keypair_signers, grind_keypair_signers_with};

/// Fills each 32-byte seed with the next counter and records host operations.
struct SequenceEntropy {
    next: u8,
    fills: usize,
}

impl Entropy for SequenceEntropy {
    type Error = &'static str;
    type Fill<'a> = Ready<Result<(), Self::Error>>;

    fn fill<'a>(&'a mut self, output: &'a mut [u8]) -> Self::Fill<'a> {
        output.fill(self.next);
        self.next += 1;
        self.fills += 1;
        ready(Ok(()))
    }
}

fn complete<T>(future: impl Future<Output = T>) -> T {
    let mut future = Box::pin(future);
    let mut context = Context::from_waker(Waker::noop());

    match future.as_mut().poll(&mut context) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("the test host must complete immediately"),
    }
}

fn address(seed: u8) -> Address {
    Keypair::from_seed(&[seed; 32]).address()
}

fn exact(seed: u8) -> String {
    address(seed).to_string()
}

#[test]
fn ground_signers_own_the_grounded_address_and_verify_their_signatures() {
    let mut entropy = SequenceEntropy { next: 0, fills: 0 };
    let literal = exact(2);
    let pattern = GrindPattern::prefix(&literal).expect("valid prefix");
    let signers = complete(grind_keypair_signers(
        &mut entropy,
        pattern,
        GrindBudget::new(1, 4),
    ))
    .expect("the third candidate matches");

    assert_eq!(signers.len(), 1);
    assert_eq!(signers[0].address(), address(2));
    assert_eq!(entropy.fills, 3);

    let message = b"the exact grounded signer bytes";
    let signature = signers[0].sign(message);
    verify(&signers[0].address(), message, &signature).expect("the grounded signer owns its slot");
}

#[test]
fn predicate_signers_compose_arbitrary_matching_and_exact_counts() {
    let mut entropy = SequenceEntropy { next: 0, fills: 0 };
    let first = address(0);
    let second = address(1);
    let signers = complete(grind_keypair_signers_with(
        &mut entropy,
        GrindBudget::new(2, 6),
        |candidate| candidate == first || candidate == second,
    ))
    .expect("the bounded predicate finds two matches");

    assert_eq!(signers.len(), 2);
    assert_eq!(entropy.fills, 2);
    assert_eq!(signers[0].address(), first);
    assert_eq!(signers[1].address(), second);
}

#[test]
fn empty_requests_and_exhaustion_keep_the_core_contract() {
    let mut entropy = SequenceEntropy { next: 0, fills: 0 };
    let pattern = GrindPattern::prefix("").expect("empty literals match every address");
    let empty = complete(grind_keypair_signers(
        &mut entropy,
        pattern,
        GrindBudget::new(0, 4),
    ))
    .expect("an empty request succeeds");

    assert!(empty.is_empty());
    assert_eq!(entropy.fills, 0);

    let literal = exact(9);
    let pattern = GrindPattern::prefix(&literal).expect("valid prefix");
    let error = match complete(grind_keypair_signers(
        &mut entropy,
        pattern,
        GrindBudget::new(1, 2),
    )) {
        Ok(_) => panic!("the target is outside the remaining budget"),
        Err(error) => error,
    };

    assert_eq!(
        error,
        GrindError::Exhausted {
            attempts: 2,
            found: 0
        }
    );
}
