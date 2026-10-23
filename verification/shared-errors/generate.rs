use std::{env, fs, path::Path};

const GROUP_SIZE: usize = 24;
const HEX_RADIX: u32 = 16;

fn main() {
    let root = env::args().nth(1).expect("owned worktree");
    let root = Path::new(&root);
    let rows = fs::read_to_string("oracle.tsv").expect("Dart oracle");
    let directory = root.join("crates/solana_kit_core/src/errors/catalog");
    fs::create_dir_all(&directory).expect("catalog directory");
    let rows: Vec<_> = rows
        .lines()
        .map(|line| line.split('\t').collect::<Vec<_>>())
        .collect();
    let mut modules = String::from(
        "//! Pinned code definitions and message templates.\nuse super::{CodeDefinition, ErrorCode};\n",
    );
    let mut slices = Vec::new();
    let mut names = Vec::new();

    for (index, group) in rows.chunks(GROUP_SIZE).enumerate() {
        let name = format!("group_{index}");
        names.push(name.clone());
        slices.push(format!("{name}::DEFINITIONS"));
        fs::write(directory.join(format!("{name}.rs")), emit_group(group)).expect("catalog source");
    }

    names.sort();

    for name in names {
        modules.push_str(&format!("mod {name};\n"));

        if name == "group_2" {
            modules.push('\n');
        }
    }

    modules.push_str(&format!("const GROUPS: &[&[CodeDefinition]] = &[{}];\npub(super) fn definitions() -> impl Iterator<Item = &'static CodeDefinition> {{ GROUPS.iter().flat_map(|group| group.iter()) }}\n",slices.join(",")));
    fs::write(directory.join("mod.rs"), modules).expect("catalog root");
    fs::create_dir_all(root.join("crates/solana_kit_core/tests/shared_errors"))
        .expect("fixture directory");
    fs::copy(
        "oracle.tsv",
        root.join("crates/solana_kit_core/tests/shared_errors/catalog.tsv"),
    )
    .expect("oracle fixture");
    println!("generated {} exact definitions", rows.len());
}

fn emit_group(group: &[Vec<&str>]) -> String {
    let mut source = String::from(
        "//! Generated from the pinned Dart error oracle.\nuse super::{CodeDefinition, ErrorCode};\nimpl ErrorCode {\n",
    );

    for row in group {
        let constant = constant_name(row[0]);
        source.push_str(&format!(
            "/// Pinned `{}` code.\npub const {constant}: Self = Self({});\n",
            row[0], row[1]
        ));
    }

    source.push_str("}\npub(super) const DEFINITIONS: &[CodeDefinition] = &[\n");

    for row in group {
        source.push_str(&format!(
            "CodeDefinition {{ code: ErrorCode({}), name: {:?}, template: {:?} }},\n",
            row[1],
            row[0],
            decode_message(row[3])
        ));
    }

    source.push_str("];\n");
    source
}

fn decode_message(hex: &str) -> String {
    let bytes: Vec<_> = hex
        .as_bytes()
        .chunks(2)
        .map(|pair| {
            u8::from_str_radix(std::str::from_utf8(pair).expect("hex pair"), HEX_RADIX)
                .expect("message hex")
        })
        .collect();
    String::from_utf8(bytes).expect("message UTF-8")
}

fn constant_name(name: &str) -> String {
    let mut output = String::new();

    for character in name.chars() {
        if character.is_ascii_uppercase() {
            output.push('_');
        }

        output.push(character.to_ascii_uppercase());
    }

    output
}
