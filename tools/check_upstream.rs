//! Validates the upstream compatibility inputs.
//!
//! `docs/planning/references.toml` owns the current pins and the `sources`
//! order, and `docs/planning/upstream-versions.toml` owns the release history.
//! `.templates/upstream.t.md` renders both into `README.md` and
//! `docs/book/reference/upstream-compatibility.md`; run `mdt update` after
//! editing either input. This checker enforces the cross-file invariants that
//! mdt cannot express.

use std::collections::BTreeMap;
use std::error::Error;
use std::fs;

type CheckResult<T> = Result<T, Box<dyn Error>>;

const REFERENCES: &str = "docs/planning/references.toml";
const HISTORY: &str = "docs/planning/upstream-versions.toml";
const CORE_MANIFEST: &str = "crates/solana_kit_core/Cargo.toml";
/// `YYYY-MM-DD` date length and separator offsets.
const DATE_LENGTH: usize = 10;
const DATE_MONTH_SEPARATOR: usize = 4;
const DATE_DAY_SEPARATOR: usize = 7;

/// One parsed TOML table. The supported subset covers every value in the three
/// input files: scalars, single-line arrays, and arrays of tables.
struct Table {
    name: String,
    array: bool,
    values: BTreeMap<String, String>,
}

/// One `[[parity]]` row: a workspace release and the upstream it tracks.
struct Parity {
    version: String,
    upstream_kit: String,
    reference: String,
    released: String,
}

fn main() -> CheckResult<()> {
    let references = parse_toml(REFERENCES)?;
    let history = parse_toml(HISTORY)?;
    let workspace = parse_toml(CORE_MANIFEST)?;
    let parity = parity_rows(&history)?;
    let dart = table(&references, "dart")?;

    validate(&parity, dart, &workspace)?;
    validate_sources(&references)?;

    println!("upstream compatibility inputs are consistent; run `mdt update` to sync the tables");
    Ok(())
}

fn parse_toml(path: &str) -> CheckResult<Vec<Table>> {
    let text = fs::read_to_string(path)?;
    let mut tables = vec![Table::new(String::new(), false)];

    for (number, original) in text.lines().enumerate() {
        let line = strip_comment(original).trim();

        if line.is_empty() {
            continue;
        }

        if let Some(header) = line.strip_prefix('[') {
            tables.push(header_table(header, path, number + 1)?);
        } else {
            insert_value(&mut tables, line, path, number + 1)?;
        }
    }

    Ok(tables)
}

fn strip_comment(line: &str) -> &str {
    let mut quoted = false;

    for (index, character) in line.char_indices() {
        match character {
            '"' => quoted = !quoted,
            '#' if !quoted => return &line[..index],
            _ => {}
        }
    }

    line
}

fn header_table(header: &str, path: &str, line: usize) -> CheckResult<Table> {
    if let Some(name) = header.strip_prefix('[') {
        let name = name
            .strip_suffix("]]")
            .ok_or_else(|| format!("{path}:{line}: malformed array table"))?;

        return Ok(Table::new(name.to_string(), true));
    }

    let name = header
        .strip_suffix(']')
        .ok_or_else(|| format!("{path}:{line}: malformed table header"))?;
    Ok(Table::new(name.to_string(), false))
}

fn insert_value(tables: &mut [Table], line: &str, path: &str, number: usize) -> CheckResult<()> {
    let (key, value) = line
        .split_once('=')
        .ok_or_else(|| format!("{path}:{number}: expected key = value"))?;
    let table = tables
        .last_mut()
        .ok_or_else(|| format!("{path}:{number}: value outside a table"))?;

    table.values.insert(key.trim().to_string(), scalar(value));
    Ok(())
}

impl Table {
    fn new(name: String, array: bool) -> Self {
        Self {
            name,
            array,
            values: BTreeMap::new(),
        }
    }
}

fn scalar(value: &str) -> String {
    let value = value.trim();

    value
        .strip_prefix('"')
        .and_then(|inner| inner.strip_suffix('"'))
        .unwrap_or(value)
        .to_string()
}

fn table<'a>(tables: &'a [Table], name: &str) -> CheckResult<&'a Table> {
    tables
        .iter()
        .find(|table| !table.array && table.name == name)
        .ok_or_else(|| format!("missing [{name}] table").into())
}

fn parity_rows(history: &[Table]) -> CheckResult<Vec<Parity>> {
    let rows: Vec<_> = history
        .iter()
        .filter(|table| table.array && table.name == "parity")
        .map(parity_row)
        .collect::<CheckResult<_>>()?;

    if rows.is_empty() {
        return Err(format!("{HISTORY} declares no [[parity]] rows").into());
    }

    Ok(rows)
}

fn parity_row(table: &Table) -> CheckResult<Parity> {
    Ok(Parity {
        version: required(table, "version")?.to_string(),
        upstream_kit: required(table, "upstream_kit")?.to_string(),
        reference: required(table, "reference")?.to_string(),
        released: required(table, "released")?.to_string(),
    })
}

fn required<'a>(table: &'a Table, key: &str) -> CheckResult<&'a str> {
    table
        .values
        .get(key)
        .map(String::as_str)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("[{}] is missing {key}", table.name).into())
}

fn array_field(table: &Table, key: &str) -> CheckResult<Vec<String>> {
    let raw = required(table, key)?;
    let inner = raw
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .ok_or_else(|| format!("[{}] {key} is not an array", table.name))?;
    let values: Vec<_> = inner
        .split(',')
        .map(|value| value.trim().trim_matches('"').to_string())
        .filter(|value| !value.is_empty())
        .collect();

    if values.is_empty() {
        return Err(format!("[{}] {key} is empty", table.name).into());
    }

    Ok(values)
}

