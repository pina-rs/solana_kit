// Generated from fixtures/system.json; see SYSTEM-LICENSE.
use solana_kit_core::Address;

#[derive(Debug)]
pub(crate) struct CreateAccount {
    pub lamports: u64,
    pub space: u64,
    pub program_address: Address,
}

pub(crate) fn create_account(args: &CreateAccount) -> Result<Vec<u8>, core::num::TryFromIntError> {
    let mut data = 0_u32.to_le_bytes().to_vec();
    data.extend_from_slice(&args.lamports.to_le_bytes());
    data.extend_from_slice(&args.space.to_le_bytes());
    data.extend_from_slice(args.program_address.as_bytes());
    Ok(data)
}

#[derive(Debug)]
pub(crate) struct CreateAccountWithSeed {
    pub base: Address,
    pub seed: String,
    pub amount: u64,
    pub space: u64,
    pub program_address: Address,
}

pub(crate) fn create_account_with_seed(
    args: &CreateAccountWithSeed,
) -> Result<Vec<u8>, core::num::TryFromIntError> {
    let mut data = 3_u32.to_le_bytes().to_vec();
    data.extend_from_slice(args.base.as_bytes());
    data.extend_from_slice(&u64::try_from(args.seed.len())?.to_le_bytes());
    data.extend_from_slice(args.seed.as_bytes());
    data.extend_from_slice(&args.amount.to_le_bytes());
    data.extend_from_slice(&args.space.to_le_bytes());
    data.extend_from_slice(args.program_address.as_bytes());
    Ok(data)
}
