//! Retains the exact Cargo lock blocks reachable from the selected SDK roots.

use super::Plan;
use anyhow::{Context, Result, ensure};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

/// Exact package block retained from Cargo's own lockfile.
#[derive(Clone, Debug)]
pub(crate) struct LockedPackage {
    pub(crate) name: String,
    pub(crate) version: String,
    source: Option<String>,
    pub(crate) checksum: Option<String>,
    dependencies: Vec<String>,
    text: String,
}

/// Parses every package block without reinterpreting Cargo's lock schema.
pub(crate) fn parse_lockfile(text: &str) -> Result<Vec<LockedPackage>> {
    let version = text
        .lines()
        .find_map(|line| line.strip_prefix("version = "))
        .context("lockfile has no version")?
        .trim();
    ensure!(version == "4", "unexpected lockfile version: {version}");
    let mut packages = Vec::new();

    for block in text.split("\n[[package]]").skip(1) {
        packages.push(parse_block(&format!("[[package]]{block}"))?);
    }

    ensure!(!packages.is_empty(), "lockfile contains no packages");
    Ok(packages)
}

/// Parses one package block while preserving its exact original text.
fn parse_block(block: &str) -> Result<LockedPackage> {
    let mut fields = BTreeMap::new();
    let mut dependencies = Vec::new();
    let mut in_dependencies = false;

    for line in block.lines() {
        if line == "dependencies = [" {
            in_dependencies = true;
            continue;
        }

        if in_dependencies {
            if line == "]" {
                in_dependencies = false;
                continue;
            }

            let entry: String = serde_json::from_str(line.trim_end_matches(','))
                .context("lock dependency must be quoted")?;
            dependencies.push(entry);
            continue;
        }

        let Some((key, value)) = line.split_once(" = ") else {
            continue;
        };

        if matches!(key, "name" | "version" | "source" | "checksum") {
            let value: String = serde_json::from_str(value).context("lock field must be quoted")?;
            ensure!(
                fields.insert(key.to_owned(), value).is_none(),
                "duplicate lock field"
            );
        }
    }

    Ok(LockedPackage {
        name: fields.remove("name").context("lock package has no name")?,
        version: fields
            .remove("version")
            .context("lock package has no version")?,
        source: fields.remove("source"),
        checksum: fields.remove("checksum"),
        dependencies,
        text: block.trim_end().to_owned(),
    })
}

/// Writes the exact lock blocks reachable from the rehearsal package roots.
pub(crate) fn closure_lockfile(lockfile: &Path, plan: &Plan, evidence: &Path) -> Result<PathBuf> {
    let packages = parse_lockfile(&fs::read_to_string(lockfile)?)?;
    let indexes = reachable(&packages, plan)?;
    let path = evidence.join("closure-Cargo.lock");
    fs::write(&path, closure_text(&packages, indexes))?;
    Ok(path)
}

/// Resolves roots and every transitive dependency to original block indexes.
fn reachable(packages: &[LockedPackage], plan: &Plan) -> Result<Vec<usize>> {
    let index = Index::new(packages);
    let mut pending: Vec<usize> = plan
        .targets
        .iter()
        .map(|target| index.exact(&target.name, &target.version))
        .collect::<Result<_>>()?;
    let mut selected = BTreeSet::new();

    while let Some(position) = pending.pop() {
        if !selected.insert(position) {
            continue;
        }

        for dependency in &packages[position].dependencies {
            pending.push(index.dependency(dependency, packages)?);
        }
    }

    Ok(selected.into_iter().collect())
}

/// Renders only the selected blocks after the original lock version.
fn closure_text(packages: &[LockedPackage], indexes: Vec<usize>) -> String {
    let mut text = String::from("version = 4\n");

    for index in indexes {
        text.push('\n');
        text.push_str(&packages[index].text);
        text.push('\n');
    }

    text
}

/// Name and exact-identity lookup for Cargo's version-qualified dependency cells.
struct Index<'a> {
    exact: BTreeMap<(&'a str, &'a str), usize>,
    names: BTreeMap<&'a str, Vec<usize>>,
}

impl<'a> Index<'a> {
    fn new(packages: &'a [LockedPackage]) -> Self {
        let mut exact = BTreeMap::new();
        let mut names: BTreeMap<&'a str, Vec<usize>> = BTreeMap::new();

        for (position, package) in packages.iter().enumerate() {
            exact.insert((package.name.as_str(), package.version.as_str()), position);
            names
                .entry(package.name.as_str())
                .or_default()
                .push(position);
        }

        Self { exact, names }
    }

