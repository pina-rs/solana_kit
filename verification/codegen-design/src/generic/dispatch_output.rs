//! Large dispatches retain concrete codecs in bounded groups of match arms.
use super::Program;

pub(super) const INLINE_LIMIT: usize = 32;
const GROUP_SIZE: usize = 16;
const U8_TAG_COUNT: usize = 256;

#[derive(Clone, Copy)]
enum Operation {
    Length,
    Write,
    Read,
    Parse,
}
impl Operation {
    fn name(self) -> &'static str {
        match self {
            Self::Length => "encoded_len",
            Self::Write => "write",
            Self::Read => "read",
            Self::Parse => "parse",
        }
    }
    fn arguments(self) -> &'static str {
        match self {
            Self::Length => "value",
            Self::Write => "value, writer",
            Self::Read => "tag, reader",
            Self::Parse => "tag, program, instruction, limits",
        }
    }
    fn signature(self, name: &str) -> String {
        match self {
            Self::Length => format!("(value: &{name}) -> Result<usize, Error>"),
            Self::Write => {
                format!("(value: &{name}, writer: &mut Writer<'_>) -> Result<(), Error>")
            }
            Self::Read => format!("(tag:u8, reader:&mut Reader<'_>) -> Result<{name}, Error>"),
            Self::Parse => format!(
                "(tag:u8, program:Address, instruction:&Instruction, limits:Limits) -> Result<{name}, Error>"
            ),
        }
    }
    fn arm(self, program: &Program, item: &super::Instruction, name: &str) -> String {
        let variant = &item.name;
        let tag = super::constants::tag(program, item.tag);

        match self {
            Self::Length => {
                format!("{name}::{variant}(value) => {variant}Codec.encoded_len(value),\n")
            }
            Self::Write => {
                format!("{name}::{variant}(value) => {variant}Codec.write(value, writer),\n")
            }
            Self::Read => {
                format!("{tag} => Ok({name}::{variant}({variant}Codec.read(reader)?)),\n")
            }
            Self::Parse => format!(
                "{tag} => Ok({name}::{variant}({variant}::parse_with_program(program,instruction,limits)?)),\n"
            ),
        }
    }
}

/// Keep each concrete operation bounded while retaining source order and sparse errors.
fn helpers(program: &Program, name: &str, operation: Operation) -> String {
    let mut out = String::new();
    let chunks = program.instructions.chunks(GROUP_SIZE);
    let count = chunks.len();
    let method = operation.name();
    let arguments = operation.arguments();

    let subject = match operation {
        Operation::Length | Operation::Write => "value",
        Operation::Read | Operation::Parse => "tag",
    };

    for (index, chunk) in chunks.enumerate() {
        out.push_str(&format!(
            "fn {method}_{index}{} {{\nmatch {subject} {{\n",
            operation.signature(name)
        ));

        for item in chunk {
            out.push_str(&operation.arm(program, item, name));
        }

        if index + 1 < count {
            out.push_str(&format!(
                "_ => Self::{method}_{}({arguments}),\n",
                index + 1
            ));
        } else {
            out.push_str("_ => Err(Error::InvalidDiscriminator),\n");
        }

        out.push_str("}\n}\n");
    }

    out
}

pub(super) fn emit(program: &Program) -> String {
    let name = format!("{}InstructionData", program.name);
    let mut out = format!(
        "/// All selected instruction data variants.\n#[derive(Clone, Debug, Eq, PartialEq)]\npub enum {name} {{\n"
    );

    for item in &program.instructions {
        out.push_str(&format!(
            "/// Data for tag {}.\n{}({}),\n",
            item.tag, item.name, item.name
        ));
    }

    out.push_str(&format!("}}\n/// Strict bounded instruction dispatch codec.\n#[derive(Clone, Copy, Debug)]\npub struct {name}Codec;\nimpl Encoder<{name}> for {name}Codec {{\nfn encoded_len(&self,value:&{name})->Result<usize,Error>{{Self::encoded_len_0(value)}}\nfn write(&self,value:&{name},writer:&mut Writer<'_>)->Result<(),Error>{{Self::write_0(value,writer)}}\n}}\nimpl Decoder<{name}> for {name}Codec {{\nfn read(&self,reader:&mut Reader<'_>)->Result<{name},Error>{{\nlet tag=*reader.remaining().first().ok_or(Error::Truncated)?;\nSelf::read_0(tag,reader)\n}}\n}}\nimpl {name}Codec {{\n"));

    for operation in [Operation::Length, Operation::Write, Operation::Read] {
        out.push_str(&helpers(program, &name, operation));
    }

    if super::constants::extended(program) {
        out.push_str(&helpers(program, &name, Operation::Parse));
    }

    out.push_str("}\n");
    out.push_str(&identification(program, &name));
    out
}

fn identification(program: &Program, name: &str) -> String {
    let mut out = "/// Identifies a tag without asserting the full payload is valid.\npub fn identify_instruction(data:&[u8])->Result<u8,Error>{\nlet tag=*data.first().ok_or(Error::Truncated)?;\n".to_owned();

    if program.instructions.len() < U8_TAG_COUNT {
        let tags = program
            .instructions
            .iter()
            .map(|item| super::constants::tag(program, item.tag))
            .collect::<Vec<_>>()
            .join(",");
        out.push_str(&format!("const TAGS:&[u8]=&[{tags}];\n\nif !TAGS.contains(&tag) {{return Err(Error::InvalidDiscriminator);}}\n"));
    }

    out.push_str("Ok(tag)\n}\n");

    if super::constants::extended(program) {
        out.push_str(&format!("/// Strictly parses a canonical program invocation.\npub fn parse_instruction(instruction:&Instruction,limits:Limits)->Result<{name},Error>{{parse_instruction_with_program(PROGRAM,instruction,limits)}}\n/// Strictly parses an explicit expected program identity.\npub fn parse_instruction_with_program(program:Address,instruction:&Instruction,limits:Limits)->Result<{name},Error>{{{name}Codec::parse_0(identify_instruction(&instruction.data)?,program,instruction,limits)}}\n"));
    } else {
        out.push_str(&format!("/// Fully parses a canonical invocation with the fixed selected program identity.\npub fn parse_instruction(instruction:&Instruction,limits:Limits)->Result<{name},Error>{{\nif instruction.program_address!=PROGRAM || !instruction.accounts.is_empty() {{return Err(Error::InvalidValue);}}\n\n{name}Codec.decode(&instruction.data,limits)\n}}\n"));
    }

    out
}
