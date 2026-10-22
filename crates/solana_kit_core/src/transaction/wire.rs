//! Private bounded legacy/v0 wire operations using the shared compact codec.

use alloc::vec::Vec;

use crate::codecs::{self, Decoder, Encoder, Limits, ShortU16, Writer};
use crate::{Address, Hash, Signature};

use super::{
    CompiledInstruction, Error, Header, Lifetime, Lookup, Message, PACKET_SIZE, Transaction,
    V1_PACKET_SIZE, Version, v1,
};

const VERSION_MASK: u8 = 128;
const SHORT_MASK: u8 = VERSION_MASK - 1;
const MAX_ACCOUNTS: usize = 256;
const SIGNATURE_SIZE: usize = 64;

// Bound the entire input first, then parse only complete fields. Validation is
// deliberately separate from the local compiler's privilege restrictions.
pub(super) fn decode(bytes: &[u8]) -> Result<Message, Error> {
    if bytes.first() == Some(&0x81) {
        let (message, offset) = v1::decode_prefix(bytes)?;

        if offset != bytes.len() {
            return Err(Error::TrailingBytes);
        }

        return Ok(message);
    }

    if bytes.len() > PACKET_SIZE {
        return Err(Error::PacketTooLarge);
    }

    let mut reader = Reader { bytes, offset: 0 };
    let (version, header) = reader.header()?;
    let count = reader.length()?;

    if count > MAX_ACCOUNTS {
        return Err(Error::InvalidIndex);
    }

    let mut static_accounts = Vec::new();

    for _ in 0..count {
        static_accounts.push(Address::from_bytes(reader.array()?));
    }

    let lifetime = Lifetime::Unresolved(Hash::from_bytes(reader.array()?));
    let instructions = reader.instructions()?;
    let lookups = if version == Version::V0 {
        reader.lookups()?
    } else {
        Vec::new()
    };

    if reader.offset != bytes.len() {
        return Err(Error::TrailingBytes);
    }

    let message = Message {
        version,
        header,
        static_accounts,
        instructions,
        lookups,
        lifetime,
        config: None,
        bytes: bytes.to_vec(),
    };
    validate(&message)?;

    Ok(message)
}

pub(super) fn validate(message: &Message) -> Result<(), Error> {
    validate_header(message)?;
    let static_count = message.static_accounts.len();
    let total = total_accounts(message)?;

    for instruction in &message.instructions {
        validate_instruction(instruction, static_count, total)?;
    }

    if message.version == Version::V1 {
        v1::validate(message)?;
    }

    packet_size(
        message.version,
        message.bytes.len(),
        message.signers().len(),
    )?;

    Ok(())
}

// The payer must be writable and signed; unsigned readonly accounts cannot
// overlap the initial signer partition. These are wire protocol constraints.
fn validate_header(message: &Message) -> Result<(), Error> {
    let header = message.header;
    let required = usize::from(header.required_signatures);
    let static_count = message.static_accounts.len();

    if required == 0
        || usize::from(header.readonly_signers) >= required
        || required + usize::from(header.readonly_unsigned) > static_count
        || (message.version == Version::Legacy && required >= usize::from(VERSION_MASK))
    {
        return Err(Error::InvalidHeader);
    }

    Ok(())
}

fn total_accounts(message: &Message) -> Result<usize, Error> {
    let mut total = message.static_accounts.len();

    for lookup in &message.lookups {
        let loaded = lookup.writable.len() + lookup.readonly.len();

        if loaded == 0 {
            return Err(Error::InvalidLookup);
        }

        total += loaded;
    }

    if total > MAX_ACCOUNTS {
        return Err(Error::InvalidIndex);
    }

    Ok(total)
}

fn validate_instruction(
    instruction: &CompiledInstruction,
    static_count: usize,
    total: usize,
) -> Result<(), Error> {
    if instruction.program_index == 0
        || usize::from(instruction.program_index) >= static_count
        || instruction
            .accounts
            .iter()
            .any(|index| usize::from(*index) >= total)
    {
        return Err(Error::InvalidIndex);
    }

    Ok(())
}

// Legacy omits the version byte and lookup section; v0 includes both even when
// there are no lookups. Every variable-length field uses canonical short-u16.
pub(super) fn encode_message(message: &Message) -> Result<Vec<u8>, Error> {
    if message.version == Version::V1 {
        return v1::encode_message(message);
    }

    // Version-dependent sections are necessary because legacy omits both the
    // version prefix and lookup count, while v0 always carries them.
    let mut bytes = Vec::new();

    if message.version == Version::V0 {
        bytes.push(VERSION_MASK);
    }

    bytes.extend([
        message.header.required_signatures,
        message.header.readonly_signers,
        message.header.readonly_unsigned,
    ]);
    length(&mut bytes, message.static_accounts.len())?;

    for address in &message.static_accounts {
        bytes.extend(address.as_bytes());
    }

    bytes.extend(message.lifetime.token().as_bytes());
    length(&mut bytes, message.instructions.len())?;

    for instruction in &message.instructions {
        bytes.push(instruction.program_index);
        sequence(&mut bytes, &instruction.accounts)?;
        sequence(&mut bytes, &instruction.data)?;
    }

    if message.version == Version::V0 {
        length(&mut bytes, message.lookups.len())?;

        for lookup in &message.lookups {
            bytes.extend(lookup.address.as_bytes());
            sequence(&mut bytes, &lookup.writable)?;
            sequence(&mut bytes, &lookup.readonly)?;
        }
    }

    packet_size(message.version, bytes.len(), message.signers().len())?;

    Ok(bytes)
}

