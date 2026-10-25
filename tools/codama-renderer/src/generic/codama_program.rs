//! Complete Codama frontend lowers accounts, events and defaults into one typed renderer.
use super::{Definition, Field, Instruction, Policy, Program, Shape, data, parse, pda_policy};
use crate::schema::Error;
use serde_json::Value;
const HEX_RADIX: u32 = 16;
const ACCOUNT_KEYS: &[&str] = &["kind", "name", "data", "pda", "docs", "discriminators"];
const EVENT_KEYS: &[&str] = &["kind", "name", "data", "discriminators", "docs"];

#[cfg(feature = "verification")]
pub(super) fn render(root: &Value, policy: &Policy<'_>) -> Result<String, Error> {
    render_schema(root, policy, false)
}

/// Public roots use the pinned1.8 schema with the same finite instruction profiles.
pub(super) fn render_codama(root: &Value, policy: &Policy<'_>) -> Result<String, Error> {
    let mut selected = root.clone();

    if let Some(program) = selected.get_mut("program").and_then(Value::as_object_mut) {
        program.remove("docs");
        program.remove("origin");
    }

    render_schema(&selected, policy, true)
}

fn render_schema(root: &Value, policy: &Policy<'_>, public: bool) -> Result<String, Error> {
    let node = program_node(root, policy, public)?;

    node["version"].as_str().ok_or("missing program version")?;
    super::layout::instruction_count(parse::array(node, "instructions")?.len())?;

    if parse::array(node, "instructions")?.is_empty() {
        return Err("empty complete program instruction collection".into());
    }

    let recipes = pda_policy::recipes(parse::array(node, "pdas")?)?;
    let defaults = pda_policy::defaults(parse::array(node, "instructions")?, &recipes)?;
    optional_defaults(parse::array(node, "instructions")?, &defaults)?;
    let mut definitions = Vec::new();

    for item in parse::array(node, "definedTypes")? {
        super::data_shapes::definition(item, &mut definitions)?;
    }

    let accounts = parse::array(node, "accounts")?;
    unique_account_headers(accounts)?;

    for account in accounts {
        definitions.push(account_record(account, &recipes)?);
    }

    let events = parse::array(node, "events")?;
    unique_events(events)?;

    for event in events {
        definitions.push(event_record(event)?);
    }

    let program = Program {
        name: parse::name(node)?,
        address: policy.address,
        definitions,
        instructions: parse::array(node, "instructions")?
            .iter()
            .map(instruction)
            .collect::<Result<_, _>>()?,
        accounts: Vec::new(),
        errors: parse::array(node, "errors")?
            .iter()
            .map(parse::program_error)
            .collect::<Result<_, _>>()?,
    };
    resolver_names(&program)?;
    super::layout::validate_data(&program)?;
    let mut output = super::emit::program(&program)?;
    output.push_str(&super::traits_output::emit(&program)?);
    output.push_str(&super::pda_output::emit(&program, &recipes, &defaults)?);
    Ok(output)
}

fn program_node<'a>(
    root: &'a Value,
    policy: &Policy<'_>,
    public: bool,
) -> Result<&'a Value, Error> {
    super::data_nodes::validate(root)?;
    parse::keys(
        root,
        &[
            "kind",
            "standard",
            "version",
            "program",
            "additionalPrograms",
        ],
    )?;
    let node = &root["program"];
    parse::keys(
        node,
        &[
            "kind",
            "name",
            "publicKey",
            "version",
            "pdas",
            "accounts",
            "instructions",
            "definedTypes",
            "errors",
            "events",
        ],
    )?;

    if root["kind"] != "rootNode"
        || root["standard"] != "codama"
        || !(root["version"] == "1.0.0" || public && root["version"] == "1.8.0")
        || !parse::array(root, "additionalPrograms")?.is_empty()
        || node["kind"] != "programNode"
        || node["name"] != policy.name
        || node["publicKey"] != policy.address.to_string()
    {
        return Err("untrusted complete Codama program".into());
    }

    Ok(node)
}

