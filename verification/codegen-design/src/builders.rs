//! Generates typed ordered account builders and strict parsers from one IR.

use crate::{
    accounts_schema::{self, Account},
    generate,
    schema::{self, Error, Instruction, Wire},
};
use serde_json::Value;
use std::path::Path;

const CREATE_SEEDED: u32 = 3;
const ALLOCATE_SEEDED: u32 = 9;
const ASSIGN_SEEDED: u32 = 10;
const PREFUND: u32 = 13;

fn fields(name: &str, accounts: &[Account]) -> String {
    let mut output = format!(
        "/// Ordered caller-supplied accounts for [`{name}`].\n#[derive(Clone, Copy, Debug, Eq, PartialEq)]\npub struct {name}Accounts {{\n"
    );

    for account in accounts.iter().filter(|account| account.fixed.is_none()) {
        let kind = if account.optional {
            "Option<Address>"
        } else {
            "Address"
        };

        output.push_str(&format!(
            "/// Address for `{}`.\npub {}: {kind},\n",
            account.name, account.name
        ));
    }

    output.push_str("}\n\n");
    output
}

/// Keeps seed byte limits and base signatures alongside their account contracts.
fn checks(item: &Instruction) -> String {
    let mut output = String::new();

    for field in item
        .fields
        .iter()
        .filter(|field| matches!(field.wire, Wire::String))
    {
        output.push_str(&format!(
            "\nif self.{}.len() > MAX_SEED_BYTES {{ return Err(ClientError::SeedTooLong); }}\n",
            field.name
        ));
    }

    if matches!(item.tag, CREATE_SEEDED | ALLOCATE_SEEDED | ASSIGN_SEEDED) {
        let base = if item.tag == CREATE_SEEDED {
            "accounts.base_account.unwrap_or(accounts.payer)"
        } else {
            "accounts.base_account"
        };

        output.push_str(&format!(
            "\nif self.base != {base} {{ return Err(ClientError::BaseMismatch); }}\n"
        ));
    }

    if item.tag == PREFUND {
        output.push_str("\nif self.lamports != 0 && accounts.payer.is_none() { return Err(ClientError::MissingPayer); }\n");
    }

    output
}

fn build(item: &Instruction, accounts: &[Account]) -> String {
    let name = &item.rust_name;
    let mutable = if accounts.iter().any(|account| account.optional) {
        "mut "
    } else {
        ""
    };

    let mut output = format!(
        "/// Builds an invocation with fixed System identity and canonical account privileges.\npub fn instruction(&self, accounts: {name}Accounts, limits: Limits) -> Result<Instruction, ClientError> {{\n"
    );
    output.push_str(&checks(item));
    output.push_str(&format!("\nlet {mutable}metas = alloc::vec![\n"));

    for account in accounts.iter().filter(|account| !account.optional) {
        let address = account
            .fixed
            .map_or_else(|| format!("accounts.{}", account.name), str::to_owned);
        output.push_str(&format!(
            "AccountMeta {{ address: {address}, role: AccountRole::{} }},\n",
            account.role
        ));
    }

    output.push_str("];\n");

    for account in accounts.iter().filter(|account| account.optional) {
        output.push_str(&format!("\nif let Some(address) = accounts.{} {{ metas.push(AccountMeta {{ address, role: AccountRole::{} }}); }}\n",account.name,account.role));
    }

    output.push_str(&format!("\nOk(Instruction {{ program_address: SYSTEM_PROGRAM, accounts: metas, data: {name}Codec.encode(self, limits.max_bytes)? }})\n}}\n\n"));
    output
}

