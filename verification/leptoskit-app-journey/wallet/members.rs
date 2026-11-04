//! Derives generated member paths from the project's own config and manifests.
use super::Result;
use std::{
    collections::BTreeMap,
    fs,
    path::{Component, Path, PathBuf},
};

const PLATFORMS: [&str; 5] = ["ios", "android", "macos", "windows", "linux"];

pub(super) struct Members {
    pub(super) app: PathBuf,
    pub(super) backend: PathBuf,
    pub(super) shared: PathBuf,
}

/// Reads every generated member before preparation so a renamed or edited
/// workspace fails before any source adaptation begins.
pub(super) fn detect(project: &Path) -> Result<Members> {
    let config = read(&project.join("leptoskit.toml"))?;
    let name = string(nested(&config, "app")?, "name")?;
    valid_name(name)?;
    let client = string(nested(&config, "build")?, "client_package")?;
    let app_package = format!("{name}_app");

    if client != app_package {
        return Err("generated client package conflicts with the app name".into());
    }

    let root = read(&project.join("Cargo.toml"))?;

    if string(nested(&root, "package")?, "name")? != name {
        return Err("generated config and root package disagree".into());
    }

    let packages = workspace_members(project, &root)?;
    let members = root_members(&packages, &root, name, &app_package)?;
    verify_shared(project, &members, name)?;
    verify_hosts(&packages, name)?;

    Ok(members)
}

/// The root app and its one sibling backend are the only root path dependencies.
fn root_members(
    packages: &BTreeMap<String, PathBuf>,
    root: &toml::Table,
    name: &str,
    app_package: &str,
) -> Result<Members> {
    let dependencies = nested(root, "dependencies")?;
    let app = member(packages, app_package)?;

    if path_dependency(dependencies, app_package, Path::new(""))?.as_path() != app {
        return Err("root app dependency does not name its workspace member".into());
    }

    let (backend_package, backend_path) = backend_member(dependencies, name)?;
    let backend = member(packages, &backend_package)?;

    if backend_path.as_path() != backend {
        return Err("root backend dependency does not name its workspace member".into());
    }

    let shared = member(packages, &format!("{name}_shared"))?;
    Ok(Members {
        app: app.to_path_buf(),
        backend: backend.to_path_buf(),
        shared: shared.to_path_buf(),
    })
}

/// Both service members must import the same shared contracts crate.
fn verify_shared(project: &Path, members: &Members, name: &str) -> Result<()> {
    let package = format!("{name}_shared");

    for (label, directory) in [("app", &members.app), ("backend", &members.backend)] {
        let manifest = read(&project.join(directory).join("Cargo.toml"))?;
        let dependency = path_dependency(nested(&manifest, "dependencies")?, &package, directory)?;

        if dependency.as_path() != members.shared {
            return Err(
                format!("generated {label} shared dependency must point to its member").into(),
            );
        }
    }

    Ok(())
}

/// All five generated hosts stay in the workspace even though only Wasm is adapted.
fn verify_hosts(packages: &BTreeMap<String, PathBuf>, name: &str) -> Result<()> {
    for platform in PLATFORMS {
        let package = format!("{name}_{platform}");

        if packages.get(&package).map(PathBuf::as_path) != Some(Path::new(platform)) {
            return Err(format!("generated workspace lacks its {platform} host member").into());
        }
    }

    Ok(())
}

/// Every workspace member is a real directory beneath the generated project.
fn workspace_members(project: &Path, root: &toml::Table) -> Result<BTreeMap<String, PathBuf>> {
    let members = root
        .get("workspace")
        .and_then(toml::Value::as_table)
        .and_then(|workspace| workspace.get("members"))
        .and_then(toml::Value::as_array)
        .ok_or("expected generated workspace members")?;
    let mut packages = BTreeMap::new();

    for member in members {
        let directory = member
            .as_str()
            .ok_or("generated workspace member must be a path")?;
        let directory = safe_member(directory)?;
        let manifest = member_manifest(project, &directory)?;
        let package = string(nested(&manifest, "package")?, "name")?.to_owned();

        if packages.insert(package, directory).is_some() {
            return Err("generated workspace package names must be unique".into());
        }
    }

    Ok(packages)
}

