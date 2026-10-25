//! Calls every typed builder against independent original Dart case-zero literals.
use super::{
    Result,
    roles::{self, Role},
    snake, text, title,
};
use serde_json::Value;

pub(super) fn render(items: &[Value]) -> Result<String> {
    let rows = include_str!("../../fixtures/token-2022-oracle.tsv");
    let mut out = String::from(
        "//! Independent original Dart builder literals.\nuse solana_kit_core::{Address,AccountMeta,AccountRole,codecs::{Decoder,Limits}};\nuse solana_kit_token_2022::*;\nconst LIMITS:Limits=Limits::new(100_000,4096);\n",
    );

    for item in items {
        out.push_str(&instruction(item, rows)?);
    }

    Ok(out)
}

fn instruction(item: &Value, rows: &str) -> Result<String> {
    let raw = text(item, "name")?;
    let name = title(raw);
    let key = format!("{raw}-0\t");
    let row = rows
        .lines()
        .find(|line| line.starts_with(&key))
        .ok_or("missing independent row")?;
    let fields: Vec<_> = row.split('\t').collect();
    let metas = fields[3]
        .split(',')
        .filter(|value| !value.is_empty())
        .map(meta)
        .collect::<Result<Vec<_>>>()?;
    let bytes = fields[1]
        .as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| Ok(u8::from_str_radix(std::str::from_utf8(pair)?, HEX_RADIX)?))
        .collect::<Result<Vec<u8>>>()?;
    let inputs = inputs(item, &metas)?;

    let expected=metas.iter().map(|(address,bits)|{
        let signer=bits&2!=0;
        let writable=bits&1!=0;
        format!("AccountMeta {{address:{address:?}.parse().unwrap(),role:AccountRole::from_privileges({signer},{writable})}}")
    }).collect::<Vec<_>>().join(",");
    Ok(format!(
        "#[test]\nfn {}_matches_dart_literal() {{\nlet limits=LIMITS;\nlet bytes={bytes:?};\n\nlet Token2022InstructionData::{name}(value)=Token2022InstructionDataCodec.decode(&bytes,limits).unwrap() else {{panic!(\"wrong variant\")}};\n\nlet instruction=value.instruction_with_program({:?}.parse().unwrap(),{name}Accounts {{{inputs}}},limits).unwrap();\nassert_eq!(instruction.data,bytes);\nassert_eq!(instruction.accounts,vec![{expected}]);\n}}\n",
        snake(raw),
        fields[2]
    ))
}

const HEX_RADIX: u32 = 16;
fn meta(value: &str) -> Result<(&str, u8)> {
    let (address, role) = value.rsplit_once(':').ok_or("invalid literal meta")?;
    Ok((address, role.parse()?))
}

/// Optional case-zero roles are omitted; all required metas come from Dart literals.
fn inputs(item: &Value, metas: &[(&str, u8)]) -> Result<String> {
    let mut inputs = Vec::new();
    let mut position = 0;

    for node in item["accounts"].as_array().ok_or("missing roles")? {
        let role = roles::parse(node)?;
        let value = if role.optional {
            "None".to_owned()
        } else {
            let (address, bits) = metas.get(position).ok_or("missing required literal meta")?;
            position += 1;
            input_value(&role, address, *bits)
        };

        inputs.push(format!("{}:{value}", role.name));
    }

    if position != metas.len() {
        return Err("unexpected case-zero remaining metas".into());
    }

    for node in item["remainingAccounts"].as_array().into_iter().flatten() {
        inputs.push(format!("{}:&[]", roles::remaining(node)?.name));
    }

    Ok(inputs.join(","))
}

fn input_value(role: &Role, address: &str, bits: u8) -> String {
    let address = format!("{address:?}.parse::<Address>().unwrap()");

    if role.signer == "Either" {
        return format!(
            "Authority {{address:{address},is_signer:{}}}",
            bits & 2 != 0
        );
    }

    if role.default.is_some() {
        return format!("Some({address})");
    }

    address
}
