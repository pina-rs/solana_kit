#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

extern crate alloc;

/// Data codecs emitted from the exact pinned discriminator-free Config input.
pub mod generated;

use alloc::vec::Vec;
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Bool, Bytes, Decoder, Encoder, Error, FixedBytes, Limits, Reader, ShortU16, Writer},
};

/// Canonical native Config program identity.
pub const PROGRAM: Address = Address::from_bytes([
    3, 6, 74, 163, 0, 47, 116, 220, 200, 110, 67, 49, 15, 12, 5, 42, 248, 197, 218, 39, 246, 16,
    64, 25, 163, 35, 239, 160, 0, 0, 0, 0,
]);

/// A key and its signature requirement for subsequent stores.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ConfigKey {
    /// Stored address; order and duplicate positions are retained.
    pub address: Address,
    /// Whether the stored key must sign subsequent updates.
    pub is_signer: bool,
}

/// Config account data and Store payload share exactly the same wire contract.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ConfigData {
    /// Compact-counted key vector.
    pub keys: Vec<ConfigKey>,
    /// Opaque remaining data, without a prefix or discriminator.
    pub data: Vec<u8>,
}

impl core::fmt::Display for ConfigData {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}

/// Strict shortU16 key vector followed by an opaque byte tail.
#[derive(Clone, Copy, Debug)]
pub struct ConfigCodec;

impl Encoder<ConfigData> for ConfigCodec {
    fn encoded_len(&self, value: &ConfigData) -> Result<usize, Error> {
        let count = u16::try_from(value.keys.len()).map_err(|_| Error::Overflow)?;
        let keys_size = value.keys.len().checked_mul(33).ok_or(Error::Overflow)?;

        ShortU16
            .encoded_len(&count)?
            .checked_add(keys_size)
            .and_then(|size| size.checked_add(value.data.len()))
            .ok_or(Error::Overflow)
    }
    fn write(&self, value: &ConfigData, writer: &mut Writer<'_>) -> Result<(), Error> {
        let count = u16::try_from(value.keys.len()).map_err(|_| Error::Overflow)?;
        ShortU16.write(&count, writer)?;

        for key in &value.keys {
            FixedBytes::<32>.write(key.address.as_bytes(), writer)?;
            Bool.write(&key.is_signer, writer)?;
        }

        Bytes.write(&value.data, writer)
    }
}
impl Decoder<ConfigData> for ConfigCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ConfigData, Error> {
        let value = generated::ConfigCodec.read(reader)?;
        let mut keys = Vec::new();
        keys.try_reserve_exact(value.keys.value.len())
            .map_err(|_| Error::AllocationFailed)?;
        keys.extend(
            value
                .keys
                .value
                .into_iter()
                .map(|(address, is_signer)| ConfigKey { address, is_signer }),
        );
        Ok(ConfigData {
            keys,
            data: value.data,
        })
    }
}

/// Exact Store account inputs, independent of the stored key vector.
#[derive(Clone, Copy, Debug)]
pub struct StoreAccounts<'a> {
    /// Writable Config account.
    pub config: Address,
    /// Required when initializing or when the previous data has no signers.
    pub config_is_signer: bool,
    /// Ordered readonly signers, including intentional duplicates.
    pub signers: &'a [Address],
}

/// Builds a Store for an explicit program, retaining signer order and duplicates.
///
/// # Errors
/// Rejects counts beyond shortU16, allocation failure, or caller byte/item limits.
pub fn store(
    program: Address,
    accounts: StoreAccounts<'_>,
    value: &ConfigData,
    limits: Limits,
) -> Result<Instruction, Error> {
    store_with_signers(
        program,
        AccountMeta {
            address: accounts.config,
            role: AccountRole::from_privileges(accounts.config_is_signer, true),
        },
        accounts.signers.iter().copied(),
        value,
        limits,
    )
}

fn store_with_signers(
    program: Address,
    config: AccountMeta,
    signers: impl Iterator<Item = Address> + Clone,
    value: &ConfigData,
    limits: Limits,
) -> Result<Instruction, Error> {
    let count = signers
        .clone()
        .count()
        .checked_add(1)
        .ok_or(Error::Overflow)?;

    if value
        .keys
        .len()
        .checked_add(value.data.len())
        .and_then(|items| items.checked_add(count))
        .ok_or(Error::Overflow)?
        > limits.max_items
    {
        return Err(Error::LimitExceeded);
    }

    let data = ConfigCodec.encode(value, limits.max_bytes)?;
    let mut metas = Vec::new();
    metas
        .try_reserve_exact(count)
        .map_err(|_| Error::AllocationFailed)?;
    metas.push(config);
    metas.extend(signers.map(|address| AccountMeta {
        address,
        role: AccountRole::ReadonlySigner,
    }));
    Ok(Instruction {
        program_address: program,
        accounts: metas,
        data,
    })
}

/// Derives the ordered signer list exactly as the original handwritten Dart helper.
/// The Config account itself is retained if present among signer keys.
///
/// # Errors
/// Rejects caller limits, unrepresentable counts, or allocation failure.
pub fn store_config(
    program: Address,
    config: Address,
    config_is_signer: bool,
    value: &ConfigData,
    limits: Limits,
) -> Result<Instruction, Error> {
    if value.keys.len() > limits.max_items {
        return Err(Error::LimitExceeded);
    }

    store_with_signers(
        program,
        AccountMeta {
            address: config,
            role: AccountRole::from_privileges(config_is_signer, true),
        },
        value
            .keys
            .iter()
            .filter(|key| key.is_signer)
            .map(|key| key.address),
        value,
        limits,
    )
}

/// Decodes Store data and validates program identity and required account roles.
/// Extra writable privileges on signer accounts are accepted.
///
/// # Errors
/// Rejects a foreign program, absent Config account, invalid roles or malformed data.
pub fn parse_store(
    instruction: &Instruction,
    program: Address,
    limits: Limits,
) -> Result<ConfigData, Error> {
    let first = instruction.accounts.first().ok_or(Error::InvalidValue)?;

    if instruction.program_address != program
        || !first.role.is_writable()
        || instruction.accounts[1..]
            .iter()
            .any(|meta| !meta.role.is_signer())
    {
        return Err(Error::InvalidValue);
    }

    if instruction.accounts.len() > limits.max_items {
        return Err(Error::LimitExceeded);
    }

    ConfigCodec.decode(
        &instruction.data,
        Limits::new(
            limits.max_bytes,
            limits.max_items - instruction.accounts.len(),
        ),
    )
}

/// Decodes an already fetched Config account without dropping allocation metadata.
/// # Errors
/// Preserves core owner, allocation and codec errors.
pub fn decode_account(
    account: solana_kit_core::accounts::EncodedAccount,
    limits: Limits,
) -> Result<solana_kit_core::accounts::Account<ConfigData>, solana_kit_core::accounts::AccountError>
{
    account.decode(&ConfigCodec, PROGRAM, limits)
}
