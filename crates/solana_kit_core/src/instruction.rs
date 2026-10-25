//! Ordered instruction data and account privileges.

use alloc::vec::Vec;

use crate::Address;

/// The signer and write privileges requested for one instruction account.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AccountMeta {
    /// The account supplied at this position.
    pub address: Address,
    /// Privileges requested by this instruction.
    pub role: AccountRole,
}

/// Privileges available to an account loaded through an address lookup table.
/// Such an account cannot sign; signer roles have no representation here.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LookupRole {
    /// Read access without a signature requirement.
    Readonly,
    /// Read and write access without a signature requirement.
    Writable,
}

impl LookupRole {
    /// Returns the corresponding ordinary account role.
    #[must_use]
    pub const fn account_role(self) -> AccountRole {
        match self {
            Self::Readonly => AccountRole::Readonly,
            Self::Writable => AccountRole::Writable,
        }
    }
}

/// A Kit-owned account address together with its lookup-table origin.
///
/// The u8 index and non-signer role encode the lookup wire constraints. The
/// caller still authenticates the table snapshot and its indexed address.
///
/// ```compile_fail
/// use solana_kit_core::{AccountLookupMeta, AccountRole, Address};
/// let address = Address::from_bytes([0; 32]);
/// let lookup = AccountLookupMeta {
///     address, lookup_table_address: address, address_index: 0,
///     role: AccountRole::ReadonlySigner,
/// };
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct AccountLookupMeta {
    /// Address loaded from the table.
    pub address: Address,
    /// Table account containing the address.
    pub lookup_table_address: Address,
    /// Exact address position in that table, in the u8 wire domain.
    pub address_index: u8,
    /// Read or write access, without signing privileges.
    pub role: LookupRole,
}

impl AccountLookupMeta {
    /// Drops lookup provenance while retaining address and privileges.
    #[must_use]
    pub const fn account_meta(self) -> AccountMeta {
        AccountMeta {
            address: self.address,
            role: self.role.account_role(),
        }
    }
}

/// An instruction account with optional, explicit lookup-table provenance.
///
/// Use ordinary metadata for static or compiler-selected accounts, and lookup
/// metadata when the exact table and index must survive compilation.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum InstructionAccount {
    /// Ordinary privileges; the compiler may select an available table snapshot.
    Account(AccountMeta),
    /// An exact table origin that compilation must validate rather than replace.
    Lookup(AccountLookupMeta),
}

impl InstructionAccount {
    /// Returns address and privileges without changing this value's provenance.
    #[must_use]
    pub const fn account_meta(self) -> AccountMeta {
        match self {
            Self::Account(account) => account,
            Self::Lookup(account) => account.account_meta(),
        }
    }
}

impl From<AccountMeta> for InstructionAccount {
    fn from(account: AccountMeta) -> Self {
        Self::Account(account)
    }
}

impl From<AccountLookupMeta> for InstructionAccount {
    fn from(account: AccountLookupMeta) -> Self {
        Self::Lookup(account)
    }
}

/// An owned program invocation, independent of transaction compilation.
///
/// Account order and duplicate positions are meaningful to the program and are
/// preserved exactly. Privilege merging belongs to transaction compilation;
/// constructing an instruction never sorts or deduplicates its accounts.
/// Select [`InstructionAccount`] for heterogeneous ordinary and lookup-origin
/// inputs. Ordinary callers use the concrete [`Instruction`] alias.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstructionWithAccounts<A> {
    /// Program to invoke.
    pub program_address: Address,
    /// Ordered positional accounts, including any intentional duplicates.
    pub accounts: Vec<A>,
    /// Program-specific instruction bytes.
    pub data: Vec<u8>,
}

/// An ordinary invocation whose account metadata remains concrete during inference.
///
/// Empty account literals need no annotation or subsequent compiler call to
/// determine their account type.
pub type Instruction = InstructionWithAccounts<AccountMeta>;

impl From<Instruction> for InstructionWithAccounts<InstructionAccount> {
    fn from(instruction: Instruction) -> Self {
        Self {
            program_address: instruction.program_address,
            accounts: instruction
                .accounts
                .into_iter()
                .map(InstructionAccount::from)
                .collect(),
            data: instruction.data,
        }
    }
}
