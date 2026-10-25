//! Public file-map boundary over the existing private complete-program engine.

use crate::{generic, schema};
use serde_json::Value;
use std::{collections::BTreeSet, fmt};

const MAX_PROGRAMS: usize = 64;
const MAX_OUTPUT_BYTES: usize = 64 * 1024 * 1024;

mod normalize;
mod spacing;

/// One complete generated module. Paths are flat, relative Rust source names.
#[derive(Debug, Eq, PartialEq)]
pub struct GeneratedFile {
    /// Destination relative to the caller's generated directory.
    pub path: String,
    /// Off-chain Rust source using Kit-owned public types.
    pub source: String,
}

/// A rejected IDL with program context and no partial generated output.
#[derive(Debug)]
pub struct RenderError {
    program: Option<String>,
    message: String,
}

impl fmt::Display for RenderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(program) = &self.program {
            return write!(formatter, "program {program}: {}", self.message);
        }

        formatter.write_str(&self.message)
    }
}

impl std::error::Error for RenderError {}

/// Resolves all Codama programs before returning any file.
///
/// # Errors
/// Rejects unsupported semantic nodes, ambiguous names, invalid addresses and
/// inputs outside the existing complete-program engine's explicit bounds.
/// Unknown fields are never discarded to make an IDL appear supported.
pub fn render_root(root: &Value) -> Result<Vec<GeneratedFile>, RenderError> {
    let normalized = normalize::root(root).map_err(|message| RenderError {
        program: None,
        message,
    })?;
    render(&normalized).map_err(|message| RenderError {
        program: None,
        message,
    })
}

fn render(root: &Value) -> Result<Vec<GeneratedFile>, String> {
    let additional = root["additionalPrograms"]
        .as_array()
        .ok_or("additionalPrograms must be an array")?;
    let mut programs = vec![&root["program"]];
    programs.extend(additional);

    if programs.len() > MAX_PROGRAMS {
        return Err("program collection exceeds 64 programs".into());
    }

    let mut names = BTreeSet::from(["mod".to_owned()]);
    let mut files = Vec::new();
    let mut modules = String::from("// Generated off-chain Solana Kit client modules.\n");
    let mut output_bytes = 0_usize;

    for program in programs {
        let (module, file) = module_file(root, program, &mut names)?;
        output_bytes = output_bytes
            .checked_add(file.source.len())
            .ok_or("output size overflow")?;

        if output_bytes > MAX_OUTPUT_BYTES {
            return Err("generated source exceeds 64 MiB".into());
        }

        modules.push_str(&format!("pub mod {module};\n"));
        files.push(file);
    }

    if output_bytes
        .checked_add(modules.len())
        .is_none_or(|size| size > MAX_OUTPUT_BYTES)
    {
        return Err("generated source exceeds 64 MiB".into());
    }

    files.push(GeneratedFile {
        path: "mod.rs".into(),
        source: modules,
    });
    files.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(files)
}

fn module_file(
    root: &Value,
    program: &Value,
    names: &mut BTreeSet<String>,
) -> Result<(String, GeneratedFile), String> {
    let name = program["name"]
        .as_str()
        .ok_or("program name must be text")?;
    let module = schema::identifier(name).map_err(|error| error.to_string())?;

    if !names.insert(module.clone()) {
        return Err(format!("duplicate generated module {module}"));
    }

    let source = render_program(root, program).map_err(|error| {
        RenderError {
            program: Some(name.to_owned()),
            message: error.to_string(),
        }
        .to_string()
    })?;
    let file = GeneratedFile {
        path: format!("{module}.rs"),
        source,
    };

    Ok((module, file))
}

fn render_program(root: &Value, program: &Value) -> Result<String, schema::Error> {
    let name = program["name"]
        .as_str()
        .ok_or("program name must be text")?;
    let address = program["publicKey"]
        .as_str()
        .ok_or("program publicKey must be text")?
        .parse()?;

    let mut selected = root.clone();
    selected["program"] = program.clone();
    selected["additionalPrograms"] = Value::Array(Vec::new());
    let source = generic::render_codama_program(&selected, &generic::Policy { name, address })?;
    let syntax = syn::parse_file(&source)?;
    let mut output = program_header(program)?;
    output.push_str(&spacing::format(&syntax)?);

    Ok(output)
}

fn program_header(program: &Value) -> Result<String, schema::Error> {
    let mut output =
        String::from("// Generated off-chain Solana Kit client. Retain applicable IDL notices.\n");

    if let Some(origin) = program.get("origin").and_then(Value::as_str) {
        output.push_str(&format!("// Source IDL toolchain: {origin}.\n"));
    }

    if let Some(docs) = program.get("docs").and_then(Value::as_array) {
        append_docs(&mut output, docs)?;
    }

    Ok(output)
}

fn append_docs(output: &mut String, docs: &[Value]) -> Result<(), schema::Error> {
    for item in docs {
        for line in item
            .as_str()
            .ok_or("invalid program documentation")?
            .lines()
        {
            output.push_str("//! ");
            output.push_str(line);
            output.push('\n');
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests;
