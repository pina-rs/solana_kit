//! Fixed full-domain reproducers plus accepted sparse, linked and compound shapes.
use crate::{generic::Policy, schema::Error, token};
use serde_json::{Value, json};
use solana_kit_core::Address;
type Case = (&'static str, Value, Policy<'static>);

pub(super) fn inputs() -> Result<Vec<Case>, Error> {
    let full_enum: Value =
        serde_json::from_str(include_str!("../../fixtures/style-full-enum.json"))?;
    let full_dispatch: Value =
        serde_json::from_str(include_str!("../../fixtures/style-full-dispatch.json"))?;
    let mut cases = Vec::new();
    cases.push(("full-enum", full_enum.clone(), boundary()));
    cases.push(("full-dispatch", full_dispatch.clone(), boundary()));
    large_dispatches(&full_dispatch, &mut cases)?;

    let mut sparse = full_dispatch.clone();
    let instructions = sparse["program"]["instructions"]
        .as_array_mut()
        .ok_or("instructions missing")?;
    let last = instructions
        .last()
        .cloned()
        .ok_or("last instruction missing")?;
    instructions.truncate(1);
    instructions.push(last);
    cases.push(("sparse", sparse, boundary()));

    let mut empty = full_dispatch;
    empty["program"]["instructions"]
        .as_array_mut()
        .ok_or("instructions missing")?
        .truncate(1);
    cases.push(("empty", empty, boundary()));

    let mut linked = full_enum;
    linked["program"]["definedTypes"][0]["type"]["variants"]
        .as_array_mut()
        .ok_or("variants missing")?
        .truncate(4);
    cases.push(("linked", linked, boundary()));
    cases.push(("token", token::input()?, token::policy()?));

    let roles = independent()?;
    cases.push(("role-only", roles.clone(), independent_policy()));
    cases.push(("compound", compound(roles), independent_policy()));
    Ok(cases)
}

/// Exercise sparse misses and account validation through the large-dispatch branch.
fn large_dispatches(root: &Value, cases: &mut Vec<Case>) -> Result<(), Error> {
    const SPARSE_COUNT: usize = 33;
    let mut sparse = root.clone();
    sparse["program"]["instructions"]
        .as_array_mut()
        .ok_or("instructions missing")?
        .truncate(SPARSE_COUNT);
    cases.push(("large-sparse", sparse, boundary()));

    let mut roles = root.clone();

    for item in roles["program"]["instructions"]
        .as_array_mut()
        .ok_or("instructions missing")?
    {
        item["accounts"] = json!([{
            "kind":"instructionAccountNode","name":"reader",
            "isWritable":false,"isSigner":false,"isOptional":false
        }]);
    }

    cases.push(("full-roles", roles, boundary()));
    Ok(())
}

fn boundary() -> Policy<'static> {
    Policy {
        name: "boundaryProbe",
        address: Address::from_bytes([93; 32]),
    }
}
fn independent_policy() -> Policy<'static> {
    Policy {
        name: "independent",
        address: Address::from_bytes([163; 32]),
    }
}
fn independent() -> Result<Value, Error> {
    let mut root: Value = serde_json::from_str(include_str!("../../fixtures/compute-budget.json"))?;
    root["program"]["name"] = json!("independent");
    root["program"]["publicKey"] = json!(Address::from_bytes([163; 32]).to_string());
    root["program"]["instructions"]
        .as_array_mut()
        .ok_or("instructions missing")?
        .truncate(1);

    let item = &mut root["program"]["instructions"][0];
    item["name"] = json!("exercise");
    item["arguments"]
        .as_array_mut()
        .ok_or("arguments missing")?
        .truncate(1);
    item["arguments"][0]["defaultValue"]["number"] = json!(163);

    item["accounts"] = json!([{
        "kind":"instructionAccountNode","name":"reader",
        "isWritable":false,"isSigner":false,"isOptional":false
    }]);
    Ok(root)
}
fn compound(mut root: Value) -> Value {
    root["program"]["instructions"][0]["arguments"].as_array_mut().expect("known fixture arguments").push(json!({
        "kind":"instructionArgumentNode","name":"payload","type":{"kind":"structTypeNode","fields":[
        {"kind":"structFieldTypeNode","name":"authority","type":{"kind":"optionTypeNode","fixed":false,"prefix":{"kind":"numberTypeNode","format":"u32","endian":"le"},"item":{"kind":"publicKeyTypeNode"}}},
        {"kind":"structFieldTypeNode","name":"members","type":{"kind":"arrayTypeNode","count":{"kind":"fixedCountNode","value":3},"item":{"kind":"publicKeyTypeNode"}}},
        {"kind":"structFieldTypeNode","name":"raw","type":{"kind":"sizePrefixTypeNode","prefix":{"kind":"numberTypeNode","format":"u8","endian":"le"},"type":{"kind":"bytesTypeNode"}}}
    ]}}));
    root
}
