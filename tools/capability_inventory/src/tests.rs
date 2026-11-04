//! Authored syntax fixtures and negative controls exercise observable ledger rules.

use crate::{
    adjudication::{self, Manifest, Status},
    ledger::compare,
    model::Export,
    parse, typescript,
};
use std::collections::BTreeMap;

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

const EVIDENCE: &str = "verification/conformance/evidence/native-success.txt";
const CASE: &str = "address-base58-golden";
const ALPHA: &str = "dart:demo:lib/demo.dart#alpha";
const BETA: &str = "dart:demo:lib/demo.dart#beta";
const HASH_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const HASH_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

fn repository() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn adjudication_row(id: &str, hash: &str, status: &str, evidence: &str, cases: &str) -> String {
    format!("{id}\t{hash}\t{status}\t{evidence}\t{cases}\treviewed note\treviewed decision")
}

fn adjudication_text(rows: &[String]) -> String {
    format!("{}\n{}\n", adjudication::HEADER, rows.join("\n"))
}

fn reviewed_manifest() -> String {
    adjudication_text(&[
        adjudication_row(ALPHA, HASH_A, "partial", EVIDENCE, CASE),
        adjudication_row(BETA, HASH_B, "complete", EVIDENCE, CASE),
    ])
}

fn canonical() -> BTreeMap<String, String> {
    BTreeMap::from([
        (ALPHA.to_owned(), HASH_A.to_owned()),
        (BETA.to_owned(), HASH_B.to_owned()),
    ])
}

fn review(text: &str) -> Result<(), String> {
    Manifest::parse(text, &repository())
        .and_then(|manifest| manifest.validate(&canonical()))
        .map_err(|error| error.to_string())
}

#[test]
fn adjudication_accepts_reviewed_partial_and_complete_rows() {
    let manifest = Manifest::parse(&reviewed_manifest(), &repository()).expect("valid manifest");
    manifest.validate(&canonical()).expect("canonical rows");

    let partial = manifest.claim(ALPHA, HASH_A).expect("known hash");
    assert_eq!(partial.map(|claim| claim.status), Some(Status::Partial));
    assert_eq!(partial.map(|claim| claim.evidence.as_str()), Some(EVIDENCE));

    let complete = manifest.claim(BETA, HASH_B).expect("known hash");
    assert_eq!(complete.map(|claim| claim.status), Some(Status::Complete));
    assert!(
        manifest
            .claim("dart:demo:lib/demo.dart#absent", HASH_A)
            .unwrap()
            .is_none()
    );
}

#[test]
fn rejects_unknown_and_duplicate_adjudication_ids() {
    let unknown = format!(
        "{}{}\n",
        reviewed_manifest(),
        adjudication_row("dart:demo:lib/demo.dart#gamma", HASH_B, "pending", "-", "-")
    );
    let error = review(&unknown).expect_err("unknown id must fail");
    assert!(error.contains("unknown adjudication id"), "{error}");

    let duplicate = format!(
        "{}{}\n",
        reviewed_manifest(),
        adjudication_row(ALPHA, HASH_A, "partial", EVIDENCE, CASE)
    );
    let error = review(&duplicate).expect_err("duplicate id must fail");
    assert!(error.contains("duplicate adjudication id"), "{error}");

    let bare_symbol =
        adjudication_text(&[adjudication_row("alpha", HASH_A, "partial", EVIDENCE, CASE)]);
    let error = review(&bare_symbol).expect_err("bare symbol must fail");
    assert!(error.contains("non-canonical adjudication id"), "{error}");
}

#[test]
fn rejects_adjudication_hash_drift() {
    let drifted = reviewed_manifest().replace(HASH_A, HASH_B);
    let error = review(&drifted).expect_err("hash drift must fail");
    assert!(error.contains("hash drift"), "{error}");

    let manifest = Manifest::parse(&reviewed_manifest(), &repository()).expect("valid manifest");
    let error = manifest
        .claim(ALPHA, HASH_B)
        .expect_err("claim must reject a different source hash")
        .to_string();
    assert!(error.contains("hash drift"), "{error}");
}

