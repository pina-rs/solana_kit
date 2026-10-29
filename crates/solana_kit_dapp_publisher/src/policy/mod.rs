//! A private immutable seal binds the caller's policy to one received transaction.

mod collection;
mod validate;

use crate::Error;
use solana_kit_core::{Address, Hash, Signature, codecs::Limits, crypto, transaction::Transaction};

/// Independently supplied expectations for the transaction presented for signing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExpectedTransaction {
    /// Exact signed lifetime token; this expectation alone does not establish freshness.
    pub blockhash: Hash,
    /// Required first static account and fee payer.
    pub payer: Address,
    /// Caller-controlled identity that will sign the immutable message.
    pub signer: Address,
}

/// Canonical publication checks. Neither variant permits caller-expanded program allowlists.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PublicationPolicy {
    /// Mint a release NFT into the expected app collection with a pre-signed mint.
    ReleaseMint {
        /// Common signed-message expectations.
        expected: ExpectedTransaction,
        /// Required release mint and existing-signature identity.
        mint: Address,
        /// Expected collection key in the first Token Metadata Create V1 payload.
        collection: Address,
    },
    /// Verify a release NFT against the expected app collection and authority.
    VerifyCollection {
        /// Common signed-message expectations.
        expected: ExpectedTransaction,
        /// Mint used to derive the required release metadata account.
        nft_mint: Address,
        /// Required collection mint and PDA seed.
        collection: Address,
        /// Required collection authority and signer-set member.
        authority: Address,
    },
}

impl PublicationPolicy {
    /// Returns the common expectations bound into the seal.
    #[must_use]
    pub const fn expected(self) -> ExpectedTransaction {
        match self {
            Self::ReleaseMint { expected, .. } | Self::VerifyCollection { expected, .. } => {
                expected
            }
        }
    }
}

/// One exact bare message and its expected signing identity, ready for caller review.
#[derive(Clone, Copy, Debug)]
pub struct SigningRequest<'a> {
    /// The only identity accepted by signature attachment.
    pub signer: Address,
    /// Unmodified transaction message bytes, without an off-chain-message envelope.
    pub message: &'a [u8],
}

/// A transaction that passed the documented policy, with every present signature verified.
///
/// This is not proof of intended economic effects, blockhash freshness, account
/// authenticity, or validator acceptance. Only the documented source policy is sealed.
#[derive(Debug)]
pub struct ValidatedPreparedTransaction {
    transaction: Transaction,
    policy: PublicationPolicy,
}

impl ValidatedPreparedTransaction {
    /// Public policy facts used to validate these exact immutable bytes.
    #[must_use]
    pub const fn policy(&self) -> PublicationPolicy {
        self.policy
    }

    /// Borrows the frozen transaction for review or an existing partial-signer capability.
    #[must_use]
    pub const fn transaction(&self) -> &Transaction {
        &self.transaction
    }

    /// Selects the exact bytes and expected address for caller-owned signing.
    #[must_use]
    pub fn signing_request(&self) -> SigningRequest<'_> {
        SigningRequest {
            signer: self.policy.expected().signer,
            message: self.transaction.message().bytes(),
        }
    }

    /// Consumes the seal, verifies the detached signature and requires complete signatures.
    ///
    /// A different signature for any occupied slot of the expected signer is
    /// rejected before cryptographic admission. Identical supplied bytes are
    /// verified again and may fill empty slots for the same identity. Existing
    /// signatures and message bytes remain unchanged. The returned Kit transaction
    /// can be explicitly encoded with `encode(true)` by its caller.
    pub fn attach_signature(self, signature: Signature) -> Result<Transaction, Error> {
        let request = self.signing_request();
        validate::occupied_signature(&self.transaction, request.signer, signature)?;
        crypto::verify(&request.signer, request.message, &signature)
            .map_err(|_| Error::InvalidSignature(request.signer))?;
        let signed = self
            .transaction
            .with_signature(request.signer, request.message, signature)
            .map_err(Error::Transaction)?;
        signed.verify().map_err(Error::Transaction)?;

        Ok(signed)
    }
}

/// Validates complete wire bytes before exposing a signable policy seal.
///
/// `max_bytes` bounds the input before core parsing. `max_items` conservatively
/// reserves three elements per input byte for core's copied message, parsed tree
/// and temporary headers; only the remainder is available to metadata decoding.
/// Core's independent 1232/4096-byte protocol caps also apply. Lookup descriptors
/// are rejected because this slice accepts no authenticated address snapshots.
pub fn validate_prepared_transaction(
    bytes: &[u8],
    policy: PublicationPolicy,
    limits: Limits,
) -> Result<ValidatedPreparedTransaction, Error> {
    let reserved = bytes.len().checked_mul(3).ok_or(Error::LimitExceeded)?;
    let items = limits
        .max_items
        .checked_sub(reserved)
        .ok_or(Error::LimitExceeded)?;

    if bytes.len() > limits.max_bytes {
        return Err(Error::LimitExceeded);
    }

    let transaction = Transaction::decode(bytes).map_err(Error::Transaction)?;
    validate::common(&transaction, policy.expected())?;
    collection::validate(&transaction, policy, Limits::new(limits.max_bytes, items))?;

    Ok(ValidatedPreparedTransaction {
        transaction,
        policy,
    })
}
