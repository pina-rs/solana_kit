// Generated from pinned System errors; see SYSTEM-LICENSE.
const CODE_ACCOUNT_ALREADY_IN_USE: u32 = 0;
const CODE_RESULT_WITH_NEGATIVE_LAMPORTS: u32 = 1;
const CODE_INVALID_PROGRAM_ID: u32 = 2;
const CODE_INVALID_ACCOUNT_DATA_LENGTH: u32 = 3;
const CODE_MAX_SEED_LENGTH_EXCEEDED: u32 = 4;
const CODE_ADDRESS_WITH_SEED_MISMATCH: u32 = 5;
const CODE_NONCE_NO_RECENT_BLOCKHASHES: u32 = 6;
const CODE_NONCE_BLOCKHASH_NOT_EXPIRED: u32 = 7;
const CODE_NONCE_UNEXPECTED_BLOCKHASH_VALUE: u32 = 8;

/// Known System custom errors with unknown numbers preserved losslessly.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SystemError {
    /// Pinned System error 0.
    AccountAlreadyInUse,
    /// Pinned System error 1.
    ResultWithNegativeLamports,
    /// Pinned System error 2.
    InvalidProgramId,
    /// Pinned System error 3.
    InvalidAccountDataLength,
    /// Pinned System error 4.
    MaxSeedLengthExceeded,
    /// Pinned System error 5.
    AddressWithSeedMismatch,
    /// Pinned System error 6.
    NonceNoRecentBlockhashes,
    /// Pinned System error 7.
    NonceBlockhashNotExpired,
    /// Pinned System error 8.
    NonceUnexpectedBlockhashValue,
    /// Future or unknown custom error code.
    Unknown(u32),
}
impl SystemError {
    /// Decodes a custom program error number without discarding unknown values.
    pub const fn from_code(code: u32) -> Self {
        match code {
            CODE_ACCOUNT_ALREADY_IN_USE => Self::AccountAlreadyInUse,
            CODE_RESULT_WITH_NEGATIVE_LAMPORTS => Self::ResultWithNegativeLamports,
            CODE_INVALID_PROGRAM_ID => Self::InvalidProgramId,
            CODE_INVALID_ACCOUNT_DATA_LENGTH => Self::InvalidAccountDataLength,
            CODE_MAX_SEED_LENGTH_EXCEEDED => Self::MaxSeedLengthExceeded,
            CODE_ADDRESS_WITH_SEED_MISMATCH => Self::AddressWithSeedMismatch,
            CODE_NONCE_NO_RECENT_BLOCKHASHES => Self::NonceNoRecentBlockhashes,
            CODE_NONCE_BLOCKHASH_NOT_EXPIRED => Self::NonceBlockhashNotExpired,
            CODE_NONCE_UNEXPECTED_BLOCKHASH_VALUE => Self::NonceUnexpectedBlockhashValue,
            _ => Self::Unknown(code),
        }
    }
    /// Returns the original custom error number.
    pub const fn code(self) -> u32 {
        match self {
            Self::AccountAlreadyInUse => CODE_ACCOUNT_ALREADY_IN_USE,
            Self::ResultWithNegativeLamports => CODE_RESULT_WITH_NEGATIVE_LAMPORTS,
            Self::InvalidProgramId => CODE_INVALID_PROGRAM_ID,
            Self::InvalidAccountDataLength => CODE_INVALID_ACCOUNT_DATA_LENGTH,
            Self::MaxSeedLengthExceeded => CODE_MAX_SEED_LENGTH_EXCEEDED,
            Self::AddressWithSeedMismatch => CODE_ADDRESS_WITH_SEED_MISMATCH,
            Self::NonceNoRecentBlockhashes => CODE_NONCE_NO_RECENT_BLOCKHASHES,
            Self::NonceBlockhashNotExpired => CODE_NONCE_BLOCKHASH_NOT_EXPIRED,
            Self::NonceUnexpectedBlockhashValue => CODE_NONCE_UNEXPECTED_BLOCKHASH_VALUE,
            Self::Unknown(code) => code,
        }
    }
    /// Returns the pinned human-readable description of a known error.
    pub const fn message(self) -> Option<&'static str> {
        match self {
            Self::AccountAlreadyInUse => Some("an account with the same address already exists"),
            Self::ResultWithNegativeLamports => {
                Some("account does not have enough SOL to perform the operation")
            }
            Self::InvalidProgramId => Some("cannot assign account to this program id"),
            Self::InvalidAccountDataLength => Some("cannot allocate account data of this length"),
            Self::MaxSeedLengthExceeded => Some("length of requested seed is too long"),
            Self::AddressWithSeedMismatch => {
                Some("provided address does not match addressed derived from seed")
            }
            Self::NonceNoRecentBlockhashes => {
                Some("advancing stored nonce requires a populated RecentBlockhashes sysvar")
            }
            Self::NonceBlockhashNotExpired => Some("stored nonce is still in recent_blockhashes"),
            Self::NonceUnexpectedBlockhashValue => {
                Some("specified nonce does not match stored nonce")
            }
            Self::Unknown(_) => None,
        }
    }
}
