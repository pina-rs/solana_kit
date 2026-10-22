//! Native private gate: every generated corpus file is explicitly scored.
mod corpus;
use crate::{generic, schema::Error};
use serde_json::Value;
use std::{fs, path::Path, process::Command};
const TOOL_VERSION: &str = "monostyle 0.3.26";
const FLOOR: f64 = 90.0;

/// Fail closed on tool identity, missing inventory and either quality floor.
pub(crate) fn run() -> Result<(), Error> {
    let version = Command::new("monostyle").arg("--version").output()?;

    if !version.status.success() || String::from_utf8(version.stdout)?.trim() != TOOL_VERSION {
        return Err("generated style requires monostyle 0.3.26".into());
    }

    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()?;
    let config = root.join("monostyle.toml");
    let destination = root.join(format!("target/generated-style-{}", std::process::id()));
    fs::create_dir_all(&destination)?;
    let mut outputs = Vec::new();

    for (name, input, policy) in corpus::inputs()? {
        let output = destination.join(format!("{name}.rs"));
        generic::replace(&input, &policy, &output)?;
        outputs.push(output);
    }

    if !config.is_file() || outputs.is_empty() {
        return Err("generated style configuration or outputs missing".into());
    }

    let result = Command::new("monostyle")
        .arg("--config")
        .arg(config)
        .args(["check", "--include-generated", "--format", "json"])
        .args(&outputs)
        .output()?;
    fs::write(destination.join("scores.json"), &result.stdout)?;
    verify(&result.stdout, &outputs)?;

    if !result.status.success() {
        return Err(format!(
            "generated style rejected outputs: {}",
            String::from_utf8_lossy(&result.stderr)
        )
        .into());
    }

    println!(
        "scored all {} generated sources with {TOOL_VERSION}",
        outputs.len()
    );
    Ok(())
}

/// Inventory membership and both finite floors are checked for each explicit output.
fn verify(bytes: &[u8], outputs: &[std::path::PathBuf]) -> Result<(), Error> {
    let report: Value = serde_json::from_slice(bytes)?;
    let files = report["files"]
        .as_array()
        .ok_or("style file inventory missing")?;

    if files.len() != outputs.len() {
        return Err("generated style output inventory incomplete".into());
    }

    for output in outputs {
        let path = output.to_str().ok_or("non-UTF8 output path")?;
        let file = files
            .iter()
            .find(|file| file["path"] == path)
            .ok_or("generated source unscored")?;
        let readability = score(file, "readability")?;
        let complexity = score(file, "complexity")?;
        println!(
            "{}: readability {readability}, complexity {complexity}",
            output
                .file_name()
                .ok_or("output name missing")?
                .to_string_lossy()
        );
    }

    Ok(())
}

fn score(file: &Value, metric: &str) -> Result<f64, Error> {
    let value = file[metric]["value"]
        .as_f64()
        .ok_or("style score missing")?;

    if !value.is_finite() || value < FLOOR {
        return Err(format!("generated source {metric} {value} below {FLOOR}").into());
    }

    Ok(value)
}
