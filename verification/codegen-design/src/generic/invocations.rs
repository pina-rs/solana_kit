//! Ordered invocation builders and strict parsers from validated role policies.
use super::{Instruction, Program, Signer};
const U8_TAG_COUNT: usize = 256;

fn count(item: &Instruction) -> String {
    let maximum = item.roles.len();

    if item.remaining.is_some() {
        return minimum(maximum);
    }

    if item.omit_optional {
        let required = item.roles.iter().take_while(|role| !role.optional).count();

        return format!("{} || accounts.len() > {maximum}", minimum(required));
    }

    if maximum == 0 {
        return "!accounts.is_empty()".to_owned();
    }

    format!("accounts.len() != {maximum}")
}

fn minimum(count: usize) -> String {
    match count {
        0 => "false".to_owned(),
        1 => "accounts.is_empty()".to_owned(),
        _ => format!("accounts.len() < {count}"),
    }
}

/// Static privilege requirements are compared without interpreting wire schemas.
pub(super) fn helpers() -> String {
    "fn role_matches(program:Address,meta:&AccountMeta,expected:&(bool,Option<bool>,bool))->bool {\nlet (writable,signer,optional)=*expected;\n\nif optional && meta.address==program { return meta.role==AccountRole::Readonly; }\n\nmeta.role.is_writable()==writable && signer.is_none_or(|signer| meta.role.is_signer()==signer)\n}\n".to_owned()
}

pub(super) fn emit(item: &Instruction) -> String {
    let name = &item.name;
    let invalid_count = count(item);
    let requirements = item
        .roles
        .iter()
        .map(|role| {
            let signer = match role.signer {
                Signer::Never => "Some(false)",
                Signer::Required => "Some(true)",
                Signer::Either => "None",
            };

            format!(
                "/* {} */ ({},{signer},{})",
                role.name,
                role.writable,
                role.optional && !item.omit_optional
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let mut out = format!(
        "impl {name} {{\nfn validate_accounts(program: Address, accounts: &[AccountMeta], limits: Limits) -> Result<(), Error> {{\nlet _ = program;\n\nif {invalid_count} {{ return Err(Error::InvalidValue); }}\n\nif accounts.len() > limits.max_items {{ return Err(Error::LimitExceeded); }}\n\nif !accounts.iter().zip([{requirements}]).all(|(meta,expected)|role_matches(program,meta,&expected)) {{ return Err(Error::InvalidValue); }}\n"
    );

    if let Some(remaining) = &item.remaining {
        let start = item.roles.len();
        out.push_str(&format!(
            "let remaining_role=AccountRole::from_privileges({},{});\n\nif accounts[{start}..].iter().any(|meta|meta.role!=remaining_role) {{ return Err(Error::InvalidValue); }}\n",
            remaining.signer,remaining.writable
        ));
    }

    out.push_str(&methods(name));

    out
}

fn methods(name: &str) -> String {
    format!(
        "Ok(())\n}}\n/// Builds an invocation preserving ordered accounts and duplicates.\n/// All role addresses and defaults are explicit caller inputs.\npub fn instruction(&self, accounts: &[AccountMeta], limits: Limits) -> Result<Instruction, Error> {{ self.instruction_with_program(PROGRAM, accounts, limits) }}\n/// Builds an invocation for an explicit program identity with the same wire contract.\npub fn instruction_with_program(&self, program: Address, accounts: &[AccountMeta], limits: Limits) -> Result<Instruction, Error> {{\nSelf::validate_accounts(program,accounts,limits)?;\nlet data = {name}Codec.encode(self,limits.max_bytes)?;\nlet mut owned = Vec::new();\nowned.try_reserve_exact(accounts.len()).map_err(|_| Error::AllocationFailed)?;\nowned.extend_from_slice(accounts);\nOk(Instruction {{ program_address:program,accounts:owned,data }})\n}}\n/// Strictly parses data, identity, ordered privileges and remaining accounts.\npub fn parse(instruction: &Instruction, limits: Limits) -> Result<Self, Error> {{ Self::parse_with_program(PROGRAM,instruction,limits) }}\n/// Strictly parses an invocation against an explicit expected program identity.\npub fn parse_with_program(program: Address,instruction: &Instruction,limits: Limits) -> Result<Self, Error> {{\n\nif instruction.program_address != program {{ return Err(Error::InvalidValue); }}\n\nSelf::validate_accounts(program,&instruction.accounts,limits)?;\n{name}Codec.decode(&instruction.data,limits)\n}}\n}}\n"
    )
}

pub(super) fn dispatch(program: &Program) -> String {
    let name = format!("{}InstructionData", program.name);
    let mut out = format!(
        "/// Strictly parses a canonical program invocation.\npub fn parse_instruction(instruction:&Instruction,limits:Limits)->Result<{name},Error> {{ parse_instruction_with_program(PROGRAM,instruction,limits) }}\n/// Strictly parses an explicit expected program identity.\npub fn parse_instruction_with_program(program:Address,instruction:&Instruction,limits:Limits)->Result<{name},Error> {{\nmatch identify_instruction(&instruction.data)? {{\n"
    );

    for item in &program.instructions {
        out.push_str(&format!(
            "{} => Ok({name}::{}({}::parse_with_program(program,instruction,limits)?)),\n",
            super::constants::tag(program, item.tag),
            item.name,
            item.name
        ));
    }

    if program.instructions.len() < U8_TAG_COUNT {
        out.push_str("_ => Err(Error::InvalidDiscriminator),\n");
    }

    out.push_str("}\n}\n");
    out
}
