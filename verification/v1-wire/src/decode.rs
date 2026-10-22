use crate::{Instruction, Invalid, MAX_PACKET, Message, config_size};

struct Cursor<'a>(&'a [u8]);

impl<'a> Cursor<'a> {
    fn take(&mut self, length: usize) -> Result<&'a [u8], Invalid> {
        let (value, rest) = self.0.split_at_checked(length).ok_or(Invalid)?;
        self.0 = rest;
        Ok(value)
    }

    fn array<const N: usize>(&mut self) -> Result<[u8; N], Invalid> {
        self.take(N)?.try_into().map_err(|_| Invalid)
    }
}

/// Decodes exact complete envelopes; no compact-u16 aliases or trailing padding.
pub fn decode(bytes: &[u8]) -> Result<(Message, Vec<[u8; 64]>), Invalid> {
    if bytes.len() > MAX_PACKET {
        return Err(Invalid);
    }

    let mut cursor = Cursor(bytes);

    if cursor.array::<1>()? != [0x81] {
        return Err(Invalid);
    }

    let header = cursor.array()?;
    let mask = u32::from_le_bytes(cursor.array()?);
    let lifetime = cursor.array()?;
    let [instruction_count, address_count] = cursor.array()?;
    let mut addresses = Vec::new();

    for _ in 0..address_count {
        addresses.push(cursor.array()?);
    }

    let config = cursor.take(config_size(mask)?)?.to_vec();
    let instructions = instructions(&mut cursor, instruction_count)?;
    let mut signatures = Vec::new();

    for _ in 0..header[0] {
        signatures.push(cursor.array()?);
    }

    let message = Message {
        header,
        mask,
        lifetime,
        addresses,
        config,
        instructions,
    };
    message.validate()?;

    if !cursor.0.is_empty() {
        return Err(Invalid);
    }

    Ok((message, signatures))
}

fn instructions(cursor: &mut Cursor<'_>, count: u8) -> Result<Vec<Instruction>, Invalid> {
    let mut headers = Vec::new();

    for _ in 0..count {
        headers.push(cursor.array::<4>()?);
    }

    let mut instructions = Vec::new();

    for [program, accounts, low, high] in headers {
        instructions.push(Instruction {
            program,
            accounts: cursor.take(usize::from(accounts))?.to_vec(),
            data: cursor
                .take(usize::from(u16::from_le_bytes([low, high])))?
                .to_vec(),
        });
    }

    Ok(instructions)
}
