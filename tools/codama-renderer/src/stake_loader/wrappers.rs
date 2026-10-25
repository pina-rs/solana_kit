//! Concrete account builders share validated layouts without a second wire emitter.
use super::roles::{Role, roles};
use crate::schema::{Error, identifier};
use serde_json::Value;

pub(super) fn render(root: &Value, owner: &str) -> Result<String, Error> {
    let address: solana_kit_core::Address = root["program"]["publicKey"]
        .as_str()
        .ok_or("missing program address")?
        .parse()?;
    let mut out = format!(
        "// Generated source-backed account wrappers; see upstream notices.\nuse super::*;\nuse solana_kit_core::{{Address,Instruction,AccountMeta,AccountRole,codecs::{{Decoder,Encoder,Error,Limits}}}};\n/// Canonical executable program address.\npub const PROGRAM:Address=Address::from_bytes({:?});\n",
        address.as_bytes()
    );
    out.push_str("/// Decoded data borrowing every original invocation metadata field.\n#[derive(Clone,Debug,Eq,PartialEq)]\npub struct ParsedInstruction<'a,T> {\n/// Exact decoded data.\npub data:T,\n/// Original instruction, including ordered accounts and remaining roles.\npub instruction:&'a Instruction,\n}\n");

    for node in root["program"]["instructions"]
        .as_array()
        .ok_or("missing instructions")?
    {
        let mut title = node["name"]
            .as_str()
            .ok_or("missing instruction")?
            .to_owned();
        title[0..1].make_ascii_uppercase();
        let roles = roles(node, owner)?;
        out.push_str(&declaration(&title, &roles)?);
        out.push_str(&implementation(&title, &roles)?);
        out.push_str(&parser(&title));
    }

    Ok(out)
}

fn declaration(title: &str, roles: &[Role]) -> Result<String, Error> {
    let mut out = format!(
        "/// Ordered source accounts for `{title}`.\n#[derive(Clone,Debug)]\npub struct {title}Accounts {{\n"
    );

    for (name, _, optional) in roles {
        let ty = if *optional {
            "Option<Address>"
        } else {
            "Address"
        };

        out.push_str(&format!(
            "/// Source account `{name}`.\npub {}:{ty},\n",
            identifier(name)?
        ));
    }

    out.push_str("}\n");
    Ok(out)
}

fn count(roles: &[Role]) -> Result<String, Error> {
    let mut count = format!("{}usize", roles.iter().filter(|role| !role.2).count());

    for (name, _, optional) in roles.iter().filter(|role| role.2) {
        debug_assert!(*optional);
        count.push_str(&format!(
            " + usize::from(self.{}.is_some())",
            identifier(name)?
        ));
    }

    Ok(count)
}

fn implementation(title: &str, roles: &[Role]) -> Result<String, Error> {
    let count = count(roles)?;
    let mut out = format!(
        "impl {title}Accounts {{\n/// Builds source account order for an explicit program.\n/// # Errors\n/// Rejects allocation failure, invalid data, or caller limits.\npub fn instruction(&self,program:Address,data:&{title},remaining:&[AccountMeta],limits:Limits)->Result<Instruction,Error>{{\nlet count=({count}).checked_add(remaining.len()).ok_or(Error::Overflow)?;\n\nif count>limits.max_items {{ return Err(Error::LimitExceeded); }}\n\nlet mut accounts=alloc::vec::Vec::new();\naccounts.try_reserve_exact(count).map_err(|_|Error::AllocationFailed)?;\n"
    );

    for role in roles {
        out.push_str(&push(role)?);
    }

    out.push_str(&format!("\naccounts.extend_from_slice(remaining);\nlet data={title}Codec.encode(data,limits.max_bytes)?;\nOk(Instruction{{program_address:program,accounts,data}})\n}}\n}}\n"));
    Ok(out)
}

fn push((name, role, optional): &Role) -> Result<String, Error> {
    let field = identifier(name)?;

    if *optional {
        return Ok(format!(
            "\nif let Some(address)=self.{field} {{\naccounts.push(AccountMeta{{address,role:AccountRole::{role}}});\n}}\n"
        ));
    }

    Ok(format!(
        "accounts.push(AccountMeta{{address:self.{field},role:AccountRole::{role}}});\n"
    ))
}

fn parser(title: &str) -> String {
    format!(
        "impl {title} {{\n/// Decodes exact data for an explicitly selected program.\n/// # Errors\n/// Rejects foreign programs, malformed data, limits, and trailing bytes.\npub fn parse_instruction(instruction:&Instruction,program:Address,limits:Limits)->Result<Self,Error>{{\nif instruction.program_address!=program {{return Err(Error::InvalidValue);}}\n\nif instruction.accounts.len()>limits.max_items {{return Err(Error::LimitExceeded);}}\n\n{title}Codec.decode(&instruction.data,Limits::new(limits.max_bytes,limits.max_items-instruction.accounts.len()))\n}}\n/// Decodes while borrowing all original invocation metadata.\n/// # Errors\n/// Preserves every program, data and budget error from the strict parser.\npub fn parse_with_metadata<'a>(instruction:&'a Instruction,program:Address,limits:Limits)->Result<ParsedInstruction<'a,Self>,Error>{{\nOk(ParsedInstruction{{data:Self::parse_instruction(instruction,program,limits)?,instruction}})\n}}\n}}\n"
    )
}
