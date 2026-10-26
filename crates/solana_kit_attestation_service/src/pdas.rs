//! Exact MIT Dart PDA seeds. The caller owns account state and all effects.
use crate::PROGRAM;
use solana_kit_core::{
    Address,
    crypto::{PdaError, ProgramDerivedAddress, find_program_address},
};

/// Credential account seed domain.
pub const CREDENTIAL_SEED: &[u8] = b"credential";
/// Schema account seed domain.
pub const SCHEMA_SEED: &[u8] = b"schema";
/// Attestation account seed domain.
pub const ATTESTATION_SEED: &[u8] = b"attestation";
/// Tokenized schema mint seed domain.
pub const SCHEMA_MINT_SEED: &[u8] = b"schemaMint";
/// Tokenized attestation mint seed domain.
pub const ATTESTATION_MINT_SEED: &[u8] = b"attestationMint";
/// Event authority seed domain.
pub const EVENT_AUTHORITY_SEED: &[u8] = b"__event_authority";
/// Program signing authority seed domain.
pub const SAS_SEED: &[u8] = b"sas";

/// Derives a credential with the canonical program.
/// # Errors
/// Rejects names exceeding 32 UTF-8 bytes or unavailable bump candidates.
pub fn find_credential(authority: Address, name: &str) -> Result<ProgramDerivedAddress, PdaError> {
    find_credential_with_program(PROGRAM, authority, name)
}

/// Derives a credential for an explicit program.
/// # Errors
/// Preserves the deterministic seed and bump errors.
pub fn find_credential_with_program(
    program: Address,
    authority: Address,
    name: &str,
) -> Result<ProgramDerivedAddress, PdaError> {
    find_program_address(
        &[CREDENTIAL_SEED, authority.as_bytes(), name.as_bytes()],
        &program,
    )
}

/// Derives a schema with its exact one-byte version seed.
/// # Errors
/// Rejects names exceeding 32 UTF-8 bytes or unavailable bump candidates.
pub fn find_schema(
    credential: Address,
    name: &str,
    version: u8,
) -> Result<ProgramDerivedAddress, PdaError> {
    find_schema_with_program(PROGRAM, credential, name, version)
}

/// Derives a schema for an explicit program and one-byte version.
/// # Errors
/// Preserves the deterministic seed and bump errors.
pub fn find_schema_with_program(
    program: Address,
    credential: Address,
    name: &str,
    version: u8,
) -> Result<ProgramDerivedAddress, PdaError> {
    find_program_address(
        &[
            SCHEMA_SEED,
            credential.as_bytes(),
            name.as_bytes(),
            &[version],
        ],
        &program,
    )
}

/// Derives an attestation for the canonical program.
/// # Errors
/// Preserves deterministic bump-search failures.
pub fn find_attestation(
    credential: Address,
    schema: Address,
    nonce: Address,
) -> Result<ProgramDerivedAddress, PdaError> {
    find_attestation_with_program(PROGRAM, credential, schema, nonce)
}

/// Derives an attestation for an explicit program in credential/schema/nonce order.
/// # Errors
/// Preserves deterministic bump-search failures.
pub fn find_attestation_with_program(
    program: Address,
    credential: Address,
    schema: Address,
    nonce: Address,
) -> Result<ProgramDerivedAddress, PdaError> {
    find_program_address(
        &[
            ATTESTATION_SEED,
            credential.as_bytes(),
            schema.as_bytes(),
            nonce.as_bytes(),
        ],
        &program,
    )
}

macro_rules! account_pda {
    ($canonical:ident, $explicit:ident, $seed:ident) => {
        #[doc = concat!("Derives the canonical `", stringify!($seed), "` account.")]
        #[doc = "# Errors"]
        #[doc = "Preserves deterministic bump-search failures."]
        pub fn $canonical(account: Address) -> Result<ProgramDerivedAddress, PdaError> {
            $explicit(PROGRAM, account)
        }

        #[doc = concat!("Derives `", stringify!($seed), "` for an explicit program.")]
        #[doc = "# Errors"]
        #[doc = "Preserves deterministic bump-search failures."]
        pub fn $explicit(
            program: Address,
            account: Address,
        ) -> Result<ProgramDerivedAddress, PdaError> {
            find_program_address(&[$seed, account.as_bytes()], &program)
        }
    };
}

account_pda!(
    find_schema_mint,
    find_schema_mint_with_program,
    SCHEMA_MINT_SEED
);
account_pda!(
    find_attestation_mint,
    find_attestation_mint_with_program,
    ATTESTATION_MINT_SEED
);

/// Derives the canonical event authority.
/// # Errors
/// Preserves deterministic bump-search failures.
pub fn find_event_authority() -> Result<ProgramDerivedAddress, PdaError> {
    find_event_authority_with_program(PROGRAM)
}

/// Derives the event authority for an explicit program.
/// # Errors
/// Preserves deterministic bump-search failures.
pub fn find_event_authority_with_program(
    program: Address,
) -> Result<ProgramDerivedAddress, PdaError> {
    find_program_address(&[EVENT_AUTHORITY_SEED], &program)
}

/// Derives the canonical SAS signing authority.
/// # Errors
/// Preserves deterministic bump-search failures.
pub fn find_sas_authority() -> Result<ProgramDerivedAddress, PdaError> {
    find_sas_authority_with_program(PROGRAM)
}

/// Derives the SAS signing authority for an explicit program.
/// # Errors
/// Preserves deterministic bump-search failures.
pub fn find_sas_authority_with_program(
    program: Address,
) -> Result<ProgramDerivedAddress, PdaError> {
    find_program_address(&[SAS_SEED], &program)
}
