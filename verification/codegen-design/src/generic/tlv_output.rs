//! The source account wrapper terminates at zero tags and consumes spare capacity.
use super::{Program, Shape, Wire, expressions};
use crate::schema::Error;

/// The tagged IR owns the zero variant; standalone enum codecs remain unrestricted.
pub(super) fn codec(item: &Wire, program: &Program) -> Result<String, Error> {
    let Wire::Link(name) = item else {
        return Err("TLV sequence requires a named tagged enum".into());
    };
    let definition = program
        .definitions
        .iter()
        .find(|definition| definition.name == *name)
        .ok_or("missing TLV enum")?;

    let Shape::Tagged {
        prefix: "u16",
        variants,
    } = &definition.shape
    else {
        return Err("TLV sequence requires u16 enum tags".into());
    };
    let zero = variants.first().ok_or("missing TLV zero variant")?;
    let payload = if zero.payload.is_some() { "(..)" } else { "" };
    let codec = expressions::codec(item, program)?;
    Ok(format!(
        "TlvSequence({codec}, |value: &{name}| matches!(value, {name}::{}{payload}))",
        zero.name
    ))
}

pub(super) const SOURCE: &str = r#"
struct TlvSequence<C,F>(C,F);
impl<C:Encoder<T>,F:Fn(&T)->bool,T> Encoder<Vec<T>> for TlvSequence<C,F> {
fn encoded_len(&self,value:&Vec<T>)->Result<usize,Error> {
if value.iter().any(&self.1) {return Err(Error::InvalidValue);}

OwnedSequence(Sequence {item:&self.0,count:Count::Remainder}).encoded_len(value)
}
fn write(&self,value:&Vec<T>,writer:&mut Writer<'_>)->Result<(),Error> {
if value.iter().any(&self.1) {return Err(Error::InvalidValue);}

OwnedSequence(Sequence {item:&self.0,count:Count::Remainder}).write(value,writer)
}
}
impl<C:Decoder<T>,F,T> Decoder<Vec<T>> for TlvSequence<C,F> {
fn read(&self,reader:&mut Reader<'_>)->Result<Vec<T>,Error> {
reader.transaction(|reader| {
let mut values=Vec::new();

while let [first,second,..]=reader.remaining() {
if u16::from_le_bytes([*first,*second])==0 {break;}

let before=reader.position();
let value=self.0.read(reader)?;

if reader.position()<=before {return Err(Error::InvalidValue);}

reader.claim(1)?;
values.try_reserve_exact(1).map_err(|_|Error::AllocationFailed)?;
values.push(value);
}

reader.take(reader.remaining().len())?;
Ok(values)
})
}
}
"#;

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn zero_tag_predicate_follows_the_typed_variant_name() {
        let mut root = crate::token_2022::root().unwrap();
        let extension = root["program"]["definedTypes"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|node| node["name"] == "extension")
            .unwrap();
        extension["type"]["variants"][0]["name"] = json!("capacityEnd");
        let output = crate::token_2022::render(&root).unwrap();
        assert!(output.contains("matches!(value, Extension::CapacityEnd)"));
    }

    #[test]
    fn tlv_sequences_reject_a_non_u16_tagged_model() {
        let mut root = crate::token_2022::root().unwrap();
        let extension = root["program"]["definedTypes"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|node| node["name"] == "extension")
            .unwrap();
        extension["type"]["size"]["format"] = json!("u8");
        assert!(crate::token_2022::render(&root).is_err());
    }
}
