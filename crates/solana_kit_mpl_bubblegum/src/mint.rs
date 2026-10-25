//! Canonical mint accounts derive authorities and preserve creator-signing tails.
use crate::{
    CompressionPrograms, MetadataArgs, MetadataArgsV2, MintV1, MintV2,
    find_tree_authority_with_program, validate_metadata,
};
use alloc::vec::Vec;
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Error, Limits},
};

/// Shared mint identity with explicit deployment and paying authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MintAccounts {
    /// Bubblegum deployment.
    pub program: Address,
    /// Mutable Merkle tree.
    pub tree: Address,
    /// Paying signer.
    pub payer: Address,
    /// Tree delegate, defaulting to the paying signer.
    pub tree_delegate: Option<Address>,
    /// New leaf owner.
    pub owner: Address,
    /// New delegate, defaulting to owner.
    pub delegate: Option<Address>,
    /// Explicit Compression/System/Noop dependencies.
    pub programs: CompressionPrograms,
}

impl MintAccounts {
    /// Mints V1 metadata with complete headers and an actual derived tree authority.
    /// # Errors
    /// Rejects invalid metadata, derivation, account/data limits and allocation failure.
    pub fn mint_v1(
        &self,
        metadata: MetadataArgs,
        creator_authorities: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        validate_metadata(
            &metadata.name,
            &metadata.symbol,
            &metadata.uri,
            metadata.seller_fee_basis_points,
            &metadata.creators,
        )?;
        let authority = self.authority()?;
        let fixed = [
            meta(authority, AccountRole::Writable),
            meta(self.owner, AccountRole::Readonly),
            meta(self.delegate.unwrap_or(self.owner), AccountRole::Readonly),
            meta(self.tree, AccountRole::Writable),
            meta(self.payer, AccountRole::ReadonlySigner),
            meta(
                self.tree_delegate.unwrap_or(self.payer),
                AccountRole::ReadonlySigner,
            ),
            meta(self.programs.log_wrapper, AccountRole::Readonly),
            meta(self.programs.compression, AccountRole::Readonly),
            meta(self.programs.system, AccountRole::Readonly),
        ];
        MintV1 { message: metadata }.instruction_with_program(
            self.program,
            &with_tail(fixed, creator_authorities, limits)?,
            limits,
        )
    }

    pub(crate) fn authority(&self) -> Result<Address, Error> {
        find_tree_authority_with_program(self.program, self.tree)
            .map(|pda| pda.address())
            .map_err(|_| Error::InvalidValue)
    }
}

/// Explicit V2 Core dependencies. None represents the actual program-ID account sentinel.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CoreMintAccounts {
    /// Shared tree, owner, delegate and payer.
    pub mint: MintAccounts,
    /// Optional collection authority signer.
    pub collection_authority: Option<Address>,
    /// Optional mutable Core collection.
    pub collection: Option<Address>,
    /// Optional real Core CPI signer PDA.
    pub cpi_signer: Option<Address>,
    /// Selected MPL Core program.
    pub core_program: Address,
}

impl CoreMintAccounts {
    /// Mints complete V2 metadata and independent optional asset/schema values.
    /// # Errors
    /// Rejects metadata, derivation, data/accounts, budgets and allocation failure.
    pub fn mint_v2(
        &self,
        data: MintV2,
        creator_authorities: &[AccountMeta],
        limits: Limits,
    ) -> Result<Instruction, Error> {
        let metadata: &MetadataArgsV2 = &data.metadata_args;
        crate::validate_metadata_v2(
            &metadata.name,
            &metadata.symbol,
            &metadata.uri,
            metadata.seller_fee_basis_points,
            &metadata.creators,
        )?;
        let mint = &self.mint;

        if metadata.collection != self.collection
            || (self.collection.is_some() && self.cpi_signer.is_none())
        {
            return Err(Error::InvalidValue);
        }

        self.validate_cpi_signer()?;

        if metadata.seller_fee_basis_points == crate::SELLER_FEE_BASIS_POINTS_INHERIT
            && self.collection.is_none()
        {
            return Err(Error::InvalidValue);
        }

        let fixed = self.fixed_accounts()?;
        data.instruction_with_program(
            mint.program,
            &with_tail(fixed, creator_authorities, limits)?,
            limits,
        )
    }

    fn validate_cpi_signer(&self) -> Result<(), Error> {
        if let Some(signer) = self.cpi_signer {
            let expected = crate::find_mpl_core_cpi_signer_with_program(self.mint.program)
                .map_err(|_| Error::InvalidValue)?
                .address();

            if signer != expected {
                return Err(Error::InvalidValue);
            }
        }

        Ok(())
    }

    fn fixed_accounts(&self) -> Result<[AccountMeta; 13], Error> {
        let mint = &self.mint;
        Ok([
            meta(mint.authority()?, AccountRole::Writable),
            meta(mint.payer, AccountRole::WritableSigner),
            optional(
                mint.program,
                mint.tree_delegate,
                AccountRole::ReadonlySigner,
            ),
            optional(
                mint.program,
                self.collection_authority,
                AccountRole::ReadonlySigner,
            ),
            meta(mint.owner, AccountRole::Readonly),
            optional(mint.program, mint.delegate, AccountRole::Readonly),
            meta(mint.tree, AccountRole::Writable),
            optional(mint.program, self.collection, AccountRole::Writable),
            optional(mint.program, self.cpi_signer, AccountRole::Readonly),
            meta(mint.programs.log_wrapper, AccountRole::Readonly),
            meta(mint.programs.compression, AccountRole::Readonly),
            meta(self.core_program, AccountRole::Readonly),
            meta(mint.programs.system, AccountRole::Readonly),
        ])
    }
}

pub(crate) fn with_tail<const N: usize>(
    fixed: [AccountMeta; N],
    tail: &[AccountMeta],
    limits: Limits,
) -> Result<Vec<AccountMeta>, Error> {
    let count = N.checked_add(tail.len()).ok_or(Error::Overflow)?;

    if count > limits.max_items {
        return Err(Error::LimitExceeded);
    }

    let mut accounts = Vec::new();
    accounts
        .try_reserve_exact(count)
        .map_err(|_| Error::AllocationFailed)?;
    accounts.extend(fixed);
    accounts.extend_from_slice(tail);
    Ok(accounts)
}

pub(crate) fn optional(
    program: Address,
    address: Option<Address>,
    role: AccountRole,
) -> AccountMeta {
    match address {
        Some(address) => meta(address, role),
        None => meta(program, AccountRole::Readonly),
    }
}

pub(crate) fn meta(address: Address, role: AccountRole) -> AccountMeta {
    AccountMeta { address, role }
}
