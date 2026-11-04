//! Reviewed declaration claims layered over the generated source inventory.
//!
//! The manifest may only replace `status`, `evidence` and `notes`. Every row is
//! keyed by the complete canonical ID and the defining source hash, so an
//! adjudication cannot drift onto a different declaration or revision.

mod report;

use crate::model::Result;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Component, Path},
};

/// Exact reviewed-manifest header; column order is part of the contract.
pub(crate) const HEADER: &str = "id\tsha256\tstatus\tevidence\tcases\tnotes\tdecision";

/// Bounds a short reviewed note so the ledger cannot absorb an essay.
const MAX_NOTES: usize = 200;

/// Bounds the recorded mapping/substitution decision.
const MAX_DECISION: usize = 320;

/// The only statuses a reviewed claim may assert.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Status {
    /// No equivalent behavior is established.
    Pending,
    /// Some exact behavior or a documented substitution is established.
    Partial,
    /// The exact public contract is exercised on a promised host.
    Complete,
}

impl Status {
    fn parse(value: &str) -> Result<Self> {
        match value {
            "pending" => Ok(Self::Pending),
            "partial" => Ok(Self::Partial),
            "complete" => Ok(Self::Complete),
            _ => Err(format!("unknown adjudication status: {value}").into()),
        }
    }

    /// Renders the exact ledger status token.
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Partial => "partial",
            Self::Complete => "complete",
        }
    }
}

/// The three reviewed values rendered into the canonical capability ledger.
#[derive(Debug)]
pub(crate) struct Claim {
    pub status: Status,
    pub evidence: String,
    pub notes: String,
}

struct Entry {
    sha256: String,
    claim: Claim,
}

/// A reviewed manifest keyed by complete canonical declaration ID.
pub(crate) struct Manifest {
    entries: BTreeMap<String, Entry>,
}

impl Manifest {
    /// Loads the reviewed manifest from its fixed repository location.
    pub(crate) fn load(repository: &Path) -> Result<Self> {
        let path = repository.join("docs/parity/adjudications.tsv");
        Self::parse(&fs::read_to_string(path)?, repository)
    }

    /// Parses and validates every row before any ledger column is replaced.
    pub(crate) fn parse(text: &str, repository: &Path) -> Result<Self> {
        let body = text
            .strip_suffix('\n')
            .ok_or("adjudication manifest must end with a newline")?;
        let mut lines = body.split('\n');
        let header = lines.next().ok_or("adjudication manifest is empty")?;

        if header != HEADER {
            return Err("adjudication header differs from the reviewed contract".into());
        }

        let mut entries = BTreeMap::new();

        for line in lines {
            if line.is_empty() {
                return Err("blank adjudication row".into());
            }

            let fields: Vec<_> = line.split('\t').collect();

            if fields.len() != 7 {
                return Err(format!("adjudication field count is {}", fields.len()).into());
            }

            let [id, sha256, status, evidence, cases, notes, decision] = fields.as_slice() else {
                return Err("adjudication field count is not seven".into());
            };
            let id = validate_id(id)?;
            let sha256 = validate_hash(sha256)?;
            let status = Status::parse(status)?;
            let evidence = paths(evidence, "evidence", repository)?;
            let case_names = paths(cases, "cases", repository)?;
            let notes = validate_text(notes, "notes", MAX_NOTES)?;
            validate_text(decision, "decision", MAX_DECISION)?;

            verify_claim(id, status, &evidence, &case_names, repository)?;

            let entry = Entry {
                sha256,
                claim: Claim {
                    status,
                    evidence: join_or_dash(&evidence),
                    notes,
                },
            };

            if entries.insert(id.to_owned(), entry).is_some() {
                return Err(format!("duplicate adjudication id: {id}").into());
            }
        }

        Ok(Self { entries })
    }

    /// Returns the reviewed claim after rejecting hash drift for that ID.
    pub(crate) fn claim(&self, id: &str, sha256: &str) -> Result<Option<&Claim>> {
        let Some(entry) = self.entries.get(id) else {
            return Ok(None);
        };

        if entry.sha256 != sha256 {
            return Err(format!("adjudication hash drift: {id}").into());
        }

        Ok(Some(&entry.claim))
    }

