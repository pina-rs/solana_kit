//! Message-first version-one wire sharing the immutable transaction owners.

use alloc::{collections::BTreeSet, vec::Vec};

use crate::{Address, Hash, Signature};

use super::{
    CompiledInstruction, Error, Header, HeapSize, Lifetime, Message, Transaction, V1_PACKET_SIZE,
    V1Config, Version, wire,
};

use super::config::{COMPUTE_BIT, HEAP_BIT, LOADED_BIT, PRIORITY_MASK};

pub(super) const MAX_SIGNERS: u8 = 12;
pub(super) const MAX_ADDRESSES: usize = 64;
pub(super) const MAX_INSTRUCTIONS: usize = 64;

// Version-one's additional bounds and uniqueness are wire constraints, unlike
// local construction's readonly-program restriction.
pub(super) fn validate(message: &Message) -> Result<(), Error> {
    let unique: BTreeSet<_> = message.static_accounts.iter().collect();

    validate_counts(
        message.header.required_signatures,
        message.static_accounts.len(),
        message.instructions.len(),
    )?;

    if unique.len() != message.static_accounts.len() || !message.lookups.is_empty() {
        return Err(Error::InvalidV1Limits);
    }

    if message.instructions.iter().any(|instruction| {
        instruction.accounts.len() > usize::from(u8::MAX)
            || instruction.data.len() > usize::from(u16::MAX)
    }) {
        return Err(Error::InvalidLength);
    }

    Ok(())
}

pub(super) fn encode_message(message: &Message) -> Result<Vec<u8>, Error> {
    validate(message)?;
    let config = message.config.ok_or(Error::InvalidConfig)?;
    let mut bytes = alloc::vec![
        0x81,
        message.header.required_signatures,
        message.header.readonly_signers,
        message.header.readonly_unsigned
    ];
    bytes.extend(config.mask().to_le_bytes());
    bytes.extend(message.lifetime.token().as_bytes());
    bytes.push(u8::try_from(message.instructions.len()).map_err(|_| Error::InvalidLength)?);
    bytes.push(u8::try_from(message.static_accounts.len()).map_err(|_| Error::InvalidLength)?);

    for address in &message.static_accounts {
        bytes.extend(address.as_bytes());
    }

    config.encode(&mut bytes);

    for instruction in &message.instructions {
        bytes.extend([
            instruction.program_index,
            u8::try_from(instruction.accounts.len()).map_err(|_| Error::InvalidLength)?,
        ]);
        bytes.extend(
            u16::try_from(instruction.data.len())
                .map_err(|_| Error::InvalidLength)?
                .to_le_bytes(),
        );
    }

    for instruction in &message.instructions {
        bytes.extend(&instruction.accounts);
        bytes.extend(&instruction.data);
    }

    wire::packet_size(Version::V1, bytes.len(), message.signers().len())?;

    Ok(bytes)
}

pub(super) fn decode_prefix(bytes: &[u8]) -> Result<(Message, usize), Error> {
    if bytes.len() > V1_PACKET_SIZE {
        return Err(Error::PacketTooLarge);
    }

    let mut reader = Reader { bytes, offset: 0 };

    if reader.byte()? != 0x81 {
        return Err(Error::UnsupportedVersion);
    }

    let header = Header {
        required_signatures: reader.byte()?,
        readonly_signers: reader.byte()?,
        readonly_unsigned: reader.byte()?,
    };
    let mask = u32::from_le_bytes(reader.array()?);
    V1Config::validate_mask(mask)?;
    let lifetime = Lifetime::Unresolved(Hash::from_bytes(reader.array()?));
    let instruction_count = usize::from(reader.byte()?);
    let address_count = usize::from(reader.byte()?);

    validate_counts(header.required_signatures, address_count, instruction_count)?;

    let mut static_accounts = Vec::new();

    for _ in 0..address_count {
        static_accounts.push(Address::from_bytes(reader.array()?));
    }

    let config = reader.config(mask)?;
    let instructions = reader.instructions(instruction_count)?;
    let message = Message {
        version: Version::V1,
        header,
        static_accounts,
        instructions,
        lookups: Vec::new(),
        lifetime,
        config: Some(config),
        bytes: bytes[..reader.offset].to_vec(),
    };
    wire::validate(&message)?;

    Ok((message, reader.offset))
}

pub(super) fn decode_transaction(bytes: &[u8]) -> Result<Transaction, Error> {
    let (message, offset) = decode_prefix(bytes)?;
    let mut reader = Reader { bytes, offset };
    let mut signatures = Vec::new();

    for _ in message.signers() {
        let value = reader.array()?;
        signatures.push((value != [0; 64]).then(|| Signature::from_bytes(value)));
    }

    if reader.offset != bytes.len() {
        return Err(Error::TrailingBytes);
    }

    Ok(Transaction {
        message,
        signatures,
    })
}

struct Reader<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, count: usize) -> Result<&'a [u8], Error> {
        let end = self.offset.checked_add(count).ok_or(Error::InvalidLength)?;
        let value = self.bytes.get(self.offset..end).ok_or(Error::Truncated)?;
        self.offset = end;
        Ok(value)
    }

    fn array<const N: usize>(&mut self) -> Result<[u8; N], Error> {
        self.take(N)?.try_into().map_err(|_| Error::Truncated)
    }

    fn byte(&mut self) -> Result<u8, Error> {
        Ok(self.array::<1>()?[0])
    }

    fn config(&mut self, mask: u32) -> Result<V1Config, Error> {
        let mut config = V1Config::default();

        if mask & PRIORITY_MASK != 0 {
            config = config.with_priority_fee(u64::from_le_bytes(self.array()?));
        }

        if mask & COMPUTE_BIT != 0 {
            config = config.with_compute_unit_limit(u32::from_le_bytes(self.array()?));
        }

        if mask & LOADED_BIT != 0 {
            config = config.with_loaded_accounts_data_limit(u32::from_le_bytes(self.array()?));
        }

        if mask & HEAP_BIT != 0 {
            config = config.with_heap_size(HeapSize::new(u32::from_le_bytes(self.array()?))?);
        }

        Ok(config)
    }

    fn instructions(&mut self, count: usize) -> Result<Vec<CompiledInstruction>, Error> {
        let mut headers = Vec::new();

        for _ in 0..count {
            headers.push(self.array::<4>()?);
        }

        let mut instructions = Vec::new();

        for [program_index, accounts, low, high] in headers {
            instructions.push(CompiledInstruction {
                program_index,
                accounts: self.take(usize::from(accounts))?.to_vec(),
                data: self
                    .take(usize::from(u16::from_le_bytes([low, high])))?
                    .to_vec(),
            });
        }

        Ok(instructions)
    }
}

fn validate_counts(signers: u8, addresses: usize, instructions: usize) -> Result<(), Error> {
    if signers > MAX_SIGNERS || addresses > MAX_ADDRESSES || instructions > MAX_INSTRUCTIONS {
        return Err(Error::InvalidV1Limits);
    }

    Ok(())
}
