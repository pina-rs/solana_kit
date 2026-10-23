//! Compare every portable body and helper predicate against the ordinary consumer.
use std::{collections::BTreeSet, env, fs, path::Path};

const CONTRACTS: usize = 54;

/// Keep quoted bytes and escapes intact so formatting cannot hide changed fixtures.
fn normalized(source: &str) -> String {
    let mut quoted = false;
    let mut escaped = false;
    let mut result = String::new();

    for character in source.chars() {
        if character == '"' && !escaped {
            quoted = !quoted;
        }

        if quoted || !character.is_whitespace() {
            result.push(character);
        }

        escaped = character == '\\' && !escaped;
    }

    result
}

fn body<'a>(source: &'a str, name: &str) -> &'a str {
    let start = source.find(&format!("fn {name}(")).unwrap();
    let open = start + source[start..].find('{').unwrap() + 1;
    let end = open + source[open..].find("\n}").unwrap();
    &source[open..end]
}

fn original_includes(source: &str) -> String {
    source
        .replace("[] as [u8; 0]", "[]")
        .replace(
            "../../../../crates/solana_kit_core/tests/fixtures/",
            "fixtures/",
        )
        .replace("../../../../docs/", "../../../docs/")
        .replace(
            "../../../codec-framing/",
            "../../../verification/codec-framing/",
        )
}

fn ordinary_source(adapted: &str) -> String {
    let lines: Vec<_> = adapted
        .lines()
        .filter(|line| {
            !matches!(
                *line,
                "use alloc::format;"
                    | "use alloc::vec;"
                    | "use alloc::vec::Vec;"
                    | "use alloc::string::ToString;"
                    | "use alloc::string::{String, ToString};"
                    | "extern crate std;"
            )
        })
        .collect();
    original_includes(&lines.join("\n"))
        .replace("use alloc::collections::", "use std::collections::")
        .replace(
            "use crate::composites as portable;",
            "#[path = \"../../../verification/conformance/src/composites.rs\"]\nmod portable;",
        )
        .replace(
            "use crate::framing as portable;",
            "#[path = \"../../../verification/conformance/src/framing.rs\"]\nmod portable;",
        )
        .replace(
            "#[path = \"../composites.rs\"]",
            "#[path = \"../../../verification/conformance/src/composites.rs\"]",
        )
        .replace(
            "#[path = \"../framing.rs\"]",
            "#[path = \"../../../verification/conformance/src/framing.rs\"]",
        )
        .replace("pub(super) fn ", "#[test]\nfn ")
}

fn predicates(source: &str) -> usize {
    ["assert!", "assert_eq!", "assert_ne!"]
        .iter()
        .map(|marker| source.matches(marker).count())
        .sum()
}

fn audit(root: &Path, registry: &str, row: &str) -> ((String, String), usize) {
    let fields: Vec<_> = row.split('\t').collect();
    assert_eq!(fields.len(), 4);
    let [module, name, path, classification] = fields[..] else {
        unreachable!()
    };
    assert_eq!(classification, "portable");
    let original = fs::read_to_string(root.join(path)).unwrap();
    let relative = format!("verification/conformance/src/collection_cases/{module}.rs");
    let adapted = fs::read_to_string(root.join(relative)).unwrap();

    assert_eq!(
        normalized(&original),
        normalized(&ordinary_source(&adapted)),
        "full module including helpers {module}"
    );
    assert_eq!(registry.matches(&format!("execute({name})")).count(), 1);
    let expected = body(&original, name);
    let actual = original_includes(body(&adapted, name));
    assert_eq!(
        normalized(expected),
        normalized(&actual),
        "body {module}::{name}"
    );
    let count = predicates(expected);
    println!("{module}\t{name}\tfull original body exact\t{count}");
    ((module.to_owned(), name.to_owned()), count)
}

fn main() {
    let argument = env::args().nth(1).unwrap();
    let root = Path::new(&argument);
    let table =
        fs::read_to_string(root.join("verification/collection-layouts/runtime-map.tsv")).unwrap();
    let registry =
        fs::read_to_string(root.join("verification/conformance/src/collection_cases/registry.rs"))
            .unwrap();
    let mut seen = BTreeSet::new();
    let mut assertions = 0;

    for row in table.lines().skip(1) {
        let (identity, count) = audit(root, &registry, row);
        assert!(seen.insert(identity));
        assertions += count;
    }

    assert_eq!(seen.len(), CONTRACTS);
    let modules: BTreeSet<_> = seen.iter().map(|(module, _)| module).collect();
    println!(
        "{CONTRACTS} complete bodies; {} complete modules with helpers",
        modules.len()
    );
    println!("{assertions} ordered body predicates; no native-only exclusions");
}
