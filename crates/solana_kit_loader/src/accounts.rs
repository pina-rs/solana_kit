//! Source-defined loader headers, separate from opaque executable bytes.
use solana_kit_core::{
    Address,
    codecs::{Decoder, Encoder, Error, Number, Reader, Writer},
};

/// Reserved Buffer header size, including a one-byte authority option.
pub const BUFFER_HEADER_SIZE: usize = 37;
/// Reserved ProgramData header size, including slot and authority option.
pub const PROGRAM_DATA_HEADER_SIZE: usize = 45;
/// Loader v4 header size; executable bytes follow it.
pub const PROGRAM_STATE_HEADER_SIZE: usize = 48;

/// Decodes a fetched v3 header while preserving its full allocation metadata.
///
/// Executable bytes after the header are intentionally outside its typed value.
/// The caller retains ownership of the fetch and chooses its transport/client.
/// # Errors
/// Rejects a foreign owner, inconsistent allocation or malformed header.
pub fn decode_upgradeable_account(
    account: solana_kit_core::accounts::EncodedAccount,
    limits: solana_kit_core::codecs::Limits,
) -> Result<
    solana_kit_core::accounts::Account<UpgradeableState>,
    solana_kit_core::accounts::AccountError,
> {
    account.decode(&Header, crate::v3::PROGRAM, limits)
}

struct Header;

impl Decoder<UpgradeableState> for Header {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpgradeableState, Error> {
        let value = UpgradeableStateCodec.read(reader)?;
        reader.take(reader.remaining().len())?;
        Ok(value)
    }
}

/// The complete upgradeable-loader state header.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum UpgradeableState {
    /// Uninitialized account (tag zero).
    Uninitialized,
    /// Buffer authority, with fixed reserved address storage.
    Buffer {
        /// Optional write authority.
        authority: Option<Address>,
    },
    /// Executable account pointing at its ProgramData account.
    Program {
        /// ProgramData account address.
        program_data: Address,
    },
    /// ProgramData slot and optional upgrade authority.
    ProgramData {
        /// Last deployment slot.
        slot: u64,
        /// Optional upgrade authority.
        authority: Option<Address>,
    },
}

/// Strict source header codec; use prefix decoding before executable bytes.
#[derive(Clone, Copy, Debug)]
pub struct UpgradeableStateCodec;

impl Encoder<UpgradeableState> for UpgradeableStateCodec {
    fn encoded_len(&self, value: &UpgradeableState) -> Result<usize, Error> {
        Ok(match value {
            UpgradeableState::Uninitialized => 4,
            UpgradeableState::Buffer { .. } => BUFFER_HEADER_SIZE,
            UpgradeableState::Program { .. } => 36,
            UpgradeableState::ProgramData { .. } => PROGRAM_DATA_HEADER_SIZE,
        })
    }

    fn write(&self, value: &UpgradeableState, writer: &mut Writer<'_>) -> Result<(), Error> {
        match value {
            UpgradeableState::Uninitialized => Number::<u32>::LE.write(&0, writer),
            UpgradeableState::Buffer { authority } => {
                Number::<u32>::LE.write(&1, writer)?;
                write_authority(*authority, writer)
            }
            UpgradeableState::Program { program_data } => {
                Number::<u32>::LE.write(&2, writer)?;
                writer.put(program_data.as_bytes())
            }
            UpgradeableState::ProgramData { slot, authority } => {
                Number::<u32>::LE.write(&3, writer)?;
                Number::<u64>::LE.write(slot, writer)?;
                write_authority(*authority, writer)
            }
        }
    }
}

impl Decoder<UpgradeableState> for UpgradeableStateCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<UpgradeableState, Error> {
        reader.transaction(|reader| match Number::<u32>::LE.read(reader)? {
            0 => Ok(UpgradeableState::Uninitialized),
            1 => Ok(UpgradeableState::Buffer {
                authority: read_authority(reader)?,
            }),
            2 => Ok(UpgradeableState::Program {
                program_data: read_address(reader)?,
            }),
            3 => Ok(UpgradeableState::ProgramData {
                slot: Number::<u64>::LE.read(reader)?,
                authority: read_authority(reader)?,
            }),
            _ => Err(Error::InvalidDiscriminator),
        })
    }
}

fn read_address(reader: &mut Reader<'_>) -> Result<Address, Error> {
    let bytes: [u8; 32] = reader.take(32)?.try_into().map_err(|_| Error::Truncated)?;
    Ok(Address::from_bytes(bytes))
}

fn write_authority(authority: Option<Address>, writer: &mut Writer<'_>) -> Result<(), Error> {
    Number::<u8>::LE.write(&u8::from(authority.is_some()), writer)?;

    match authority {
        Some(address) => writer.put(address.as_bytes()),
        None => writer.fill(32, 0),
    }
}

fn read_authority(reader: &mut Reader<'_>) -> Result<Option<Address>, Error> {
    let tag = Number::<u8>::LE.read(reader)?;

    match tag {
        0 => {
            reader.take(32)?;
            Ok(None)
        }
        1 => Ok(Some(read_address(reader)?)),
        _ => Err(Error::InvalidDiscriminator),
    }
}

/// Loader v4 lifecycle status, encoded as an exact little-endian u64.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LoaderV4Status {
    /// Writable, undeployed state.
    Retracted,
    /// Deployed executable state.
    Deployed,
    /// Immutable executable state with the next-version address.
    Finalized,
}

/// Complete Loader v4 account header.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ProgramStateAccount {
    /// Last deployment slot.
    pub slot: u64,
    /// Upgrade authority or finalized next-version address.
    pub authority_address_or_next_version: Address,
    /// Program lifecycle status.
    pub status: LoaderV4Status,
}

impl core::fmt::Display for ProgramStateAccount {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}

impl core::fmt::Display for UpgradeableState {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, formatter)
    }
}

/// Exact 48-byte Loader v4 header codec.
#[derive(Clone, Copy, Debug)]
pub struct ProgramStateAccountCodec;

impl Encoder<ProgramStateAccount> for ProgramStateAccountCodec {
    fn encoded_len(&self, _: &ProgramStateAccount) -> Result<usize, Error> {
        Ok(PROGRAM_STATE_HEADER_SIZE)
    }

    fn write(&self, value: &ProgramStateAccount, writer: &mut Writer<'_>) -> Result<(), Error> {
        Number::<u64>::LE.write(&value.slot, writer)?;
        writer.put(value.authority_address_or_next_version.as_bytes())?;

        let status = match value.status {
            LoaderV4Status::Retracted => 0,
            LoaderV4Status::Deployed => 1,
            LoaderV4Status::Finalized => 2,
        };

        Number::<u64>::LE.write(&status, writer)
    }
}

impl Decoder<ProgramStateAccount> for ProgramStateAccountCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<ProgramStateAccount, Error> {
        reader.transaction(|reader| {
            let slot = Number::<u64>::LE.read(reader)?;
            let authority_address_or_next_version = read_address(reader)?;
            let status = match Number::<u64>::LE.read(reader)? {
                0 => LoaderV4Status::Retracted,
                1 => LoaderV4Status::Deployed,
                2 => LoaderV4Status::Finalized,
                _ => return Err(Error::InvalidDiscriminator),
            };

            Ok(ProgramStateAccount {
                slot,
                authority_address_or_next_version,
                status,
            })
        })
    }
}
