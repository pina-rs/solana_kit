use super::{MessageError, MessageSignatureOutput, SignableMessage};
use solana_kit_core::{Address, Signature};

pub(super) fn signatures(message: &SignableMessage) -> Result<(), MessageError> {
    for (owner, signature) in message.signatures() {
        verify(*owner, message.content(), *signature)?;
    }

    Ok(())
}

fn verify(owner: Address, bytes: &[u8], signature: Signature) -> Result<(), MessageError> {
    if signature.as_bytes() == &[0; 64] {
        return Err(MessageError::InvalidSignature);
    }

    #[cfg(feature = "crypto")]
    solana_kit_core::crypto::verify(&owner, bytes, &signature)
        .map_err(|_| MessageError::InvalidSignature)?;

    #[cfg(not(feature = "crypto"))]
    let _ = (owner, bytes);

    Ok(())
}

pub(super) fn merge(
    message: &mut SignableMessage,
    output: MessageSignatureOutput,
    owner: Address,
) -> Result<(), MessageError> {
    if output.content() != message.content() {
        return Err(MessageError::ChangedSnapshot);
    }

    if output.signatures().keys().any(|address| *address != owner) {
        return Err(MessageError::ForeignSigner);
    }

    let signature = output
        .signatures()
        .get(&owner)
        .copied()
        .ok_or(MessageError::MissingSignature)?;
    verify(owner, message.content(), signature)?;
    message.signatures.insert(owner, signature);
    Ok(())
}

pub(super) fn batch_modification(
    previous: &[SignableMessage],
    next: &[SignableMessage],
    owner: Address,
) -> Result<(), MessageError> {
    if previous.len() != next.len() {
        return Err(MessageError::Cardinality);
    }

    for (previous, replacement) in previous.iter().zip(next) {
        modification(previous, replacement, owner)?;
    }

    Ok(())
}

/// Changed bytes require fresh authorization; unchanged bytes must preserve
/// foreign signatures while allowing the modifier to replace its own signature.
fn modification(
    previous: &SignableMessage,
    next: &SignableMessage,
    owner: Address,
) -> Result<(), MessageError> {
    let changed = previous.content() != next.content();

    for (address, signature) in next.signatures() {
        modified_signature(previous, owner, *address, *signature, changed)?;
    }

    if !changed
        && previous.signatures().iter().any(|(address, signature)| {
            *address != owner && next.signatures().get(address) != Some(signature)
        })
    {
        return Err(MessageError::ForeignSigner);
    }

    if !next.signatures().contains_key(&owner) {
        return Err(MessageError::MissingSignature);
    }

    signatures(next)
}

fn modified_signature(
    previous: &SignableMessage,
    owner: Address,
    address: Address,
    signature: Signature,
    changed: bool,
) -> Result<(), MessageError> {
    let old = previous.signatures().get(&address);

    if changed && previous.signatures().values().any(|old| *old == signature) {
        return Err(MessageError::ChangedSnapshot);
    }

    if address != owner && (changed || old != Some(&signature)) {
        return Err(MessageError::ForeignSigner);
    }

    Ok(())
}
