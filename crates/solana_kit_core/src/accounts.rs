//! Owned account snapshots and deterministic decoding for off-chain clients.

use alloc::vec::Vec;
use core::fmt;

use crate::{
    Address,
    codecs::{self, Decoder, Limits},
};

/// Reference account header size, excluding account data.
pub const BASE_ACCOUNT_SIZE: usize = 128;

/// An account snapshot whose data may be encoded bytes or a decoded value.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Account<T> {
    /// Address requested by the client.
    pub address: Address,
    /// Program that owns this account.
    pub owner: Address,
    /// Balance in lamports.
    pub lamports: u64,
    /// Whether this is an executable account.
    pub executable: bool,
    /// Reported data allocation size, independent of decoded representation.
    pub space: u64,
    /// Rent epoch when supplied by the source; absent sources remain absent.
    pub rent_epoch: Option<u64>,
    /// Encoded or decoded account data.
    pub data: T,
}

/// An account with owned encoded bytes.
pub type EncodedAccount = Account<Vec<u8>>;

/// An encoded snapshot or an explicitly missing requested address.
pub type MaybeEncodedAccount = MaybeAccount<Vec<u8>>;

/// Missing accounts retain only their address and cannot contain account data.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum MaybeAccount<T> {
    /// An existing snapshot.
    Present(Account<T>),
    /// The requested account does not exist.
    Missing(Address),
}

/// Account validation preserves the address and the original codec error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccountError {
    /// The requested account is absent.
    Missing(Address),
    /// The account belongs to a different program.
    OwnerMismatch {
        /// Account being validated.
        address: Address,
        /// Required program.
        expected: Address,
        /// Actual program.
        actual: Address,
    },
    /// Reported allocation and encoded bytes disagree.
    DataSizeMismatch(Address),
    /// Decoding rejected the data.
    Decode {
        /// Account being decoded.
        address: Address,
        /// Underlying deterministic codec failure.
        source: codecs::Error,
    },
}

impl fmt::Display for AccountError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "account error: {self:?}")
    }
}

impl core::error::Error for AccountError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match self {
            Self::Decode { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl<T> MaybeAccount<T> {
    /// Reports whether the requested account exists without consuming it.
    #[must_use]
    pub const fn exists(&self) -> bool {
        matches!(self, Self::Present(_))
    }

    /// Returns the requested address in either state.
    #[must_use]
    pub const fn address(&self) -> Address {
        match self {
            Self::Present(account) => account.address,
            Self::Missing(address) => *address,
        }
    }

    /// Requires a present account, preserving an absent address in the error.
    pub fn require(self) -> Result<Account<T>, AccountError> {
        match self {
            Self::Present(account) => Ok(account),
            Self::Missing(address) => Err(AccountError::Missing(address)),
        }
    }
}

impl EncodedAccount {
    /// Decodes exact account data after checking its owner and reported size.
    ///
    /// All metadata is preserved. Trailing bytes follow `Decoder::decode`'s
    /// strict policy. Data discriminators and layout validation belong to the
    /// supplied decoder. No RPC or account fetching occurs here.
    pub fn decode<T>(
        self,
        decoder: &impl Decoder<T>,
        expected_owner: Address,
        limits: Limits,
    ) -> Result<Account<T>, AccountError> {
        if self.owner != expected_owner {
            return Err(AccountError::OwnerMismatch {
                address: self.address,
                expected: expected_owner,
                actual: self.owner,
            });
        }

        if u64::try_from(self.data.len()).ok() != Some(self.space) {
            return Err(AccountError::DataSizeMismatch(self.address));
        }

        let data = decoder
            .decode(&self.data, limits)
            .map_err(|source| AccountError::Decode {
                address: self.address,
                source,
            })?;

        Ok(Account {
            address: self.address,
            owner: self.owner,
            lamports: self.lamports,
            executable: self.executable,
            space: self.space,
            rent_epoch: self.rent_epoch,
            data,
        })
    }
}

impl MaybeAccount<Vec<u8>> {
    /// Decodes present data while preserving the missing state without decoding.
    pub fn decode<T>(
        self,
        decoder: &impl Decoder<T>,
        expected_owner: Address,
        limits: Limits,
    ) -> Result<MaybeAccount<T>, AccountError> {
        match self {
            Self::Missing(address) => Ok(MaybeAccount::Missing(address)),
            Self::Present(account) => account
                .decode(decoder, expected_owner, limits)
                .map(MaybeAccount::Present),
        }
    }
}
