use alloc::{boxed::Box, vec, vec::Vec};
use core::{
    future::{Future, poll_fn},
    pin::Pin,
    task::{Context, Poll},
};

use solana_kit_core::{Address, Signature, transaction::Transaction};

use crate::{
    Error, SignatureOutput, Signer, SignerCapability, TransactionModifyingSigner,
    TransactionPartialSigner, TransactionSendingSigner,
};

/// A deduplicated, insertion-ordered set of caller-selected partial capabilities.
///
/// Composite signers have an explicit role. Supplying one address in both the
/// modifying and partial stages is a conflict, rather than implicit demotion.
pub struct TransactionSigners<'a, P> {
    partials: Vec<&'a SignerCapability<'a, P>>,
}

impl<'a, P: TransactionPartialSigner> TransactionSigners<'a, P> {
    /// Deduplicates repeated handles and rejects distinct handles with one address.
    ///
    /// # Errors
    /// Returns [`Error::ConflictingSigner`] for conflicting identities.
    pub fn new(partials: &[&'a SignerCapability<'a, P>]) -> Result<Self, Error> {
        Ok(Self {
            partials: deduplicate(partials)?,
        })
    }

    /// Signs one shared immutable snapshot concurrently, merging in input order.
    ///
    /// Dropping this future drops every unfinished backend future. No task is
    /// detached. Backend errors propagate without retrying or committing output.
    ///
    /// # Errors
    /// Propagates rejection, cancellation, foreign owners, and changed snapshots.
    pub async fn partially_sign(&self, transaction: &Transaction) -> Result<Transaction, Error> {
        let mut pending: Vec<_> = self
            .partials
            .iter()
            .map(|signer| Some(Box::pin(signer.signer.sign_transaction(transaction))))
            .collect();
        let mut outputs = vec![None; pending.len()];

        poll_fn(|context| poll_all(&mut pending, &mut outputs, context)).await?;

        let mut result = transaction.clone();

        for (signer, output) in self.partials.iter().zip(outputs.into_iter().flatten()) {
            result = output.apply(signer.address(), &result)?;
        }

        Ok(result)
    }

    /// Signs the snapshot and requires every address-owned slot to be populated.
    ///
    /// # Errors
    /// Also rejects missing signatures. With `crypto`, verifies every signature.
    pub async fn sign(&self, transaction: &Transaction) -> Result<Transaction, Error> {
        let result = self.partially_sign(transaction).await?;
        require_complete(&result)?;
        Ok(result)
    }

