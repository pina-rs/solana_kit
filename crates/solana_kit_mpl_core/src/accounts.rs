//! Typed fetched snapshots reuse Kit's owner and allocation boundary.
use crate::generated::*;
use alloc::vec::Vec;
use solana_kit_core::{
    Address,
    accounts::{Account, AccountError, EncodedAccount},
    codecs::{Decoder, Error, Limits, Reader},
};

/// A known account base with the source-permitted trailing extension bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountData<T> {
    /// Decoded fixed or variable base fields.
    pub base: T,
    /// Uninterpreted extension bytes; preservation does not authenticate a registry.
    pub extension_bytes: Vec<u8>,
}

struct Extended<C>(C);

impl<T, C: Decoder<T>> Decoder<AccountData<T>> for Extended<C> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AccountData<T>, Error> {
        let base = self.0.read(reader)?;
        let size = reader.remaining().len();
        reader.claim(size)?;
        let mut extension_bytes = Vec::new();
        extension_bytes
            .try_reserve_exact(size)
            .map_err(|_| Error::AllocationFailed)?;
        extension_bytes.extend_from_slice(reader.take(size)?);
        Ok(AccountData {
            base,
            extension_bytes,
        })
    }
}

macro_rules! account_decoder {
    ($canonical:ident, $custom:ident, $value:ident, $codec:ident) => {
        #[doc = concat!("Decodes a canonical owned `", stringify!($value), "` snapshot.")]
        #[doc = "# Errors"]
        #[doc = "Preserves owner, allocation, wire, discriminator and budget failures."]
        pub fn $canonical(
            account: EncodedAccount,
            limits: Limits,
        ) -> Result<Account<AccountData<$value>>, AccountError> {
            $custom(account, PROGRAM, limits)
        }

        #[doc = concat!("Decodes `", stringify!($value), "` against an explicit owner.")]
        #[doc = "# Errors"]
        #[doc = "Preserves owner, allocation, wire, discriminator and budget failures."]
        pub fn $custom(
            account: EncodedAccount,
            program: Address,
            limits: Limits,
        ) -> Result<Account<AccountData<$value>>, AccountError> {
            account.decode(&Extended($codec), program, limits)
        }
    };
}

account_decoder!(
    decode_asset_v1,
    decode_asset_v1_with_program,
    AssetV1,
    AssetV1Codec
);
account_decoder!(
    decode_collection_v1,
    decode_collection_v1_with_program,
    CollectionV1,
    CollectionV1Codec
);
account_decoder!(
    decode_group_v1,
    decode_group_v1_with_program,
    GroupV1,
    GroupV1Codec
);
account_decoder!(
    decode_hashed_asset_v1,
    decode_hashed_asset_v1_with_program,
    HashedAssetV1,
    HashedAssetV1Codec
);
account_decoder!(
    decode_plugin_header_v1,
    decode_plugin_header_v1_with_program,
    PluginHeaderV1,
    PluginHeaderV1Codec
);
account_decoder!(
    decode_plugin_registry_v1,
    decode_plugin_registry_v1_with_program,
    PluginRegistryV1,
    PluginRegistryV1Codec
);