fn validate(parity: &[Parity], dart: &Table, workspace: &[Table]) -> CheckResult<()> {
    validate_history(parity)?;

    let newest = &parity[0];
    check_equal(
        "pinned Dart commit",
        &newest.reference,
        required(dart, "commit")?,
    )?;
    check_equal(
        "@solana/kit marker",
        &newest.upstream_kit,
        required(dart, "upstream_kit_reference")?,
    )?;
    check_equal(
        "workspace version",
        &newest.version,
        required(table(workspace, "package")?, "version")?,
    )?;

    Ok(())
}

fn check_equal(label: &str, recorded: &str, expected: &str) -> CheckResult<()> {
    if recorded == expected {
        Ok(())
    } else {
        Err(format!("{label} is {recorded}, expected {expected}").into())
    }
}

/// The `sources` array fixes the rendered table order. Every top-level pin must
/// be listed, every listed name must have a table, and `dart` must be present
/// because it carries the upstream marker.
fn validate_sources(references: &[Table]) -> CheckResult<()> {
    let listed = source_names(references)?;

    validate_listed(references, &listed)?;
    validate_entries(references, &listed)?;
    validate_dart(references, &listed)
}

fn source_names(references: &[Table]) -> CheckResult<Vec<String>> {
    let root = references
        .iter()
        .find(|table| !table.array && table.name.is_empty())
        .ok_or("missing root table")?;

    array_field(root, "sources")
}

fn validate_listed(references: &[Table], listed: &[String]) -> CheckResult<()> {
    for source in references
        .iter()
        .filter(|table| !table.array && !table.name.is_empty())
    {
        if !listed.contains(&source.name) {
            return Err(format!("[{}] is missing from the sources list", source.name).into());
        }
    }

    Ok(())
}

fn validate_entries(references: &[Table], listed: &[String]) -> CheckResult<()> {
    for name in listed {
        validate_source(table(references, name)?, name)?;
    }

    Ok(())
}

fn validate_source(source: &Table, name: &str) -> CheckResult<()> {
    required(source, "repository")?;
    array_field(source, "crates")?;
    validate_pin(source, name)
}

fn validate_pin(source: &Table, name: &str) -> CheckResult<()> {
    let has_release = has_value(source, "release");
    let has_selected = has_value(source, "selected_commit");
    let has_commit = has_value(source, "commit");

    if !has_release && !has_selected && !has_commit {
        return Err(format!("[{name}] has no release or commit pin").into());
    }

    if has_selected && !has_commit {
        return Err(format!("[{name}] has selected_commit without commit").into());
    }

    Ok(())
}

fn validate_dart(references: &[Table], listed: &[String]) -> CheckResult<()> {
    let dart = table(references, "dart")?;
    required(dart, "upstream_kit_repository")?;
    required(dart, "upstream_kit_reference")?;

    if !listed.iter().any(|name| name == "dart") {
        return Err("sources must include dart".into());
    }

    Ok(())
}

fn has_value(table: &Table, key: &str) -> bool {
    table.values.get(key).is_some_and(|value| !value.is_empty())
}

/// Validates release labels and newest-first ordering across the history.
///
/// The newest row is the one compared with the current pins, so every older
/// row must already carry a release date and a larger-to-smaller version.
fn validate_history(rows: &[Parity]) -> CheckResult<()> {
    for row in rows {
        validate_release(&row.released)?;
    }

    validate_unreleased(rows)?;
    validate_order(rows)
}

/// Only the newest row may still be unreleased; older rows record their date.
fn validate_unreleased(rows: &[Parity]) -> CheckResult<()> {
    for row in rows.iter().skip(1) {
        if row.released == "unreleased" {
            return Err(format!(
                "only the newest parity row may be unreleased: {}",
                row.version
            )
            .into());
        }
    }

    Ok(())
}

/// Rejects duplicate or out-of-order rows: each version must be strictly
/// greater than the version after it.
fn validate_order(rows: &[Parity]) -> CheckResult<()> {
    for pair in rows.windows(2) {
        if version_key(&pair[0].version)? <= version_key(&pair[1].version)? {
            return Err(format!(
                "parity rows must be newest first: {} follows {}",
                pair[1].version, pair[0].version
            )
            .into());
        }
    }

    Ok(())
}

fn validate_release(released: &str) -> CheckResult<()> {
    if released == "unreleased" || is_date(released) {
        Ok(())
    } else {
        Err(format!("invalid released value {released}; use a date or unreleased").into())
    }
}

fn is_date(value: &str) -> bool {
    let bytes = value.as_bytes();

    bytes.len() == DATE_LENGTH
        && bytes[DATE_MONTH_SEPARATOR] == b'-'
        && bytes[DATE_DAY_SEPARATOR] == b'-'
        && bytes.iter().enumerate().all(|(index, byte)| {
            index == DATE_MONTH_SEPARATOR || index == DATE_DAY_SEPARATOR || byte.is_ascii_digit()
        })
}

fn version_key(version: &str) -> CheckResult<(u32, u32, u32)> {
    let parts: Vec<u32> = version
        .split('.')
        .map(str::parse)
        .collect::<Result<_, _>>()
        .map_err(|_| format!("invalid version {version}"))?;

    match parts.as_slice() {
        [major, minor, patch] => Ok((*major, *minor, *patch)),
        _ => Err(format!("invalid version {version}").into()),
    }
}
