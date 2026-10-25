//! Tests the official renderer's dependency remapping against actual Kit types.

use crate::inputs::{fetch, hash};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

type Error = Box<dyn std::error::Error>;

/// Locates the Kit library built by Cargo without ignoring directory failures.
fn core_library(directory: &Path) -> Result<PathBuf, Error> {
    let mut libraries = Vec::new();

    for entry in fs::read_dir(directory)? {
        let path = entry?.path();

        if path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with("libsolana_kit_core-") && name.ends_with(".rlib"))
        {
            libraries.push(path);
        }
    }

    libraries.sort();
    libraries
        .into_iter()
        .next()
        .ok_or_else(|| "build prototype before renderer probe".into())
}

/// Extracts just the actual upstream constructor operations under examination.
fn constructor(upstream: &str) -> Result<String, Error> {
    let prefix = upstream
        .split_once("        solana_instruction::Instruction {")
        .ok_or("upstream constructor changed")?
        .0;
    let pushes = prefix
        .lines()
        .filter(|line| line.contains("accounts.push(solana_instruction::AccountMeta::new("))
        .collect::<Vec<_>>()
        .join("\n");
    let start = upstream
        .find("        solana_instruction::Instruction {")
        .ok_or("upstream constructor changed")?;
    let literal = upstream[start..]
        .split_once("\n        }")
        .ok_or("upstream constructor end missing")?
        .0;
    Ok(format!(
        "use solana_kit_core as solana_instruction;\nuse solana_kit_core::Address;\nconst SYSTEM_ID: Address = Address::from_bytes([0;32]);\nstruct CreateAccount {{ payer: Address, new_account: Address }}\nimpl CreateAccount {{ fn probe(&self) -> solana_instruction::Instruction {{\nlet mut accounts = Vec::new();\n{pushes}\nlet data = Vec::new();\n{literal}\n}}\n}} }}\n"
    ))
}

/// Checks the downloaded renderer and selected constructor against pinned hashes.
fn verify_source(source: &Path) -> Result<PathBuf, Error> {
    let input = source.join("e2e/system/src/generated/instructions/create_account.rs");
    let cache = source.parent().ok_or("renderer cache missing")?;

    if hash(&cache.join("source.tar.gz"))?
        != "08d2744525b6f57db6f1517b82764233eaf5fb901a22743d3f943a9635aab618"
        || hash(&input)? != "0d68496e36e3e7c73f5c6c40cc6f0c05ca2cebbcd5df71a371672443c644410f"
    {
        return Err("renderer provenance checksum mismatch".into());
    }

    Ok(input)
}

/// Invokes rustc on the original expressions and retains exact diagnostics.
fn compile_probe(cache: &Path, path: &Path) -> Result<(), Error> {
    let dependencies = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/debug/deps");
    let library = core_library(&dependencies)?;
    let result = Command::new("rustc")
        .args([
            "--edition=2024",
            "--crate-name=dependency_map_probe",
            "--crate-type=lib",
            "--emit=metadata",
            "--extern",
        ])
        .arg(format!("solana_kit_core={}", library.display()))
        .arg("-L")
        .arg(format!("dependency={}", dependencies.display()))
        .arg("--out-dir")
        .arg(cache)
        .arg(path)
        .output()?;
    let diagnostics = String::from_utf8(result.stderr)?;
    fs::write(cache.join("dependency-map-diagnostics.txt"), &diagnostics)?;

    if result.status.success()
        || !diagnostics.contains("error[E0599]")
        || !diagnostics.contains("struct `AccountMeta`")
        || !diagnostics.contains("no field named `program_id`")
    {
        return Err(format!("unexpected dependency-map probe outcome: {diagnostics}").into());
    }

    Ok(())
}

/// Compiles original constructor expressions after mapping their crate paths.
pub(crate) fn run() -> Result<(), Error> {
    let revision = "26f0136703d77ea64c77124d055115256a850954";
    let source = fetch(
        "renderer",
        "https://github.com/codama-idl/renderers-rust",
        revision,
    )?;
    let input = verify_source(&source)?;
    let cache = source.parent().ok_or("renderer cache missing")?;
    let probe = constructor(&fs::read_to_string(&input)?)?;
    let path = cache.join("dependency-map-probe.rs");
    fs::write(&path, probe)?;
    compile_probe(cache, &path)?;

    println!(
        "official renderer {revision}: direct mapping rejected AccountMeta::new and program_id, as expected"
    );
    println!(
        "upstream source sha256 {}; archive sha256 {}",
        hash(&input)?,
        hash(&cache.join("source.tar.gz"))?
    );
    Ok(())
}
