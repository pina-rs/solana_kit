//! Collection minting supplies the full source authority and metadata account order.
use crate::{
    MetadataArgs, MintAccounts, MintToCollectionV1,
    mint::{meta, with_tail},
    validate_metadata,
};
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Error, Limits},
};

/// V1 collection roles, supplied explicitly instead of using placeholder public keys.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CollectionMintAccounts {
    /// Shared tree and mint authorities.
    pub mint: MintAccounts,
    /// Required collection-authority signer.
    pub collection_authority: Address,
    /// Delegation record or the selected Bubblegum program-ID sentinel.
    pub authority_record: Option<Address>,
    /// Collection mint.
    pub collection_mint: Address,
    /// Writable collection metadata.
    pub collection_metadata: Address,
    /// Collection master edition.
    pub edition: Address,
    /// Explicit retained Bubblegum signer account.
    pub bubblegum_signer: Address,
    /// Explicit Token Metadata deployment.
    pub token_metadata_program: Address,
}

impl CollectionMintAccounts {
    /// Mints with all16 fixed accounts and caller-owned creator-authority tails.
    /// # Errors
    /// Rejects metadata, derivation, exact role/data limits and allocation failure.
    pub fn mint_to_collection_v1(
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

        if metadata
            .collection
            .as_ref()
            .map(|collection| collection.key)
            != Some(self.collection_mint)
        {
            return Err(Error::InvalidValue);
        }

        let fixed = self.fixed_accounts()?;
        MintToCollectionV1 {
            metadata_args: metadata,
        }
        .instruction_with_program(
            self.mint.program,
            &with_tail(fixed, creator_authorities, limits)?,
            limits,
        )
    }

    fn fixed_accounts(&self) -> Result<[AccountMeta; 16], Error> {
        let mint = &self.mint;
        Ok([
            meta(mint.authority()?, AccountRole::Writable),
            meta(mint.owner, AccountRole::Readonly),
            meta(mint.delegate.unwrap_or(mint.owner), AccountRole::Readonly),
            meta(mint.tree, AccountRole::Writable),
            meta(mint.payer, AccountRole::ReadonlySigner),
            meta(
                mint.tree_delegate.unwrap_or(mint.payer),
                AccountRole::ReadonlySigner,
            ),
            meta(self.collection_authority, AccountRole::ReadonlySigner),
            meta(
                self.authority_record.unwrap_or(mint.program),
                AccountRole::Readonly,
            ),
            meta(self.collection_mint, AccountRole::Readonly),
            meta(self.collection_metadata, AccountRole::Writable),
            meta(self.edition, AccountRole::Readonly),
            meta(self.bubblegum_signer, AccountRole::Readonly),
            meta(mint.programs.log_wrapper, AccountRole::Readonly),
            meta(mint.programs.compression, AccountRole::Readonly),
            meta(self.token_metadata_program, AccountRole::Readonly),
            meta(mint.programs.system, AccountRole::Readonly),
        ])
    }
}