    /// Applies modifiers sequentially before partial signing the final snapshot.
    ///
    /// # Errors
    /// Rejects capability conflicts, foreign output, and reused stale signatures.
    pub async fn modify_and_partially_sign<M: TransactionModifyingSigner>(
        &self,
        transaction: &Transaction,
        modifiers: &[&SignerCapability<'_, M>],
    ) -> Result<Transaction, Error> {
        let modifiers = deduplicate(modifiers)?;
        let mut current = transaction.clone();

        if modifiers.iter().any(|modifier| {
            self.partials
                .iter()
                .any(|partial| partial.address() == modifier.address())
        }) {
            return Err(Error::ConflictingSigner);
        }

        for modifier in modifiers {
            let next = modifier.signer.modify_transaction(&current).await?;
            validate_modification(&current, &next, modifier.address())?;
            current = next;
        }

        self.partially_sign(&current).await
    }

    /// Partially signs, then invokes exactly one explicit sending capability once.
    ///
    /// The sender may authorize its own remaining slot. A returned signature is
    /// a submission identifier, with no execution or confirmation guarantee.
    ///
    /// # Errors
    /// Rejects missing or multiple senders and duplicate roles before any signing.
    pub async fn sign_and_send<S: TransactionSendingSigner>(
        &self,
        transaction: &Transaction,
        senders: &[&SignerCapability<'_, S>],
    ) -> Result<Signature, Error> {
        let sender = match senders {
            [] => return Err(Error::MissingSender),
            [sender] => *sender,
            _ => return Err(Error::MultipleSenders),
        };

        if self
            .partials
            .iter()
            .any(|partial| partial.address() == sender.address())
        {
            return Err(Error::ConflictingSigner);
        }

        let signed = self.partially_sign(transaction).await?;

        for (address, slot) in signed.message().signers().iter().zip(signed.signatures()) {
            if *address != sender.address() && slot.is_none() {
                return Err(solana_kit_core::transaction::Error::MissingSignature.into());
            }

            if let Some(signature) = slot {
                SignatureOutput::new(*address, signed.message().bytes(), *signature)
                    .validate(*address, signed.message().bytes())?;
            }
        }

        let identifier = sender.signer.send_transaction(&signed).await?;

        if identifier.as_bytes() == &[0; 64] {
            return Err(solana_kit_core::transaction::Error::InvalidSignature.into());
        }

        Ok(identifier)
    }
}

fn deduplicate<'a, S: Signer>(
    signers: &[&'a SignerCapability<'a, S>],
) -> Result<Vec<&'a SignerCapability<'a, S>>, Error> {
    let mut result: Vec<&SignerCapability<'_, S>> = Vec::new();

    for signer in signers {
        if let Some(existing) = result
            .iter()
            .find(|item| item.address() == signer.address())
        {
            if !core::ptr::eq(*existing, *signer) {
                return Err(Error::ConflictingSigner);
            }
        } else {
            result.push(signer);
        }
    }

    Ok(result)
}

pub(crate) fn require_complete(transaction: &Transaction) -> Result<(), Error> {
    transaction.encode(true)?;

    #[cfg(feature = "crypto")]
    transaction.verify()?;

    Ok(())
}

pub(crate) fn validate_modification(
    previous: &Transaction,
    next: &Transaction,
    owner: Address,
) -> Result<(), Error> {
    for (address, signature) in next.message().signers().iter().zip(next.signatures()) {
        if let Some(signature) = signature {
            validate_slot(previous, next, owner, *address, *signature)?;
            crate::SignatureOutput::new(*address, next.message().bytes(), *signature)
                .validate(*address, next.message().bytes())?;
        }
    }

    Ok(())
}

fn validate_slot(
    previous: &Transaction,
    next: &Transaction,
    owner: Address,
    address: Address,
    signature: Signature,
) -> Result<(), Error> {
    let changed = previous.message().bytes() != next.message().bytes();
    let old = previous
        .message()
        .signers()
        .iter()
        .zip(previous.signatures())
        .find(|(old_address, _)| **old_address == address)
        .and_then(|(_, signature)| *signature);

    if changed && Some(signature) == old {
        return Err(Error::ChangedSnapshot);
    }

    if address != owner && (changed || Some(signature) != old) {
        return Err(Error::ForeignSigner);
    }

    Ok(())
}

pub(crate) fn poll_all<F: Future<Output = Result<SignatureOutput, Error>> + ?Sized>(
    pending: &mut [Option<Pin<Box<F>>>],
    outputs: &mut [Option<SignatureOutput>],
    context: &mut Context<'_>,
) -> Poll<Result<(), Error>> {
    for (operation, output) in pending.iter_mut().zip(outputs) {
        poll_one(operation, output, context)?;
    }

    if pending.iter().all(Option::is_none) {
        return Poll::Ready(Ok(()));
    }

    Poll::Pending
}

fn poll_one<F: Future<Output = Result<SignatureOutput, Error>> + ?Sized>(
    operation: &mut Option<Pin<Box<F>>>,
    output: &mut Option<SignatureOutput>,
    context: &mut Context<'_>,
) -> Result<(), Error> {
    let Some(future) = operation else {
        return Ok(());
    };

    if let Poll::Ready(result) = Pin::as_mut(future).poll(context) {
        *output = Some(result?);
        *operation = None;
    }

    Ok(())
}
