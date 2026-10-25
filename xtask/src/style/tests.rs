//! Negative controls for the report boundary, independent of monostyle's exit status.

use super::{FLOOR, coverage, require_version, validate};
use serde_json::{Value, json};

const HIGH_AGGREGATE: f64 = 99.8;
const ABOVE_MAXIMUM: f64 = 100.1;

#[test]
fn requires_the_reviewed_parser_version() {
    assert!(require_version(b"monostyle 0.3.27\n").is_ok());

    for output in [
        b"monostyle 0.3.26".as_slice(),
        b"monostyle 0.3.28",
        b"monostyle 0.3.27 extra",
        b"",
        b"\xff",
    ] {
        assert!(require_version(output).is_err());
    }
}

/// Builds the required cells from the actual pinned JSON model.
fn report() -> Value {
    json!({
        "readability": {"value": HIGH_AGGREGATE}, "complexity": {"value": 100.0},
        "total_lines": 10, "code_lines": 8,
        "files": [{
            "path": "generated.rs", "total_lines": 10, "code_lines": 8,
            "readability": {"value": FLOOR}, "complexity": {"value": FLOOR}
        }]
    })
}

#[test]
fn accepts_inclusive_floors() {
    assert!(validate(report().to_string().as_bytes()).is_ok());
}

#[test]
fn rejects_file_below_floor_despite_passing_repository() {
    for category in ["readability", "complexity"] {
        let mut report = report();
        report["files"][0][category]["value"] = json!(88.7);
        let error = validate(report.to_string().as_bytes())
            .err()
            .expect("must fail");
        assert!(error.to_string().contains("generated.rs"));
        assert!(error.to_string().contains(category));
    }
}

#[test]
fn rejects_repository_below_floor() {
    for category in ["readability", "complexity"] {
        let mut report = report();
        report[category]["value"] = json!(89.99);
        assert!(validate(report.to_string().as_bytes()).is_err());
    }
}

#[test]
fn rejects_malformed_empty_and_truncated_reports() {
    for bytes in [b"".as_slice(), b"{}", b"{", b"null", b"[]", b"{} trailing"] {
        assert!(validate(bytes).is_err());
    }
}

#[test]
fn rejects_missing_score_cells() {
    for category in ["readability", "complexity"] {
        let mut root = report();
        root.as_object_mut().expect("object").remove(category);
        assert!(validate(root.to_string().as_bytes()).is_err());
        let mut file = report();
        file["files"][0]
            .as_object_mut()
            .expect("object")
            .remove(category);
        assert!(validate(file.to_string().as_bytes()).is_err());
    }
}

#[test]
fn rejects_missing_and_duplicate_files() {
    let mut empty = report();
    empty["files"] = json!([]);
    assert!(validate(empty.to_string().as_bytes()).is_err());

    let mut duplicate = report();
    let file = duplicate["files"][0].clone();
    duplicate["files"].as_array_mut().expect("array").push(file);
    assert!(validate(duplicate.to_string().as_bytes()).is_err());

    let mut missing = report();
    missing["total_lines"] = json!(11);
    assert!(validate(missing.to_string().as_bytes()).is_err());
}

#[test]
fn rejects_out_of_range_or_non_numeric_scores() {
    for value in [json!(ABOVE_MAXIMUM), json!(-1), Value::Null, json!("NaN")] {
        let mut report = report();
        report["files"][0]["complexity"]["value"] = value;
        assert!(validate(report.to_string().as_bytes()).is_err());
    }
}

#[test]
fn rejects_overflowing_line_totals() {
    for count in ["total_lines", "code_lines"] {
        let mut report = report();
        report["files"][0][count] = json!(usize::MAX);
        let mut second = report["files"][0].clone();
        second["path"] = json!("another.rs");
        report["files"].as_array_mut().expect("array").push(second);
        assert!(validate(report.to_string().as_bytes()).is_err());
    }
}

#[test]
fn coverage_rejects_unknown_missing_and_aliased_paths() {
    let inventory = ["generated.rs".to_owned()].into_iter().collect();

    for path in ["unknown.rs", "../generated.rs", "/generated.rs"] {
        let mut input = report();
        input["files"][0]["path"] = json!(path);
        let report = validate(input.to_string().as_bytes()).expect("valid score report");
        assert!(coverage::check(&report, &inventory).is_err());
    }

    let input = validate(report().to_string().as_bytes()).expect("valid score report");
    let missing = ["generated.rs".to_owned(), "missing.rs".to_owned()]
        .into_iter()
        .collect();
    assert!(coverage::check(&input, &missing).is_err());

    let mut duplicate = report();
    let mut alias = duplicate["files"][0].clone();
    alias["path"] = json!("./generated.rs");
    duplicate["files"]
        .as_array_mut()
        .expect("files")
        .push(alias);
    duplicate["total_lines"] = json!(20);
    duplicate["code_lines"] = json!(16);
    let input = validate(duplicate.to_string().as_bytes()).expect("valid score report");
    assert!(coverage::check(&input, &inventory).is_err());
}
