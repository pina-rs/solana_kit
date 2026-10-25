//! PDA and default resolvers consume the same validated source graph as instruction roles.
use super::{
    Program, Role, Signer,
    pda_policy::{Default, Pda, RoleDefault, Seed},
};
use crate::schema::{Error, identifier};
use std::collections::BTreeSet;

pub(super) fn emit(
    program: &Program,
    recipes: &[Pda],
    defaults: &[RoleDefault],
) -> Result<String, Error> {
    let mut output = String::new();

    if recipes.is_empty() && defaults.is_empty() {
        return Ok(output);
    }

    if !super::constants::extended(program) {
        output.push_str("#[cfg(feature=\"pda\")]\nuse solana_kit_core::AccountMeta;\n");
    }

    output.push_str(ERROR_SOURCE);

    for recipe in recipes {
        output.push_str(&pda(recipe));
    }

    for instruction in &program.instructions {
        output.push_str(&accounts(
            instruction,
            recipes,
            defaults,
            super::constants::extended(program),
        )?);
    }

    Ok(output)
}

fn pda(recipe: &Pda) -> String {
    let name = &recipe.function;
    let mut parameters = Vec::new();
    let mut arguments = Vec::new();
    let mut seeds = Vec::new();

    for (index, seed) in recipe.seeds.iter().enumerate() {
        match seed {
            Seed::Constant(bytes) => seeds.push(format!("&{bytes:?}")),
            Seed::Address(_) => {
                let name = format!("seed_{index}");
                parameters.push(format!("{name}:Address"));
                arguments.push(name.clone());
                seeds.push(format!("{name}.as_bytes()"));
            }
            Seed::U64(_) => {
                let name = format!("seed_{index}");
                parameters.push(format!("{name}:u64"));
                arguments.push(name.clone());
                seeds.push(format!("&{name}.to_le_bytes()"));
            }
        }
    }

    let joined = parameters.join(",");
    let separator = if joined.is_empty() { "" } else { "," };
    let args = arguments.join(",");
    let mut output = format!(
        "#[cfg(feature=\"pda\")]\n/// Finds the canonical `{}` PDA with the complete pinned seed recipe.\npub fn {name}({joined})->Result<solana_kit_core::crypto::ProgramDerivedAddress,solana_kit_core::crypto::PdaError>{{{name}_with_program(PROGRAM{separator}{args})}}\n",
        recipe.name
    );
    output.push_str(&format!("#[cfg(feature=\"pda\")]\n/// Finds the PDA for an explicit program; integer seeds are complete u64 little-endian.\npub fn {name}_with_program(program:Address{separator}{joined})->Result<solana_kit_core::crypto::ProgramDerivedAddress,solana_kit_core::crypto::PdaError>{{solana_kit_core::crypto::find_program_address(&[{}],&program)}}\n",seeds.join(",")));
    output
}

/// The declaration and resolver share one exact ordered role/default table.
fn accounts(
    instruction: &super::Instruction,
    recipes: &[Pda],
    defaults: &[RoleDefault],
    extended: bool,
) -> Result<String, Error> {
    let name = &instruction.name;
    let selected = selected_defaults(name, defaults)?;
    let mut output = format!(
        "#[cfg(feature=\"pda\")]\n/// Ordered `{name}` inputs; None selects a default, a program sentinel, or an omitted suffix.\n#[derive(Clone,Debug)]\npub struct {name}Accounts{{\n"
    );

    output.push_str(&declaration(&instruction.roles, &selected));

    output.push_str(&format!("}}\n#[cfg(feature=\"pda\")]\nimpl {name}Accounts{{\n/// Resolves the complete checked default graph, preserving explicit overrides.\npub fn resolve(&self,program:Address)->Result<Vec<AccountMeta>,DefaultError>{{\n"));
    resolve(&mut output, &instruction.roles, recipes, &selected)?;
    let count = instruction.roles.len();
    output.push_str(&format!("let mut accounts=Vec::new();\naccounts.try_reserve_exact({count}).map_err(|_|DefaultError::Codec(Error::AllocationFailed))?;\n"));
    output.push_str(&push_roles(instruction));
    output.push_str("Ok(accounts)\n}\n");
    let build = if extended {
        "let accounts=self.resolve(program)?;\nOk(data.instruction_with_program(program,&accounts,limits)?)"
    } else {
        "let mut instruction=data.instruction(limits)?;\ninstruction.program_address=program;\nOk(instruction)"
    };

    output.push_str(&format!("/// Builds the canonical invocation with resolved defaults and caller limits.\npub fn instruction(&self,data:&{name},limits:Limits)->Result<Instruction,DefaultError>{{self.instruction_with_program(PROGRAM,data,limits)}}\n/// Builds with an explicit program and derives every PDA against that program.\npub fn instruction_with_program(&self,program:Address,data:&{name},limits:Limits)->Result<Instruction,DefaultError>{{{build}}}\n}}\n"));
    Ok(output)
}

fn selected_defaults<'a>(
    name: &str,
    defaults: &'a [RoleDefault],
) -> Result<Vec<&'a RoleDefault>, Error> {
    let source_name = identifier(name)?;
    let source_name = source_name.strip_prefix('_').unwrap_or(&source_name);
    let mut selected = Vec::new();

    for item in defaults {
        if identifier(&item.instruction)? == source_name {
            selected.push(item);
        }
    }

    Ok(selected)
}

fn declaration(roles: &[Role], selected: &[&RoleDefault]) -> String {
    let mut output = String::new();

    for role in roles {
        let optional = role.optional
            || selected
                .iter()
                .any(|item| identifier(&item.role).is_ok_and(|value| value == role.name));
        let ty = if optional {
            "Option<Address>"
        } else {
            "Address"
        };

        output.push_str(&format!(
            "/// The `{}` role, with explicit override support.\npub {}:{ty},\n",
            role.name, role.name
        ));
    }

    output
}

