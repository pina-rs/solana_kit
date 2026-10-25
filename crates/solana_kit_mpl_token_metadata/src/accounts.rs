//! Explicit account allocation policy preserves padding and future trailing data.
use crate::generated::*;
use alloc::vec::Vec;
use solana_kit_core::{
    Address,
    accounts::{Account, AccountError, EncodedAccount},
    codecs::{Decoder, Error, Limits, Reader},
};

const EDITION_KEY: u8 = 1;
const MASTER_EDITION_V1_KEY: u8 = 2;
const RESERVATION_LIST_V1_KEY: u8 = 3;
const METADATA_KEY: u8 = 4;
const RESERVATION_LIST_V2_KEY: u8 = 5;
const MASTER_EDITION_V2_KEY: u8 = 6;
const EDITION_MARKER_KEY: u8 = 7;
const USE_AUTHORITY_RECORD_KEY: u8 = 8;

const COLLECTION_AUTHORITY_RECORD_KEY: u8 = 9;
const TOKEN_OWNED_ESCROW_KEY: u8 = 10;
const TOKEN_RECORD_KEY: u8 = 11;
const METADATA_DELEGATE_RECORD_KEY: u8 = 12;
const EDITION_MARKER_V2_KEY: u8 = 13;
const HOLDER_DELEGATE_RECORD_KEY: u8 = 14;

/// Known allocated account families, identified without assuming payload validity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccountKind {
    /// Printed edition state.
    Edition,
    /// Legacy master edition state.
    MasterEditionV1,
    /// Legacy reservation list.
    ReservationListV1,
    /// Mint metadata state.
    Metadata,
    /// Current reservation list.
    ReservationListV2,
    /// Master edition state.
    MasterEditionV2,
    /// Fixed edition bitmap.
    EditionMarker,
    /// Use approval state.
    UseAuthorityRecord,
    /// Collection authority approval.
    CollectionAuthorityRecord,
    /// Token-owned escrow state.
    TokenOwnedEscrow,
    /// Programmable token state.
    TokenRecord,
    /// Metadata delegate state.
    MetadataDelegateRecord,
    /// Variable edition bitmap.
    EditionMarkerV2,
    /// Holder delegate state.
    HolderDelegateRecord,
}

/// Identifies a known allocated account by its literal key byte.
/// # Errors
/// Rejects empty data, uninitialized state and unknown keys. This does not validate fields.
pub fn identify_account(bytes: &[u8]) -> Result<AccountKind, Error> {
    match *bytes.first().ok_or(Error::Truncated)? {
        EDITION_KEY => Ok(AccountKind::Edition),
        MASTER_EDITION_V1_KEY => Ok(AccountKind::MasterEditionV1),
        RESERVATION_LIST_V1_KEY => Ok(AccountKind::ReservationListV1),
        METADATA_KEY => Ok(AccountKind::Metadata),
        RESERVATION_LIST_V2_KEY => Ok(AccountKind::ReservationListV2),
        MASTER_EDITION_V2_KEY => Ok(AccountKind::MasterEditionV2),
        EDITION_MARKER_KEY => Ok(AccountKind::EditionMarker),
        USE_AUTHORITY_RECORD_KEY => Ok(AccountKind::UseAuthorityRecord),
        COLLECTION_AUTHORITY_RECORD_KEY => Ok(AccountKind::CollectionAuthorityRecord),
        TOKEN_OWNED_ESCROW_KEY => Ok(AccountKind::TokenOwnedEscrow),
        TOKEN_RECORD_KEY => Ok(AccountKind::TokenRecord),
        METADATA_DELEGATE_RECORD_KEY => Ok(AccountKind::MetadataDelegateRecord),
        EDITION_MARKER_V2_KEY => Ok(AccountKind::EditionMarkerV2),
        HOLDER_DELEGATE_RECORD_KEY => Ok(AccountKind::HolderDelegateRecord),
        _ => Err(Error::InvalidDiscriminator),
    }
}

/// Treatment of bytes after a complete pinned account value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AllocationPolicy {
    /// Reject all trailing bytes.
    Exact,
    /// Accept only zero allocation padding and retain it byte-exact.
    ZeroPadding,
    /// Retain arbitrary appended bytes without interpreting their version.
    PreserveTail,
}

/// A complete pinned account value together with its retained allocation tail.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Allocated<T> {
    /// Decoded pinned representation.
    pub value: T,
    /// Original bytes following the pinned representation.
    pub trailing: Vec<u8>,
}

