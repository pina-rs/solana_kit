//! Ordered instruction data and account privileges.

use alloc::vec::Vec;

use crate::Address;

/// The signer and write privileges requested for one instruction account.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccountRole {
    /// Read access without a signature requirement.
    Readonly,
    /// Read and write access without a signature requirement.
    Writable,
    /// Read access with a signature requirement.
    ReadonlySigner,
    /// Read and write access with a signature requirement.
    WritableSigner,
}

impl AccountRole {
    /// Whether this role requires the account to sign the transaction.
    #[must_use]
    pub const fn is_signer(self) -> bool {
        matches!(self, Self::ReadonlySigner | Self::WritableSigner)
    }

    /// Whether this role permits the account to be modified.
    #[must_use]
    pub const fn is_writable(self) -> bool {
        matches!(self, Self::Writable | Self::WritableSigner)
    }

    /// Takes the union of both roles without dropping either privilege.
    #[must_use]
    pub const fn merge(self, other: Self) -> Self {
        Self::from_privileges(
            self.is_signer() || other.is_signer(),
            self.is_writable() || other.is_writable(),
        )
    }

    /// Constructs the exact role for the supplied privileges.
    #[must_use]
    pub const fn from_privileges(signer: bool, writable: bool) -> Self {
        match (signer, writable) {
            (false, false) => Self::Readonly,
            (false, true) => Self::Writable,
            (true, false) => Self::ReadonlySigner,
            (true, true) => Self::WritableSigner,
        }
    }
}

/// One account in an instruction's positional account list.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AccountMeta {
    /// The account supplied at this position.
    pub address: Address,
    /// Privileges requested by this instruction.
    pub role: AccountRole,
}

/// An owned program invocation, independent of transaction compilation.
///
/// Account order and duplicate positions are meaningful to the program and are
/// preserved exactly. Privilege merging belongs to transaction compilation;
/// constructing an instruction never sorts or deduplicates its accounts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Instruction {
    /// Program to invoke.
    pub program_address: Address,
    /// Ordered positional accounts, including any intentional duplicates.
    pub accounts: Vec<AccountMeta>,
    /// Program-specific instruction bytes.
    pub data: Vec<u8>,
}
