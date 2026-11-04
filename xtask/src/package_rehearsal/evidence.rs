//! Serializes one evidence document with stable indentation.

use anyhow::Result;
use serde::Serialize;
use std::{fs, path::Path};

/// Writes one JSON document with a final newline.
pub(super) fn json(path: &Path, value: &impl Serialize) -> Result<()> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    fs::write(path, bytes)?;
    Ok(())
}
