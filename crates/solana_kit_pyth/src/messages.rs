//! Current Pyth verification and price-message prefixes, in network byte order.
use alloc::string::String;
use solana_kit_core::codecs::{Base16, Decoder, Endian, Error, FixedBytes, Limits, Number, Reader};

/// Magic of the Wormhole verification message, distinct from its VAA envelope.
pub const WORMHOLE_VERIFICATION_MAGIC: [u8; 4] = *b"AUWV";
/// Length of the current verification prefix, including magic and variant.
pub const WORMHOLE_VERIFICATION_SIZE: usize = 37;
/// Length of the current price feed prefix, including its variant.
pub const PRICE_FEED_MESSAGE_SIZE: usize = 85;

/// Merkle commitment carried by a Pyth verification message.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WormholeMerkleRoot {
    /// Pythnet slot of the committed update.
    pub slot: u64,
    /// Size of the source accumulator ring.
    pub ring_size: u32,
    /// Keccak-160 root, without local proof verification.
    pub root: [u8; 20],
}

/// Official `AUWV` message and its original bytes, including future tail fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PythWormholeMessage<'a> {
    /// Current variant-zero Merkle commitment.
    pub merkle_root: WormholeMerkleRoot,
    /// Entire caller-owned message.
    pub raw_payload: &'a [u8],
}

/// Parses the current verification prefix and retains all trailing extensions.
/// The pinned Dart helper's synthetic version/kind envelope is rejected.
///
/// # Errors
/// Rejects incorrect magic, an unknown payload variant, truncation or an
/// exceeded byte limit. This function performs no allocation or authentication.
pub fn parse_pyth_wormhole_message(
    payload: &[u8],
    limits: Limits,
) -> Result<PythWormholeMessage<'_>, Error> {
    let mut reader = Reader::new(payload, limits)?;

    if reader.take(WORMHOLE_VERIFICATION_MAGIC.len())? != WORMHOLE_VERIFICATION_MAGIC {
        return Err(Error::InvalidConstant {
            offset: 0,
            length: WORMHOLE_VERIFICATION_MAGIC.len(),
        });
    }

    if Number::<u8>::BE.read(&mut reader)? != 0 {
        return Err(Error::InvalidDiscriminator);
    }

    Ok(PythWormholeMessage {
        merkle_root: WormholeMerkleRoot {
            slot: Number::<u64>::BE.read(&mut reader)?,
            ring_size: Number::<u32>::BE.read(&mut reader)?,
            root: FixedBytes::<20>.read(&mut reader)?,
        },
        raw_payload: payload,
    })
}

/// Current variant-zero price message. Monetary components are exact integers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PythPriceFeedMessage {
    /// Exact price feed identifier; this is not a Solana account address.
    pub feed_id: [u8; 32],
    /// Price component, scaled by `10^exponent`.
    pub price: i64,
    /// Confidence component, scaled by the same exponent.
    pub confidence: u64,
    /// Decimal exponent shared by price, confidence and EMA components.
    pub exponent: i32,
    /// Unix publication time in seconds.
    pub publish_time: i64,
    /// Previous publication time in seconds.
    pub prev_publish_time: i64,
    /// Exponential moving average price component.
    pub ema_price: i64,
    /// Exponential moving average confidence component.
    pub ema_conf: u64,
}

impl PythPriceFeedMessage {
    /// Returns the feed identifier as 64 lowercase hexadecimal digits.
    ///
    /// # Errors
    /// Returns a codec allocation error when the output cannot be allocated.
    pub fn feed_id_hex(&self) -> Result<String, Error> {
        Base16.decode(
            &self.feed_id,
            Limits::new(self.feed_id.len(), self.feed_id.len() * 2),
        )
    }
}

/// Parses the known price prefix. Future trailing fields remain caller-owned.
///
/// # Errors
/// Rejects an unknown variant, truncation or an exceeded byte limit. No
/// allocation, floating-point conversion or proof verification is performed.
pub fn parse_pyth_price_feed_message(
    bytes: &[u8],
    limits: Limits,
) -> Result<PythPriceFeedMessage, Error> {
    let mut reader = Reader::new(bytes, limits)?;

    if Number::<u8>::BE.read(&mut reader)? != 0 {
        return Err(Error::InvalidDiscriminator);
    }

    read_price_message(&mut reader, Endian::Big)
}

pub(crate) fn read_price_message(
    reader: &mut Reader<'_>,
    endian: Endian,
) -> Result<PythPriceFeedMessage, Error> {
    Ok(PythPriceFeedMessage {
        feed_id: FixedBytes::<32>.read(reader)?,
        price: Number::<i64>::new(endian).read(reader)?,
        confidence: Number::<u64>::new(endian).read(reader)?,
        exponent: Number::<i32>::new(endian).read(reader)?,
        publish_time: Number::<i64>::new(endian).read(reader)?,
        prev_publish_time: Number::<i64>::new(endian).read(reader)?,
        ema_price: Number::<i64>::new(endian).read(reader)?,
        ema_conf: Number::<u64>::new(endian).read(reader)?,
    })
}
