//! Off-chain receiver instruction construction; no proof, owner or price authentication.
mod accounts;
mod wire;
use crate::{
    DEFAULT_TREASURY_ID, MerklePriceUpdate, POST_UPDATE_ATOMIC_DISCRIMINATOR,
    POST_UPDATE_DISCRIMINATOR,
};
pub use accounts::ReceiverAccounts;
use alloc::vec::Vec;
use solana_kit_core::{
    Address, Instruction,
    codecs::{Error, Limits},
};
use wire::Wire;

/// Parameters after the VAA has already been verified in a Wormhole account.
#[derive(Clone, Copy, Debug)]
pub struct PostUpdateParams<'a> {
    /// Borrowed message and fixed-width proof hashes.
    pub merkle_price_update: &'a MerklePriceUpdate<'a>,
    /// Treasury id, encoded as one byte.
    pub treasury_id: u8,
}

impl<'a> PostUpdateParams<'a> {
    /// Selects the default treasury without copying message or proof data.
    pub const fn new(merkle_price_update: &'a MerklePriceUpdate<'a>) -> Self {
        Self {
            merkle_price_update,
            treasury_id: DEFAULT_TREASURY_ID,
        }
    }

    /// Encodes exact Borsh parameters without the Anchor discriminator.
    ///
    /// # Errors
    /// Rejects exceeded byte/proof budgets, unrepresentable lengths or allocation failure.
    pub fn encode(self, limits: Limits) -> Result<Vec<u8>, Error> {
        self.wire(None).encode(limits, 0)
    }

    fn wire(self, discriminator: Option<[u8; 8]>) -> Wire<'a> {
        Wire {
            vaa: None,
            update: self.merkle_price_update,
            treasury_id: self.treasury_id,
            discriminator,
        }
    }
}

/// Parameters including the opaque VAA that the receiver will verify.
#[derive(Clone, Copy, Debug)]
pub struct PostUpdateAtomicParams<'a> {
    /// Borrowed VAA bytes; construction does not verify or trim signatures.
    pub vaa: &'a [u8],
    /// Borrowed message and fixed-width proof hashes.
    pub merkle_price_update: &'a MerklePriceUpdate<'a>,
    /// Treasury id, encoded as one byte.
    pub treasury_id: u8,
}

impl<'a> PostUpdateAtomicParams<'a> {
    /// Selects the default treasury and retains caller-owned encoded inputs.
    pub const fn new(vaa: &'a [u8], merkle_price_update: &'a MerklePriceUpdate<'a>) -> Self {
        Self {
            vaa,
            merkle_price_update,
            treasury_id: DEFAULT_TREASURY_ID,
        }
    }

    /// Encodes exact Borsh parameters without the Anchor discriminator.
    ///
    /// # Errors
    /// Rejects exceeded byte/proof budgets, unrepresentable lengths or allocation failure.
    pub fn encode(self, limits: Limits) -> Result<Vec<u8>, Error> {
        self.wire(None).encode(limits, 0)
    }

    fn wire(self, discriminator: Option<[u8; 8]>) -> Wire<'a> {
        Wire {
            vaa: Some(self.vaa),
            update: self.merkle_price_update,
            treasury_id: self.treasury_id,
            discriminator,
        }
    }
}

/// Posting variant fixes the meaning of its verification account and discriminator.
#[derive(Clone, Copy, Debug)]
pub enum ReceiverInstruction<'a> {
    /// Consumes a previously verified Wormhole encoded-VAA account.
    PostUpdate {
        /// Shared receiver account context.
        accounts: ReceiverAccounts,
        /// Previously verified encoded-VAA account, readonly.
        encoded_vaa: Address,
        /// Message, proof and treasury id.
        params: PostUpdateParams<'a>,
    },
    /// Sends a VAA for atomic guardian verification by the receiver.
    PostUpdateAtomic {
        /// Shared receiver account context.
        accounts: ReceiverAccounts,
        /// Selected Wormhole guardian-set account, readonly.
        guardian_set: Address,
        /// Opaque VAA, message, proof and treasury id.
        params: PostUpdateAtomicParams<'a>,
    },
}

impl ReceiverInstruction<'_> {
    /// Encodes the discriminator, parameters and all seven ordered account roles.
    /// `max_items` counts proof hashes plus the seven allocated account entries.
    ///
    /// # Errors
    /// Rejects byte/item limits, length overflow or fallible allocation failures.
    pub fn instruction(self, limits: Limits) -> Result<Instruction, Error> {
        let (accounts, verification, wire) = match self {
            Self::PostUpdate {
                accounts,
                encoded_vaa,
                params,
            } => (
                accounts,
                encoded_vaa,
                params.wire(Some(POST_UPDATE_DISCRIMINATOR)),
            ),
            Self::PostUpdateAtomic {
                accounts,
                guardian_set,
                params,
            } => (
                accounts,
                guardian_set,
                params.wire(Some(POST_UPDATE_ATOMIC_DISCRIMINATOR)),
            ),
        };
        let metadata = accounts.metadata(verification);
        let data = wire.encode(limits, metadata.len())?;
        let mut accounts_out = Vec::new();
        accounts_out
            .try_reserve_exact(metadata.len())
            .map_err(|_| Error::AllocationFailed)?;
        accounts_out.extend_from_slice(&metadata);

        Ok(Instruction {
            program_address: accounts.program_address,
            accounts: accounts_out,
            data,
        })
    }
}