    /// Rejects a reviewed row whose canonical declaration no longer exists.
    pub(crate) fn ensure_observed(&self, observed: &BTreeSet<String>) -> Result<()> {
        if let Some(id) = self.entries.keys().find(|id| !observed.contains(*id)) {
            return Err(format!("unknown adjudication id: {id}").into());
        }

        Ok(())
    }

    /// Validates the manifest against an in-memory canonical ID/hash map.
    pub(crate) fn validate(&self, canonical: &BTreeMap<String, String>) -> Result<()> {
        for (id, entry) in &self.entries {
            let hash = canonical
                .get(id)
                .ok_or_else(|| format!("unknown adjudication id: {id}"))?;

            if hash != &entry.sha256 {
                return Err(format!("adjudication hash drift: {id}").into());
            }
        }

        self.ensure_observed(&canonical.keys().cloned().collect())
    }
}

/// Complete IDs must remain canonical, never package or bare symbol keys.
fn validate_id(value: &str) -> Result<&str> {
    if !(value.starts_with("dart:") || value.starts_with("ts:")) {
        return Err(format!("non-canonical adjudication id: {value}").into());
    }

    if value.chars().any(char::is_control) || value.trim() != value {
        return Err(format!("invalid adjudication id: {value}").into());
    }

    Ok(value)
}

/// Hashes are lowercase SHA-256 hex with no abbreviation.
fn validate_hash(value: &str) -> Result<String> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(format!("invalid adjudication sha256: {value}").into());
    }

    Ok(value.to_owned())
}

/// Text fields are single-line, bounded and never a placeholder.
fn validate_text(value: &str, label: &str, maximum: usize) -> Result<String> {
    if value.is_empty() || value == "-" {
        return Err(format!("empty adjudication {label}").into());
    }

    if value.chars().count() > maximum {
        return Err(format!("adjudication {label} exceeds {maximum} characters").into());
    }

    if value.chars().any(char::is_control) {
        return Err(format!("adjudication {label} contains a control character").into());
    }

    Ok(value.to_owned())
}

/// Parses a dash or semicolon-separated list, validating each path or case.
fn paths(value: &str, label: &str, repository: &Path) -> Result<Vec<String>> {
    if value == "-" {
        return Ok(Vec::new());
    }

    let mut values = Vec::new();

    for item in value.split(';') {
        if item.is_empty() {
            return Err(format!("blank {label} entry").into());
        }

        if item.trim() != item {
            return Err(format!("padded {label} entry: {item}").into());
        }

        if label == "evidence" {
            verify_evidence(repository, item)?;
        }

        values.push(item.to_owned());
    }

    let unique: BTreeSet<_> = values.iter().collect();

    if unique.len() != values.len() {
        return Err(format!("duplicate {label} entry").into());
    }

    Ok(values)
}

/// Evidence is an existing repository-relative file reached without `..`.
fn verify_evidence(repository: &Path, value: &str) -> Result<()> {
    let relative = Path::new(value);

    if relative.is_absolute() {
        return Err(format!("absolute evidence path: {value}").into());
    }

    for component in relative.components() {
        if !matches!(component, Component::Normal(_)) {
            return Err(format!("unsafe evidence path: {value}").into());
        }
    }

    let full = repository.join(relative);

    if !full.is_file() {
        return Err(format!("absent evidence path: {value}").into());
    }

    // Security: a symlinked report must not escape the repository boundary.
    let root = repository.canonicalize()?;
    let resolved = full.canonicalize()?;

    if !resolved.starts_with(&root) {
        return Err(format!("evidence path escapes the repository: {value}").into());
    }

    Ok(())
}

/// A complete claim needs a successful executed case, not a prose reference.
fn verify_claim(
    id: &str,
    status: Status,
    evidence: &[String],
    cases: &[String],
    repository: &Path,
) -> Result<()> {
    if status == Status::Complete {
        if evidence.is_empty() {
            return Err(format!("complete adjudication needs evidence: {id}").into());
        }

        if cases.is_empty() {
            return Err(format!("complete adjudication needs a case: {id}").into());
        }
    }

    for case in cases {
        if !evidence
            .iter()
            .any(|path| report::contains(repository, path, case))
        {
            return Err(format!("unverified case {case} for {id}").into());
        }
    }

    Ok(())
}

fn join_or_dash(values: &[String]) -> String {
    if values.is_empty() {
        "-".into()
    } else {
        values.join(";")
    }
}
