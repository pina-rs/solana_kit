//! One typed variant table owns both directions of contiguous enum tags.
use super::{Program, Variant, expressions};
use crate::schema::Error;

pub(super) fn tagged(
    name: &str,
    prefix: &str,
    variants: &[Variant],
    program: &Program,
) -> Result<String, Error> {
    let derives = super::equality::derives(program);
    let mut out = format!(
        "/// Tagged data with source-defined payload framing.\n#[derive({derives})]\npub enum {name} {{\n"
    );

    for variant in variants {
        let payload = variant
            .payload
            .as_ref()
            .map(|wire| format!("({})", expressions::rust_type(wire)))
            .unwrap_or_default();
        out.push_str(&format!(
            "/// The `{}` variant.\n{}{payload},\n",
            variant.name, variant.name
        ));
    }

    out.push_str(&format!("}}\n/// Bounded codec for [`{name}`].\n#[derive(Clone,Copy,Debug)]\npub struct {name}Codec;\n"));
    let mut lengths = String::new();
    let mut writes = String::new();
    let mut reads = String::new();

    for (index, variant) in variants.iter().enumerate() {
        let label = &variant.name;
        let tag = format!("{}_TAG_{index}", name.to_uppercase());
        out.push_str(&format!("const {tag}:{prefix}={index};\n"));
        let (pattern, length, write, read) = arms(name, label, variant.payload.as_ref(), program)?;
        lengths.push_str(&format!("{pattern}=>{length},\n"));
        writes.push_str(&format!(
            "{pattern}=>{{ Number::<{prefix}>::LE.write(&{tag},writer)?; {write} }},\n"
        ));
        reads.push_str(&format!("{tag}=>{read},\n"));
    }

    out.push_str(&format!("impl Encoder<{name}> for {name}Codec {{\nfn encoded_len(&self,value:&{name})->Result<usize,Error>{{\nlet size=match value {{\n{lengths}}};\nsize.checked_add(core::mem::size_of::<{prefix}>()).ok_or(Error::Overflow)\n}}\nfn write(&self,value:&{name},writer:&mut Writer<'_>)->Result<(),Error>{{\nmatch value {{\n{writes}}}\n}}\n}}\nimpl Decoder<{name}> for {name}Codec {{\nfn read(&self,reader:&mut Reader<'_>)->Result<{name},Error>{{\nreader.transaction(|reader| {{\nmatch Number::<{prefix}>::LE.read(reader)? {{\n{reads}_=>Err(Error::InvalidDiscriminator),\n}}\n}})\n}}\n}}\n"));
    Ok(out)
}

fn arms(
    name: &str,
    label: &str,
    payload: Option<&super::Wire>,
    program: &Program,
) -> Result<(String, String, String, String), Error> {
    let Some(payload) = payload else {
        return Ok((
            format!("{name}::{label}"),
            "0usize".to_owned(),
            "Ok(())".to_owned(),
            format!("Ok({name}::{label})"),
        ));
    };
    let codec = expressions::codec(payload, program)?;
    let value = expressions::value(payload, "value");
    let value = value.strip_prefix('&').unwrap_or(&value);
    Ok((
        format!("{name}::{label}(value)"),
        format!("{codec}.encoded_len({value})?"),
        format!("{codec}.write({value},writer)"),
        format!("Ok({name}::{label}({codec}.read(reader)?))"),
    ))
}

#[cfg(feature = "verification")]
pub(super) fn emit(name: &str, variants: &[String]) -> String {
    let mut out = format!(
        "/// Finite u8 tagged enum.\n#[derive(Clone, Debug, Eq, PartialEq)]\npub enum {name} {{\n"
    );

    for variant in variants {
        out.push_str(&format!("/// The `{variant}` variant.\n{variant},\n"));
    }

    out.push_str(&format!("}}\n/// Strict codec for [`{name}`].\n#[derive(Clone, Copy, Debug)]\npub struct {name}Codec;\nimpl {name}Codec {{\nconst VALUES: &'static [{name}] = &[\n"));

    for variant in variants {
        out.push_str(&format!("{name}::{variant},\n"));
    }

    out.push_str("];}\n");
    out.push_str(&format!("impl Encoder<{name}> for {name}Codec {{\nfn encoded_len(&self, _: &{name}) -> Result<usize, Error> {{ Ok(1) }}\nfn write(&self, value: &{name}, writer: &mut Writer<'_>) -> Result<(), Error> {{\nlet index = Self::VALUES.iter().position(|candidate| candidate == value).ok_or(Error::InvalidDiscriminator)?;\nlet tag = u8::try_from(index).map_err(|_| Error::Overflow)?;\nNumber::<u8>::LE.write(&tag, writer)\n}}\n}}\nimpl Decoder<{name}> for {name}Codec {{\nfn read(&self, reader: &mut Reader<'_>) -> Result<{name}, Error> {{\nSelf::VALUES.get(usize::from(Number::<u8>::LE.read(reader)?)).cloned().ok_or(Error::InvalidDiscriminator)\n}}\n}}\n"));
    out
}
