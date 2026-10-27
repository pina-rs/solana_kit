//! Exact integer fields shared by legacy Pyth price accounts.
use solana_kit_core::{
    Address,
    codecs::{Decoder, Error, FixedBytes, Limits, Number, Reader},
};

/// Numeric kind of a legacy Pyth account.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum PythAccountType {
    /// Uninitialized or unrecognized kind.
    Unknown = 0,
    /// Mapping from products to accounts.
    Mapping = 1,
    /// Product metadata.
    Product = 2,
    /// Price aggregation.
    Price = 3,
    /// Test account.
    Test = 4,
    /// Permissions account.
    Permission = 5,
}

impl PythAccountType {
    /// Maps unrecognized numeric kinds to `Unknown`, matching the reference.
    #[must_use]
    pub const fn from_value(value: u32) -> Self {
        match value {
            1 => Self::Mapping,
            2 => Self::Product,
            3 => Self::Price,
            4 => Self::Test,
            5 => Self::Permission,
            _ => Self::Unknown,
        }
    }
}

/// Recorded state of a legacy price aggregation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum PythPriceStatus {
    /// Unavailable or unrecognized status.
    Unknown = 0,
    /// An actively published price.
    Trading = 1,
    /// Publication has halted.
    Halted = 2,
    /// An aggregation auction is in progress.
    Auction = 3,
    /// Excluded from aggregation.
    Ignored = 4,
}

impl PythPriceStatus {
    /// Maps unrecognized numeric statuses to `Unknown`.
    #[must_use]
    pub const fn from_value(value: u32) -> Self {
        match value {
            1 => Self::Trading,
            2 => Self::Halted,
            3 => Self::Auction,
            4 => Self::Ignored,
            _ => Self::Unknown,
        }
    }
}

/// Whether a legacy account holds an ordinary price.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum PythPriceType {
    /// Uninitialized or unrecognized kind.
    Unknown = 0,
    /// Ordinary price data.
    Price = 1,
}

impl PythPriceType {
    /// Recognizes the ordinary price kind and maps other values to `Unknown`.
    #[must_use]
    pub const fn from_value(value: u32) -> Self {
        match value {
            1 => Self::Price,
            _ => Self::Unknown,
        }
    }
}

/// Currently specified corporate action marker.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum PythCorpAction {
    /// No corporate action. Decoded samples retain all raw marker values.
    NoCorpAct = 0,
}

/// One exact legacy price sample; no floating-point scaling is applied.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PythPriceInfo {
    /// Signed price component, scaled by the parent account's exponent.
    pub price_component: i64,
    /// Unsigned confidence component with the same scale.
    pub confidence_component: u64,
    /// Recorded aggregation status.
    pub status: PythPriceStatus,
    /// Raw corporate action marker, including future values.
    pub corporate_action: u32,
    /// Slot of publication, preserved across the full unsigned range.
    pub publish_slot: u64,
}

impl PythPriceInfo {
    /// Decodes one 32-byte sample at a checked byte offset.
    ///
    /// # Errors
    /// Rejects an exceeded byte limit, invalid offset or truncated sample.
    pub fn from_bytes(bytes: &[u8], offset: usize, limits: Limits) -> Result<Self, Error> {
        let mut reader = Reader::new(bytes, limits)?;
        reader.seek(offset)?;
        Self::read(&mut reader)
    }

    pub(crate) fn read(reader: &mut Reader<'_>) -> Result<Self, Error> {
        Ok(Self {
            price_component: Number::<i64>::LE.read(reader)?,
            confidence_component: Number::<u64>::LE.read(reader)?,
            status: PythPriceStatus::from_value(Number::<u32>::LE.read(reader)?),
            corporate_action: Number::<u32>::LE.read(reader)?,
            publish_slot: Number::<u64>::LE.read(reader)?,
        })
    }
}

/// Exact rational state of an exponential moving average.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PythEma {
    /// Current signed EMA component.
    pub value_component: i64,
    /// Signed update numerator.
    pub numerator: i64,
    /// Signed update denominator; parsing does not divide or normalize it.
    pub denominator: i64,
}

impl PythEma {
    /// Decodes one 24-byte rational state at a checked byte offset.
    ///
    /// # Errors
    /// Rejects an exceeded byte limit, invalid offset or truncated state.
    pub fn from_bytes(bytes: &[u8], offset: usize, limits: Limits) -> Result<Self, Error> {
        let mut reader = Reader::new(bytes, limits)?;
        reader.seek(offset)?;
        Self::read(&mut reader)
    }

    pub(crate) fn read(reader: &mut Reader<'_>) -> Result<Self, Error> {
        Ok(Self {
            value_component: Number::<i64>::LE.read(reader)?,
            numerator: Number::<i64>::LE.read(reader)?,
            denominator: Number::<i64>::LE.read(reader)?,
        })
    }
}

/// One publisher and its aggregate and latest samples, in encoded order.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PythPriceComponent {
    /// Publisher account address.
    pub publisher: Address,
    /// Publisher contribution to the aggregate.
    pub aggregate: PythPriceInfo,
    /// Latest publisher sample.
    pub latest: PythPriceInfo,
}

impl PythPriceComponent {
    pub(crate) fn read(reader: &mut Reader<'_>) -> Result<Self, Error> {
        Ok(Self {
            publisher: Address::from_bytes(FixedBytes::<32>.read(reader)?),
            aggregate: PythPriceInfo::read(reader)?,
            latest: PythPriceInfo::read(reader)?,
        })
    }
}

/// Known operational bits of the legacy account's flag byte.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PythPriceFlags {
    /// The account uses accumulator version two.
    pub accumulator_v2: bool,
    /// Its message buffer has been cleared.
    pub message_buffer_cleared: bool,
}

impl PythPriceFlags {
    /// Reads the two known bits without assigning meaning to future bits.
    #[must_use]
    pub const fn from_bits(value: u8) -> Self {
        Self {
            accumulator_v2: value & 1 != 0,
            message_buffer_cleared: value & 2 != 0,
        }
    }
}
