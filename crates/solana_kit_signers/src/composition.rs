//! Shared pure role resolution for explicit local and native adapters.
use crate::Error;
use alloc::vec::Vec;
use solana_kit_core::Address;

pub(crate) const MAX_ITEMS: usize = 256;

/// Automatically selected role for a deduplicated logical capability.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Role {
    /// Authorizes the final immutable snapshot alongside other partial signers.
    Partial,
    /// May replace the snapshot before partial authorization.
    Modifying,
    /// Submits once after the other selected roles have completed.
    Sending,
    /// No declared interface participates in this operation.
    Ignored,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) struct Interfaces {
    pub partial: bool,
    pub modifying: bool,
    pub sending: bool,
}

#[derive(Clone, Copy)]
pub(crate) struct Descriptor {
    pub address: Address,
    pub identity: usize,
    pub interfaces: Interfaces,
}

/// Validate borrowed logical identities before allocating backend operations.
pub(crate) fn validate_registrations(inputs: &[Descriptor]) -> Result<Vec<usize>, Error> {
    if inputs.len() > MAX_ITEMS {
        return Err(Error::TooManySigners);
    }

    let mut unique: Vec<usize> = Vec::new();

    for (index, item) in inputs.iter().enumerate() {
        let existing = unique
            .iter()
            .map(|index| &inputs[*index])
            .find(|existing| existing.address == item.address);

        match existing {
            Some(existing) => validate_identity(existing, item)?,
            None => unique.push(index),
        }
    }

    Ok(unique)
}

fn validate_identity(existing: &Descriptor, item: &Descriptor) -> Result<(), Error> {
    if existing.identity != item.identity || existing.interfaces != item.interfaces {
        return Err(Error::ConflictingSigner);
    }

    Ok(())
}

/// Resolve the source algorithm once, before any asynchronous backend operation.
pub(crate) fn select_roles(
    inputs: &[Descriptor],
    sending: bool,
) -> Result<Vec<(Address, Role)>, Error> {
    let sender = resolve_sender(inputs, sending)?;
    let modifiers = select_modifiers(inputs, sender);
    let roles = inputs
        .iter()
        .enumerate()
        .map(|(index, item)| {
            (
                item.address,
                resolve_role(index, item.interfaces, sender, modifiers),
            )
        })
        .collect();

    Ok(roles)
}

#[derive(Clone, Copy)]
enum Modifiers {
    AllNonPartial,
    First(usize),
    None,
}

fn select_modifiers(inputs: &[Descriptor], sender: Option<usize>) -> Modifiers {
    let candidates: Vec<_> = inputs
        .iter()
        .enumerate()
        .filter(|(index, item)| Some(*index) != sender && item.interfaces.modifying)
        .collect();

    if candidates.iter().any(|(_, item)| !item.interfaces.partial) {
        return Modifiers::AllNonPartial;
    }

    candidates
        .first()
        .map_or(Modifiers::None, |(index, _)| Modifiers::First(*index))
}

fn resolve_role(
    index: usize,
    interfaces: Interfaces,
    sender: Option<usize>,
    modifiers: Modifiers,
) -> Role {
    if Some(index) == sender {
        return Role::Sending;
    }

    let modifying = match modifiers {
        Modifiers::AllNonPartial => interfaces.modifying && !interfaces.partial,
        Modifiers::First(selected) => index == selected,
        Modifiers::None => false,
    };

    if modifying {
        return Role::Modifying;
    }

    if interfaces.partial {
        return Role::Partial;
    }

    Role::Ignored
}

fn resolve_sender(inputs: &[Descriptor], sending: bool) -> Result<Option<usize>, Error> {
    if !sending {
        return Ok(None);
    }

    let senders: Vec<_> = inputs
        .iter()
        .enumerate()
        .filter(|(_, item)| item.interfaces.sending)
        .collect();
    let only: Vec<_> = senders
        .iter()
        .filter(|(_, item)| !item.interfaces.partial && !item.interfaces.modifying)
        .collect();

    if only.len() > 1 {
        return Err(Error::MultipleSenders);
    }

    only.first()
        .map(|item| item.0)
        .or_else(|| senders.first().map(|item| item.0))
        .map(Some)
        .ok_or(Error::MissingSender)
}