/// Omitted roles cannot secretly become mandatory inputs of another default resolver.
fn optional_defaults(
    instructions: &[Value],
    defaults: &[pda_policy::RoleDefault],
) -> Result<(), Error> {
    for instruction in instructions {
        instruction_defaults(instruction, defaults)?;
    }

    Ok(())
}

fn instruction_defaults(
    instruction: &Value,
    defaults: &[pda_policy::RoleDefault],
) -> Result<(), Error> {
    let optional = parse::array(instruction, "accounts")?
        .iter()
        .filter(|role| role["isOptional"] == true)
        .map(|role| role["name"].as_str().ok_or("missing optional role name"))
        .collect::<Result<std::collections::BTreeSet<_>, _>>()?;

    for default in defaults
        .iter()
        .filter(|item| instruction["name"] == item.instruction)
    {
        if optional.contains(default.role.as_str()) {
            return Err(
                "default on an omitted optional role needs an explicit resolver policy".into(),
            );
        }

        if let pda_policy::Default::Pda { accounts, .. } = &default.value
            && accounts.iter().any(|name| optional.contains(name.as_str()))
        {
            return Err("PDA default depends on an omitted optional role".into());
        }
    }

    Ok(())
}

fn record(node: &Value) -> Result<Vec<Field>, Error> {
    parse::keys(node, &["kind", "fields"])?;

    if node["kind"] != "structTypeNode" {
        return Err("account/event requires a complete record".into());
    }

    let fields = parse::array(node, "fields")?;

    if fields.iter().any(|field| {
        field.get("defaultValue").is_some() || field.get("defaultValueStrategy").is_some()
    }) {
        return Err("record field defaults require an explicit typed contract".into());
    }

    data::fields(fields, "structFieldTypeNode")
}

fn account_record(node: &Value, recipes: &[pda_policy::Pda]) -> Result<Definition, Error> {
    parse::keys(node, ACCOUNT_KEYS)?;

    if node["kind"] != "accountNode" {
        return Err("unexpected account node".into());
    }

    account_pda(node, recipes)?;
    let fields = record(&node["data"])?;
    let discriminator = node
        .get("discriminators")
        .map(|_| event_header(node))
        .transpose()?;

    if discriminator.is_none() && node.get("pda").is_none() {
        return Err("account requires an explicit discriminator or PDA contract".into());
    }

    Ok(Definition {
        name: parse::name(node)?,
        shape: Shape::Struct(fields),
        discriminator,
    })
}

fn account_pda(node: &Value, recipes: &[pda_policy::Pda]) -> Result<(), Error> {
    let Some(pda) = node.get("pda") else {
        return Ok(());
    };
    parse::keys(pda, &["kind", "name"])?;

    if pda["kind"] != "pdaLinkNode" || !recipes.iter().any(|recipe| pda["name"] == recipe.name) {
        return Err("unresolved account PDA contract".into());
    }

    Ok(())
}

fn unique_account_headers(accounts: &[Value]) -> Result<(), Error> {
    let mut headers: Vec<Vec<u8>> = Vec::new();

    for account in accounts
        .iter()
        .filter(|node| node.get("discriminators").is_some())
    {
        let header = event_header(account)?;

        if headers
            .iter()
            .any(|other| header.starts_with(other) || other.starts_with(&header))
        {
            return Err("ambiguous account discriminator prefix".into());
        }

        headers.push(header);
    }

    Ok(())
}

/// Constant discriminators constrain the declared layout without inserting bytes.
fn event_record(node: &Value) -> Result<Definition, Error> {
    parse::keys(node, EVENT_KEYS)?;

    if node["kind"] != "eventNode" {
        return Err("unexpected event kind".into());
    }

    let fields = record(&node["data"])?;
    let bytes = event_header(node)?;
    Ok(Definition {
        name: parse::name(node)?,
        shape: Shape::Struct(fields),
        discriminator: Some(bytes),
    })
}

