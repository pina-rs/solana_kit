//! Deterministic package discovery, source hashing and export closure.

use crate::{
    model::{Declaration, Package, Parsed, Result, Source},
    parse, typescript,
};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Component, Path},
};

pub(crate) struct Inventory {
    pub packages: Vec<Package>,
    pub sources: BTreeMap<String, Source>,
}

type Namespace = BTreeSet<(String, Declaration)>;
type Namespaces = BTreeMap<String, Namespace>;

impl Inventory {
    /// Loads exactly the manifest package set, then resolves all export roots.
    pub fn load(reference: &Path, repository: &Path) -> Result<Self> {
        let package_rows = fs::read_to_string(repository.join("docs/planning/parity.tsv"))?;
        let mut packages = Vec::new();
        let discovered = manifests(reference)?;

        for row in package_rows.lines().skip(1) {
            let fields: Vec<_> = row.split('\t').collect();

            if fields.len() != 6 {
                return Err("invalid package ledger row".into());
            }

            packages.push(Package {
                name: fields[0].into(),
                manifest: fields[1].into(),
                owner: fields[2].into(),
                phase: fields[3].into(),
            });
        }

        let expected: BTreeSet<_> = packages
            .iter()
            .map(|package| package.manifest.clone())
            .collect();

        if expected != discovered || expected.len() != 71 {
            return Err("source manifests differ from the 71-package ledger".into());
        }

        let mut sources = BTreeMap::new();

        for package in &packages {
            verify_manifest(reference, package)?;
            load_package(reference, package, &mut sources)?;
        }

        let mut inventory = Self { packages, sources };
        inventory.resolve()?;
        Ok(inventory)
    }

    /// Builds namespace closure before choosing entrypoints, preserving combinators.
    fn resolve(&mut self) -> Result<()> {
        let mut namespaces: BTreeMap<String, BTreeSet<(String, Declaration)>> = self
            .sources
            .iter()
            .map(|(path, source)| {
                (
                    path.clone(),
                    source
                        .parsed
                        .declarations
                        .iter()
                        .map(|item| (path.clone(), item.clone()))
                        .collect(),
                )
            })
            .collect();
        let edges = self.edges()?;

        loop {
            let previous = namespaces.clone();
            let mut changed = false;

            for (path, targets) in &edges {
                let namespace = namespaces.get_mut(path).ok_or("missing namespace")?;

                changed |= merge_exports(namespace, targets, &previous)?;
            }

            if !changed {
                break;
            }
        }

        for (path, namespace) in namespaces.iter().filter(|(path, _)| entrypoint(path)) {
            for (origin, declaration) in namespace {
                let source = self
                    .sources
                    .get_mut(origin)
                    .ok_or("missing origin source")?;
                source.exported.insert(declaration.clone());
                source.roots.insert(path.clone());
            }
        }

        Ok(())
    }

    /// External dependency exports remain visible gaps instead of guessed symbols.
    fn edges(&mut self) -> Result<BTreeMap<String, Vec<(String, crate::model::Export)>>> {
        let known: BTreeSet<_> = self.sources.keys().cloned().collect();
        let mut edges = BTreeMap::new();

        for (path, source) in &mut self.sources {
            edges.insert(path.clone(), source_edges(path, source, &known)?);
        }

        Ok(edges)
    }
}

/// A matching folder is insufficient when its manifest declares a different name.
fn verify_manifest(reference: &Path, package: &Package) -> Result<()> {
    let source = fs::read_to_string(reference.join(&package.manifest))?;
    let name = if package.manifest.ends_with(".json") {
        serde_json::from_str::<serde_json::Value>(&source)?
            .get("name")
            .and_then(serde_json::Value::as_str)
            .ok_or("missing package name")?
            .to_owned()
    } else {
        source
            .lines()
            .find_map(|line| line.strip_prefix("name:"))
            .ok_or("missing Dart package name")?
            .trim()
            .to_owned()
    };

    if name != package.name {
        return Err(format!("manifest package name mismatch: {name}").into());
    }

    Ok(())
}

/// Each export contributes only names allowed by its own show/hide namespace.
fn merge_exports(
    namespace: &mut Namespace,
    targets: &[(String, crate::model::Export)],
    previous: &Namespaces,
) -> Result<bool> {
    let mut changed = false;

    for (target, export) in targets {
        let exported = previous.get(target).ok_or("missing target namespace")?;

        for item in exported
            .iter()
            .filter(|(_, declaration)| export.includes(&declaration.root))
        {
            changed |= namespace.insert(item.clone());
        }
    }

    Ok(changed)
}

/// Unresolvable dependencies are retained as gaps at the defining source file.
fn source_edges(
    path: &str,
    source: &mut Source,
    known: &BTreeSet<String>,
) -> Result<Vec<(String, crate::model::Export)>> {
    let mut targets = Vec::new();

    for export in &source.parsed.exports {
        let target = resolve_target(path, &export.target)?;

        if known.contains(&target) {
            targets.push((target, export.clone()));
        } else {
            source
                .parsed
                .warnings
                .push(format!("unresolved-export:{}", export.target));
        }
    }

    Ok(targets)
}

