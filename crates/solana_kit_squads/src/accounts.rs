//! Header identity and typed decoding preserve the existing core account owner.
use crate::generated::*;
use solana_kit_core::{
    Address,
    accounts::{Account, AccountError, EncodedAccount},
    codecs::{Error, Limits},
};

/// Primary eight-byte account identity; identifying bytes alone does not validate a payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccountKind {
    /// A batch of vault transactions.
    Batch,
    /// One transaction within a vault batch.
    VaultBatchTransaction,
    /// Proposed multisig configuration actions.
    ConfigTransaction,
    /// Multisig settings and members.
    Multisig,
    /// Program-wide configuration.
    ProgramConfig,
    /// Voting and execution proposal state.
    Proposal,
    /// A scoped spending allowance.
    SpendingLimit,
    /// An incrementally assembled transaction message.
    TransactionBuffer,
    /// A vault transaction message.
    VaultTransaction,
}

/// Identifies the canonical account header without guessing from variable allocation size.
/// # Errors
/// Rejects truncated or unknown headers. Full payload validation belongs to the selected codec.
pub fn identify_account(bytes: &[u8]) -> Result<AccountKind, Error> {
    const HEADER_BYTES: usize = 8;
    let header: &[u8; 8] = bytes
        .get(..HEADER_BYTES)
        .ok_or(Error::Truncated)?
        .try_into()
        .map_err(|_| Error::Truncated)?;

    match header {
        [0x9c, 0xc2, 0x46, 0x2c, 0x16, 0x58, 0x89, 0x2c] => Ok(AccountKind::Batch),
        [0xc4, 0x79, 0x2e, 0x24, 0x0c, 0x13, 0xfc, 0x07] => Ok(AccountKind::VaultBatchTransaction),
        [0x5e, 0x08, 0x04, 0x23, 0x71, 0x8b, 0x8b, 0x70] => Ok(AccountKind::ConfigTransaction),
        [0xe0, 0x74, 0x79, 0xba, 0x44, 0xa1, 0x4f, 0xec] => Ok(AccountKind::Multisig),
        [0xc4, 0xd2, 0x5a, 0xe7, 0x90, 0x95, 0x8c, 0x3f] => Ok(AccountKind::ProgramConfig),
        [0x1a, 0x5e, 0xbd, 0xbb, 0x74, 0x88, 0x35, 0x21] => Ok(AccountKind::Proposal),
        [0x0a, 0xc9, 0x1b, 0xa0, 0xda, 0xc3, 0xde, 0x98] => Ok(AccountKind::SpendingLimit),
        [0x5a, 0x24, 0x23, 0xdb, 0x5d, 0xe1, 0x6e, 0x60] => Ok(AccountKind::TransactionBuffer),
        [0xa8, 0xfa, 0xa2, 0x64, 0x51, 0x0e, 0xa2, 0xcf] => Ok(AccountKind::VaultTransaction),
        _ => Err(Error::InvalidDiscriminator),
    }
}

macro_rules! decoder {
    ($name:ident, $with:ident, $value:ident, $codec:ident) => {
        #[doc = concat!(
            "Decodes a fetched `", stringify!($value), "` snapshot with its canonical owner."
        )]
        #[doc = "# Errors"]
        #[doc = "Preserves owner/allocation, complete-header, wire and budget errors and all metadata."]
        pub fn $name(
            account: EncodedAccount,
            limits: Limits,
        ) -> Result<Account<$value>, AccountError> {
            $with(account, PROGRAM, limits)
        }

        #[doc = concat!(
            "Decodes `", stringify!($value), "` against an explicit expected program owner."
        )]
        #[doc = "# Errors"]
        #[doc = "Preserves the existing core account validation and strict codec errors."]
        pub fn $with(account: EncodedAccount, program: Address, limits: Limits)
            -> Result<Account<$value>, AccountError>
        {
            account.decode(&$codec, program, limits)
        }
    };
}

decoder!(decode_batch, decode_batch_with_program, Batch, BatchCodec);
decoder!(
    decode_vault_batch_transaction,
    decode_vault_batch_transaction_with_program,
    VaultBatchTransaction,
    VaultBatchTransactionCodec
);
decoder!(
    decode_config_transaction,
    decode_config_transaction_with_program,
    ConfigTransaction,
    ConfigTransactionCodec
);
decoder!(
    decode_multisig,
    decode_multisig_with_program,
    Multisig,
    MultisigCodec
);
decoder!(
    decode_program_config,
    decode_program_config_with_program,
    ProgramConfig,
    ProgramConfigCodec
);
decoder!(
    decode_proposal,
    decode_proposal_with_program,
    Proposal,
    ProposalCodec
);
decoder!(
    decode_spending_limit,
    decode_spending_limit_with_program,
    SpendingLimit,
    SpendingLimitCodec
);
decoder!(
    decode_transaction_buffer,
    decode_transaction_buffer_with_program,
    TransactionBuffer,
    TransactionBufferCodec
);

decoder!(
    decode_vault_transaction,
    decode_vault_transaction_with_program,
    VaultTransaction,
    VaultTransactionCodec
);