    fn exact(&self, name: &str, version: &str) -> Result<usize> {
        self.exact
            .get(&(name, version))
            .copied()
            .with_context(|| format!("lockfile lacks selected package {name} {version}"))
    }

    fn dependency(&self, dependency: &str, packages: &[LockedPackage]) -> Result<usize> {
        let (name, version) = dependency
            .split_once(' ')
            .map_or((dependency, None), |(name, version)| (name, Some(version)));
        let position = match version {
            Some(version) => self.exact(name, version)?,
            None => {
                let matches = self
                    .names
                    .get(name)
                    .with_context(|| format!("lockfile lacks {name}"))?;
                ensure!(
                    matches.len() == 1,
                    "ambiguous unversioned lock dependency: {name}"
                );
                matches[0]
            }
        };

        if let Some(source) = &packages[position].source {
            ensure!(
                !source.starts_with("git+"),
                "selected closure contains unpublished Git dependency {}",
                packages[position].name
            );
        }

        Ok(position)
    }
}

#[cfg(test)]
mod tests {
    use super::{closure_lockfile, parse_lockfile};
    use crate::package_rehearsal::model::{Plan, Publication, Target};
    use std::fs;

    #[test]
    fn lockfile_parser_preserves_exact_blocks_and_qualified_edges() {
        let text = "version = 4\n\n[[package]]\nname = \"root\"\nversion = \"0.1.0\"\ndependencies = [\n \"dep 0.2.0\",\n]\n\n[[package]]\nname = \"dep\"\nversion = \"0.2.0\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\nchecksum = \"abc\"\n";
        let packages = parse_lockfile(text).unwrap();
        assert_eq!(packages.len(), 2);
        assert_eq!(packages[0].dependencies, ["dep 0.2.0"]);
        assert_eq!(packages[1].checksum.as_deref(), Some("abc"));
        assert_eq!(
            packages[1].text,
            "[[package]]\nname = \"dep\"\nversion = \"0.2.0\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\nchecksum = \"abc\""
        );
    }

    #[test]
    fn closure_lockfile_retains_exact_reachable_blocks_only() {
        let directory =
            std::env::temp_dir().join(format!("rehearsal-closure-{}", std::process::id()));
        let _ = fs::remove_dir_all(&directory);
        fs::create_dir(&directory).unwrap();
        let lockfile = directory.join("Cargo.lock");
        fs::write(
            &lockfile,
            "version = 4\n\n[[package]]\nname = \"root\"\nversion = \"0.1.0\"\ndependencies = [\n \"dep\",\n]\n\n[[package]]\nname = \"dep\"\nversion = \"0.2.0\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\nchecksum = \"abc\"\n\n[[package]]\nname = \"unused\"\nversion = \"9.9.9\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\nchecksum = \"def\"\n",
        )
        .unwrap();
        let evidence = directory.join("evidence");
        fs::create_dir(&evidence).unwrap();
        let output = closure_lockfile(&lockfile, &plan("root"), &evidence).unwrap();
        let text = fs::read_to_string(output).unwrap();
        assert!(text.contains("name = \"root\""));
        assert!(text.contains("name = \"dep\""));
        assert!(!text.contains("name = \"unused\""));
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn closure_lockfile_rejects_git_dependencies() {
        let directory =
            std::env::temp_dir().join(format!("rehearsal-git-closure-{}", std::process::id()));
        let _ = fs::remove_dir_all(&directory);
        fs::create_dir(&directory).unwrap();
        let lockfile = directory.join("Cargo.lock");
        fs::write(
            &lockfile,
            "version = 4\n\n[[package]]\nname = \"root\"\nversion = \"0.1.0\"\ndependencies = [\n \"gitdep\",\n]\n\n[[package]]\nname = \"gitdep\"\nversion = \"0.1.0\"\nsource = \"git+https://example.invalid/repo?rev=abc#abc\"\n",
        )
        .unwrap();
        let evidence = directory.join("evidence");
        fs::create_dir(&evidence).unwrap();
        assert!(closure_lockfile(&lockfile, &plan("root"), &evidence).is_err());
        fs::remove_dir_all(directory).unwrap();
    }

    fn plan(root: &str) -> Plan {
        Plan {
            roots: vec![],
            targets: vec![Target {
                name: root.into(),
                version: "0.1.0".into(),
                manifest_path: "Cargo.toml".into(),
                source: ".".into(),
                publication: Publication::Publishable,
                dependencies: vec![],
            }],
            excluded: vec![],
        }
    }
}