#[test]
fn rejects_absent_unsafe_and_prose_evidence() {
    let absent = reviewed_manifest().replace(
        EVIDENCE,
        "verification/conformance/evidence/absent-success.txt",
    );
    let error = review(&absent).expect_err("absent report must fail");
    assert!(error.contains("absent evidence path"), "{error}");

    let unsafe_path = reviewed_manifest().replace(EVIDENCE, "../outside-success.txt");
    let error = review(&unsafe_path).expect_err("parent traversal must fail");
    assert!(error.contains("unsafe evidence path"), "{error}");

    let absolute = reviewed_manifest().replace(EVIDENCE, "/tmp/native-success.txt");
    let error = review(&absolute).expect_err("absolute path must fail");
    assert!(error.contains("absolute evidence path"), "{error}");

    let prose = reviewed_manifest().replace(EVIDENCE, "docs/parity/README.md");
    let error = review(&prose).expect_err("prose is not execution evidence");
    assert!(error.contains("unverified case"), "{error}");
}

#[test]
fn rejects_complete_claims_without_an_executed_case() {
    let missing = reviewed_manifest().replace(
        &format!("\tcomplete\t{EVIDENCE}\t{CASE}\t"),
        &format!("\tcomplete\t{EVIDENCE}\t-\t"),
    );
    let error = review(&missing).expect_err("complete without case must fail");
    assert!(error.contains("needs a case"), "{error}");

    let invented = reviewed_manifest().replace(CASE, "invented-case-name");
    let error = review(&invented).expect_err("invented case must fail");
    assert!(error.contains("unverified case"), "{error}");

    let blank = reviewed_manifest().replace(CASE, "address-base58-golden;;");
    let error = review(&blank).expect_err("blank case must fail");
    assert!(error.contains("blank cases entry"), "{error}");

    let no_evidence = reviewed_manifest().replace(
        &format!("\tcomplete\t{EVIDENCE}\t{CASE}\t"),
        &format!("\tcomplete\t-\t{CASE}\t"),
    );
    let error = review(&no_evidence).expect_err("complete without evidence must fail");
    assert!(error.contains("needs evidence"), "{error}");
}

#[test]
fn rejects_malformed_adjudication_manifests() {
    let no_newline = reviewed_manifest().trim_end().to_owned();
    let error = review(&no_newline).expect_err("missing final newline must fail");
    assert!(error.contains("must end with a newline"), "{error}");

    let wrong_header = reviewed_manifest().replace(adjudication::HEADER, "id\tsha256\tstatus");
    let error = review(&wrong_header).expect_err("wrong header must fail");
    assert!(error.contains("adjudication header"), "{error}");

    let short_row = reviewed_manifest().replace("\treviewed decision", "");
    let error = review(&short_row).expect_err("wrong field count must fail");
    assert!(error.contains("adjudication field count"), "{error}");

    let blank_row = format!("{}\n{}", reviewed_manifest(), "");
    let error = review(&blank_row).expect_err("blank row must fail");
    assert!(error.contains("blank adjudication row"), "{error}");

    let bad_status = reviewed_manifest().replace("\tcomplete\t", "\tverified\t");
    let error = review(&bad_status).expect_err("unknown status must fail");
    assert!(error.contains("unknown adjudication status"), "{error}");
}

#[test]
fn rejects_unbounded_or_empty_adjudication_notes() {
    let long_note =
        adjudication_text(&[adjudication_row(ALPHA, HASH_A, "partial", EVIDENCE, CASE)
            .replace("reviewed note", &"n".repeat(201))]);
    let error = review(&long_note).expect_err("overlong note must fail");
    assert!(error.contains("exceeds 200 characters"), "{error}");

    let empty_note =
        adjudication_text(&[adjudication_row(ALPHA, HASH_A, "partial", EVIDENCE, CASE)
            .replace("reviewed note", "-")]);
    let error = review(&empty_note).expect_err("placeholder note must fail");
    assert!(error.contains("empty adjudication notes"), "{error}");
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
