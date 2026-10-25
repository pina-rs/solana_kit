//! Five bincode-compatible instruction data variants and positional account contracts.
use alloc::{vec, vec::Vec};
use solana_kit_core::{
    AccountMeta, AccountRole, Address, Instruction,
    codecs::{Decoder, Encoder, Error, Limits, Reader, Writer},
};

/// All five known instruction data variants; unknown tags fail decoding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InstructionData {
    /// Create using the explicit slot and bump encoded in the instruction.
    Create {
        /// Recent slot in little-endian order.
        recent_slot: u64,
        /// PDA bump.
        bump: u8,
    },
    /// Permanently remove the authority.
    Freeze,
    /// Append the supplied addresses, with a u64 element count.
    Extend(Vec<Address>),
    /// Begin deactivation.
    Deactivate,
    /// Close and return lamports to the recipient.
    Close,
}

/// Strict four-byte discriminator and complete-frame data codec.
#[derive(Clone, Copy, Debug)]
pub struct InstructionDataCodec;

impl Encoder<InstructionData> for InstructionDataCodec {
    fn encoded_len(&self, value: &InstructionData) -> Result<usize, Error> {
        use crate::generated::*;

        match value {
            InstructionData::Create { recent_slot, bump } => {
                CreateLookupTableCodec.encoded_len(&CreateLookupTable {
                    recent_slot: *recent_slot,
                    bump: *bump,
                })
            }
            InstructionData::Freeze => FreezeLookupTableCodec.encoded_len(&FreezeLookupTable {}),
            InstructionData::Extend(addresses) => {
                ExtendLookupTableCodec.encoded_len(&ExtendLookupTable {
                    addresses: copy_addresses(addresses)?,
                })
            }
            InstructionData::Deactivate => {
                DeactivateLookupTableCodec.encoded_len(&DeactivateLookupTable {})
            }
            InstructionData::Close => CloseLookupTableCodec.encoded_len(&CloseLookupTable {}),
        }
    }
    fn write(&self, value: &InstructionData, writer: &mut Writer<'_>) -> Result<(), Error> {
        use crate::generated::*;

        match value {
            InstructionData::Create { recent_slot, bump } => CreateLookupTableCodec.write(
                &CreateLookupTable {
                    recent_slot: *recent_slot,
                    bump: *bump,
                },
                writer,
            ),
            InstructionData::Freeze => FreezeLookupTableCodec.write(&FreezeLookupTable {}, writer),
            InstructionData::Extend(addresses) => ExtendLookupTableCodec.write(
                &ExtendLookupTable {
                    addresses: copy_addresses(addresses)?,
                },
                writer,
            ),
            InstructionData::Deactivate => {
                DeactivateLookupTableCodec.write(&DeactivateLookupTable {}, writer)
            }
            InstructionData::Close => CloseLookupTableCodec.write(&CloseLookupTable {}, writer),
        }
    }
}
fn copy_addresses(addresses: &[Address]) -> Result<Vec<Address>, Error> {
    let mut output = Vec::new();
    output
        .try_reserve_exact(addresses.len())
        .map_err(|_| Error::AllocationFailed)?;
    output.extend_from_slice(addresses);
    Ok(output)
}
impl Decoder<InstructionData> for InstructionDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<InstructionData, Error> {
        use crate::generated::*;
        let bytes: [u8; 4] = reader
            .remaining()
            .get(..4)
            .ok_or(Error::Truncated)?
            .try_into()
            .map_err(|_| Error::Truncated)?;

        match u32::from_le_bytes(bytes) {
            0 => {
                let value = CreateLookupTableCodec.read(reader)?;
                Ok(InstructionData::Create {
                    recent_slot: value.recent_slot,
                    bump: value.bump,
                })
            }
            1 => {
                FreezeLookupTableCodec.read(reader)?;
                Ok(InstructionData::Freeze)
            }
            2 => Ok(InstructionData::Extend(
                ExtendLookupTableCodec.read(reader)?.addresses,
            )),
            3 => {
                DeactivateLookupTableCodec.read(reader)?;
                Ok(InstructionData::Deactivate)
            }
            4 => {
                CloseLookupTableCodec.read(reader)?;
                Ok(InstructionData::Close)
            }
            _ => Err(Error::InvalidDiscriminator),
        }
    }
}

/// Optional Extend funding is one atomic pair, matching the pinned Rust constructor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Funding {
    /// Writable signer supplying rent.
    pub payer: Address,
    /// System program used for the transfer.
    pub system_program: Address,
}

/// Typed operation inputs, with no invalid combinations of data and accounts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    /// Table, authority, funding payer, system program, recent slot and bump.
    Create {
        /// Writable table account.
        table: Address,
        /// Readonly authority; creation does not require signing.
        authority: Address,
        /// Required payer and System account.
        funding: Funding,
        /// PDA slot.
        recent_slot: u64,
        /// PDA bump.
        bump: u8,
    },
    /// Writable table and readonly signer authority.
    Freeze {
        /// Table account.
        table: Address,
        /// Signer authority.
        authority: Address,
    },
    /// Writable table, signer authority, optional funding pair and new addresses.
    Extend {
        /// Table account.
        table: Address,
        /// Signer authority.
        authority: Address,
        /// Both funding accounts or neither.
        funding: Option<Funding>,
        /// Appended addresses.
        addresses: Vec<Address>,
    },
    /// Writable table and signer authority.
    Deactivate {
        /// Table account.
        table: Address,
        /// Signer authority.
        authority: Address,
    },
    /// Writable table, signer authority and writable recipient.
    Close {
        /// Table account.
        table: Address,
        /// Signer authority.
        authority: Address,
        /// Lamport recipient.
        recipient: Address,
    },
}

