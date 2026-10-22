//! Program-owned error codes with lossless unknown-code preservation.
use super::Program;

/// Preserve every known code and message, including unknown numeric codes.
pub(super) fn emit(program: &Program) -> String {
    if program.errors.is_empty() {
        return String::new();
    }

    let name = format!("{}Error", program.name);
    let mut out = format!(
        "/// Program error codes. Unknown numeric codes remain lossless.\n#[derive(Clone,Copy,Debug,Eq,PartialEq)]\npub enum {name} {{\n"
    );

    for error in &program.errors {
        out.push_str(&format!(
            "/// The `{}` program error.\n{},\n",
            error.name, error.name
        ));
    }

    out.push_str(&format!("/// An unrecognized program error code.\nUnknown(u32),\n}}\nimpl {name} {{\n/// Decodes known or unknown numeric program error codes.\npub const fn from_code(code:u32)->Self {{ match code {{\n"));

    for error in &program.errors {
        out.push_str(&format!(
            "PROGRAM_ERROR_{} => Self::{},\n",
            error.code, error.name
        ));
    }

    out.push_str("code => Self::Unknown(code),\n} }\n/// Returns the exact numeric code.\npub const fn code(self)->u32 { match self {\n");

    for error in &program.errors {
        out.push_str(&format!(
            "Self::{} => PROGRAM_ERROR_{},\n",
            error.name, error.code
        ));
    }

    out.push_str("Self::Unknown(code) => code,\n} }\n/// Returns the source message for known errors.\npub const fn message(self)->Option<&'static str> { match self {\n");

    for error in &program.errors {
        out.push_str(&format!(
            "Self::{} => Some({:?}),\n",
            error.name, error.message
        ));
    }

    out.push_str("Self::Unknown(_) => None,\n} }\n}\n");
    out
}