pub(super) fn event_header(node: &Value) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new();

    for item in parse::array(node, "discriminators")? {
        let text = event_constant(item, bytes.len())?;

        if text.is_empty() || !text.is_ascii() || !text.len().is_multiple_of(2) {
            return Err("invalid event header hex".into());
        }

        for pair in text.as_bytes().as_chunks::<2>().0 {
            bytes.push(u8::from_str_radix(core::str::from_utf8(pair)?, HEX_RADIX)?);
        }
    }

    super::instruction_header::Header::new(bytes.clone())?;
    Ok(bytes)
}

fn resolver_names(program: &Program) -> Result<(), Error> {
    for instruction in &program.instructions {
        let generated = format!("{}Accounts", instruction.name);

        if program
            .definitions
            .iter()
            .any(|item| item.name == generated)
            || program
                .instructions
                .iter()
                .any(|item| item.name == generated)
        {
            return Err("generated account resolver namespace collision".into());
        }
    }

    Ok(())
}

fn event_constant(item: &Value, offset: usize) -> Result<&str, Error> {
    parse::keys(item, &["kind", "constant", "offset"])?;
    let value = &item["constant"];
    parse::keys(value, &["kind", "type", "value"])?;
    parse::keys(&value["type"], &["kind"])?;
    parse::keys(&value["value"], &["kind", "data", "encoding"])?;

    if item["kind"] != "constantDiscriminatorNode"
        || item["offset"].as_u64() != Some(u64::try_from(offset)?)
        || value["kind"] != "constantValueNode"
        || value["type"]["kind"] != "bytesTypeNode"
        || value["value"]["kind"] != "bytesValueNode"
        || value["value"]["encoding"] != "base16"
    {
        return Err("event header requires contiguous literal byte constants".into());
    }

    value["value"]["data"]
        .as_str()
        .ok_or("missing event bytes".into())
}

fn unique_events(events: &[Value]) -> Result<(), Error> {
    let mut headers = Vec::new();

    for event in events {
        let header = event_header(event)?;

        if headers
            .iter()
            .any(|other: &Vec<u8>| header.starts_with(other) || other.starts_with(&header))
        {
            return Err("ambiguous event discriminator prefix".into());
        }

        headers.push(header);
    }

    Ok(())
}

/// Role defaults were validated and are retained separately for actual resolver emission.
fn instruction(node: &Value) -> Result<Instruction, Error> {
    parse::keys(
        node,
        &[
            "kind",
            "name",
            "accounts",
            "arguments",
            "discriminators",
            "optionalAccountStrategy",
            "docs",
            "remainingAccounts",
        ],
    )?;

    if node["kind"] != "instructionNode" {
        return Err("unexpected complete program instruction".into());
    }

    let strategy = match node.get("optionalAccountStrategy") {
        None => "programId",
        Some(value) => value.as_str().ok_or("optional strategy must be text")?,
    };

    if !["omitted", "programId"].contains(&strategy) {
        return Err("unsupported optional account strategy".into());
    }

    let mut accounts = parse::array(node, "accounts")?.to_vec();

    for account in &mut accounts {
        account
            .as_object_mut()
            .ok_or("invalid instruction account")?
            .remove("defaultValue");
        let object = account
            .as_object_mut()
            .ok_or("invalid instruction account")?;
        object.entry("isOptional").or_insert(Value::Bool(false));
    }

    let arguments = parse::array(node, "arguments")?;
    let (tag, skip) = super::tags::parse(node, arguments)?;
    Ok(Instruction {
        name: parse::name(node)?,
        tag: super::instruction_header::Header::new(tag.ok_or("missing instruction header")?)?,
        fields: parse::fields(&arguments[skip..], "instructionArgumentNode")?,
        roles: super::roles::parse(&accounts)?,
        remaining: super::roles::remaining(node)?,
        omit_optional: strategy == "omitted",
    })
}
