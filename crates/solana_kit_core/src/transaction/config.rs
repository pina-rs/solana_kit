//! Validated inline version-one configuration; masks are derived, never supplied.

use alloc::vec::Vec;

use super::Error;

const MIN_HEAP: u32 = 32768;
const MAX_HEAP: u32 = 262144;
const HEAP_ALIGNMENT: u32 = 1024;
pub(super) const PRIORITY_MASK: u32 = 3;
pub(super) const COMPUTE_BIT: u32 = 4;
pub(super) const LOADED_BIT: u32 = 8;
pub(super) const HEAP_BIT: u32 = 16;
const KNOWN_MASK: u32 = 31;

/// An explicitly requested heap frame, in bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeapSize(u32);

impl HeapSize {
    /// Validates an inclusive 32..256 KiB request aligned to one KiB.
    ///
    /// # Errors
    /// Returns [`Error::InvalidConfig`] for an out-of-range or unaligned value.
    pub fn new(bytes: u32) -> Result<Self, Error> {
        if !(MIN_HEAP..=MAX_HEAP).contains(&bytes) || bytes % HEAP_ALIGNMENT != 0 {
            return Err(Error::InvalidConfig);
        }

        Ok(Self(bytes))
    }

    /// Requested byte count.
    #[must_use]
    pub const fn bytes(self) -> u32 {
        self.0
    }
}

/// Optional version-one resource requests, preserving absent versus explicit zero.
///
/// Priority fees are whole lamports. Absent compute/loaded limits mean zero;
/// absent heap means 32 KiB. These are wire values, not execution guarantees.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct V1Config {
    priority_fee: Option<u64>,
    compute_unit_limit: Option<u32>,
    loaded_accounts_data_limit: Option<u32>,
    heap_size: Option<HeapSize>,
}

impl V1Config {
    /// Sets the total priority fee in whole lamports, preserving all 64 bits.
    #[must_use]
    pub const fn with_priority_fee(mut self, lamports: u64) -> Self {
        self.priority_fee = Some(lamports);
        self
    }

    /// Sets the requested compute unit limit.
    #[must_use]
    pub const fn with_compute_unit_limit(mut self, limit: u32) -> Self {
        self.compute_unit_limit = Some(limit);
        self
    }

    /// Sets the requested loaded accounts data byte limit.
    #[must_use]
    pub const fn with_loaded_accounts_data_limit(mut self, limit: u32) -> Self {
        self.loaded_accounts_data_limit = Some(limit);
        self
    }

    /// Sets a previously validated heap frame request.
    #[must_use]
    pub const fn with_heap_size(mut self, heap: HeapSize) -> Self {
        self.heap_size = Some(heap);
        self
    }

    /// Explicit priority fee, or absence with effective zero.
    #[must_use]
    pub const fn priority_fee(self) -> Option<u64> {
        self.priority_fee
    }

    /// Explicit compute limit, or absence with effective zero.
    #[must_use]
    pub const fn compute_unit_limit(self) -> Option<u32> {
        self.compute_unit_limit
    }

    /// Explicit loaded data limit, or absence with effective zero.
    #[must_use]
    pub const fn loaded_accounts_data_limit(self) -> Option<u32> {
        self.loaded_accounts_data_limit
    }

    /// Explicit heap request, or absence with effective 32 KiB.
    #[must_use]
    pub const fn heap_size(self) -> Option<HeapSize> {
        self.heap_size
    }

    /// Canonical supported mask; the two priority bits are always atomic.
    #[must_use]
    pub fn mask(self) -> u32 {
        u32::from(self.priority_fee.is_some()) * PRIORITY_MASK
            | u32::from(self.compute_unit_limit.is_some()) * COMPUTE_BIT
            | u32::from(self.loaded_accounts_data_limit.is_some()) * LOADED_BIT
            | u32::from(self.heap_size.is_some()) * HEAP_BIT
    }

    pub(super) fn encode(self, bytes: &mut Vec<u8>) {
        if let Some(value) = self.priority_fee {
            bytes.extend(value.to_le_bytes());
        }

        for value in [
            self.compute_unit_limit,
            self.loaded_accounts_data_limit,
            self.heap_size.map(HeapSize::bytes),
        ]
        .into_iter()
        .flatten()
        {
            bytes.extend(value.to_le_bytes());
        }
    }

    pub(super) fn validate_mask(mask: u32) -> Result<(), Error> {
        if mask & !KNOWN_MASK != 0 || matches!(mask & PRIORITY_MASK, 1 | 2) {
            return Err(Error::InvalidConfig);
        }

        Ok(())
    }
}
