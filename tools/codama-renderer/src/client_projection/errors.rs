//! Exact pinned error vocabulary with constant-time numeric preservation.
use super::Result;

pub(super) fn render() -> Result<String> {
    let rows = include_str!("../../fixtures/token-2022-rust-error-labels.tsv");
    let mut variants = String::new();
    let mut values = Vec::new();
    let mut labels = Vec::new();

    for (index, row) in rows.lines().enumerate() {
        let (code, label) = row.split_once('\t').ok_or("invalid error row")?;

        if code.parse::<usize>()? != index {
            return Err("noncontiguous pinned errors".into());
        }

        variants.push_str(&format!("/// Pinned upstream `{label}` error.\n{label},\n"));
        values.push(format!("KnownToken2022Error::{label}"));
        labels.push(format!("{label:?}"));
    }

    if values.len() != ERROR_COUNT {
        return Err("unexpected error inventory".into());
    }

    Ok(format!(
        "//! Exact pinned Token-2022 error vocabulary.\nuse solana_kit_core::{{Address,program_error::{{InstructionFailure,ProgramError}}}};\n/// Pinned contiguous source discriminants.\n#[repr(u32)]\n#[derive(Clone,Copy,Debug,Eq,PartialEq)]\npub enum KnownToken2022Error {{\n{variants}}}\nconst KNOWN:&[KnownToken2022Error]=&[{}];\nconst LABELS:&[&str]=&[{}];\n/// Program custom error with unknown numeric codes preserved.\n#[derive(Clone,Copy,Debug,Eq,PartialEq)]\npub enum Token2022Error {{\n/// A pinned upstream error.\nKnown(KnownToken2022Error),\n/// An unrecognized future numeric code.\nUnknown(u32),\n}}\nimpl Token2022Error {{\n/// Classifies the exact custom numeric code.\npub fn from_code(code:u32)->Self {{\n\nmatch usize::try_from(code).ok().and_then(|index|KNOWN.get(index)).copied() {{\nSome(known)=>Self::Known(known),None=>Self::Unknown(code),\n}}\n}}\n/// Returns the original custom numeric code.\npub fn code(self)->u32 {{\n\nmatch self {{ Self::Known(known)=>known as u32,Self::Unknown(code)=>code }}\n}}\n/// Returns the pinned Rust label for recognized errors.\npub fn label(self)->Option<&'static str> {{\n\nmatch self {{Self::Known(known)=>usize::try_from(known as u32).ok().and_then(|index|LABELS.get(index)).copied(),Self::Unknown(_)=>None}}\n}}\n}}\n/// Classifies only the actual instruction index for the explicit expected program.\npub fn classify_failure(failure:&InstructionFailure,programs:&[Address],program:Address)->Option<Token2022Error>{{\n\nif programs.get(failure.index)!=Some(&program) {{return None;}}\n\nmatch failure.error {{ProgramError::Custom(code)=>Some(Token2022Error::from_code(code)),_=>None}}\n}}\n",
        values.join(","),
        labels.join(",")
    ))
}

const ERROR_COUNT: usize = 69;
