//! Validated primitive formats retain their byte order through every numeric use.
use super::parse::keys;
use crate::schema::Error;
use serde_json::Value;
use solana_kit_core::codecs::Endian;

#[derive(Clone, Copy)]
pub(crate) struct Numeric {
    pub(crate) format: &'static str,
    pub(crate) endian: Endian,
}

#[derive(Clone, Copy)]
pub(crate) enum CountPrefix {
    Number(Numeric),
    ShortU16,
}

impl Numeric {
    pub(super) fn parse(node: &Value) -> Result<Self, Error> {
        keys(node, &["kind", "format", "endian", "display"])?;

        if node["kind"] != "numberTypeNode" {
            return Err("expected numeric encoding".into());
        }

        let endian = match node["endian"].as_str() {
            Some("le") => Endian::Little,
            Some("be") => Endian::Big,
            _ => return Err("unsupported numeric byte order".into()),
        };
        let format = format(&node["format"])?;

        Ok(Self { format, endian })
    }

    pub(super) fn unsigned(node: &Value) -> Result<Self, Error> {
        let number = Self::parse(node)?;

        if !["u8", "u16", "u32", "u64"].contains(&number.format) {
            return Err("length and count prefixes must be unsigned integers".into());
        }

        Ok(number)
    }

    pub(super) fn codec(self) -> String {
        let order = match self.endian {
            Endian::Little => "LE",
            Endian::Big => "BE",
        };

        format!("Number::<{}>::{order}", self.format)
    }

    pub(super) fn legacy_option(self) -> bool {
        self.format == "u8" || (self.format == "u32" && self.endian == Endian::Little)
    }
}

fn format(value: &Value) -> Result<&'static str, Error> {
    const FORMATS: &[&str] = &[
        "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128", "f32", "f64",
    ];
    let original = value.as_str().ok_or("numeric format must be a string")?;

    FORMATS
        .iter()
        .copied()
        .find(|format| *format == original)
        .ok_or_else(|| "unsupported numeric format".into())
}
