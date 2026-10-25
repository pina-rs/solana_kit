//! The authenticated MIT Dart constants and quoted messages are the source input.
use crate::schema::{Error, identifier};
use serde_json::{Value, json};

const SOURCE: &str = include_str!("../../fixtures/mpl-token-metadata/errors.dart.txt");
const PREFIX: &str = "mplTokenMetadataError";
const COUNT: usize = 203;

pub(super) fn errors() -> Result<Vec<Value>, Error> {
    parse(SOURCE)
}

/// Keep pending Dart declarations until their terminator, then enforce contiguous wire codes.
fn parse(source: &str) -> Result<Vec<Value>, Error> {
    let constants = source
        .split_once("/// Map of error codes")
        .ok_or("missing Dart error-map boundary")?
        .0;
    let mut result = Vec::new();
    let mut message = String::new();
    let mut declaration = String::new();

    for line in constants.lines().map(str::trim) {
        if let Some(quoted) = line.strip_prefix("/// Message: ") {
            message = serde_json::from_str(quoted)?;
        }

        if !line.starts_with("const int") && declaration.is_empty() {
            continue;
        }

        declaration.push_str(line);

        if !declaration.contains(';') {
            continue;
        }

        result.push(error(&declaration, &message, result.len())?);
        declaration.clear();
        message.clear();
    }

    if !declaration.is_empty() || result.len() != COUNT {
        return Err("incomplete Dart Token Metadata error constants".into());
    }

    Ok(result)
}

fn error(source: &str, message: &str, index: usize) -> Result<Value, Error> {
    let statement = source
        .strip_prefix("const int")
        .ok_or("unexpected Dart error declaration")?
        .split_once(';')
        .ok_or("unterminated Dart error declaration")?
        .0;
    let (name, value) = statement
        .split_once('=')
        .ok_or("missing Dart error value")?;
    let name = name
        .trim()
        .strip_prefix(PREFIX)
        .ok_or("unknown Dart error owner")?;
    identifier(name)?;
    let code = u32::from_str_radix(
        value
            .trim()
            .strip_prefix("0x")
            .ok_or("Dart error requires a hexadecimal literal")?,
        16,
    )?;

    if code != u32::try_from(index)? {
        return Err("Dart error codes must retain complete contiguous source order".into());
    }

    Ok(json!({"kind":"errorNode","name":name,"code":code,"message":message}))
}

#[cfg(all(test, feature = "verification"))]
mod tests {
    use super::*;

    #[test]
    fn complete_error_input_and_literal_escape_failures_are_explicit() {
        const CLOSING_CONDITIONS_INDEX: usize = 202;
        let rows = errors().unwrap();
        assert_eq!(rows[5]["name"], "InvalidMetadataKey");
        assert_eq!(
            rows[5]["message"],
            " Metadata's key must match seed of ['metadata', program id, mint] provided"
        );
        assert_eq!(
            rows[CLOSING_CONDITIONS_INDEX]["name"],
            "ConditionsForClosingNotMet"
        );
        assert!(parse(&SOURCE.replace("= 0x0;", "= 0x1;")).is_err());
        assert!(parse(&SOURCE.replace("/// Map of error codes", "/// changed")).is_err());
        assert!(parse(&SOURCE.replace("= 0xca;", "= 0xcb;")).is_err());
    }
}
