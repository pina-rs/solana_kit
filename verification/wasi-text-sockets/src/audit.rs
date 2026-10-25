//! Actual component imports, producer identity and forbidden-service sensitivity.

#[path = "../../wasi-sockets/src/imports.rs"]
mod imports;

use anyhow::{Result, ensure};
use std::path::Path;
use wasm_encoder::{
    Component, ComponentImportSection, ComponentTypeRef, ComponentTypeSection, InstanceType,
};
use wasmparser::{Parser, Payload, Validator};

fn main() -> Result<()> {
    let arguments: Vec<_> = std::env::args().collect();
    let path = Path::new(&arguments[1]);
    let bytes = std::fs::read(path)?;
    Validator::new().validate_all(&bytes)?;
    println!("IMPORTS\n{}", imports::inspect(path)?);
    let mut producer = false;

    for payload in Parser::new(0).parse_all(&bytes) {
        if let Payload::CustomSection(section) = payload?
            && section.name() == "producers"
        {
            let text = String::from_utf8_lossy(section.data());
            producer |= text.contains("rustc") && text.contains("1.99.1");
            println!("PRODUCER {text:?}");
        }
    }

    ensure!(producer, "actual component missing expected rustc producer");
    let negative = Path::new(&arguments[2]);
    std::fs::write(negative, forbidden())?;
    Validator::new().validate_all(&std::fs::read(negative)?)?;
    let error = imports::inspect(negative).expect_err("forbidden environment import accepted");
    ensure!(error.to_string().contains("unexpected service import"));
    println!("PASS valid forbidden-service component rejected");
    Ok(())
}

fn forbidden() -> Vec<u8> {
    let mut types = ComponentTypeSection::new();
    types.instance(&InstanceType::new());
    let mut imports = ComponentImportSection::new();
    imports.import("wasi:sockets/tcp@0.2.12", ComponentTypeRef::Instance(0));
    imports.import("env:fetch/network", ComponentTypeRef::Instance(0));
    let mut component = Component::new();
    component.section(&types).section(&imports);
    component.finish()
}
