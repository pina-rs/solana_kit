//! Preconfirmation wire decoding preserves exact transaction bytes and early-signal status.

use solana_kit_core::transaction::Transaction;

/// The only wire version understood by the pinned contract.
pub const WIRE_VERSION: u8 = 1;
/// Header bytes before the versioned transaction payload.
pub const HEADER_LENGTH: usize = 18;

/// The provider's unfiltered preconfirmation subscription on a dedicated binary namespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Subscribe {
    /// Maximum complete frame bytes accepted by the provider decoder.
    pub max_frame_bytes: usize,
}

impl serde::Serialize for Subscribe {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        serializer.serialize_seq(Some(0))?.end()
    }
}

impl solana_kit_client::methods::Method for Subscribe {
    type Response = Notification;
    const NAME: &'static str = "preconfSubscribe";

    fn decode(&self, _result: &str) -> Result<Self::Response, solana_kit_client::Error> {
        Err(solana_kit_client::Error::Envelope)
    }
}

impl solana_kit_client::subscriptions::ProviderMethod for Subscribe {
    const UNSUBSCRIBE: &'static str = "preconfUnsubscribe";
    const NOTIFICATION: &'static str = "preconfNotification";
    const NOTIFICATION_KIND: solana_kit_client::subscriptions::NotificationKind =
        solana_kit_client::subscriptions::NotificationKind::Binary("helius-preconf-v1");

    fn decode_binary(&self, bytes: &[u8]) -> Result<Self::Response, solana_kit_client::Error> {
        decode_frame(bytes, self.max_frame_bytes).map_err(|error| match error {
            Error::BodyLimit => solana_kit_client::Error::BodyLimit,
            other => solana_kit_client::Error::Schema(
                <serde_json::Error as serde::de::Error>::custom(other.to_string()),
            ),
        })
    }
}

/// Scheduler status, which does not guarantee transaction finality or landing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The scheduler reported failure.
    Failed,
    /// The scheduler reported success.
    Success,
    /// The scheduler reported an unrecognized status byte.
    Unknown,
}

/// A structurally decoded, untrusted early transaction notification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notification {
    /// Header wire version, currently one.
    pub version: u8,
    /// Exact target slot.
    pub slot: u64,
    /// Exact transaction position in the scheduled batch.
    pub transaction_index: u64,
    /// Scheduler status, not a confirmation result.
    pub status: Status,
    /// Kit-owned structurally validated transaction. Signatures are not authenticated here.
    pub transaction: Transaction,
    /// Original transaction payload, without the preconfirmation header.
    pub transaction_bytes: Vec<u8>,
}

/// Failure to decode a bounded preconfirmation frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Missing complete header and at least one transaction byte.
    TooShort,
    /// The frame exceeds the caller's receive budget.
    BodyLimit,
    /// The header version is unsupported.
    Version(u8),
    /// The transaction payload is structurally invalid.
    Transaction(solana_kit_core::transaction::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "invalid Helius preconfirmation frame: {self:?}")
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Transaction(error) => Some(error),
            _ => None,
        }
    }
}

/// Decodes one complete frame with a byte cap before copying its payload.
pub fn decode_frame(bytes: &[u8], max_frame_bytes: usize) -> Result<Notification, Error> {
    if bytes.len() > max_frame_bytes {
        return Err(Error::BodyLimit);
    }

    if bytes.len() <= HEADER_LENGTH {
        return Err(Error::TooShort);
    }

    if bytes[0] != WIRE_VERSION {
        return Err(Error::Version(bytes[0]));
    }

    let slot = read_u64(&bytes[1..9]);
    let transaction_index = read_u64(&bytes[9..17]);

    let status = match bytes[17] {
        0 => Status::Failed,
        1 => Status::Success,
        _ => Status::Unknown,
    };

    let transaction = Transaction::decode(&bytes[HEADER_LENGTH..]).map_err(Error::Transaction)?;

    Ok(Notification {
        version: bytes[0],
        slot,
        transaction_index,
        status,
        transaction,
        transaction_bytes: bytes[HEADER_LENGTH..].to_vec(),
    })
}

fn read_u64(bytes: &[u8]) -> u64 {
    let mut value = [0; 8];
    value.copy_from_slice(bytes);
    u64::from_le_bytes(value)
}