/// Only an absent suffix may be omitted; program sentinels retain every position.
fn push_roles(instruction: &super::Instruction) -> String {
    let mut output = String::new();

    if instruction.omit_optional && instruction.roles.iter().any(|role| role.optional) {
        output.push_str("let mut _omitted = false;\n");
    }

    for (index, role) in instruction.roles.iter().enumerate() {
        let binding = binding(index);
        let signer = matches!(role.signer, Signer::Required);
        let value = format!(
            "AccountMeta{{address:{},role:AccountRole::from_privileges({signer},{})}}",
            binding, role.writable
        );

        if !role.optional {
            output.push_str(&format!("accounts.push({value});\n"));
            continue;
        }

        output.push_str(&optional_role(&binding, &value, instruction.omit_optional));
    }

    output
}

fn optional_role(binding: &str, value: &str, omit: bool) -> String {
    if omit {
        return format!(
            "if let Some({binding})={binding}{{if _omitted{{return Err(DefaultError::Codec(Error::InvalidValue));}}accounts.push({value});}}else{{_omitted=true;}}\n"
        );
    }

    format!(
        "accounts.push(match {binding}{{Some({binding})=>{value},None=>AccountMeta{{address:program,role:AccountRole::Readonly}}}});\n"
    )
}

fn binding(index: usize) -> String {
    format!("_role_{index}")
}

fn resolve(
    output: &mut String,
    roles: &[Role],
    recipes: &[Pda],
    defaults: &[&RoleDefault],
) -> Result<(), Error> {
    let mut ready = BTreeSet::new();
    let mut pending: Vec<_> = roles.iter().enumerate().collect();
    output.push_str("let _ = program;\n");

    while !pending.is_empty() {
        let before = pending.len();
        let mut next = Vec::new();

        for (index, role) in pending {
            let default = defaults
                .iter()
                .find(|item| identifier(&item.role).is_ok_and(|name| name == role.name));
            let expression = role_expression(role, default.copied(), recipes, &ready, roles)?;

            let Some(expression) = expression else {
                next.push((index, role));
                continue;
            };

            output.push_str(&format!("let {}={expression};\n", binding(index)));
            ready.insert(role.name.clone());
        }

        if next.len() == before {
            return Err("unresolved role-default dependency".into());
        }

        pending = next;
    }

    Ok(())
}

fn role_expression(
    role: &Role,
    default: Option<&RoleDefault>,
    recipes: &[Pda],
    ready: &BTreeSet<String>,
    roles: &[Role],
) -> Result<Option<String>, Error> {
    let Some(default) = default else {
        return Ok(Some(format!("self.{}", role.name)));
    };
    let Some(value) = default_expression(&default.value, recipes, ready, roles)? else {
        return Ok(None);
    };
    Ok(Some(format!(
        "match self.{}{{Some(address)=>address,None=>{value}}}",
        role.name
    )))
}

/// A default can be emitted only after every referenced role has a resolved binding.
fn default_expression(
    value: &Default,
    recipes: &[Pda],
    ready: &BTreeSet<String>,
    roles: &[Role],
) -> Result<Option<String>, Error> {
    match value {
        Default::Address(address) => Ok(Some(format!(
            "Address::from_bytes({:?})",
            address.as_bytes()
        ))),
        Default::Pda { name, accounts } => {
            let recipe = recipes
                .iter()
                .find(|recipe| recipe.name == *name)
                .ok_or("unknown default PDA")?;
            let arguments = accounts
                .iter()
                .map(|name| identifier(name))
                .collect::<Result<Vec<_>, _>>()?;

            if arguments.iter().any(|name| !ready.contains(name)) {
                return Ok(None);
            }

            let arguments = arguments
                .iter()
                .map(|name| {
                    roles
                        .iter()
                        .position(|role| role.name == *name)
                        .map(binding)
                        .ok_or_else(|| "unresolved role binding".into())
                })
                .collect::<Result<Vec<_>, Error>>()?;
            let name = &recipe.function;
            let separator = if arguments.is_empty() { "" } else { "," };
            Ok(Some(format!(
                "{name}_with_program(program{separator}{})?.address()",
                arguments.join(",")
            )))
        }
    }
}

const ERROR_SOURCE: &str = r#"
#[cfg(feature="pda")]
/// A declared-default derivation or instruction construction failure.
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum DefaultError {
/// Exact deterministic PDA failure.
Pda(solana_kit_core::crypto::PdaError),
/// Exact allocation, privilege, wire or caller-budget failure.
Codec(Error),
}
#[cfg(feature="pda")]
impl From<solana_kit_core::crypto::PdaError> for DefaultError {
fn from(error:solana_kit_core::crypto::PdaError)->Self{Self::Pda(error)}
}
#[cfg(feature="pda")]
impl From<Error> for DefaultError {
fn from(error:Error)->Self{Self::Codec(error)}
}
#[cfg(feature="pda")]
impl core::fmt::Display for DefaultError {
fn fmt(&self,formatter:&mut core::fmt::Formatter<'_>)->core::fmt::Result {
match self { Self::Pda(error)=>core::fmt::Display::fmt(error,formatter),Self::Codec(error)=>core::fmt::Display::fmt(error,formatter) }
}
}
#[cfg(feature="pda")]
impl core::error::Error for DefaultError {
fn source(&self)->Option<&(dyn core::error::Error+'static)> {
match self { Self::Pda(error)=>Some(error),Self::Codec(error)=>Some(error) }
}
}
"#;
