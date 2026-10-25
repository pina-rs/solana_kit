//! Hoists source tuple payloads without changing field order or wire types.
use crate::schema::Error;
use serde_json::{Value, json};

pub(super) fn project(original: &Value) -> Result<Value, Error> {
    if original["version"] != "1.9.2" || original["program"]["name"] != "stake" {
        return Err("unsupported Stake source identity".into());
    }

    let mut root = original.clone();
    let mut additions = Vec::new();

    for definition in root["program"]["definedTypes"]
        .as_array_mut()
        .ok_or("missing types")?
    {
        definition_payloads(definition, &mut additions)?;
    }

    root["program"]["definedTypes"]
        .as_array_mut()
        .ok_or("missing definitions")?
        .extend(additions);
    Ok(root)
}

fn definition_payloads(definition: &mut Value, additions: &mut Vec<Value>) -> Result<(), Error> {
    let owner = definition["name"]
        .as_str()
        .ok_or("missing type name")?
        .to_owned();

    let Some(variants) = definition["type"]
        .get_mut("variants")
        .and_then(Value::as_array_mut)
    else {
        return Ok(());
    };

    for variant in variants {
        hoist(variant, &owner, additions)?;
    }

    Ok(())
}

fn hoist(variant: &mut Value, owner: &str, additions: &mut Vec<Value>) -> Result<(), Error> {
    if variant["kind"] != "enumTupleVariantTypeNode" {
        return Ok(());
    }

    let items = variant["tuple"]["items"]
        .as_array()
        .ok_or("missing enum tuple")?;

    if items.len() <= 1 {
        return Ok(());
    }

    let mut label = variant["name"]
        .as_str()
        .ok_or("missing variant")?
        .to_owned();
    label[0..1].make_ascii_uppercase();
    let name = format!("{owner}{label}Payload");
    let fields: Vec<_> = items
        .iter()
        .enumerate()
        .map(|(index, item)| field(index, item))
        .collect();
    additions.push(json!({
        "kind":"definedTypeNode",
        "name":name,
        "type":{"kind":"structTypeNode","fields":fields}
    }));
    variant["tuple"]["items"] = json!([{"kind":"definedTypeLinkNode","name":name}]);

    Ok(())
}

fn field(index: usize, item: &Value) -> Value {
    json!({
        "kind":"structFieldTypeNode",
        "name":format!("item{index}"),
        "type":item
    })
}
