//! Authored syntax fixtures and negative controls exercise observable ledger rules.

use crate::{ledger::compare, model::Export, parse, typescript};

#[test]
fn extracts_public_members_and_excludes_executable_locals() {
    let source = "class Box { Box(); Box.named(); final int value = 1; int get size => value; set size(int n) {} int call(int x) { final local = x; return local; } void _private() {} } bool publicFunction() => true;";
    let parsed = parse::dart(source).expect("valid fixture");
    let names: Vec<_> = parsed
        .declarations
        .iter()
        .map(|item| item.symbol.as_str())
        .collect();
    assert!(parsed.warnings.is_empty(), "{:?}", parsed.warnings);

    for name in [
        "Box",
        "Box.new",
        "Box.named",
        "Box.value",
        "Box.size",
        "Box.call",
        "publicFunction",
    ] {
        assert!(names.contains(&name), "missing {name}: {names:?}");
    }
    assert!(
        !names
            .iter()
            .any(|name| name.contains("private") || name.contains("local"))
    );
}

#[test]
fn exports_preserve_show_and_hide_combinators() {
    let parsed =
        parse::dart("export 'api.dart' show Box, value hide value;").expect("valid export");
    let export = &parsed.exports[0];
    assert!(export.includes("Box"));
    assert!(!export.includes("value"));
    assert!(!export.includes("Other"));
    let open = Export {
        target: "api.dart".into(),
        show: Default::default(),
        hide: Default::default(),
    };
    assert!(open.includes("Other"));
}

#[test]
fn unfamiliar_syntax_never_becomes_a_clean_coverage_claim() {
    let parsed = parse::dart("class Broken { void go( }").expect("parser returns an error tree");
    assert!(!parsed.warnings.is_empty());
    let conditional = parse::dart("export 'stub.dart' if (dart.library.io) 'native.dart';")
        .expect("conditional syntax");
    assert!(
        conditional
            .warnings
            .iter()
            .any(|warning| warning.starts_with("conditional-export"))
    );
}

#[test]
fn typescript_reexports_and_public_interface_members_are_extracted() {
    let parsed = typescript::parse("export { render } from './render.js'; export interface Options { name: string; run(): void; }").expect("valid TypeScript");
    assert!(parsed.warnings.is_empty(), "{:?}", parsed.warnings);
    assert_eq!(parsed.exports[0].target, "./render.ts");
    assert!(parsed.exports[0].includes("render"));
    assert!(
        parsed
            .declarations
            .iter()
            .any(|item| item.symbol == "Options.run")
    );
    assert!(
        parsed
            .declarations
            .iter()
            .any(|item| item.symbol == "Options.name")
    );
}

#[test]
fn rejects_deleted_duplicate_and_fake_verified_evidence_rows() {
    let expected = "id\tstatus\tevidence\na\tpending\t-\nb\tpending\t-\n";
    assert!(compare(expected, expected, "capability").is_ok());
    assert!(
        compare(
            "id\tstatus\tevidence\na\tpending\t-\n",
            expected,
            "capability"
        )
        .is_err()
    );
    assert!(
        compare(
            &format!("{expected}a\tpending\t-\n"),
            expected,
            "capability"
        )
        .is_err()
    );
    let fake = expected.replace("a\tpending\t-", "a\tverified\tnonexistent-evidence.txt");
    assert!(compare(&fake, expected, "capability").is_err());
}

#[test]
fn rejects_absent_pilot_evidence() {
    assert!(
        crate::ledger::validate_pilot(std::path::Path::new(env!("CARGO_MANIFEST_DIR"))).is_err()
    );
}

#[test]
fn rejects_missing_source_rows_and_reference_checkout() {
    let expected = "package\tpath\nexample\tlib/api.dart\nexample\tlib/types.dart\n";
    let missing = "package\tpath\nexample\tlib/api.dart\n";
    assert!(compare(missing, expected, "source file").is_err());
    let duplicate = format!("{expected}example\tlib/api.dart\n");
    assert!(compare(&duplicate, expected, "source file").is_err());
    let absent = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    assert!(crate::verify_pin(&absent).is_err());
}

/// Private syntax never exports members, even when public names also contain a hash.
#[test]
fn typescript_excludes_private_identifiers_but_keeps_public_hash_names() {
    let source = r##"
        export class Example {
            #field: string = "";
            static #staticField: number = 0;
            #method(): void {}
            get #accessor(): string { return this.#field; }
            private hidden(): void {}
            protected guarded(): void {}
            visible(): void { class Local { leaked(): void {} } }
            ["#computed"](): void {}
            "#quoted": string = "";
        }
    "##;
    let parsed = typescript::parse(source).expect("valid private and public names");
    let names: Vec<_> = parsed
        .declarations
        .iter()
        .map(|item| item.symbol.as_str())
        .collect();

    assert!(parsed.warnings.is_empty(), "{:?}", parsed.warnings);
    assert_eq!(
        names,
        [
            "Example",
            "Example.visible",
            "Example.[\"#computed\"]",
            "Example.\"#quoted\""
        ]
    );
}
