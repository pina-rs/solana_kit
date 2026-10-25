//! Lookup table metadata and the fixed on-chain address-table offset.
use alloc::vec::Vec;
use solana_kit_core::{
    Address,
    codecs::{Decoder, Encoder, Error, Limits, Reader, Writer},
};

/// Metadata occupies this many bytes even when authority is absent.
pub const META_SIZE: usize = 56;
/// On-chain tables have at most 256 address entries.
pub const MAX_ADDRESSES: usize = 256;
const ADDRESS_BYTES: usize = 32;

/// Lookup metadata and addresses in exact table index order.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AddressLookupTable {
    /// Last slot at which the table may remain active; u64::MAX means active.
    pub deactivation_slot: u64,
    /// Slot in which the table was last extended.
    pub last_extended_slot: u64,
    /// First address index appended during that slot.
    pub last_extended_slot_start_index: u8,
    /// None for an immutable frozen table.
    pub authority: Option<Address>,
    /// Reserved alignment field, retained rather than silently discarded.
    pub padding: u16,
    /// Addresses in lookup index order.
    pub addresses: Vec<Address>,
}

impl core::fmt::Display for AddressLookupTable {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}

/// Strict initialized-state discriminator and fixed-header account codec.
#[derive(Clone, Copy, Debug)]
pub struct AddressLookupTableCodec;

impl Encoder<AddressLookupTable> for AddressLookupTableCodec {
    fn encoded_len(&self, value: &AddressLookupTable) -> Result<usize, Error> {
        if value.addresses.len() > MAX_ADDRESSES {
            return Err(Error::LimitExceeded);
        }

        value
            .addresses
            .len()
            .checked_mul(ADDRESS_BYTES)
            .and_then(|size| size.checked_add(META_SIZE))
            .ok_or(Error::Overflow)
    }

    fn write(&self, value: &AddressLookupTable, writer: &mut Writer<'_>) -> Result<(), Error> {
        let size = self.encoded_len(value)?;
        let mut addresses = Vec::new();
        addresses
            .try_reserve_exact(value.addresses.len())
            .map_err(|_| Error::AllocationFailed)?;
        addresses.extend_from_slice(&value.addresses);
        let raw = crate::generated::AddressLookupTable {
            deactivation_slot: value.deactivation_slot,
            last_extended_slot: value.last_extended_slot,
            last_extended_slot_start_index: value.last_extended_slot_start_index,
            authority: value.authority,
            padding: value.padding,
            addresses,
        };
        let mut bytes = crate::generated::AddressLookupTableCodec.encode(&raw, size)?;
        // The pinned Rust state packs absent-authority padding before its reserved bytes.
        if value.authority.is_none() {
            bytes[22..24].copy_from_slice(&value.padding.to_le_bytes());

            bytes[24..META_SIZE].fill(0);
        }

        writer.put(&bytes)
    }
}
impl Decoder<AddressLookupTable> for AddressLookupTableCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AddressLookupTable, Error> {
        let length = reader.remaining().len();

        if length < META_SIZE {
            return Err(Error::Truncated);
        }

        if (length - META_SIZE) / ADDRESS_BYTES > MAX_ADDRESSES {
            return Err(Error::LimitExceeded);
        }

        let count = (length - META_SIZE) / ADDRESS_BYTES;
        reader.claim(count)?;
        let mut bytes = Vec::new();

        bytes
            .try_reserve_exact(length)
            .map_err(|_| Error::AllocationFailed)?;
        bytes.extend_from_slice(reader.take(length)?);
        // Normalize only the documented Rust/Dart absent-authority padding difference.
        if bytes[21] == 0 {
            let padding = [bytes[22], bytes[23]];
            bytes[54..META_SIZE].copy_from_slice(&padding);
        }

        let raw =
            crate::generated::AddressLookupTableCodec.decode(&bytes, Limits::new(length, count))?;
        Ok(AddressLookupTable {
            deactivation_slot: raw.deactivation_slot,
            last_extended_slot: raw.last_extended_slot,
            last_extended_slot_start_index: raw.last_extended_slot_start_index,
            authority: raw.authority,
            padding: raw.padding,
            addresses: raw.addresses,
        })
    }
}

impl AddressLookupTable {
    /// Selects addresses usable at the supplied bank slot and recent-slot history.
    ///
    /// Newly appended entries become usable after their extension slot. A table
    /// remains usable during deactivation while its slot is current or retained
    /// in the bank's SlotHashes history. The caller owns that bank snapshot.
    /// # Errors
    /// Rejects history above the canonical 512-entry bound, invalid start indexes
    /// and tables whose deactivation slot has left the supplied history.
    pub fn active_addresses(
        &self,
        current_slot: u64,
        recent_slots: &[u64],
    ) -> Result<&[Address], Error> {
        if recent_slots.len() > 512 || self.addresses.len() > MAX_ADDRESSES {
            return Err(Error::LimitExceeded);
        }

        if self.deactivation_slot != u64::MAX
            && self.deactivation_slot != current_slot
            && !recent_slots.contains(&self.deactivation_slot)
        {
            return Err(Error::InvalidValue);
        }

        if current_slot > self.last_extended_slot {
            return Ok(&self.addresses);
        }

        self.addresses
            .get(..usize::from(self.last_extended_slot_start_index))
            .ok_or(Error::InvalidValue)
    }

    /// Consumes decoded state into the existing core transaction lookup snapshot.
    /// Activation, bank-slot and deactivation validation remain caller-owned.
    #[must_use]
    pub fn into_lookup_table(self, address: Address) -> solana_kit_core::transaction::LookupTable {
        solana_kit_core::transaction::LookupTable {
            address,
            addresses: self.addresses,
        }
    }
}

/// Decodes an already fetched table while retaining the complete account metadata.
///
/// Transport and commitment selection remain with the caller's existing client.
/// # Errors
/// Preserves owner, allocation and codec errors from the core account decoder.
pub fn decode_account(
    account: solana_kit_core::accounts::EncodedAccount,
    limits: Limits,
) -> Result<
    solana_kit_core::accounts::Account<AddressLookupTable>,
    solana_kit_core::accounts::AccountError,
> {
    account.decode(&AddressLookupTableCodec, crate::PROGRAM, limits)
}
