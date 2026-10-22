//! One typed variant table owns both directions of contiguous enum tags.
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
