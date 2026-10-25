//! Typed seed recipes and role-default dependencies, validated before rendering.
use super::parse::{array, keys};
use crate::schema::{Error, identifier};
use serde_json::Value;
use solana_kit_core::Address;
use std::collections::BTreeSet;
const MAX_SEEDS: usize = 16;
const MAX_SEED_BYTES: usize = 32;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Seed {
    Constant(Vec<u8>),
    Address(String),
    U64(String),
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct Pda {
    pub(crate) name: String,
    pub(crate) function: String,
    pub(crate) seeds: Vec<Seed>,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Default {
    Address(Address),
    Pda { name: String, accounts: Vec<String> },
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct RoleDefault {
    pub(crate) instruction: String,
    pub(crate) role: String,
    pub(crate) value: Default,
}

/// Parsing does not derive addresses or erase caller-supplied override support.
pub(crate) fn recipes(nodes: &[Value]) -> Result<Vec<Pda>, Error> {
    let mut names = BTreeSet::new();
    let mut functions = BTreeSet::new();
    let mut recipes = Vec::new();

    for node in nodes {
        keys(node, &["kind", "name", "seeds"])?;
        let name = name(node)?;

        if node["kind"] != "pdaNode" || !names.insert(name.clone()) {
            return Err("invalid or duplicate PDA recipe".into());
        }

        let seeds = recipe_seeds(node)?;
        let function = function_name(&name, &mut functions)?;
        recipes.push(Pda {
            name,
            function,
            seeds,
        });
    }

    Ok(recipes)
}

/// Reserving both exports prevents a source name ending in WithProgram from aliasing a helper.
fn function_name(name: &str, functions: &mut BTreeSet<String>) -> Result<String, Error> {
    let mut function = format!("find_{}", identifier(name)?);

    while functions.contains(&function) || functions.contains(&format!("{function}_with_program")) {
        function.push_str("_pda");
    }

    functions.insert(function.clone());
    functions.insert(format!("{function}_with_program"));
    Ok(function)
}

fn recipe_seeds(node: &Value) -> Result<Vec<Seed>, Error> {
    let seeds = array(node, "seeds")?
        .iter()
        .map(seed)
        .collect::<Result<Vec<_>, _>>()?;
    let mut variable_names = BTreeSet::new();

    for seed in &seeds {
        if let Seed::Address(name) | Seed::U64(name) = seed
            && !variable_names.insert(name)
        {
            return Err("duplicate PDA variable seed".into());
        }
    }

    if seeds.is_empty() || seeds.len() >= MAX_SEEDS {
        return Err("PDA recipe must leave room for its bump seed".into());
    }

    Ok(seeds)
}

fn name(node: &Value) -> Result<String, Error> {
    let name = node["name"].as_str().ok_or("missing policy name")?;
    identifier(name)?;
    Ok(name.to_owned())
}

fn seed(node: &Value) -> Result<Seed, Error> {
    match node["kind"].as_str() {
        Some("constantPdaSeedNode") => constant(node),
        Some("variablePdaSeedNode") => variable(node),
        _ => Err("unsupported PDA seed kind".into()),
    }
}

fn constant(node: &Value) -> Result<Seed, Error> {
    keys(node, &["kind", "type", "value"])?;
    keys(&node["type"], &["kind", "encoding"])?;
    keys(&node["value"], &["kind", "string"])?;

    if node["type"]["kind"] != "stringTypeNode"
        || node["type"]["encoding"] != "utf8"
        || node["value"]["kind"] != "stringValueNode"
    {
        return Err("unsupported constant PDA seed encoding".into());
    }

    let value = node["value"]["string"]
        .as_str()
        .ok_or("missing constant seed")?;

    if value.len() > MAX_SEED_BYTES {
        return Err("PDA seed exceeds protocol byte limit".into());
    }

    Ok(Seed::Constant(value.as_bytes().to_vec()))
}

fn variable(node: &Value) -> Result<Seed, Error> {
    keys(node, &["kind", "name", "type"])?;
    let name = name(node)?;

    if node["type"] == serde_json::json!({"kind":"publicKeyTypeNode"}) {
        return Ok(Seed::Address(name));
    }

    if node["type"] == serde_json::json!({"kind":"numberTypeNode","format":"u64","endian":"le"}) {
        return Ok(Seed::U64(name));
    }

    Err("unsupported PDA variable seed type".into())
}

/// Validates every default reference and preserves its exact recipe seed order.
pub(crate) fn defaults(instructions: &[Value], pdas: &[Pda]) -> Result<Vec<RoleDefault>, Error> {
    let mut output = Vec::new();

    for instruction in instructions {
        let instruction_name = name(instruction)?;
        let accounts = array(instruction, "accounts")?;
        let names = accounts
            .iter()
            .map(name)
            .collect::<Result<BTreeSet<_>, _>>()?;

        if names.len() != accounts.len() {
            return Err("duplicate instruction account in default graph".into());
        }

        let start = output.len();

        for account in accounts {
            if let Some(value) = account.get("defaultValue") {
                output.push(RoleDefault {
                    instruction: instruction_name.clone(),
                    role: name(account)?,
                    value: default(value, pdas, &names)?,
                });
            }
        }

        for role in &output[start..] {
            acyclic(&role.role, &output[start..], &mut BTreeSet::new())?;
        }
    }

    Ok(output)
}

fn default(node: &Value, pdas: &[Pda], roles: &BTreeSet<String>) -> Result<Default, Error> {
    match node["kind"].as_str() {
        Some("publicKeyValueNode") => {
            keys(node, &["kind", "publicKey"])?;
            let value = node["publicKey"]
                .as_str()
                .ok_or("missing default address")?;
            Ok(Default::Address(value.parse()?))
        }
        Some("pdaValueNode") => pda_default(node, pdas, roles),
        _ => Err("unsupported account default kind".into()),
    }
}

fn pda_default(node: &Value, pdas: &[Pda], roles: &BTreeSet<String>) -> Result<Default, Error> {
    keys(node, &["kind", "pda", "seeds"])?;
    keys(&node["pda"], &["kind", "name"])?;
    let pda_name = name(&node["pda"])?;

    if node["pda"]["kind"] != "pdaLinkNode" {
        return Err("invalid PDA default link".into());
    }

    let recipe = pdas
        .iter()
        .find(|pda| pda.name == pda_name)
        .ok_or("unresolved PDA default")?;
    let expected = address_seeds(recipe)?;
    let actual = array(node, "seeds")?;

    if expected.len() != actual.len() {
        return Err("PDA default seed count mismatch".into());
    }

    let accounts = actual
        .iter()
        .zip(expected)
        .map(|(seed, expected_name)| account_seed(seed, expected_name, roles))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Default::Pda {
        name: pda_name,
        accounts,
    })
}

fn address_seeds(recipe: &Pda) -> Result<Vec<&String>, Error> {
    Ok(recipe
        .seeds
        .iter()
        .filter_map(|seed| match seed {
            Seed::Address(name) => Some(Ok(name)),
            Seed::U64(_) => Some(Err(
                "integer PDA seed requires an explicit argument resolver",
            )),
            Seed::Constant(_) => None,
        })
        .collect::<Result<Vec<_>, _>>()?)
}

fn account_seed(
    seed: &Value,
    expected_name: &str,
    roles: &BTreeSet<String>,
) -> Result<String, Error> {
    keys(seed, &["kind", "name", "value"])?;
    keys(&seed["value"], &["kind", "name"])?;
    let account = name(&seed["value"])?;

    if seed["kind"] != "pdaSeedValueNode"
        || seed["name"] != expected_name
        || seed["value"]["kind"] != "accountValueNode"
        || !roles.contains(&account)
    {
        return Err("unresolved or reordered PDA default seed".into());
    }

    Ok(account)
}

fn acyclic<'a>(
    name: &'a str,
    defaults: &'a [RoleDefault],
    active: &mut BTreeSet<&'a str>,
) -> Result<(), Error> {
    if !active.insert(name) {
        return Err("cyclic account default dependency".into());
    }

    if let Some(RoleDefault {
        value: Default::Pda { accounts, .. },
        ..
    }) = defaults.iter().find(|item| item.role == name)
    {
        for account in accounts {
            acyclic(account, defaults, active)?;
        }
    }

    active.remove(name);
    Ok(())
}
