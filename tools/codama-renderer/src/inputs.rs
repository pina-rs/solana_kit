//! Downloads immutable source archives and audits the actual IDL surfaces.

use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

type Error = Box<dyn std::error::Error>;

/// Hashes the bytes actually downloaded, rather than trusting a version banner.
pub(crate) fn hash(path: &Path) -> Result<String, Error> {
    Ok(format!("{:x}", Sha256::digest(fs::read(path)?)))
}

/// Runs an external fetch/extract tool with explicit failure propagation.
fn run(command: &mut Command) -> Result<(), Error> {
    let status = command.status()?;

    if !status.success() {
        return Err(format!("command failed: {command:?}: {status}").into());
    }

    Ok(())
}

/// Fetches one exact revision once and retains both archive and extracted sources.
pub(crate) fn fetch(name: &str, repository: &str, revision: &str) -> Result<PathBuf, Error> {
    let cache = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("cache")
        .join(name);
    let archive = cache.join("source.tar.gz");
    let source = cache.join("source");
    fs::create_dir_all(&cache)?;

    if !archive.exists() {
        let temporary = cache.join("download.tmp");
        let url = format!(
            "{}/tar.gz/{revision}",
            repository.replace("https://github.com", "https://codeload.github.com")
        );
        run(Command::new("curl")
            .args([
                "--fail",
                "--location",
                "--silent",
                "--show-error",
                "--max-time",
                "120",
                &url,
                "-o",
            ])
            .arg(&temporary))?;
        fs::rename(temporary, &archive)?;
    }

    if !source.exists() {
        fs::create_dir(&source)?;
        run(Command::new("tar")
            .arg("-xzf")
            .arg(&archive)
            .arg("--strip-components=1")
            .arg("-C")
            .arg(&source))?;
    }

    Ok(source)
}

/// Collects source paths deterministically for source and generated split counts.
fn files(directory: &Path, extension: &str) -> Result<Vec<PathBuf>, Error> {
    let mut output = Vec::new();

    if !directory.exists() {
        return Ok(output);
    }

    for entry in fs::read_dir(directory)? {
        let path = entry?.path();

        if path.is_dir() {
            output.extend(files(&path, extension)?);
        } else if path.extension().is_some_and(|value| value == extension) {
            output.push(path);
        }
    }

    output.sort();
    Ok(output)
}

/// Selects known upstream paths explicitly; no arbitrary JSON file becomes an IDL.
fn idl_path(name: &str) -> &'static str {
    match name {
        "mpl-token-metadata" => "idls/token_metadata.json",
        "mpl-core" => "idls/mpl_core.json",
        "mpl-bubblegum" => "idls/bubblegum.json",
        "squads-v4" => "sdk/multisig/idl/squads_multisig_program.json",
        "solana-attestation-service" => "idl/solana_attestation_service.json",
        "account-compression" => "account-compression/sdk/idl/spl_account_compression.json",
        "associated-token-account" => "interface/src/instruction.rs",
        "config" => "program/idl.json",
        "subscriptions" => "idl/subscriptions.json",
        _ => "idl.json",
    }
}

/// Counts only explicitly declared nodes; unavailable schema features stay unknown.
fn count(program: &Value, key: &str) -> String {
    program
        .get(key)
        .and_then(Value::as_array)
        .map_or_else(|| "unmodeled".into(), |items| items.len().to_string())
}

/// Describes the raw schema without inventing omitted capability counts.
fn schema(path: &Path) -> Result<String, Error> {
    let idl: Value = serde_json::from_slice(&fs::read(path)?)?;
    let program = idl.get("program").unwrap_or(&idl);
    let format = idl
        .get("standard")
        .map(Value::to_string)
        .unwrap_or_else(|| {
            if idl.get("kind").is_some() {
                "Codama".into()
            } else {
                "Anchor/Shank".into()
            }
        });
    let version = idl
        .get("version")
        .or_else(|| idl.pointer("/metadata/version"))
        .map_or_else(|| "unspecified".into(), Value::to_string);

    Ok(format!(
        "{format}:{version}\t{}\t{}\t{}\t{}\tavailable",
        count(program, "accounts"),
        count(program, "instructions"),
        count(program, "pdas"),
        count(program, "errors")
    ))
}

/// Records the first conventional root notice, leaving absent notices unresolved.
fn license(source: &Path) -> Result<String, Error> {
    let path = ["LICENSE", "LICENSE.md", "LICENSE.txt", "LICENSE-MIT"]
        .into_iter()
        .find(|candidate| source.join(candidate).exists());

    path.map_or_else(
        || Ok("unresolved".into()),
        |path| hash(&source.join(path)).map(|digest| format!("{path}:{digest}")),
    )
}

