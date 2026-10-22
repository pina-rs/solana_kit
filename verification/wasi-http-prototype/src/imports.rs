//! Exact component import inspection. Rust target names do not prove host isolation.

use std::path::Path;
use wasmparser::{Encoding, Parser, Payload};

/// Tracks nested component/module depth so only actual outer host interfaces count.
pub fn inspect(component: &Path) {
    let bytes = std::fs::read(component).expect("component");
    let imports = outer(&bytes);
    assert!(
        allowed(&imports),
        "outer imports differ from the exact interface grant"
    );
    negative();
    println!("IMPORTS\n{}", imports.join("\n"));
}

/// Depth excludes imports from nested modules and components.
fn outer(bytes: &[u8]) -> Vec<String> {
    let mut payloads = Parser::new(0).parse_all(bytes);
    assert!(matches!(
        payloads.next().expect("header").expect("valid header"),
        Payload::Version {
            encoding: Encoding::Component,
            ..
        }
    ));
    let mut depth = 0;
    let mut imports = Vec::new();

    for payload in payloads {
        match payload.expect("valid component") {
            Payload::ModuleSection { .. } | Payload::ComponentSection { .. } => depth += 1,

            Payload::End(_) if depth > 0 => depth -= 1,

            Payload::ComponentImportSection(section) if depth == 0 => {
                imports.extend(
                    section
                        .into_iter()
                        .map(|item| item.expect("import").name.0.to_owned()),
                );
            }

            _ => {}
        }
    }

    imports.sort();
    imports.dedup();
    imports
}

fn negative() {
    use wasm_encoder::{
        Component, ComponentImportSection, ComponentTypeRef, ComponentTypeSection, InstanceType,
    };
    let mut types = ComponentTypeSection::new();
    types.instance(&InstanceType::new());
    let mut imports = ComponentImportSection::new();
    imports.import(
        "wasi:unknown/interface@0.2.12",
        ComponentTypeRef::Instance(0),
    );
    let mut component = Component::new();
    component.section(&types).section(&imports);
    let bytes = component.finish();
    wasmparser::Validator::new_with_features(wasmparser::WasmFeatures::all())
        .validate_all(&bytes)
        .expect("valid injected component");
    assert!(
        !allowed(&outer(&bytes)),
        "unknown interface artifact escaped policy"
    );

    println!("PASS actual unknown-interface component rejected by exact import policy");
}

fn allowed(imports: &[String]) -> bool {
    const EXPECTED: [&str; 16] = [
        "wasi:cli/environment@0.2.12",
        "wasi:cli/exit@0.2.12",
        "wasi:cli/stderr@0.2.12",
        "wasi:cli/stdin@0.2.12",
        "wasi:cli/stdout@0.2.12",
        "wasi:cli/terminal-input@0.2.12",
        "wasi:cli/terminal-output@0.2.12",
        "wasi:cli/terminal-stderr@0.2.12",
        "wasi:cli/terminal-stdin@0.2.12",
        "wasi:cli/terminal-stdout@0.2.12",
        "wasi:clocks/monotonic-clock@0.2.12",
        "wasi:http/outgoing-handler@0.2.12",
        "wasi:http/types@0.2.12",
        "wasi:io/error@0.2.12",
        "wasi:io/poll@0.2.12",
        "wasi:io/streams@0.2.12",
    ];

    imports.iter().map(String::as_str).eq(EXPECTED)
}
