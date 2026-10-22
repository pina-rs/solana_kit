use crate::{Invalid, Message};

/// Encodes the complete signed envelope, enforcing the inclusive 4096-byte cap.
pub fn encode(message: &Message, signatures: &[[u8; 64]]) -> Result<Vec<u8>, Invalid> {
    message.validate()?;

    if signatures.len() != usize::from(message.header[0]) {
        return Err(Invalid);
    }

    let mut bytes = vec![0x81];
    bytes.extend(message.header);
    bytes.extend(message.mask.to_le_bytes());
    bytes.extend(message.lifetime);
    bytes.push(u8::try_from(message.instructions.len()).map_err(|_| Invalid)?);
    bytes.push(u8::try_from(message.addresses.len()).map_err(|_| Invalid)?);

    for address in &message.addresses {
        bytes.extend(address);
    }

    bytes.extend(&message.config);

    for instruction in &message.instructions {
        bytes.push(instruction.program);
        bytes.push(u8::try_from(instruction.accounts.len()).map_err(|_| Invalid)?);
        bytes.extend(
            u16::try_from(instruction.data.len())
                .map_err(|_| Invalid)?
                .to_le_bytes(),
        );
    }

    for instruction in &message.instructions {
        bytes.extend(&instruction.accounts);
        bytes.extend(&instruction.data);
    }

    for signature in signatures {
        bytes.extend(signature);
    }

    if bytes.len() > 4096 {
        return Err(Invalid);
    }

    Ok(bytes)
}