pub(super) fn decode_transaction(bytes: &[u8]) -> Result<Transaction, Error> {
    if bytes.first() == Some(&0x81) {
        return v1::decode_transaction(bytes);
    }

    if bytes.len() > PACKET_SIZE {
        return Err(Error::PacketTooLarge);
    }

    let mut reader = Reader { bytes, offset: 0 };
    let count = reader.length()?;

    if count > PACKET_SIZE / SIGNATURE_SIZE {
        return Err(Error::InvalidLength);
    }

    let mut signatures = Vec::new();

    for _ in 0..count {
        let bytes = reader.array()?;
        signatures.push((bytes != [0; 64]).then(|| Signature::from_bytes(bytes)));
    }

    let message = decode(reader.take(bytes.len() - reader.offset)?)?;

    if message.signers().len() != signatures.len() {
        return Err(Error::InvalidHeader);
    }

    Ok(Transaction {
        message,
        signatures,
    })
}

pub(super) fn encode_transaction(
    transaction: &Transaction,
    require_all: bool,
) -> Result<Vec<u8>, Error> {
    packet_size(
        transaction.message.version,
        transaction.message.bytes.len(),
        transaction.signatures.len(),
    )?;
    let mut bytes = Vec::new();
    if transaction.message.version == Version::V1 {
        bytes.extend(&transaction.message.bytes);
    } else {
        length(&mut bytes, transaction.signatures.len())?;
    }

    for signature in &transaction.signatures {
        if require_all && signature.is_none() {
            return Err(Error::MissingSignature);
        }

        bytes.extend(signature.map_or([0; 64], Signature::to_bytes));
    }

    if transaction.message.version != Version::V1 {
        bytes.extend(&transaction.message.bytes);
    }

    Ok(bytes)
}

pub(super) fn packet_limit(version: Version) -> usize {
    if version == Version::V1 {
        V1_PACKET_SIZE
    } else {
        PACKET_SIZE
    }
}

pub(super) fn packet_size(
    version: Version,
    message: usize,
    signatures: usize,
) -> Result<(), Error> {
    let count = u16::try_from(signatures).map_err(|_| Error::InvalidLength)?;
    let prefix = if version == Version::V1 {
        0
    } else {
        ShortU16.encoded_len(&count).map_err(length_error)?
    };

    if message + signatures * SIGNATURE_SIZE + prefix > packet_limit(version) {
        return Err(Error::PacketTooLarge);
    }

    Ok(())
}

fn sequence(bytes: &mut Vec<u8>, value: &[u8]) -> Result<(), Error> {
    length(bytes, value.len())?;
    bytes.extend(value);

    if bytes.len() > PACKET_SIZE {
        return Err(Error::PacketTooLarge);
    }

    Ok(())
}

// A stack buffer avoids allocating a separate vector for each compact count.
fn length(bytes: &mut Vec<u8>, value: usize) -> Result<(), Error> {
    let value = u16::try_from(value).map_err(|_| Error::InvalidLength)?;
    let mut buffer = [0; 3];
    let mut writer = Writer::new(&mut buffer);
    ShortU16.write(&value, &mut writer).map_err(length_error)?;
    let count = writer.position();
    bytes.extend_from_slice(&buffer[..count]);

    Ok(())
}

// Preserve the transaction error contract while delegating canonicality checks.
fn length_error(error: codecs::Error) -> Error {
    match error {
        codecs::Error::Truncated => Error::Truncated,
        _ => Error::InvalidLength,
    }
}

struct Reader<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Reader<'a> {
    fn header(&mut self) -> Result<(Version, Header), Error> {
        let first = self.byte()?;

        let version = match first {
            0..=SHORT_MASK => Version::Legacy,
            VERSION_MASK => Version::V0,
            _ => return Err(Error::UnsupportedVersion),
        };

        let required_signatures = if version == Version::Legacy {
            first
        } else {
            self.byte()?
        };

        Ok((
            version,
            Header {
                required_signatures,
                readonly_signers: self.byte()?,
                readonly_unsigned: self.byte()?,
            },
        ))
    }

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

    // Counts allocate nothing; the shared codec rejects aliases and overflow.
    fn length(&mut self) -> Result<usize, Error> {
        let (value, offset) = ShortU16
            .decode_prefix(self.bytes, self.offset, Limits::new(PACKET_SIZE, 0))
            .map_err(length_error)?;
        self.offset = offset;

        Ok(usize::from(value))
    }

    fn sequence(&mut self) -> Result<Vec<u8>, Error> {
        let count = self.length()?;
        Ok(self.take(count)?.to_vec())
    }

    fn instructions(&mut self) -> Result<Vec<CompiledInstruction>, Error> {
        let count = self.length()?;
        let mut instructions = Vec::new();

        for _ in 0..count {
            instructions.push(CompiledInstruction {
                program_index: self.byte()?,
                accounts: self.sequence()?,
                data: self.sequence()?,
            });
        }

        Ok(instructions)
    }

    fn lookups(&mut self) -> Result<Vec<Lookup>, Error> {
        let count = self.length()?;
        let mut lookups = Vec::new();

        for _ in 0..count {
            lookups.push(Lookup {
                address: Address::from_bytes(self.array()?),
                writable: self.sequence()?,
                readonly: self.sequence()?,
            });
        }

        Ok(lookups)
    }
}
