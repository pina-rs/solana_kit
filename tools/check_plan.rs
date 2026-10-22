//! Checks the package ledger, optionally against an archived Dart reference.

use std::collections::BTreeSet;
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

type CheckResult<T> = Result<T, Box<dyn Error>>;

const HEADER: &str = "package\tsource\towner\tphase\tstatus\tevidence";
const COLUMN_COUNT: usize = 6;
const LAST_PHASE: u8 = 8;
const FIRST_RECORD_LINE: usize = 2;

struct Package<'a> {
    name: &'a str,
    manifest: &'a str,
}

fn main() -> CheckResult<()> {
    let reference = reference_argument()?;
    let pins = fs::read_to_string("docs/planning/references.toml")?;
    let ledger = fs::read_to_string("docs/planning/parity.tsv")?;
    let packages = read_packages(&ledger)?;
    let expected =
        count(&pins, "top_level_dart_packages")? + count(&pins, "top_level_renderer_packages")?;

    if packages.len() != expected {
        return Err(format!("expected {expected} packages, found {}", packages.len()).into());
    }

    if let Some(root) = reference {
        check_reference(Path::new(&root), &packages)?;
    }

    println!("verified {expected} unique package records and their planning states");
    Ok(())
}

fn reference_argument() -> CheckResult<Option<String>> {
    let mut arguments = env::args().skip(1);
    let reference = arguments.next();

    if arguments.next().is_some() {
        return Err("usage: check-plan [reference-root]".into());
    }

    Ok(reference)
}

fn read_packages(ledger: &str) -> CheckResult<Vec<Package<'_>>> {
    let mut lines = ledger.lines();
    let mut names = BTreeSet::new();
    let mut manifests = BTreeSet::new();
    let mut packages = Vec::new();

    if lines.next() != Some(HEADER) {
        return Err("unexpected parity ledger header".into());
    }

    for (index, line) in lines.enumerate() {
        let package = read_package(line, index + FIRST_RECORD_LINE)?;

        if !names.insert(package.name) || !manifests.insert(package.manifest) {
            return Err(format!("duplicate package or manifest for {}", package.name).into());
        }

        packages.push(package);
    }

    Ok(packages)
}

fn read_package(line: &str, line_number: usize) -> CheckResult<Package<'_>> {
    let fields: Vec<_> = line.split('\t').collect();
    let error = || format!("ledger line {line_number} needs six nonempty fields");
    let fields: [&str; COLUMN_COUNT] = fields.try_into().map_err(|_| error())?;
    let [name, manifest, owner, phase, status, evidence] = fields;

    if fields.contains(&"") {
        return Err(error().into());
    }

    validate_manifest(manifest)?;
    validate_state(phase, status, evidence)?;

    if owner.trim().is_empty() {
        return Err(format!("{name} has no proposed Rust owner").into());
    }

    Ok(Package { name, manifest })
}

fn validate_manifest(source: &str) -> CheckResult<()> {
    let parts: Vec<_> = source.split('/').collect();

    match parts.as_slice() {
        ["packages", directory, "pubspec.yaml" | "package.json"]
            if !matches!(*directory, "." | ".." | "") =>
        {
            Ok(())
        }
        _ => Err(format!("invalid top-level package manifest {source}").into()),
    }
}

fn validate_state(phase: &str, status: &str, evidence: &str) -> CheckResult<()> {
    let phase = phase.parse::<u8>()?;

    if !(1..=LAST_PHASE).contains(&phase) {
        return Err(format!("invalid implementation phase {phase}").into());
    }

    match (status, evidence) {
        ("implemented" | "verified", "-") => Err("implementation claim needs evidence".into()),
        ("inventoried" | "implemented" | "verified" | "blocked", _) => Ok(()),
        _ => Err(format!("unknown implementation status {status}").into()),
    }
}

fn check_reference(root: &Path, packages: &[Package<'_>]) -> CheckResult<()> {
    let mut recorded = BTreeSet::new();

    for package in packages {
        let path = root.join(package.manifest);
        let manifest = fs::read_to_string(&path)?;

        if manifest_name(&manifest) != Some(package.name) {
            return Err(format!("package name differs from {}", path.display()).into());
        }

        recorded.insert(package.manifest.to_owned());
    }

    if recorded != reference_manifests(root)? {
        return Err("reference package manifests differ from the parity ledger".into());
    }

    Ok(())
}

fn manifest_name(manifest: &str) -> Option<&str> {
    manifest.lines().find_map(|line| {
        let trimmed = line.trim();
        trimmed
            .strip_prefix("name:")
            .or_else(|| trimmed.strip_prefix("\"name\":"))
            .map(|name| name.trim().trim_end_matches(',').trim_matches(['\'', '"']))
    })
}

fn reference_manifests(root: &Path) -> CheckResult<BTreeSet<String>> {
    let mut manifests = BTreeSet::new();

    for entry in fs::read_dir(root.join("packages"))? {
        let path = entry?.path();

        for filename in ["pubspec.yaml", "package.json"] {
            let manifest = path.join(filename);

            if manifest.is_file() {
                manifests.insert(manifest.strip_prefix(root)?.to_string_lossy().into_owned());
            }
        }
    }

    Ok(manifests)
}

fn count(pins: &str, key: &str) -> CheckResult<usize> {
    let values: Vec<_> = pins
        .lines()
        .filter_map(|line| {
            let (name, value) = line.split_once('=')?;
            (name.trim() == key).then_some(value.trim())
        })
        .collect();

    match values.as_slice() {
        [value] => Ok(value.parse()?),
        _ => Err(format!("expected one {key} value in reference pins").into()),
    }
}