/// An explicit wire wrapper suitable for the existing typed account fetch methods.
#[derive(Clone, Copy, Debug)]
pub struct AllocatedCodec<C> {
    /// Pinned account codec, which validates the account key.
    pub codec: C,
    /// Caller-selected allocation contract.
    pub policy: AllocationPolicy,
}

impl<T, C: Decoder<T>> Decoder<Allocated<T>> for AllocatedCodec<C> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<Allocated<T>, Error> {
        let value = self.codec.read(reader)?;
        let tail = reader.remaining();

        match self.policy {
            AllocationPolicy::Exact if !tail.is_empty() => return Err(Error::TrailingBytes),

            AllocationPolicy::ZeroPadding if tail.iter().any(|byte| *byte != 0) => {
                return Err(Error::InvalidValue);
            }

            _ => {}
        }

        let count = tail.len();
        reader.claim(count)?;
        let mut trailing = Vec::new();
        trailing
            .try_reserve_exact(count)
            .map_err(|_| Error::AllocationFailed)?;
        trailing.extend_from_slice(reader.take(count)?);

        Ok(Allocated { value, trailing })
    }
}

macro_rules! decoder {
    ($default:ident, $explicit:ident, $value:ident, $codec:ident) => {
        #[doc = concat!("Decodes a fetched `", stringify!($value), "` snapshot.")]
        #[doc = "# Errors"]
        #[doc = "Preserves owner, allocation, key, wire, tail and caller-budget failures."]
        pub fn $default(
            account: EncodedAccount,
            policy: AllocationPolicy,
            limits: Limits,
        ) -> Result<Account<Allocated<$value>>, AccountError> {
            $explicit(account, PROGRAM, policy, limits)
        }

        #[doc = concat!("Decodes `", stringify!($value), "` with an explicit expected owner.")]
        #[doc = "# Errors"]
        #[doc = "Preserves the core account decoder's exact failures and metadata."]
        pub fn $explicit(
            account: EncodedAccount,
            program: Address,
            policy: AllocationPolicy,
            limits: Limits,
        ) -> Result<Account<Allocated<$value>>, AccountError> {
            account.decode(
                &AllocatedCodec {
                    codec: $codec,
                    policy,
                },
                program,
                limits,
            )
        }
    };
}

decoder!(
    decode_edition,
    decode_edition_with_program,
    Edition,
    EditionCodec
);
decoder!(
    decode_master_edition_v1,
    decode_master_edition_v1_with_program,
    MasterEditionV1,
    MasterEditionV1Codec
);
decoder!(
    decode_master_edition_v2,
    decode_master_edition_v2_with_program,
    MasterEditionV2,
    MasterEditionV2Codec
);
decoder!(
    decode_metadata,
    decode_metadata_with_program,
    Metadata,
    MetadataCodec
);
decoder!(
    decode_reservation_list_v1,
    decode_reservation_list_v1_with_program,
    ReservationListV1,
    ReservationListV1Codec
);
decoder!(
    decode_reservation_list_v2,
    decode_reservation_list_v2_with_program,
    ReservationListV2,
    ReservationListV2Codec
);
decoder!(
    decode_edition_marker,
    decode_edition_marker_with_program,
    EditionMarker,
    EditionMarkerCodec
);
decoder!(
    decode_edition_marker_v2,
    decode_edition_marker_v2_with_program,
    EditionMarkerV2,
    EditionMarkerV2Codec
);

decoder!(
    decode_use_authority_record,
    decode_use_authority_record_with_program,
    UseAuthorityRecord,
    UseAuthorityRecordCodec
);
decoder!(
    decode_collection_authority_record,
    decode_collection_authority_record_with_program,
    CollectionAuthorityRecord,
    CollectionAuthorityRecordCodec
);
decoder!(
    decode_token_owned_escrow,
    decode_token_owned_escrow_with_program,
    TokenOwnedEscrow,
    TokenOwnedEscrowCodec
);
decoder!(
    decode_token_record,
    decode_token_record_with_program,
    TokenRecord,
    TokenRecordCodec
);
decoder!(
    decode_metadata_delegate_record,
    decode_metadata_delegate_record_with_program,
    MetadataDelegateRecord,
    MetadataDelegateRecordCodec
);
decoder!(
    decode_holder_delegate_record,
    decode_holder_delegate_record_with_program,
    HolderDelegateRecord,
    HolderDelegateRecordCodec
);
