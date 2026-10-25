//! Modular invocation emission from one typed account-role projection.
use super::{
    Result,
    roles::{self, Role},
    snake, text, title,
};
use serde_json::Value;

pub(super) struct Model {
    pub name: String,
    pub roles: Vec<Role>,
    pub remaining: Vec<Role>,
    pub omitted: bool,
}

pub(super) fn render(items: &[Value]) -> Result<Vec<(String, String)>> {
    let mut files = Vec::new();
    let program: solana_kit_core::Address =
        "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb".parse()?;
    let mut index = format!(
        "//! Source-generated typed invocation wrappers.\n/// Canonical pinned program identity.\npub const PROGRAM:solana_kit_core::Address=solana_kit_core::Address::from_bytes({:?});\n",
        program.to_bytes()
    );
    let mut arms = String::new();

    for item in items {
        let raw = text(item, "name")?;
        let model = model(item)?;
        let name = &model.name;
        let module = snake(raw);
        let source = format!(
            "{}{}{}",
            super::accounts_output::render(&model)?,
            super::methods_output::render(&model),
            super::methods_output::validation(&model)
        );
        files.push((format!("{module}.rs"), source));
        index.push_str(&format!("mod {module};\npub use {module}::*;\n"));

        arms.push_str(&format!("Token2022InstructionData::{name}(value)=>value.validate_accounts(program,&instruction.accounts,limits)?,\n"));
    }

    index.push_str(&dispatch(&arms));
    files.push(("mod.rs".to_owned(), index));
    Ok(files)
}

fn model(item: &Value) -> Result<Model> {
    let roles = item["accounts"]
        .as_array()
        .ok_or("accounts required")?
        .iter()
        .map(roles::parse)
        .collect::<Result<_>>()?;
    let remaining = item["remainingAccounts"]
        .as_array()
        .map_or(Ok(Vec::new()), |nodes| {
            nodes
                .iter()
                .map(roles::remaining)
                .collect::<Result<Vec<_>>>()
        })?;
    let name = title(text(item, "name")?);

    let omitted = match item["optionalAccountStrategy"].as_str() {
        Some("omitted") => true,
        Some("programId") => false,
        _ => return Err("unknown optional-account strategy".into()),
    };

    if remaining.len() > 1 && name != "WithdrawWithheldTokensFromAccounts" {
        return Err("unknown remaining segment boundary".into());
    }

    Ok(Model {
        name,
        roles,
        remaining,
        omitted,
    })
}

fn dispatch(arms: &str) -> String {
    format!(
        "use crate::{{*,invocation::ParsedInstruction}};\nuse solana_kit_core::{{Address,Instruction,codecs::{{Decoder,Error,Limits}}}};\n/// Parses the canonical program, complete data frame and exact positional accounts.\npub fn parse_instruction(instruction:&Instruction,limits:Limits)->Result<ParsedInstruction,Error>{{parse_instruction_with_program(PROGRAM,instruction,limits)}}\n/// Parses with an explicit expected program identity.\npub fn parse_instruction_with_program(program:Address,instruction:&Instruction,limits:Limits)->Result<ParsedInstruction,Error>{{\n\nif instruction.program_address!=program {{return Err(Error::InvalidValue);}}\n\nlet data=Token2022InstructionDataCodec.decode(&instruction.data,limits)?;\n\nmatch &data {{\n{arms}}}\n\nOk(ParsedInstruction {{data,accounts:instruction.accounts.clone()}})\n}}\n"
    )
}
