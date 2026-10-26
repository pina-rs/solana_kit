//! Ordered form fields use the HTML form byte encoding, without URL concatenation.

use super::{Budgets, Error};

/// Encodes bounded fields one component at a time so separators cannot be injected.
pub(super) fn encode(fields: Vec<(String, String)>, budgets: Budgets) -> Result<Vec<u8>, Error> {
    if fields.len() > budgets.items {
        return Err(Error::Items);
    }

    let mut output = Vec::new();

    for (index, (key, value)) in fields.into_iter().enumerate() {
        if key.len() > budgets.string_bytes || value.len() > budgets.string_bytes {
            return Err(Error::String);
        }

        if index > 0 {
            push(&mut output, b"&", budgets.bytes)?;
        }

        component(&mut output, &key, budgets.bytes)?;
        push(&mut output, b"=", budgets.bytes)?;
        component(&mut output, &value, budgets.bytes)?;
    }

    Ok(output)
}

fn component(output: &mut Vec<u8>, value: &str, limit: usize) -> Result<(), Error> {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    const LOW_NIBBLE: u8 = 0x0f;

    for byte in value.bytes() {
        match byte {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'*' | b'-' | b'.' | b'_' => {
                push(output, &[byte], limit)?;
            }
            b' ' => push(output, b"+", limit)?,
            _ => push(
                output,
                &[
                    b'%',
                    HEX[usize::from(byte >> 4)],
                    HEX[usize::from(byte & LOW_NIBBLE)],
                ],
                limit,
            )?,
        }
    }

    Ok(())
}

fn push(output: &mut Vec<u8>, bytes: &[u8], limit: usize) -> Result<(), Error> {
    if bytes.len() > limit.saturating_sub(output.len()) {
        return Err(Error::Bytes);
    }

    output.extend_from_slice(bytes);
    Ok(())
}
