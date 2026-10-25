//! Exact source account metadata and typed optional inputs.
use crate::schema::{Error, identifier};
use serde_json::Value;
use std::{fs, path::Path};

pub(super) type Role = (String, String, bool);

pub(super) fn roles(node: &Value, owner: &str) -> Result<Vec<Role>, Error> {
    if owner == "stake" {
        let name = identifier(node["name"].as_str().ok_or("missing name")?)?;
        let source = fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("fixtures/stake-loader/dart-stake")
                .join(format!("{name}.dart.txt")),
        )?;

        return dart_roles(&source);
    }

    node["accounts"]
        .as_array()
        .ok_or("missing accounts")?
        .iter()
        .map(|role| {
            let name = role["name"]
                .as_str()
                .ok_or("missing account name")?
                .to_owned();
            let writable = role["isWritable"]
                .as_bool()
                .ok_or("missing writable role")?;
            let signer = role["isSigner"].as_bool().ok_or("missing signer role")?;
            let role_name = match (signer, writable) {
                (true, true) => "WritableSigner",
                (true, false) => "ReadonlySigner",
                (false, true) => "Writable",
                (false, false) => "Readonly",
            };

            Ok((
                name,
                role_name.to_owned(),
                role["isOptional"]
                    .as_bool()
                    .ok_or("missing optional role")?,
            ))
        })
        .collect()
}

/// Reads only the actual builder's concrete AccountMeta expressions and typed optionals.
fn dart_roles(source: &str) -> Result<Vec<Role>, Error> {
    let builder = source
        .split("Instruction get")
        .nth(1)
        .ok_or("missing Dart builder")?;
    let builder = builder
        .split("InstructionData parse")
        .next()
        .ok_or("missing builder body")?;
    let mut output = Vec::new();

    for line in builder.lines() {
        let Some(meta) = line.trim().strip_prefix("AccountMeta(address: ") else {
            continue;
        };
        let (name, role) = meta
            .split_once(", role: AccountRole.")
            .ok_or("unsupported Dart role expression")?;
        let role = role
            .strip_suffix("),")
            .ok_or("unsupported Dart role suffix")?;

        let role = match role {
            "writable" => "Writable",
            "readonly" => "Readonly",
            "writableSigner" => "WritableSigner",
            "readonlySigner" => "ReadonlySigner",
            _ => return Err("unknown Dart account role".into()),
        };

        output.push((
            name.to_owned(),
            role.to_owned(),
            builder.contains(&format!("Address? {name},")),
        ));
    }

    Ok(output)
}
