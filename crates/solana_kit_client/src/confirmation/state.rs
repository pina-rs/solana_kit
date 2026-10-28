//! Typed observations share the RPC and transaction domain owners.
use crate::{
    methods::{
        Commitment, Contextual,
        history::ConfirmationStatus,
        node::EpochInfo,
        submission::{SignatureStatus, TransactionFailure},
    },
    subscriptions::SignatureInfo,
};
use alloc::boxed::Box;
use core::{cmp::Ordering, fmt};
use solana_kit_core::{Address, Hash, Signature, transaction::Lifetime};

/// Explicit confirmation target and minimum evaluation context.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Policy {
    /// Required execution commitment.
    pub commitment: Commitment,
    /// Older evaluation snapshots are ignored, including nonce replacements.
    pub min_context_slot: u64,
}
impl Default for Policy {
    fn default() -> Self {
        Self {
            commitment: Commitment::Confirmed,
            min_context_slot: 0,
        }
    }
}

/// One observation from an explicitly scoped, trusted adapter.
#[derive(Debug)]
pub enum Event {
    /// Positional RPC lookup, already correlated to this signature.
    Status {
        /// Exact queried signature.
        signature: Signature,
        /// RPC evaluation context and optional known status.
        snapshot: Contextual<Option<SignatureStatus>>,
    },
    /// A subscription result after its configured commitment was reached.
    Signature {
        /// Exact subscription subject.
        signature: Signature,
        /// Evaluation slot from the notification context.
        slot: u64,
        /// Commitment used when registering the subscription.
        commitment: Commitment,
        /// Receipt or actual execution result.
        value: SignatureInfo,
    },
    /// Actual RPC epoch information; slot estimates cannot establish expiry.
    Epoch(EpochInfo),
    /// Adapter-validated initialized nonce state, or verified account absence.
    ///
    /// The adapter must validate System ownership, exact layout and initialized
    /// current nonce state before emitting this event. Raw untrusted bytes must
    /// never be converted directly into this assertion. Context is node-reported,
    /// not cryptographic authentication. RPC nonce decoding is a pending adapter.
    Nonce {
        /// Account actually queried and validated.
        address: Address,
        /// Evaluation slot.
        slot: u64,
        /// Current token; None means the account was absent.
        token: Option<Hash>,
    },
    /// An injected deadline expired; no ambient clock is consulted.
    Timeout,
}

/// Confirmation and observation failures remain separate from cleanup failures.
#[derive(Debug)]
pub enum Failure {
    /// A reducer already consumed its terminal event before waiting started.
    AlreadySettled,
    /// Transaction wire encoding failed before submission.
    Encoding(solana_kit_core::transaction::Error),
    /// No primary signature exists.
    MissingSignature,
    /// RPC reported another transaction identity.
    ReportedSignatureMismatch {
        /// Primary signature from the exact submitted transaction.
        expected: Signature,
        /// Signature returned by the node.
        actual: Box<Signature>,
    },
    /// Received wire metadata cannot establish expiry.
    UnresolvedLifetime,
    /// Observer delivered an event for another signature or nonce account.
    ForeignObservation,
    /// Transaction execution failed, preserving the typed node failure.
    Transaction(TransactionFailure),
    /// The actual block height strictly exceeded the inclusive last valid height.
    BlockHeightExceeded {
        /// Observed height.
        current: u64,
        /// Inclusive expiry boundary.
        last_valid: u64,
    },
    /// Validated nonce account observation reports absence.
    NonceAccountMissing(Address),
    /// Validated current nonce differs from the signed lifetime token.
    NonceInvalidated {
        /// Expected signed token.
        expected: Hash,
        /// Current observed token.
        actual: Hash,
    },
    /// Injected deadline elapsed.
    Timeout,
    /// Caller cancellation won the race.
    Cancelled,
    /// Observation stream ended without a decisive result.
    ObserverEnded,
    /// Existing RPC or transport failure, preserved without retry.
    Rpc(crate::Error),
    /// Existing subscription owner failure, including false unsubscribe and gaps.
    Subscription(crate::subscriptions::Failure),
}
impl Failure {
    /// Keeps the established high-level mismatch contract when low-level decoding rejects it.
    pub(super) fn submission_rpc(error: crate::Error) -> Self {
        match error {
            crate::Error::SubmissionSignatureMismatch { expected, actual } => {
                Self::ReportedSignatureMismatch { expected, actual }
            }
            error => Self::Rpc(error),
        }
    }
}

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "confirmation failure: {self:?}")
    }
}
impl core::error::Error for Failure {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Rpc(error) => Some(error),
            Self::Encoding(error) => Some(error),
            Self::Subscription(error) => Some(error),
            _ => None,
        }
    }
}

/// One single-use confirmation reducer; late observations cannot change outcome.
#[derive(Debug)]
pub struct Confirmation {
    signature: Signature,
    lifetime: Lifetime,
    policy: Policy,
    settled: bool,
    nonce_observed_slot: u64,
}
impl Confirmation {
    /// Whether a terminal event has already been consumed.
    #[must_use]
    pub const fn is_settled(&self) -> bool {
        self.settled
    }

