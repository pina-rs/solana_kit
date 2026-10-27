//! Bounded decoding of the classic little-endian Pyth price account.
use alloc::vec::Vec;
use solana_kit_core::{
    Address,
    codecs::{Decoder, Error, FixedBytes, Limits, Number, Reader},
};

use crate::{
    PythAccountType, PythEma, PythPriceComponent, PythPriceFlags, PythPriceInfo, PythPriceStatus,
    PythPriceType,
};

/// Magic shared by legacy Pyth accounts.
pub const PYTH_MAGIC: u32 = 0xa1b2_c3d4;
/// Supported classic price layout version.
pub const PYTH_ACCOUNT_VERSION: u32 = 2;
/// Inclusive number of slots for which a trading aggregate remains current.
pub const PYTH_MAX_SLOT_DIFFERENCE: u64 = 25;
/// Fixed prefix before the publisher components.
pub const PYTH_PRICE_ACCOUNT_HEADER_SIZE: usize = 240;
/// Publisher key followed by two price samples.
pub const PYTH_PRICE_COMPONENT_SIZE: usize = 96;

/// Decoded legacy account data. Parsing establishes no owner or price authenticity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PythPriceAccount<'a> {
    /// Validated account magic.
    pub magic: u32,
    /// Validated classic layout version.
    pub version: u32,
    /// Validated price account kind.
    pub account_type: PythAccountType,
    /// Recorded used size; retained without interpreting extension allocation.
    pub size: u32,
    /// Ordinary or unrecognized price kind.
    pub price_type: PythPriceType,
    /// Shared decimal exponent.
    pub exponent: i32,
    /// Recorded number of publisher components.
    pub num_component_prices: u32,
    /// Number of contributors to the aggregate.
    pub num_quoters: u32,
    /// Last valid aggregation slot.
    pub last_slot: u64,
    /// Valid slot of the aggregate.
    pub valid_slot: u64,
    /// Exact price EMA state.
    pub ema_price: PythEma,
    /// Exact confidence EMA state.
    pub ema_confidence: PythEma,
    /// Aggregate Unix timestamp in seconds.
    pub timestamp: i64,
    /// Minimum required publisher count.
    pub min_publishers: u8,
    /// Recorded message-sent byte.
    pub message_sent: u8,
    /// Recorded maximum latency byte.
    pub max_latency: u8,
    /// Known operational flags.
    pub flags: PythPriceFlags,
    /// Signed feed index.
    pub feed_index: i32,
    /// Product metadata account.
    pub product_account_key: Address,
    /// Next account in the linked list; an all-zero key means none.
    pub next_price_account_key: Option<Address>,
    /// Previous valid slot.
    pub previous_slot: u64,
    /// Previous signed price component.
    pub previous_price_component: i64,
    /// Previous unsigned confidence component.
    pub previous_confidence_component: u64,
    /// Previous Unix timestamp in seconds.
    pub previous_timestamp: i64,
    /// Recorded aggregate sample, without changing its stored status.
    pub aggregate: PythPriceInfo,
    /// Publisher components in their encoded order, bounded by `Limits::max_items`.
    pub price_components: Vec<PythPriceComponent>,
    /// Effective status after optional caller-supplied slot freshness checking.
    pub status: PythPriceStatus,
    /// Uninterpreted account tail, including unused publisher capacity.
    pub extension: &'a [u8],
}

/// Decodes a legacy price account with exact integers and bounded allocation.
/// `current_slot` demotes only a trading aggregate more than 25 slots old. A
/// future publication slot remains trading. No system clock is read.
///
/// # Errors
/// Rejects bad magic, version or account kind, truncated fields/components,
/// overflowing sizes, exceeded limits or allocation failure. All component
/// bytes are required before reserving memory for the recorded count.
pub fn decode_pyth_price_account(
    bytes: &[u8],
    current_slot: Option<u64>,
    limits: Limits,
) -> Result<PythPriceAccount<'_>, Error> {
    let mut reader = Reader::new(bytes, limits)?;
    let magic = Number::<u32>::LE.read(&mut reader)?;
    let version = Number::<u32>::LE.read(&mut reader)?;
    let account_type = Number::<u32>::LE.read(&mut reader)?;

    if magic != PYTH_MAGIC || version != PYTH_ACCOUNT_VERSION || account_type != 3 {
        return Err(Error::InvalidDiscriminator);
    }

    let mut account = read_header(&mut reader)?;
    account.price_components = read_components(&mut reader, account.num_component_prices)?;
    account.extension = reader.remaining();
    account.status = account.aggregate.status;

    if account.status == PythPriceStatus::Trading
        && current_slot.is_some_and(|slot| {
            slot.saturating_sub(account.aggregate.publish_slot) > PYTH_MAX_SLOT_DIFFERENCE
        })
    {
        account.status = PythPriceStatus::Unknown;
    }

    Ok(account)
}

fn read_header<'a>(reader: &mut Reader<'a>) -> Result<PythPriceAccount<'a>, Error> {
    Ok(PythPriceAccount {
        magic: PYTH_MAGIC,
        version: PYTH_ACCOUNT_VERSION,
        account_type: PythAccountType::Price,
        size: Number::<u32>::LE.read(reader)?,
        price_type: PythPriceType::from_value(Number::<u32>::LE.read(reader)?),
        exponent: Number::<i32>::LE.read(reader)?,
        num_component_prices: Number::<u32>::LE.read(reader)?,
        num_quoters: Number::<u32>::LE.read(reader)?,
        last_slot: Number::<u64>::LE.read(reader)?,
        valid_slot: Number::<u64>::LE.read(reader)?,
        ema_price: PythEma::read(reader)?,
        ema_confidence: PythEma::read(reader)?,
        timestamp: Number::<i64>::LE.read(reader)?,
        min_publishers: Number::<u8>::LE.read(reader)?,
        message_sent: Number::<u8>::LE.read(reader)?,
        max_latency: Number::<u8>::LE.read(reader)?,
        flags: PythPriceFlags::from_bits(Number::<u8>::LE.read(reader)?),
        feed_index: Number::<i32>::LE.read(reader)?,
        product_account_key: Address::from_bytes(FixedBytes::<32>.read(reader)?),
        next_price_account_key: optional_address(FixedBytes::<32>.read(reader)?),
        previous_slot: Number::<u64>::LE.read(reader)?,
        previous_price_component: Number::<i64>::LE.read(reader)?,
        previous_confidence_component: Number::<u64>::LE.read(reader)?,
        previous_timestamp: Number::<i64>::LE.read(reader)?,
        aggregate: PythPriceInfo::read(reader)?,
        price_components: Vec::new(),
        status: PythPriceStatus::Unknown,
        extension: &[],
    })
}

fn read_components(reader: &mut Reader<'_>, count: u32) -> Result<Vec<PythPriceComponent>, Error> {
    let count = usize::try_from(count).map_err(|_| Error::Overflow)?;
    let size = count
        .checked_mul(PYTH_PRICE_COMPONENT_SIZE)
        .ok_or(Error::Overflow)?;

    if size > reader.remaining().len() {
        return Err(Error::Truncated);
    }

    reader.claim(count)?;
    let mut components = Vec::new();
    components
        .try_reserve_exact(count)
        .map_err(|_| Error::AllocationFailed)?;

    for _ in 0..count {
        components.push(PythPriceComponent::read(reader)?);
    }

    Ok(components)
}

fn optional_address(bytes: [u8; 32]) -> Option<Address> {
    (bytes != [0; 32]).then(|| Address::from_bytes(bytes))
}
