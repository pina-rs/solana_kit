//! Complete-log provenance checks protect against foreign CPI event forgery.

use crate::{Coder, Error, Value, coder::strip, decode};
use alloc::{string::String, vec::Vec};
use base64::{Engine, engine::general_purpose::STANDARD};

/// An event emitted while the IDL program was the active invocation.
#[derive(Clone, Debug, PartialEq)]
pub struct DecodedEvent {
    /// Event entry name.
    pub name: String,
    /// Event definition fields or enum variant.
    pub data: Value,
}

impl Coder<'_> {
    /// Extracts events only from eligible complete transaction log sequences.
    ///
    /// Check transaction success separately before acting on an event. Failed
    /// nested invocations restore their callers, matching the pinned source.
    /// Invalid eligible base64 or recognized malformed event data is an error;
    /// foreign and unknown payloads are ignored. Limits cover total log bytes,
    /// number of log entries/events/stack entries, and each payload's decode.
    pub fn decode_event_logs(&self, logs: &[impl AsRef<str>]) -> Result<Vec<DecodedEvent>, Error> {
        let mut programs: Vec<&str> = Vec::new();
        let mut events = Vec::new();
        let mut bytes_left = self.limits.max_bytes;
        let mut items_left = self
            .limits
            .max_items
            .checked_sub(logs.len())
            .ok_or(Error::Limit)?;

        if logs.len() > self.limits.max_items {
            return Err(Error::Limit);
        }

        for log in logs {
            let log = log.as_ref();
            bytes_left = bytes_left.checked_sub(log.len()).ok_or(Error::Limit)?;

            if update_stack(log, &mut programs, self.limits.max_depth)? {
                continue;
            }

            if programs.last().copied() != Some(self.idl.address.as_str()) {
                continue;
            }

            if let Some(payload) = log.strip_prefix("Program data: ") {
                self.event(payload, &mut events, &mut items_left)?;
            }
        }

        Ok(events)
    }

    fn event(
        &self,
        encoded: &str,
        events: &mut Vec<DecodedEvent>,
        items_left: &mut usize,
    ) -> Result<(), Error> {
        let payload = decode_payload(encoded)?;

        for entry in &self.idl.events {
            if !payload.starts_with(&entry.discriminator) {
                continue;
            }

            if events.len() >= self.limits.max_items {
                return Err(Error::Limit);
            }

            *items_left = items_left
                .checked_sub(entry.name.len() + 1)
                .ok_or(Error::Limit)?;
            let limits = crate::Limits {
                max_items: *items_left,
                ..self.limits
            };
            let decoder = Coder::new(self.idl, limits);
            let mut reader = decoder.reader(&payload)?;
            strip(&mut reader, &entry.discriminator, self.limits.max_bytes)?;
            let data = decode::defined(
                self.idl,
                self.definition(&entry.name)?,
                &mut reader,
                limits,
                0,
            )?;
            *items_left = items_left
                .checked_sub(allocation_cost(&data)?)
                .ok_or(Error::Limit)?;
            events.push(DecodedEvent {
                name: entry.name.clone(),
                data,
            });
            break;
        }

        Ok(())
    }
}

fn decode_payload(encoded: &str) -> Result<Vec<u8>, Error> {
    // Dart accepts both alphabets, including mixed input, while retaining
    // canonical padding and unused-bit validation.
    let normalized: Vec<_> = encoded
        .bytes()
        .map(|byte| match byte {
            b'-' => b'+',
            b'_' => b'/',
            other => other,
        })
        .collect();

    STANDARD.decode(normalized).map_err(|_| Error::Base64)
}

fn allocation_cost(value: &Value) -> Result<usize, Error> {
    match value {
        Value::String(text) => Ok(text.len()),
        Value::Bytes(bytes) => Ok(bytes.len()),
        Value::Sequence(items) => items.iter().try_fold(items.len(), |cost, item| {
            add_cost(cost, allocation_cost(item)?)
        }),
        Value::Option(Some(inner)) => add_cost(1, allocation_cost(inner)?),
        Value::Struct(fields) => field_cost(fields),
        Value::Enum { variant, fields } => add_cost(variant.len(), field_cost(fields)?),
        _ => Ok(0),
    }
}

fn field_cost(fields: &crate::Values) -> Result<usize, Error> {
    fields.iter().try_fold(fields.len(), |cost, (name, value)| {
        add_cost(add_cost(cost, name.len())?, allocation_cost(value)?)
    })
}

fn add_cost(left: usize, right: usize) -> Result<usize, Error> {
    left.checked_add(right).ok_or(Error::Limit)
}

fn update_stack<'a>(
    log: &'a str,
    programs: &mut Vec<&'a str>,
    max_depth: usize,
) -> Result<bool, Error> {
    let Some(line) = log.strip_prefix("Program ") else {
        return Ok(false);
    };

    if let Some((program, depth)) = invocation(line) {
        if depth == 1 {
            programs.clear();
        }

        if depth != programs.len() + 1 {
            programs.clear();

            return Ok(true);
        }

        if programs.len() >= max_depth {
            return Err(Error::Limit);
        }

        programs.push(program);

        return Ok(true);
    }

    if let Some(program) = completion(line) {
        if programs.last().copied() == Some(program) {
            programs.pop();
        } else {
            programs.clear();
        }

        return Ok(true);
    }

    Ok(false)
}

fn invocation(line: &str) -> Option<(&str, usize)> {
    let (program, depth) = line.split_once(" invoke [")?;
    let depth = depth.strip_suffix(']')?;

    if !base58(program) || depth.is_empty() || !depth.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }

    Some((program, depth.parse().unwrap_or(usize::MAX)))
}

fn completion(line: &str) -> Option<&str> {
    let (program, result) = line.split_once(' ')?;

    if !base58(program) || (result != "success" && !result.starts_with("failed: ")) {
        return None;
    }

    Some(program)
}

fn base58(text: &str) -> bool {
    !text.is_empty()
        && text.bytes().all(|byte| {
            b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".contains(&byte)
        })
}