    /// Requires explicit lifetime metadata before starting any observer.
    pub fn new(signature: Signature, lifetime: Lifetime, policy: Policy) -> Result<Self, Failure> {
        if matches!(lifetime, Lifetime::Unresolved(_)) {
            return Err(Failure::UnresolvedLifetime);
        }

        Ok(Self {
            signature,
            lifetime,
            policy,
            settled: false,
            nonce_observed_slot: policy.min_context_slot,
        })
    }

    /// Applies one event; None means pending or already settled.
    ///
    /// Failures in execution win even below the desired commitment. Stale
    /// contexts and preliminary receipt notifications cannot settle confirmation.
    pub fn observe(&mut self, event: Event) -> Option<Result<(), Failure>> {
        if self.settled {
            return None;
        }

        let outcome = self.evaluate(event);
        self.settled = outcome.is_some();
        outcome
    }

    fn evaluate(&mut self, event: Event) -> Option<Result<(), Failure>> {
        match event {
            Event::Status {
                signature,
                snapshot,
            } => self.status(signature, snapshot),
            Event::Signature {
                signature,
                slot,
                commitment,
                value,
            } => self.notification(signature, slot, commitment, value),
            Event::Epoch(info) => self.epoch(info),
            Event::Nonce {
                address,
                slot,
                token,
            } => self.nonce(address, slot, token),
            Event::Timeout => Some(Err(Failure::Timeout)),
        }
    }

    fn status(
        &mut self,
        signature: Signature,
        snapshot: Contextual<Option<SignatureStatus>>,
    ) -> Option<Result<(), Failure>> {
        if signature != self.signature {
            return Some(Err(Failure::ForeignObservation));
        }

        if snapshot.context.slot < self.policy.min_context_slot {
            return None;
        }

        let status = snapshot.value?;

        if let Some(error) = status.err {
            return Some(Err(Failure::Transaction(error)));
        }

        let commitment = match status.confirmation_status? {
            ConfirmationStatus::Processed => Commitment::Processed,
            ConfirmationStatus::Confirmed => Commitment::Confirmed,
            ConfirmationStatus::Finalized => Commitment::Finalized,
        };
        self.reached(commitment)
    }

    fn notification(
        &mut self,
        signature: Signature,
        slot: u64,
        commitment: Commitment,
        value: SignatureInfo,
    ) -> Option<Result<(), Failure>> {
        if signature != self.signature {
            return Some(Err(Failure::ForeignObservation));
        }

        if slot < self.policy.min_context_slot {
            return None;
        }

        match value {
            SignatureInfo::Received(_) => None,
            SignatureInfo::Processed { err: Some(error) } => Some(Err(Failure::Transaction(error))),
            SignatureInfo::Processed { err: None } => self.reached(commitment),
        }
    }

    fn fresh_nonce(&mut self, slot: u64) -> bool {
        if slot < self.nonce_observed_slot {
            return false;
        }

        self.nonce_observed_slot = slot;
        true
    }

    fn reached(&self, commitment: Commitment) -> Option<Result<(), Failure>> {
        (commitment_comparator(commitment, self.policy.commitment) != Ordering::Less)
            .then_some(Ok(()))
    }

    fn epoch(&mut self, info: EpochInfo) -> Option<Result<(), Failure>> {
        let Lifetime::Blockhash {
            last_valid_block_height,
            ..
        } = self.lifetime
        else {
            return None;
        };

        if info.absolute_slot < self.policy.min_context_slot
            || info.block_height <= last_valid_block_height
        {
            return None;
        }

        Some(Err(Failure::BlockHeightExceeded {
            current: info.block_height,
            last_valid: last_valid_block_height,
        }))
    }

    fn nonce(
        &mut self,
        address: Address,
        slot: u64,
        token: Option<Hash>,
    ) -> Option<Result<(), Failure>> {
        let Lifetime::DurableNonce(nonce) = self.lifetime else {
            return None;
        };

        if address != nonce.advance().nonce_account {
            return Some(Err(Failure::ForeignObservation));
        }

        if !self.fresh_nonce(slot) {
            return None;
        }

        let Some(actual) = token else {
            return Some(Err(Failure::NonceAccountMissing(address)));
        };

        (actual != nonce.token()).then_some(Err(Failure::NonceInvalidated {
            expected: nonce.token(),
            actual,
        }))
    }
}

/// Canonical processed < confirmed < finalized ordering.
#[must_use]
pub fn commitment_comparator(left: Commitment, right: Commitment) -> Ordering {
    fn rank(value: Commitment) -> u8 {
        match value {
            Commitment::Processed => 0,
            Commitment::Confirmed => 1,
            Commitment::Finalized => 2,
        }
    }

    rank(left).cmp(&rank(right))
}

/// Pinned timeout duration, applied by an explicitly injected timer.
#[must_use]
pub const fn timeout_millis(commitment: Commitment) -> u64 {
    match commitment {
        Commitment::Processed => 30_000,
        Commitment::Confirmed | Commitment::Finalized => 60_000,
    }
}