/// Finds manifests only one level beneath packages, excluding examples/fixtures.
fn manifests(reference: &Path) -> Result<BTreeSet<String>> {
    let mut found = BTreeSet::new();

    for entry in fs::read_dir(reference.join("packages"))? {
        let path = entry?.path();

        for manifest in ["pubspec.yaml", "package.json"] {
            if path.join(manifest).is_file() {
                found.insert(
                    path.join(manifest)
                        .strip_prefix(reference)?
                        .to_string_lossy()
                        .into_owned(),
                );
            }
        }
    }

    Ok(found)
}

/// Parses implementation directories and public lint configuration contracts.
fn load_package(
    reference: &Path,
    package: &Package,
    sources: &mut BTreeMap<String, Source>,
) -> Result<()> {
    let manifest = reference.join(&package.manifest);
    let folder = manifest.parent().ok_or("manifest without parent")?;
    let mut files = Vec::new();

    for directory in ["lib", "bin", "src"] {
        if folder.join(directory).is_dir() {
            walk(&folder.join(directory), &mut files)?;
        }
    }

    for file in files {
        let extension = file
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("");

        if !matches!(extension, "dart" | "ts" | "yaml") {
            continue;
        }

        let source = load_file(reference, package, &file, extension)?;
        sources.insert(source.path.clone(), source);
    }

    Ok(())
}

/// Hashes the exact source bytes independently from the declaration parser.
fn load_file(reference: &Path, package: &Package, file: &Path, extension: &str) -> Result<Source> {
    let path = file.strip_prefix(reference)?.to_string_lossy().into_owned();
    let content = fs::read_to_string(file)?;

    let parsed = match extension {
        "dart" => parse::dart(&content)?,
        "ts" => typescript::parse(&content)?,
        _ => Parsed {
            declarations: vec![Declaration {
                symbol: "lint-policy".into(),
                root: "lint-policy".into(),
                kind: "configuration".into(),
                line: 1,
            }],
            ..Default::default()
        },
    };

    let generated = content
        .lines()
        .take(8)
        .any(|line| line.to_lowercase().contains("generated"));

    Ok(Source {
        package: package.name.clone(),
        path,
        hash: format!("{:x}", Sha256::digest(content.as_bytes())),
        lines: content.lines().count(),
        generated,
        parsed,
        exported: BTreeSet::new(),
        roots: BTreeSet::new(),
    })
}

/// Reads all files in stable order; symlinks are rejected at the source boundary.
fn walk(directory: &Path, files: &mut Vec<std::path::PathBuf>) -> Result<()> {
    let mut entries = fs::read_dir(directory)?.collect::<std::io::Result<Vec<_>>>()?;
    entries.sort_by_key(|entry| entry.path());

    for entry in entries {
        let kind = entry.file_type()?;

        if kind.is_symlink() {
            return Err(format!("source symlink: {}", entry.path().display()).into());
        }

        if kind.is_dir() && entry.file_name() != "target" {
            walk(&entry.path(), files)?;
        } else {
            files.push(entry.path());
        }
    }

    Ok(())
}

fn resolve_target(source: &str, target: &str) -> Result<String> {
    if let Some(package) = target.strip_prefix("package:") {
        let (name, file) = package.split_once('/').ok_or("invalid package export")?;

        return Ok(format!("packages/{name}/lib/{file}"));
    }

    let parent = Path::new(source).parent().ok_or("source without parent")?;
    let mut parts = Vec::new();
    let joined = parent.join(target);

    for component in joined.components() {
        match component {
            Component::Normal(value) => parts.push(value.to_string_lossy().into_owned()),
            Component::ParentDir => {
                parts.pop().ok_or("export escaped source root")?;
            }
            Component::CurDir => {}
            _ => return Err("unsupported export path".into()),
        }
    }

    Ok(parts.join("/"))
}

fn entrypoint(path: &str) -> bool {
    let parts: Vec<_> = path.split('/').collect();
    (parts.len() == 4 && matches!(parts[2], "lib" | "bin"))
        || path.ends_with("codama-renderers-dart/src/index.ts")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_closure_handles_cycles_and_filters_container_members() {
        let mut sources = BTreeMap::new();
        let fixtures = [
            (
                "packages/demo/lib/demo.dart",
                "export 'src/api.dart' show Box hide secret;",
            ),
            (
                "packages/demo/lib/src/api.dart",
                "export '../demo.dart'; class Box { void run() {} } const secret = 1;",
            ),
        ];

        for (path, content) in fixtures {
            sources.insert(
                path.into(),
                Source {
                    package: "demo".into(),
                    path: path.into(),
                    hash: String::new(),
                    lines: 1,
                    generated: false,
                    parsed: parse::dart(content).expect("valid syntax"),
                    exported: BTreeSet::new(),
                    roots: BTreeSet::new(),
                },
            );
        }

        let mut inventory = Inventory {
            packages: Vec::new(),
            sources,
        };
        inventory.resolve().expect("resolvable cyclic export");
        let api = &inventory.sources["packages/demo/lib/src/api.dart"];
        let names: BTreeSet<_> = api
            .exported
            .iter()
            .map(|item| item.symbol.as_str())
            .collect();
        assert_eq!(names, BTreeSet::from(["Box", "Box.run"]));
    }
}
