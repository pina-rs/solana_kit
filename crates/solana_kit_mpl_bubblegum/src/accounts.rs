//! Fetched snapshots retain core owner/allocation checks and strict data discriminators.
use crate::{PROGRAM, TreeConfig, TreeConfigCodec, Voucher, VoucherCodec};
use solana_kit_core::{
    Address,
    accounts::{Account, AccountError, EncodedAccount},
    codecs::{Decoder, Error, Limits, Reader},
};

/// Tree configuration's serialized prefix, including its discriminator.
pub const TREE_CONFIG_SERIALIZED_SIZE: usize = 91;
/// Allocated tree configuration, including the program's five reserved bytes.
pub const TREE_CONFIG_ACCOUNT_SIZE: usize = 96;

struct AllocatedTreeConfig;

impl Decoder<TreeConfig> for AllocatedTreeConfig {
    fn read(&self, reader: &mut Reader<'_>) -> Result<TreeConfig, Error> {
        let value = TreeConfigCodec.read(reader)?;

        match reader.remaining() {
            [] => {}
            [0, 0, 0, 0, 0] => {
                reader.take(5)?;
            }
            _ => return Err(Error::TrailingBytes),
        }

        Ok(value)
    }
}

/// Decodes a canonical owned tree snapshot with exact reserved-byte policy.
/// # Errors
/// Preserves wrong-owner, allocation, discriminator, enum, bool and budget errors.
pub fn decode_tree_config(
    account: EncodedAccount,
    limits: Limits,
) -> Result<Account<TreeConfig>, AccountError> {
    decode_tree_config_with_program(account, PROGRAM, limits)
}

/// Decodes the same account for an explicit deployment.
/// # Errors
/// Preserves the core account validation and exact data failure.
pub fn decode_tree_config_with_program(
    account: EncodedAccount,
    program: Address,
    limits: Limits,
) -> Result<Account<TreeConfig>, AccountError> {
    account.decode(&AllocatedTreeConfig, program, limits)
}

/// Decodes a complete V1 or V2 voucher, including every V2 hash commitment.
/// # Errors
/// Preserves wrong-owner, allocation, discriminator, branch and budget errors.
pub fn decode_voucher(
    account: EncodedAccount,
    limits: Limits,
) -> Result<Account<Voucher>, AccountError> {
    decode_voucher_with_program(account, PROGRAM, limits)
}

/// Decodes a complete voucher for an explicit deployment.
/// # Errors
/// Preserves the core account validation and exact data failure.
pub fn decode_voucher_with_program(
    account: EncodedAccount,
    program: Address,
    limits: Limits,
) -> Result<Account<Voucher>, AccountError> {
    account.decode(&VoucherCodec, program, limits)
}
