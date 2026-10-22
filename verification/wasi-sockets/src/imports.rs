//! Records the exact outer imports and rejects accidental HTTP/JavaScript coupling.

use anyhow::{Result, ensure};
use wasmparser::{Encoding, Parser, Payload};

/// Requires a component with real socket imports and no HTTP or non-WASI services.
pub fn inspect(component: &std::path::Path) -> Result<String> {
    let bytes = std::fs::read(component)?;
    let mut lines = Vec::new();
    let mut depth = 0_u32;
    let mut payloads = Parser::new(0).parse_all(&bytes);
    ensure!(
        matches!(
            payloads.next().transpose()?,
            Some(Payload::Version {
                encoding: Encoding::Component,
                ..
            })
        ),
        "expected actual component"
    );

    for payload in payloads {
        match payload? {
            Payload::ModuleSection { .. } | Payload::ComponentSection { .. } => depth += 1,

            Payload::End(_) if depth > 0 => depth -= 1,

            Payload::ComponentImportSection(section) if depth == 0 => lines.extend(
                section
                    .into_iter()
                    .map(|import| import.map(|entry| entry.name.0.to_owned()))
                    .collect::<std::result::Result<Vec<_>, _>>()?,
            ),

            _ => {}
        }
    }

    lines.sort();
    lines.dedup();
    ensure!(
        lines
            .iter()
            .any(|line| line.starts_with("wasi:sockets/tcp@")),
        "missing TCP imports"
    );
    ensure!(
        lines
            .iter()
            .all(|line| line.starts_with("wasi:") && !line.starts_with("wasi:http/")),
        "unexpected service import"
    );

    Ok(lines.join("\n") + "\n")
}
