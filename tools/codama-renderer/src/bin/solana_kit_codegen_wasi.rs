//! Capability-free stdio bridge bundled by the NPM renderer's explicit build.

use serde_json::{Value, json};
use std::io::{Read, Write};

const MAX_INPUT_BYTES: u64 = 8 * 1024 * 1024;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = Vec::new();
    std::io::stdin()
        .take(MAX_INPUT_BYTES + 1)
        .read_to_end(&mut input)?;

    if u64::try_from(input.len())? > MAX_INPUT_BYTES {
        return Err("Codama JSON exceeds 8 MiB".into());
    }

    let root: Value = serde_json::from_slice(&input)?;
    let files = solana_kit_codegen::render_root(&root)?;
    let response: Vec<_> = files
        .into_iter()
        .map(|file| json!({"path": file.path, "source": file.source}))
        .collect();
    serde_json::to_writer(std::io::stdout().lock(), &response)?;
    std::io::stdout().flush()?;
    Ok(())
}