/// Renders metadata for shared validation, then delegates to the sole data codec.
fn parse(item: &Instruction, accounts: &[Account]) -> String {
    let name = &item.rust_name;
    let minimum = accounts.iter().filter(|account| !account.optional).count();
    let mut output = format!(
        "/// Parses program identity, tag, positional privileges and fixed sysvars.\npub fn parse(instruction: &Instruction, limits: Limits) -> Result<(Self, {name}Accounts), ClientError> {{\nvalidate_accounts(instruction, &[\n"
    );

    for account in accounts {
        let fixed = account
            .fixed
            .map_or_else(|| "None".to_owned(), |name| format!("Some({name})"));
        output.push_str(&format!("(AccountRole::{}, {fixed}),\n", account.role));
    }

    output.push_str(&format!("], {minimum})?;\nlet value = {name}Codec.decode(&instruction.data, limits)?;\nlet accounts = {name}Accounts {{\n"));

    for (index, account) in accounts
        .iter()
        .enumerate()
        .filter(|(_, account)| account.fixed.is_none())
    {
        let expression = if account.optional {
            format!("instruction.accounts.get({index}).map(|meta| meta.address)")
        } else {
            format!("instruction.accounts[{index}].address")
        };

        output.push_str(&format!("{}: {expression},\n", account.name));
    }

    output.push_str("};\nvalue.instruction(accounts, limits)?;\nOk((value, accounts))\n}\n");
    output
}

pub(crate) fn render(root: &Value) -> Result<String, Error> {
    accounts_schema::validate_bindings(root)?;
    let items = schema::parse(root)?;
    let nodes = root["program"]["instructions"]
        .as_array()
        .ok_or("missing instructions")?;
    let contracts = nodes
        .iter()
        .map(accounts_schema::parse)
        .collect::<Result<Vec<_>, _>>()?;
    let mut output = "// Generated from pinned System account metadata; see SYSTEM-LICENSE.\nuse solana_kit_core::{Address, Instruction, AccountMeta, AccountRole, codecs::{Encoder, Decoder, Limits}};\nuse crate::{generated::*, client::validate_accounts, ClientError, SYSTEM_PROGRAM, RECENT_BLOCKHASHES, RENT};\nconst MAX_SEED_BYTES: usize = 32;\n\n".to_owned();

    for (item, accounts) in items.iter().zip(&contracts) {
        output.push_str(&fields(&item.rust_name, accounts));
        output.push_str(&format!("impl {} {{\n", item.rust_name));
        output.push_str(&build(item, accounts));
        output.push_str(&parse(item, accounts));
        output.push_str("}\n\n");
    }

    output.push_str(&dispatch(&items));
    Ok(output)
}

pub(crate) fn replace(root: &Value, output: &Path) -> Result<(), Error> {
    generate::replace_source(&render(root)?, output)
}

pub(crate) fn run() -> Result<(), Error> {
    let root = serde_json::from_str(include_str!("../fixtures/system.json"))?;
    replace(
        &root,
        &Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../crates/solana_kit_system/src/builders.rs"),
    )
}

fn dispatch(items: &[Instruction]) -> String {
    let mut output = "/// Identifies the tag prefix; full validation is provided by [`parse_instruction`].\npub fn identify_instruction(data: &[u8]) -> Result<u32, ClientError> {\nlet tag = solana_kit_core::codecs::Number::<u32>::LE.decode(data.get(..4).ok_or(solana_kit_core::codecs::Error::Truncated)?, Limits::new(4,0))?;\n\nif tag > LAST_SYSTEM_TAG { return Err(solana_kit_core::codecs::Error::InvalidDiscriminator.into()); }\n\nOk(tag)\n}\n\nconst LAST_SYSTEM_TAG: u32 = 13;\n\n/// Parses canonical data and ordered account contracts.\npub fn parse_instruction(instruction: &Instruction, limits: Limits) -> Result<SystemInstructionData, ClientError> {\nlet parsed = match identify_instruction(&instruction.data)? {\n".to_owned();

    for item in items {
        output.push_str(&format!(
            "{} => SystemInstructionData::{}({}::parse(instruction,limits)?.0),\n",
            item.tag, item.rust_name, item.rust_name
        ));
    }

    output.push_str("_ => return Err(solana_kit_core::codecs::Error::InvalidDiscriminator.into()),\n};\n\nOk(parsed)\n}\n");
    output
}
