//! Checks the package ledger, optionally against an archived Dart reference.

use std::collections::BTreeSet;
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let mut arguments = env::args().skip(1);
    let reference = arguments.next();
    let pins = fs::read_to_string("docs/planning/references.toml")?;
    let ledger = fs::read_to_string("docs/planning/parity.tsv")?;
    let expected =
        count(&pins, "top_level_dart_packages")? + count(&pins, "top_level_renderer_packages")?;
    let mut packages = BTreeSet::new();
    let mut manifests = BTreeSet::new();

    if arguments.next().is_some() {
        return Err("usage: check-plan [reference-root]".into());
    }

    if ledger.lines().next() != Some("package\tsource\towner\tphase\tstatus\tevidence") {
        return Err("unexpected parity ledger header".into());
    }

    for (index, line) in ledger.lines().skip(1).enumerate() {
        let columns: Vec<_> = line.split('\t').collect();

        if columns.len() != 6 || columns.iter().any(|column| column.is_empty()) {
            return Err(format!("ledger line {} needs six nonempty fields", index + 2).into());
        }

        let [package, source, _owner, phase, status, evidence] = columns.as_slice() else {
            return Err("invalid ledger row".into());
        };
        let parts: Vec<_> = source.split('/').collect();

        if parts.len() != 3
            || parts[0] != "packages"
            || matches!(parts[1], "." | ".." | "")
            || !matches!(parts[2], "pubspec.yaml" | "package.json")
        {
            return Err(format!("invalid top-level package manifest {source}").into());
        }

        if !packages.insert(*package) || !manifests.insert(*source) {
            return Err(format!("duplicate package or manifest for {package}").into());
        }

        if !matches!(*phase, "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8") {
            return Err(format!("invalid implementation phase for {package}").into());
        }

        if !matches!(
            *status,
            "inventoried" | "implemented" | "verified" | "blocked"
        ) {
            return Err(format!("unknown implementation status for {package}").into());
        }

        if matches!(*status, "implemented" | "verified") && *evidence == "-" {
            return Err(format!("{package} claims implementation without evidence").into());
        }

        if let Some(root) = &reference {
            let path = Path::new(root).join(source);
            let manifest = fs::read_to_string(&path)?;
            let name = manifest.lines().find_map(|line| {
                let trimmed = line.trim();
                trimmed
                    .strip_prefix("name:")
                    .or_else(|| trimmed.strip_prefix("\"name\":"))
                    .map(|name| name.trim().trim_end_matches(',').trim_matches(['\'', '"']))
            });

            if name != Some(*package) {
                return Err(format!("package name differs from {}", path.display()).into());
            }
        }
    }

    if packages.len() != expected {
        return Err(format!("expected {expected} packages, found {}", packages.len()).into());
    }

    if let Some(root) = reference {
        let mut actual = BTreeSet::new();

        for entry in fs::read_dir(Path::new(&root).join("packages"))? {
            let path = entry?.path();

            for filename in ["pubspec.yaml", "package.json"] {
                let manifest = path.join(filename);

                if manifest.is_file() {
                    actual.insert(manifest.strip_prefix(&root)?.to_string_lossy().into_owned());
                }
            }
        }

        let recorded: BTreeSet<_> = manifests.into_iter().map(str::to_owned).collect();

        if recorded != actual {
            return Err("reference package manifests differ from the parity ledger".into());
        }
    }

    println!("verified {expected} unique package records and their planning states");
    Ok(())
}

fn count(pins: &str, key: &str) -> Result<usize, Box<dyn Error>> {
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