fn meta(address: Address, role: AccountRole) -> AccountMeta {
    AccountMeta { address, role }
}

impl Operation {
    /// Builds an instruction for the explicit canonical or custom program identity.
    ///
    /// # Errors
    /// Rejects caller byte/item limits or allocation failure.
    pub fn build(&self, program: Address, limits: Limits) -> Result<Instruction, Error> {
        let (data, accounts) = self.parts(limits)?;

        if accounts.len() > limits.max_items {
            return Err(Error::LimitExceeded);
        }

        if let InstructionData::Extend(addresses) = &data
            && addresses.len() > limits.max_items - accounts.len()
        {
            return Err(Error::LimitExceeded);
        }

        Ok(Instruction {
            program_address: program,
            accounts,
            data: InstructionDataCodec.encode(&data, limits.max_bytes)?,
        })
    }

    fn parts(&self, limits: Limits) -> Result<(InstructionData, Vec<AccountMeta>), Error> {
        let parts = match self {
            Self::Create {
                table,
                authority,
                funding,
                recent_slot,
                bump,
            } => (
                InstructionData::Create {
                    recent_slot: *recent_slot,
                    bump: *bump,
                },
                vec![
                    meta(*table, AccountRole::Writable),
                    meta(*authority, AccountRole::Readonly),
                    meta(funding.payer, AccountRole::WritableSigner),
                    meta(funding.system_program, AccountRole::Readonly),
                ],
            ),
            Self::Freeze { table, authority } => {
                (InstructionData::Freeze, base(*table, *authority))
            }
            Self::Deactivate { table, authority } => {
                (InstructionData::Deactivate, base(*table, *authority))
            }
            Self::Close {
                table,
                authority,
                recipient,
            } => (
                InstructionData::Close,
                vec![
                    meta(*table, AccountRole::Writable),
                    meta(*authority, AccountRole::ReadonlySigner),
                    meta(*recipient, AccountRole::Writable),
                ],
            ),
            Self::Extend {
                table,
                authority,
                funding,
                addresses,
            } => return extend_parts(*table, *authority, *funding, addresses, limits),
        };

        Ok(parts)
    }
}

fn extend_parts(
    table: Address,
    authority: Address,
    funding: Option<Funding>,
    addresses: &[Address],
    limits: Limits,
) -> Result<(InstructionData, Vec<AccountMeta>), Error> {
    if addresses.len() > limits.max_items {
        return Err(Error::LimitExceeded);
    }

    let mut copied = Vec::new();
    copied
        .try_reserve_exact(addresses.len())
        .map_err(|_| Error::AllocationFailed)?;
    copied.extend_from_slice(addresses);
    let mut accounts = base(table, authority);

    if let Some(funding) = funding {
        accounts.extend([
            meta(funding.payer, AccountRole::WritableSigner),
            meta(funding.system_program, AccountRole::Readonly),
        ]);
    }

    Ok((InstructionData::Extend(copied), accounts))
}

fn base(table: Address, authority: Address) -> Vec<AccountMeta> {
    vec![
        meta(table, AccountRole::Writable),
        meta(authority, AccountRole::ReadonlySigner),
    ]
}

/// Parses a full instruction, checking program, positional privileges and exact accounts.
/// Stronger privileges and duplicate account addresses remain valid.
///
/// # Errors
/// Rejects foreign identity, malformed data, invalid roles/counts and caller limits.
pub fn parse_instruction(
    instruction: &Instruction,
    program: Address,
    limits: Limits,
) -> Result<InstructionData, Error> {
    if instruction.program_address != program || instruction.accounts.len() > limits.max_items {
        return Err(Error::InvalidValue);
    }

    let data = InstructionDataCodec.decode(
        &instruction.data,
        Limits::new(
            limits.max_bytes,
            limits.max_items - instruction.accounts.len(),
        ),
    )?;

    let roles: &[AccountRole] = match &data {
        InstructionData::Create { .. } => &[
            AccountRole::Writable,
            AccountRole::Readonly,
            AccountRole::WritableSigner,
            AccountRole::Readonly,
        ],
        InstructionData::Extend(_) if instruction.accounts.len() == 4 => &[
            AccountRole::Writable,
            AccountRole::ReadonlySigner,
            AccountRole::WritableSigner,
            AccountRole::Readonly,
        ],
        InstructionData::Close => &[
            AccountRole::Writable,
            AccountRole::ReadonlySigner,
            AccountRole::Writable,
        ],
        _ => &[AccountRole::Writable, AccountRole::ReadonlySigner],
    };

    if roles.len() != instruction.accounts.len()
        || instruction
            .accounts
            .iter()
            .zip(roles)
            .any(|(meta, required)| {
                (required.is_signer() && !meta.role.is_signer())
                    || (required.is_writable() && !meta.role.is_writable())
            })
    {
        return Err(Error::InvalidValue);
    }

    Ok(data)
}
