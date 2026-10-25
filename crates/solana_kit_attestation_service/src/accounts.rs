//! Already fetched accounts retain core ownership and strict full-frame decoding.
use crate::generated::*;
use solana_kit_core::{
    Address,
    accounts::{Account, AccountError, EncodedAccount},
    codecs::{Error, Limits},
};

/// The three allocated account kinds at the pinned revision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccountKind {
    /// Issuer identity and authorized signers.
    Credential,
    /// Schema field layout and status.
    Schema,
    /// Opaque attestation data and signed expiry.
    Attestation,
}

/// Identifies the account header without validating its payload.
/// # Errors
/// Rejects empty input and unknown discriminants.
pub fn identify_account(bytes: &[u8]) -> Result<AccountKind, Error> {
    match bytes.first() {
        Some(0) => Ok(AccountKind::Credential),
        Some(1) => Ok(AccountKind::Schema),
        Some(2) => Ok(AccountKind::Attestation),
        None => Err(Error::Truncated),
        _ => Err(Error::InvalidDiscriminator),
    }
}

macro_rules! account_decoder {
    ($function:ident, $explicit:ident, $value:ident, $codec:ident) => {
        #[doc = concat!("Decodes a fetched `", stringify!($value), "` with the canonical owner.")]
        #[doc = "# Errors"]
        #[doc = "Preserves exact owner, allocation, header, wire and caller-limit failures."]
        pub fn $function(
            account: EncodedAccount,
            limits: Limits,
        ) -> Result<Account<$value>, AccountError> {
            $explicit(account, PROGRAM, limits)
        }

        #[doc = concat!("Decodes a fetched `", stringify!($value), "` with an explicit owner.")]
        #[doc = "# Errors"]
        #[doc = "Preserves exact owner, allocation, header, wire and caller-limit failures."]
        pub fn $explicit(
            account: EncodedAccount,
            program: Address,
            limits: Limits,
        ) -> Result<Account<$value>, AccountError> {
            account.decode(&$codec, program, limits)
        }
    };
}

account_decoder!(
    decode_credential,
    decode_credential_with_program,
    Credential,
    CredentialCodec
);
account_decoder!(
    decode_schema,
    decode_schema_with_program,
    Schema,
    SchemaCodec
);
account_decoder!(
    decode_attestation,
    decode_attestation_with_program,
    Attestation,
    AttestationCodec
);
