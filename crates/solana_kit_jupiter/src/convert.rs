//! Explicit conversions validate raw fields and reuse Kit-owned values and codecs.

use crate::{
    AccountMetaPayload, BlockhashWithMetadata, Error, InstructionPayload, LookupTablePayload,
};
use solana_kit_core::{
    AccountMeta, AccountRole, Hash, Instruction,
    codecs::{Base64, Encoder},
    transaction::{Lifetime, LookupTable, Transaction},
};

const MAX_LOOKUP_ADDRESSES: usize = 256;

/// Decodes a base64 wire transaction using Kit's strict transaction decoder.
///
/// Preserves message bytes and signature slots without authenticating them or
/// inferring expiry metadata. Kit's Base64 codec retains the pinned tolerant
/// padding rules. The decoded allocation is bounded by Kit's largest envelope.
///
/// # Errors
/// Rejects invalid base64, oversized envelopes and malformed transaction wire data.
pub fn decode_base64_swap_transaction(encoded: &str) -> Result<Transaction, Error> {
    let size = Base64.encoded_len(encoded).map_err(Error::Codec)?;

    if size > solana_kit_core::transaction::V1_PACKET_SIZE {
        return Err(Error::Transaction(
            solana_kit_core::transaction::Error::PacketTooLarge,
        ));
    }

    let bytes = Base64.encode(encoded, size).map_err(Error::Codec)?;
    Transaction::decode(&bytes).map_err(Error::Transaction)
}

impl AccountMetaPayload {
    /// Validates an address and both explicitly supplied privilege flags.
    ///
    /// # Errors
    /// Rejects missing fields and noncanonical Kit addresses.
    pub fn to_account_meta(&self) -> Result<AccountMeta, Error> {
        let address = self
            .pubkey
            .as_deref()
            .ok_or(Error::Payload("missing pubkey"))?
            .parse()
            .map_err(|_| Error::Payload("invalid pubkey"))?;
        let signer = self.is_signer.ok_or(Error::Payload("missing isSigner"))?;
        let writable = self
            .is_writable
            .ok_or(Error::Payload("missing isWritable"))?;
        Ok(AccountMeta {
            address,
            role: AccountRole::from_privileges(signer, writable),
        })
    }
}

impl InstructionPayload {
    /// Converts a complete payload without sorting or deduplicating accounts.
    ///
    /// # Errors
    /// Rejects missing fields, malformed addresses, incomplete privileges and invalid base64.
    pub fn to_instruction(&self) -> Result<Instruction, Error> {
        let program_address = self
            .program_id
            .as_deref()
            .ok_or(Error::Payload("missing programId"))?
            .parse()
            .map_err(|_| Error::Payload("invalid programId"))?;
        let payloads = self
            .accounts
            .as_ref()
            .ok_or(Error::Payload("missing accounts"))?;
        let accounts = payloads
            .iter()
            .map(AccountMetaPayload::to_account_meta)
            .collect::<Result<_, _>>()?;
        let encoded = self.data.as_deref().ok_or(Error::Payload("missing data"))?;
        let data = Base64
            .encode(encoded, encoded.len())
            .map_err(Error::Codec)?;
        Ok(Instruction {
            program_address,
            accounts,
            data,
        })
    }
}

impl LookupTablePayload {
    /// Converts one resolved table without changing its on-chain address indices.
    ///
    /// # Errors
    /// Rejects malformed addresses and tables exceeding the u8 index domain.
    pub fn to_lookup_table(&self) -> Result<LookupTable, Error> {
        if self.addresses.len() > MAX_LOOKUP_ADDRESSES {
            return Err(Error::Payload("too many lookup addresses"));
        }

        let address = self
            .address
            .parse()
            .map_err(|_| Error::Payload("invalid lookup table address"))?;
        let addresses = self
            .addresses
            .iter()
            .map(|text| {
                text.parse()
                    .map_err(|_| Error::Payload("invalid lookup address"))
            })
            .collect::<Result<_, _>>()?;
        Ok(LookupTable { address, addresses })
    }
}

impl BlockhashWithMetadata {
    /// Converts complete metadata to Kit's locally supplied blockhash lifetime.
    ///
    /// The caller is responsible for establishing trust in this provider metadata.
    ///
    /// # Errors
    /// Rejects non-32-byte hashes and expiry heights outside Kit's unsigned u64 domain.
    pub fn to_lifetime(&self) -> Result<Lifetime, Error> {
        let bytes: [u8; 32] = self
            .blockhash
            .as_slice()
            .try_into()
            .map_err(|_| Error::Payload("invalid blockhash length"))?;
        let last_valid_block_height = self
            .last_valid_block_height
            .as_str()
            .parse()
            .map_err(|_| Error::Payload("invalid expiry height"))?;
        Ok(Lifetime::Blockhash {
            blockhash: Hash::from_bytes(bytes),
            last_valid_block_height,
        })
    }
}