/// Writes an authoritative row for each input, keeping Loader v3/v4 separate.
fn row(repo: &Value, dart: &Path) -> Result<String, Error> {
    let name = repo["name"].as_str().ok_or("missing repo name")?;
    let repository = repo["url"].as_str().ok_or("missing repo url")?;
    let revision = repo["checkedCommit"]
        .as_str()
        .ok_or("missing immutable revision")?;
    let package = repo["package"].as_str().ok_or("missing package")?;

    let source = fetch(name, repository, revision)?;
    let relative = idl_path(name);
    let path = source.join(relative);
    let archive_hash = hash(
        &source
            .parent()
            .ok_or("missing cache parent")?
            .join("source.tar.gz"),
    )?;
    let dart_files = files(&dart.join("packages").join(package).join("lib/src"), "dart")?;

    let generated = dart_files
        .iter()
        .filter(|path| {
            path.components()
                .any(|part| part.as_os_str() == "generated")
        })
        .count();
    let license_record = license(&source)?;
    let prefix = format!(
        "{package}\t{name}\t{repository}\t{revision}\t{relative}\t{archive_hash}\t{license_record}\t{generated}\t{}",
        dart_files.len() - generated
    );

    if !path.exists() {
        return Ok(format!(
            "{prefix}\tmissing\tunknown\tunknown\tunknown\tunknown\tunknown\tunknown"
        ));
    }

    if path.extension().is_some_and(|extension| extension == "rs") {
        return Ok(format!(
            "{prefix}\t{}\tRust-handwritten\tunmodeled\t3\tunmodeled\tunmodeled\tmanual-source",
            hash(&path)?
        ));
    }

    Ok(format!("{prefix}\t{}\t{}", hash(&path)?, schema(&path)?))
}

/// Downloads all 18 program revisions in bounded independent worker threads.
pub(crate) fn audit() -> Result<(), Error> {
    let reference: Value = serde_json::from_str(include_str!("../fixtures/reference-repos.json"))?;
    let repos = reference["repos"].as_array().ok_or("missing repos")?;
    let dart = fetch(
        "dart",
        "https://github.com/openbudgetfun/solana_kit",
        "0138bb956d2d81ccc0c56edc602401c0df2f2b46",
    )?;
    verify_dart(&dart)?;
    let mut rows = Vec::new();

    for chunk in repos
        .iter()
        .filter(|repo| {
            repo["package"].as_str().is_some_and(|name| {
                name.starts_with("solana_kit_")
                    && !["solana_kit_helius", "solana_kit_mobile_wallet_adapter"].contains(&name)
            })
        })
        .cloned()
        .collect::<Vec<_>>()
        .chunks(4)
    {
        let handles: Vec<_> = chunk
            .iter()
            .cloned()
            .map(|repo| {
                let dart = dart.clone();
                std::thread::spawn(move || row(&repo, &dart).map_err(|error| error.to_string()))
            })
            .collect();

        for handle in handles {
            rows.push(handle.join().map_err(|_| "input worker panicked")??);
        }
    }

    rows.sort();
    let header = "client\trepository_key\trepository\trevision\tidl_path\tarchive_sha256\tlicense_path_sha256\tdart_generated_files\tdart_handwritten_files\tidl_sha256\tschema_version\taccounts\tinstructions\tpdas\terrors\tstatus\n";
    let destination =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/parity/program-inputs.tsv");
    let actual = format!("{header}{}\n", rows.join("\n"));

    if fs::read_to_string(destination)? != actual {
        return Err("pinned input inventory differs from committed ledger".into());
    }

    println!(
        "audited {} immutable program revisions for 17 clients",
        rows.len()
    );
    Ok(())
}

/// Checks authority fixtures and script bytes against the immutable Dart archive.
fn verify_dart(dart: &Path) -> Result<(), Error> {
    let expected = "b229acdc682bf9ea4b00e6f215551d7e72d5d1aa85a98ec0ae42ebd71cd3259d";
    let archive = dart
        .parent()
        .ok_or("missing Dart cache parent")?
        .join("source.tar.gz");

    if hash(&archive)? != expected {
        return Err("Dart archive checksum mismatch".into());
    }

    let fixtures = [
        (
            "config/reference-repos.json",
            include_bytes!("../fixtures/reference-repos.json").as_slice(),
        ),
        (
            "config/programs/artifacts.json",
            include_bytes!("../fixtures/artifacts.json").as_slice(),
        ),
    ];

    for (relative, fixture) in fixtures {
        if fs::read(dart.join(relative))? != fixture {
            return Err(format!("Dart authority fixture mismatch: {relative}").into());
        }
    }

    if hash(&dart.join("scripts/generate_program_packages.mjs"))?
        != "405de73569eb4e24d9c09bf5d8a3327de022b4b9a21dd0f8d46f2b992ff61654"
    {
        return Err("Dart generation script checksum mismatch".into());
    }

    Ok(())
}
