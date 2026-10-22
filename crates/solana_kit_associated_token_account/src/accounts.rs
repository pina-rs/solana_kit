//! Exact positional account contracts from the pinned handwritten client.

use solana_kit_core::{AccountMeta, AccountRole, Address, codecs::Error};

const CREATE_ROLES: [AccountRole; 6] = [
    AccountRole::WritableSigner,
    AccountRole::Writable,
    AccountRole::Readonly,
    AccountRole::Readonly,
    AccountRole::Readonly,
    AccountRole::Readonly,
];
const RECOVER_ROLES: [AccountRole; 7] = [
    AccountRole::Writable,
    AccountRole::Readonly,
    AccountRole::Writable,
    AccountRole::Readonly,
    AccountRole::Readonly,
    AccountRole::WritableSigner,
    AccountRole::Readonly,
];

fn metadata<const N: usize>(addresses: [Address; N], roles: [AccountRole; N]) -> [AccountMeta; N] {
    core::array::from_fn(|index| AccountMeta {
        address: addresses[index],
        role: roles[index],
    })
}

fn addresses<const N: usize>(
    accounts: &[AccountMeta],
    roles: [AccountRole; N],
) -> Result<[Address; N], Error> {
    if accounts.len() != N
        || accounts
            .iter()
            .zip(roles)
            .any(|(account, role)| account.role != role)
    {
        return Err(Error::InvalidValue);
    }

    Ok(core::array::from_fn(|index| accounts[index].address))
}

/// Ordered inputs shared by create and idempotent create.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CreateAccounts {
    /// Writable funding signer.
    pub payer: Address,
    /// Writable associated account to create.
    pub associated_token: Address,
    /// Readonly wallet owner.
    pub owner: Address,
    /// Readonly token mint.
    pub mint: Address,
    /// Readonly System program supplied by the caller.
    pub system_program: Address,
    /// Readonly token program. Both Token and Token-2022 are supported.
    pub token_program: Address,
}

impl CreateAccounts {
    /// Produces the exact six ordered account positions without deduplication.
    #[must_use]
    pub fn metadata(self) -> [AccountMeta; 6] {
        metadata(
            [
                self.payer,
                self.associated_token,
                self.owner,
                self.mint,
                self.system_program,
                self.token_program,
            ],
            CREATE_ROLES,
        )
    }

    pub(crate) fn parse(accounts: &[AccountMeta]) -> Result<Self, Error> {
        let [
            payer,
            associated_token,
            owner,
            mint,
            system_program,
            token_program,
        ] = addresses(accounts, CREATE_ROLES)?;
        Ok(Self {
            payer,
            associated_token,
            owner,
            mint,
            system_program,
            token_program,
        })
    }
}

/// Ordered inputs for nested account recovery.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecoverNestedAccounts {
    /// Writable nested account to close.
    pub nested_associated_token: Address,
    /// Readonly mint held by the nested account.
    pub nested_mint: Address,
    /// Writable destination owned by the wallet.
    pub destination_associated_token: Address,
    /// Readonly associated account that incorrectly owns the nested account.
    pub owner_associated_token: Address,
    /// Readonly mint of the owner associated account.
    pub owner_mint: Address,
    /// Writable wallet signer receiving the recovered lamports.
    pub wallet: Address,
    /// Readonly token program.
    pub token_program: Address,
}

impl RecoverNestedAccounts {
    /// Produces the exact seven ordered account positions without deduplication.
    #[must_use]
    pub fn metadata(self) -> [AccountMeta; 7] {
        metadata(
            [
                self.nested_associated_token,
                self.nested_mint,
                self.destination_associated_token,
                self.owner_associated_token,
                self.owner_mint,
                self.wallet,
                self.token_program,
            ],
            RECOVER_ROLES,
        )
    }

    pub(crate) fn parse(accounts: &[AccountMeta]) -> Result<Self, Error> {
        let [
            nested_associated_token,
            nested_mint,
            destination_associated_token,
            owner_associated_token,
            owner_mint,
            wallet,
            token_program,
        ] = addresses(accounts, RECOVER_ROLES)?;
        Ok(Self {
            nested_associated_token,
            nested_mint,
            destination_associated_token,
            owner_associated_token,
            owner_mint,
            wallet,
            token_program,
        })
    }
}
