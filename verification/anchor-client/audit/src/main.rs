//! Inspects actual Wasm boundaries, including component root capabilities.

use std::{collections::BTreeSet, error::Error};
use wasmparser::{Encoding, Parser, Payload, Validator, WasmFeatures};

const COMPONENT_IMPORTS: &[&str] = &[
    "wasi:io/poll@0.2.12",
    "wasi:clocks/monotonic-clock@0.2.12",
    "wasi:io/error@0.2.12",
    "wasi:io/streams@0.2.12",
    "wasi:cli/stdout@0.2.12",
    "wasi:cli/stderr@0.2.12",
    "wasi:cli/stdin@0.2.12",
    "wasi:cli/environment@0.2.12",
    "wasi:cli/exit@0.2.12",
    "wasi:cli/terminal-input@0.2.12",
    "wasi:cli/terminal-output@0.2.12",
    "wasi:cli/terminal-stdin@0.2.12",
    "wasi:cli/terminal-stdout@0.2.12",
    "wasi:cli/terminal-stderr@0.2.12",
];

const PACKAGES: &[&str] = &[
    "solana_kit_anchor",
    "solana_kit_core",
    "base64",
    "bs58",
    "serde_json",
    "serde_core",
    "itoa",
    "memchr",
    "zmij",
    "sha2",
    "cfg-if",
    "digest",
    "block-buffer",
    "crypto-common",
    "generic-array",
    "typenum",
    "version_check",
];

fn imports(bytes: &[u8], wasi: bool) -> Result<BTreeSet<String>, Box<dyn Error>> {
    Validator::new_with_features(WasmFeatures::all()).validate_all(bytes)?;
    let mut state = Inspection {
        names: BTreeSet::new(),
        depth: 0,
        component: false,
        wasi,
    };

    for payload in Parser::new(0).parse_all(bytes) {
        state.payload(payload?)?;
    }

    Ok(state.names)
}

struct Inspection {
    names: BTreeSet<String>,
    depth: usize,
    component: bool,
    wasi: bool,
}

impl Inspection {
    // A component's nested core imports are internal wiring. Only root component
    // capabilities are ambient; validation proves all internal bindings match.
    fn payload(&mut self, payload: Payload<'_>) -> Result<(), Box<dyn Error>> {
        match payload {
            Payload::Version { encoding, .. } => self.version(encoding),
            Payload::End(_) => self.depth -= 1,
            Payload::ImportSection(section) => self.core_imports(section)?,
            Payload::ComponentImportSection(section) => self.component_imports(section)?,
            _ => {}
        }

        Ok(())
    }

    fn version(&mut self, encoding: Encoding) {
        if self.depth == 0 {
            self.component = encoding == Encoding::Component;
        }

        self.depth += 1;
    }

    fn core_imports(
        &mut self,
        section: wasmparser::ImportSectionReader<'_>,
    ) -> Result<(), Box<dyn Error>> {
        if self.component {
            return Ok(());
        }

        for import in section {
            let import = import?;
            check_core(import.module, import.name, self.wasi)?;
            self.names
                .insert(format!("{}::{}", import.module, import.name));
        }

        Ok(())
    }

    fn component_imports(
        &mut self,
        section: wasmparser::ComponentImportSectionReader<'_>,
    ) -> Result<(), Box<dyn Error>> {
        if self.depth != 1 {
            return Ok(());
        }

        for import in section {
            let name = import?.name.0;
            check_component(name, self.wasi)?;
            self.names.insert(name.to_string());
        }

        Ok(())
    }
}

fn check_core(module: &str, name: &str, wasi: bool) -> Result<(), Box<dyn Error>> {
    let functions = ["environ_get", "environ_sizes_get", "fd_write", "proc_exit"];

    if !wasi || module != "wasi_snapshot_preview1" || !functions.contains(&name) {
        return Err(format!("forbidden import {module}::{name}").into());
    }

    Ok(())
}

fn check_component(name: &str, wasi: bool) -> Result<(), Box<dyn Error>> {
    if !wasi || !COMPONENT_IMPORTS.contains(&name) {
        return Err(format!("forbidden import {name}").into());
    }

    Ok(())
}

fn graph(tree: &str) -> Result<(), Box<dyn Error>> {
    let selected: BTreeSet<_> = tree
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .collect();
    let allowed: BTreeSet<_> = PACKAGES.iter().copied().collect();

    if selected != allowed {
        return Err(format!("closed dependency mismatch: {selected:?}").into());
    }

    Ok(())
}

fn run(mode: &str, path: &str) -> Result<(), Box<dyn Error>> {
    if mode == "graph" {
        let tree = std::fs::read_to_string(path)?;
        graph(&tree)?;
        assert!(graph(&format!("{tree}\ntokio v1.0.0\n")).is_err());
        println!("17-package closed normal/build graph; injected tokio rejected");

        return Ok(());
    }

    let bytes = std::fs::read(path)?;
    let selected = imports(&bytes, mode == "wasi")?;
    let negative = wat::parse_str(r#"(module (import "env" "fetch" (func)))"#)?;
    let component_negative = wat::parse_str(
        r#"(component (type (instance)) (import "env:fetch/api" (instance (type 0))))"#,
    )?;
    assert!(
        imports(&negative, mode == "wasi")
            .unwrap_err()
            .to_string()
            .contains("forbidden import")
    );
    assert!(
        imports(&component_negative, mode == "wasi")
            .unwrap_err()
            .to_string()
            .contains("forbidden import")
    );
    println!("actual root imports: {selected:?}; core and component negatives rejected");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<_> = std::env::args().skip(1).collect();

    if arguments.len() != 2 {
        return Err("expected mode and artifact path".into());
    }

    run(&arguments[0], &arguments[1])
}
