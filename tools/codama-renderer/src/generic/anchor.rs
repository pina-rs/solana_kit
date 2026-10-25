//! Strict legacy Anchor preparation; no byte layout is inferred from instruction order.
use super::{Policy, anchor_types, parse};
use crate::schema::{Error, identifier};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const DISCRIMINATOR_BYTES: usize = 8;

/// Metadata absent from old IDLs must come from reviewed pinned program sources.
pub(crate) struct AnchorPolicy<'a> {
    pub(crate) program: Policy<'a>,
    pub(crate) source_name: &'a str,
    pub(crate) remaining_proofs: &'a [&'a str],
}

/// Returns a complete canonical tree without discarding definitions, errors or documentation.
pub(crate) fn prepare(root: &Value, policy: &AnchorPolicy<'_>) -> Result<Value, Error> {
    validate_identity(root, policy)?;

    let instructions = parse::array(root, "instructions")?
        .iter()
        .map(|node| instruction(node, policy))
        .collect::<Result<Vec<_>, _>>()?;
    validate_proof_names(&instructions, policy.remaining_proofs)?;
    let definitions = parse::array(root, "types")?
        .iter()
        .map(anchor_types::definition)
        .collect::<Result<Vec<_>, _>>()?;
    let errors = parse::array(root, "errors")?
        .iter()
        .map(program_error)
        .collect::<Result<Vec<_>, _>>()?;
    let prepared = json!({"kind":"rootNode","standard":"codama","version":"1.0.0",
        "additionalPrograms":[],"program":{"kind":"programNode","name":policy.program.name,
        "publicKey":policy.program.address.to_string(),"version":root["version"],
        "origin":"anchor","prefix":"","pdas":[],"accounts":[],
        "definedTypes":definitions,"instructions":instructions,"errors":errors}});
    super::data_nodes::validate(&prepared)?;
    super::prepared_layouts::validate(&prepared)?;
    Ok(prepared)
}

fn validate_identity(root: &Value, policy: &AnchorPolicy<'_>) -> Result<(), Error> {
    parse::keys(
        root,
        &[
            "version",
            "name",
            "instructions",
            "types",
            "errors",
            "metadata",
        ],
    )?;
    parse::keys(
        &root["metadata"],
        &["address", "origin", "binaryVersion", "libVersion"],
    )?;

    if root["name"] != policy.source_name
        || root["metadata"]["address"] != policy.program.address.to_string()
        || root["metadata"]["origin"] != "anchor"
    {
        return Err("untrusted Anchor program identity or origin".into());
    }

    root["version"].as_str().ok_or("missing Anchor version")?;

    for key in ["binaryVersion", "libVersion"] {
        root["metadata"][key]
            .as_str()
            .ok_or("missing Anchor provenance version")?;
    }

    Ok(())
}

fn validate_proof_names(instructions: &[Value], proofs: &[&str]) -> Result<(), Error> {
    let mut seen = std::collections::BTreeSet::new();

    for name in proofs {
        if !seen.insert(name) || !instructions.iter().any(|item| item["name"] == *name) {
            return Err("unknown or duplicate source proof policy".into());
        }
    }

    Ok(())
}

pub(super) fn docs(source: &Value, output: &mut Value) -> Result<(), Error> {
    let Some(docs) = source.get("docs") else {
        return Ok(());
    };

    if !docs
        .as_array()
        .is_some_and(|items| items.iter().all(Value::is_string))
    {
        return Err("documentation must contain only strings".into());
    }

    output["docs"] = docs.clone();
    Ok(())
}

fn instruction(node: &Value, policy: &AnchorPolicy<'_>) -> Result<Value, Error> {
    parse::keys(node, &["name", "docs", "accounts", "args"])?;
    let name = node["name"]
        .as_str()
        .ok_or("missing Anchor instruction name")?;
    let rust_name = identifier(name)?;
    let digest = Sha256::digest(format!("global:{rust_name}"));
    let header = digest[..DISCRIMINATOR_BYTES]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    let mut arguments = vec![
        json!({"kind":"instructionArgumentNode","name":"discriminator",
        "type":{"kind":"fixedSizeTypeNode","size":DISCRIMINATOR_BYTES,"type":{"kind":"bytesTypeNode"}},
        "defaultValue":{"kind":"bytesValueNode","encoding":"base16","data":header},
        "defaultValueStrategy":"omitted"}),
    ];

    for field in parse::array(node, "args")? {
        arguments.push(anchor_types::field(field, "instructionArgumentNode")?);
    }

    let accounts = parse::array(node, "accounts")?
        .iter()
        .map(account)
        .collect::<Result<Vec<_>, _>>()?;
    let mut output = json!({"kind":"instructionNode","name":name,"accounts":accounts,
        "arguments":arguments,"optionalAccountStrategy":"programId",
        "discriminators":[{"kind":"fieldDiscriminatorNode","name":"discriminator","offset":0}]});

    if policy.remaining_proofs.contains(&name) {
        output["remainingAccounts"] = json!([{"kind":"instructionRemainingAccountsNode",
            "value":{"kind":"argumentValueNode","name":"proof"},
            "isOptional":true,"isSigner":false,"isWritable":false}]);
    }

    docs(node, &mut output)?;
    Ok(output)
}

fn account(node: &Value) -> Result<Value, Error> {
    parse::keys(node, &["name", "isMut", "isSigner", "docs"])?;
    let name = node["name"]
        .as_str()
        .ok_or("missing Anchor account role name")?;
    identifier(name)?;
    let writable = node["isMut"]
        .as_bool()
        .ok_or("missing Anchor writable flag")?;
    let signer = node["isSigner"]
        .as_bool()
        .ok_or("missing Anchor signer flag")?;
    let mut output = json!({"kind":"instructionAccountNode","name":name,
        "isWritable":writable,"isSigner":signer,"isOptional":false});
    docs(node, &mut output)?;
    Ok(output)
}

fn program_error(node: &Value) -> Result<Value, Error> {
    parse::keys(node, &["name", "code", "msg"])?;
    let output = json!({"kind":"errorNode","name":node["name"],"code":node["code"],
        "message":node["msg"]});
    parse::program_error(&output)?;
    Ok(output)
}
