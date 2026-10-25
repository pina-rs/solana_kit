//! Lossless source-defined Stake errors and index-based program attribution.
use crate::schema::{Error, identifier};
use serde_json::Value;
const FIRST_ERROR_GROUP_LAST: u32 = 7;

pub(super) fn render(root: &Value) -> Result<String, Error> {
    let mut constants = String::new();
    let mut variants = String::new();
    let mut from = String::new();
    let mut codes = String::new();
    let mut messages = String::new();

    for error in root["program"]["errors"]
        .as_array()
        .ok_or("missing errors")?
    {
        let original = error["name"].as_str().ok_or("missing error name")?;
        let mut name = original.to_owned();
        name[0..1].make_ascii_uppercase();
        let constant = format!("ERROR_{}", identifier(original)?.to_uppercase());
        let code = u32::try_from(error["code"].as_u64().ok_or("invalid error code")?)?;
        let message = error["message"].as_str().ok_or("missing error message")?;

        constants.push_str(&format!("const {constant}:u32={code};\n"));

        if code == FIRST_ERROR_GROUP_LAST {
            constants.push('\n');
        }

        variants.push_str(&format!("/// Pinned custom error {code}.\n{name},\n"));
        from.push_str(&format!("{constant}=>Self::{name},\n"));
        codes.push_str(&format!("Self::{name}=>{constant},\n"));
        messages.push_str(&format!("Self::{name}=>Some({message:?}),\n"));
    }

    Ok(format!(
        "// Generated from the complete pinned Stake error table.\n{constants}\n/// Known Stake errors with future codes retained losslessly.\n#[derive(Clone,Copy,Debug,Eq,PartialEq)]\npub enum StakeError {{\n{variants}/// An unknown custom code.\nUnknown(u32),\n}}\nimpl StakeError {{\n/// Decodes the exact u32 program code.\npub const fn from_code(code:u32)->Self{{\nmatch code{{{from}_=>Self::Unknown(code)}}}}\n/// Recovers the original custom code.\npub const fn code(self)->u32{{\nmatch self{{{codes}Self::Unknown(code)=>code}}}}\n/// Returns the exact pinned source message for known codes.\npub const fn message(self)->Option<&'static str>{{\nmatch self{{{messages}Self::Unknown(_)=>None}}}}\n{}\n}}\n",
        failure()
    ))
}

fn failure() -> &'static str {
    "/// Attributes an indexed failure before interpreting its custom code.\npub fn from_failure(failure:&solana_kit_core::program_error::InstructionFailure,programs:&[solana_kit_core::Address],program:solana_kit_core::Address)->Option<Self>{\nif !failure.is_program_error(programs,program,None){return None;}\n\nmatch failure.error{solana_kit_core::program_error::ProgramError::Custom(code)=>Some(Self::from_code(code)),_=>None}\n}\n"
}
