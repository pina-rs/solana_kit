//! Validated literal instruction prefixes shared by every projection.
use crate::schema::Error;
use std::fmt;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Header(Vec<u8>);

impl Header {
    pub(super) fn new(bytes: Vec<u8>) -> Result<Self, Error> {
        if bytes.is_empty() || bytes.len() > 32 {
            return Err("instruction header must contain one to 32 bytes".into());
        }

        Ok(Self(bytes))
    }

    pub(super) fn bytes(&self) -> &[u8] {
        &self.0
    }

    pub(super) fn literal(&self) -> String {
        self.0
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl fmt::Display for Header {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        output.write_str(&self.literal())
    }
}

pub(super) fn byte_dispatch(program: &super::Program) -> bool {
    program
        .instructions
        .iter()
        .all(|item| item.tag.bytes().len() == 1)
}

pub(super) fn complete_byte_domain(program: &super::Program) -> bool {
    program.instructions.len() == 256 && byte_dispatch(program)
}

pub(super) fn pattern(program: &super::Program, header: &Header) -> String {
    if byte_dispatch(program) {
        return super::constants::tag(program, header);
    }

    format!("[{},..]", header.literal())
}

pub(super) fn read_subject(program: &super::Program) -> &'static str {
    if byte_dispatch(program) {
        return "let tag = *reader.remaining().first().ok_or(Error::Truncated)?;";
    }

    "let tag = reader.remaining();\nif tag.is_empty() { return Err(Error::Truncated); }"
}

pub(super) fn identification(program: &super::Program) -> String {
    let mut output = "/// Identifies a literal header without validating its payload.\npub fn identify_instruction(data:&[u8])->Result<&[u8],Error>{\nmatch data {\n".to_owned();

    for item in &program.instructions {
        output.push_str(&format!(
            "[{},..]=>Ok(&data[..{}]),\n",
            item.tag.literal(),
            item.tag.bytes().len()
        ));
    }

    output.push_str("[]=>Err(Error::Truncated),\n_=>Err(Error::InvalidDiscriminator),\n}\n}\n");
    output.push_str(&super::invocations::dispatch(program));
    output
}
