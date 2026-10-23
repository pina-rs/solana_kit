//! Private data-layout generation from the complete immutable Token-2022 IDL.
use crate::{generic, schema::Error};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::path::Path;

const PIN: &str = "13cbda7213599f3312e1b7ffd1c7f176f36ebd74dba9feabe6e9a6874dc1d5d2";

pub(crate) fn root() -> Result<Value, Error> {
    let bytes = include_bytes!("../fixtures/token-2022-original.json");

    if format!("{:x}", Sha256::digest(bytes)) != PIN {
        return Err("Token-2022 source pin mismatch".into());
    }

    Ok(serde_json::from_slice(bytes)?)
}

pub(crate) fn render(root: &Value) -> Result<String, Error> {
    generic::data::render_tlv(
        root,
        &generic::Policy {
            name: "token2022",
            address: "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb".parse()?,
        },
    )
}

pub(crate) fn run(output: &Path) -> Result<(), Error> {
    crate::generate::replace_source(&render(&root()?)?, output)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn complete_pinned_data_shapes_render_deterministically() {
        let root = super::root().unwrap();
        let source = super::render(&root).unwrap();
        assert_eq!(source, super::render(&root).unwrap());
        assert!(source.contains("pub struct Multisig"));
        assert!(source.contains("pub enum Extension"));
        assert!(source.contains("pub struct InitializeTokenMetadata"));
    }

    #[test]
    fn source_mutations_reject_unknown_nodes_cycles_collisions_and_tags() {
        let original = super::root().unwrap();
        let mut mutations = Vec::new();
        let mut unknown = original.clone();
        unknown["program"]["instructions"][0]["arguments"][1]["type"]["kind"] =
            json!("unknownTypeNode");
        mutations.push(unknown);
        let mut cycle = original.clone();
        cycle["program"]["definedTypes"][2]["type"]["fields"][0]["type"] =
            json!({"kind":"definedTypeLinkNode","name":"transferFee"});
        mutations.push(cycle);
        let mut collision = original.clone();
        collision["program"]["definedTypes"][0]["name"] = json!("OwnedString");
        mutations.push(collision);
        let mut tag = original.clone();
        tag["program"]["instructions"][1]["arguments"][0]["defaultValue"]["number"] = json!(0);
        mutations.push(tag);
        let mut identity = original.clone();
        identity["program"]["name"] = json!("untrusted");
        mutations.push(identity);

        for mutation in mutations {
            assert!(super::render(&mutation).is_err());
        }
    }

    #[test]
    fn mixed_width_prefix_ambiguity_is_rejected_before_replacement() {
        let mut root: serde_json::Value =
            serde_json::from_str(include_str!("../fixtures/alt-original.json")).unwrap();
        let mut byte = root["program"]["instructions"][0].clone();
        byte["name"] = json!("byteHeader");
        byte["arguments"] = json!([
            {"kind":"instructionArgumentNode","name":"discriminator","defaultValueStrategy":"omitted","defaultValue":{"kind":"numberValueNode","number":1},"type":{"kind":"numberTypeNode","format":"u8","endian":"le"}},
            {"kind":"instructionArgumentNode","name":"a","type":{"kind":"numberTypeNode","format":"u8","endian":"le"}},
            {"kind":"instructionArgumentNode","name":"b","type":{"kind":"numberTypeNode","format":"u8","endian":"le"}},
            {"kind":"instructionArgumentNode","name":"c","type":{"kind":"numberTypeNode","format":"u8","endian":"le"}}
        ]);
        let mut word = byte.clone();
        word["name"] = json!("wordHeader");
        word["arguments"].as_array_mut().unwrap().truncate(1);

        word["arguments"][0]["type"]["format"] = json!("u32");
        root["program"]["instructions"] = json!([byte, word]);
        let path = std::env::temp_dir().join(format!("kit226-prefix-{}.rs", std::process::id()));
        std::fs::write(&path, "preserved output").unwrap();
        let policy = crate::generic::Policy {
            name: "addressLookupTable",
            address: "AddressLookupTab1e1111111111111111111111111"
                .parse()
                .unwrap(),
        };
        assert!(crate::generic::data::replace(&root, &policy, &path).is_err());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "preserved output");
        assert!(!path.with_extension("rs.pending").exists());
    }
}
