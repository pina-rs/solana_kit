//! Complete MIT Dart seed recipes, with explicit-program counterparts.
use crate::{EDITION_MARKER_BIT_SIZE, MetadataDelegateRole, PROGRAM};
use alloc::string::ToString;
use solana_kit_core::{
    Address,
    crypto::{PdaError, ProgramDerivedAddress, find_program_address},
};

macro_rules! recipe {
    ($default:ident, $explicit:ident, [$($input:ident),*], [$($seed:expr),*]) => {
        #[doc = "Finds the canonical PDA using the complete pinned seed recipe."]
        #[doc = "# Errors"]
        #[doc = "Preserves exact seed and deterministic derivation failures."]
        pub fn $default($($input: Address),*) -> Result<ProgramDerivedAddress, PdaError> {
            $explicit(PROGRAM, $($input),*)
        }

        #[doc = "Finds the PDA with an explicit program in both the domain and seed bytes."]
        #[doc = "# Errors"]
        #[doc = "Preserves exact seed and deterministic derivation failures."]
        pub fn $explicit(program: Address, $($input: Address),*)
            -> Result<ProgramDerivedAddress, PdaError>
        {
            find_program_address(&[b"metadata", program.as_bytes(), $($seed),*], &program)
        }
    };
}

recipe!(
    find_metadata,
    find_metadata_with_program,
    [mint],
    [mint.as_bytes()]
);
recipe!(
    find_master_edition,
    find_master_edition_with_program,
    [mint],
    [mint.as_bytes(), b"edition"]
);
recipe!(
    find_edition_marker_v2,
    find_edition_marker_v2_with_program,
    [mint],
    [mint.as_bytes(), b"edition", b"marker"]
);
recipe!(
    find_token_record,
    find_token_record_with_program,
    [mint, token],
    [mint.as_bytes(), b"token_record", token.as_bytes()]
);
recipe!(
    find_use_authority_record,
    find_use_authority_record_with_program,
    [mint, authority],
    [mint.as_bytes(), b"user", authority.as_bytes()]
);
recipe!(
    find_collection_authority_record,
    find_collection_authority_record_with_program,
    [mint, authority],
    [
        mint.as_bytes(),
        b"collection_authority",
        authority.as_bytes()
    ]
);
recipe!(
    find_holder_delegate_record,
    find_holder_delegate_record_with_program,
    [mint, owner, delegate],
    [
        mint.as_bytes(),
        b"print_delegate",
        owner.as_bytes(),
        delegate.as_bytes()
    ]
);
recipe!(
    find_program_as_burner,
    find_program_as_burner_with_program,
    [],
    [b"burn"]
);

/// Finds the marker using the decimal floor of edition divided by248.
/// # Errors
/// Preserves exact deterministic derivation failures.
pub fn find_edition_marker(mint: Address, edition: u64) -> Result<ProgramDerivedAddress, PdaError> {
    find_edition_marker_with_program(PROGRAM, mint, edition)
}

/// Derives an explicit-program marker using decimal bytes, including the full u64 range.
/// # Errors
/// Preserves exact deterministic derivation failures.
pub fn find_edition_marker_with_program(
    program: Address,
    mint: Address,
    edition: u64,
) -> Result<ProgramDerivedAddress, PdaError> {
    let index = (edition / EDITION_MARKER_BIT_SIZE).to_string();

    find_program_address(
        &[
            b"metadata",
            program.as_bytes(),
            mint.as_bytes(),
            b"edition",
            index.as_bytes(),
        ],
        &program,
    )
}

/// Exact source-defined UTF-8 seed for a metadata-level delegation role.
pub fn metadata_delegate_role_seed(role: &MetadataDelegateRole) -> &'static [u8] {
    match role {
        MetadataDelegateRole::AuthorityItem => b"authority_item_delegate",
        MetadataDelegateRole::Collection => b"collection_delegate",
        MetadataDelegateRole::Use => b"use_delegate",
        MetadataDelegateRole::Data => b"data_delegate",
        MetadataDelegateRole::ProgrammableConfig => b"programmable_config_delegate",
        MetadataDelegateRole::DataItem => b"data_item_delegate",
        MetadataDelegateRole::CollectionItem => b"collection_item_delegate",
        MetadataDelegateRole::ProgrammableConfigItem => b"prog_config_item_delegate",
    }
}

/// Finds the canonical metadata-level delegation record.
/// # Errors
/// Preserves exact deterministic derivation failures.
pub fn find_metadata_delegate_record(
    mint: Address,
    role: &MetadataDelegateRole,
    update_authority: Address,
    delegate: Address,
) -> Result<ProgramDerivedAddress, PdaError> {
    find_metadata_delegate_record_with_program(PROGRAM, mint, role, update_authority, delegate)
}

/// Finds a record with explicit program, role, update authority and delegate seed ordering.
/// # Errors
/// Preserves exact deterministic derivation failures.
pub fn find_metadata_delegate_record_with_program(
    program: Address,
    mint: Address,
    role: &MetadataDelegateRole,
    update_authority: Address,
    delegate: Address,
) -> Result<ProgramDerivedAddress, PdaError> {
    find_program_address(
        &[
            b"metadata",
            program.as_bytes(),
            mint.as_bytes(),
            metadata_delegate_role_seed(role),
            update_authority.as_bytes(),
            delegate.as_bytes(),
        ],
        &program,
    )
}