fn member_manifest(project: &Path, directory: &Path) -> Result<toml::Table> {
    let directory = project.join(directory);
    let metadata = fs::symlink_metadata(&directory)?;

    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err("generated workspace member must be an ordinary directory".into());
    }

    read(&directory.join("Cargo.toml"))
}

/// Rejects absolute and traversing member paths before they reach the filesystem.
fn safe_member(directory: &str) -> Result<PathBuf> {
    let directory = Path::new(directory);
    let relative = !directory.as_os_str().is_empty()
        && directory
            .components()
            .all(|part| matches!(part, Component::Normal(_)));

    if !relative {
        return Err("generated workspace member must be a relative directory".into());
    }

    Ok(directory.to_path_buf())
}

/// The root app and its one sibling backend are the only root path dependencies.
fn backend_member(dependencies: &toml::Table, name: &str) -> Result<(String, PathBuf)> {
    let app = format!("{name}_app");
    let mut backend = None;

    for (package, dependency) in dependencies {
        if package == &app || declared_path(dependency).is_none() {
            continue;
        }

        let path = path_dependency(dependencies, package, Path::new(""))?;

        if backend.replace((package.clone(), path)).is_some() {
            return Err("expected one generated backend path dependency".into());
        }
    }

    let (package, path) = backend.ok_or("expected a generated backend path dependency")?;

    if package != format!("{name}_backend") {
        return Err("generated backend package conflicts with the app name".into());
    }

    Ok((package, path))
}

/// Resolves a relative dependency path lexically without following it twice.
fn path_dependency(dependencies: &toml::Table, package: &str, base: &Path) -> Result<PathBuf> {
    let dependency = dependencies
        .get(package)
        .ok_or_else(|| format!("generated manifest lacks {package}"))?;
    let path = declared_path(dependency)
        .ok_or_else(|| format!("generated {package} must be a path dependency"))?;
    within_project(path, base)
}

fn declared_path(dependency: &toml::Value) -> Option<&str> {
    dependency.get("path").and_then(toml::Value::as_str)
}

fn within_project(path: &str, base: &Path) -> Result<PathBuf> {
    let path = Path::new(path);

    if path.is_absolute() {
        return Err("generated dependency path must stay inside the project".into());
    }

    let mut resolved = PathBuf::new();

    for component in base.join(path).components() {
        resolved = apply_component(resolved, component)?;
    }

    if resolved.as_os_str().is_empty() {
        return Err("generated dependency path must name a member".into());
    }

    Ok(resolved)
}

/// Applies one lexical component so `..` can escape only the relative base,
/// never the generated project root.
fn apply_component(mut resolved: PathBuf, component: Component<'_>) -> Result<PathBuf> {
    match component {
        Component::Normal(part) => {
            resolved.push(part);
        }
        Component::CurDir => {}
        Component::ParentDir => {
            if !resolved.pop() {
                return Err("generated dependency path escaped the project".into());
            }
        }
        _ => return Err("generated dependency path must stay inside the project".into()),
    }

    Ok(resolved)
}

fn member<'a>(packages: &'a BTreeMap<String, PathBuf>, package: &str) -> Result<&'a Path> {
    packages
        .get(package)
        .map(PathBuf::as_path)
        .ok_or_else(|| format!("generated workspace lacks {package}").into())
}

fn nested<'a>(table: &'a toml::Table, key: &str) -> Result<&'a toml::Table> {
    table
        .get(key)
        .and_then(toml::Value::as_table)
        .ok_or_else(|| format!("expected a generated {key} table").into())
}

fn string<'a>(table: &'a toml::Table, key: &str) -> Result<&'a str> {
    table
        .get(key)
        .and_then(toml::Value::as_str)
        .ok_or_else(|| format!("expected a generated {key}").into())
}

fn valid_name(name: &str) -> Result<()> {
    let mut characters = name.chars();
    let valid = characters
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        && characters
            .all(|letter| letter.is_ascii_alphanumeric() || letter == '_' || letter == '-');

    if valid {
        Ok(())
    } else {
        Err("generated app name must be a valid cargo package name".into())
    }
}

fn read(path: &Path) -> Result<toml::Table> {
    Ok(fs::read_to_string(path)?.parse()?)
}
