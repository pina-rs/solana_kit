//! Exact pinned input entry points for reusable data emission.
use crate::{
    generate,
    generic::{self, Policy},
    schema::Error,
};
use sha2::{Digest, Sha256};
use std::path::Path;

const INPUTS: [(&str, &str, &str, &str, &str); 2] = [
    (
        include_str!("../fixtures/alt-original.json"),
        "f23e8397def424afc9b38648ca3d7b7ed7fbcfa1c690af2c910b9e556bfd4a8a",
        "addressLookupTable",
        "AddressLookupTab1e1111111111111111111111111",
        "solana_kit_address_lookup_table",
    ),
    (
        include_str!("../fixtures/config-original.json"),
        "b11fc73aa20bdd5199a5ae4193b835e16ad65f7b563844bd200baa097b209131",
        "solanaConfig",
        "Config1111111111111111111111111111111111111",
        "solana_kit_config",
    ),
];

pub(crate) fn run() -> Result<(), Error> {
    // Render both complete inputs before the first atomic single-file replacement.
    let outputs = INPUTS
        .iter()
        .map(|(source, hash, name, address, package)| {
            if format!("{:x}", Sha256::digest(source.as_bytes())) != *hash {
                return Err("pinned data input drift".into());
            }

            let policy = Policy {
                name,
                address: address.parse()?,
            };
            Ok((
                *package,
                generic::data::render(&serde_json::from_str(source)?, &policy)?,
            ))
        })
        .collect::<Result<Vec<_>, Error>>()?;

    for (package, source) in outputs {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../crates")
            .join(package)
            .join("src/generated.rs");
        generate::replace_source(&source, &path)?;
    }

    println!("PASS exact ALT and Config input data emission");
    Ok(())
}

#[cfg(all(test, feature = "verification"))]
mod tests {
    use super::*;

    #[test]
    fn exact_inputs_regenerate_the_authenticated_golden_sources() {
        for (index, (source, _, name, address, package)) in INPUTS.iter().enumerate() {
            let policy = Policy {
                name,
                address: address.parse().unwrap(),
            };
            let root = serde_json::from_str(source).unwrap();
            let output = std::env::temp_dir().join(format!(
                "kit226-production-{index}-{}.rs",
                std::process::id()
            ));
            generic::data::replace(&root, &policy, &output).unwrap();

            let expected = match *package {
                "solana_kit_address_lookup_table" => {
                    include_str!("../fixtures/alt-generated.rs")
                }
                "solana_kit_config" => {
                    include_str!("../fixtures/config-generated.rs")
                }
                _ => unreachable!(),
            };

            assert_eq!(std::fs::read_to_string(output).unwrap(), expected);
        }
    }

    #[test]
    fn exact_inputs_are_deterministic_and_unknown_wire_rejects() {
        for (source, _, name, address, _) in INPUTS {
            let policy = Policy {
                name,
                address: address.parse().unwrap(),
            };
            let mut root: serde_json::Value = serde_json::from_str(source).unwrap();
            let first = generic::data::render(&root, &policy).unwrap();
            assert_eq!(first, generic::data::render(&root, &policy).unwrap());
            root["program"]["instructions"][0]["arguments"][0]["type"]["kind"] =
                serde_json::json!("unknownTypeNode");
            assert!(generic::data::render(&root, &policy).is_err());
        }
    }

    #[test]
    fn invalid_actual_inputs_preserve_all_output_states() {
        for (index, (source, _, name, address, _)) in INPUTS.iter().enumerate() {
            let policy = Policy {
                name,
                address: address.parse().unwrap(),
            };
            let mut root: serde_json::Value = serde_json::from_str(source).unwrap();
            root["program"]["instructions"][0]["arguments"][0]["type"]["kind"] =
                serde_json::json!("unsupportedNode");

            for state in 0..4 {
                preserved_state(&root, &policy, index, state);
            }
        }
    }

    fn preserved_state(root: &serde_json::Value, policy: &Policy<'_>, index: usize, state: usize) {
        let directory =
            std::env::temp_dir().join(format!("kit216-{}-{index}-{state}", std::process::id()));
        std::fs::create_dir(&directory).unwrap();

        let output = directory.join("generated.rs");
        let pending = output.with_extension("rs.pending");

        if state & 1 != 0 {
            std::fs::write(&output, b"prior output").unwrap();
        }

        if state & 2 != 0 {
            std::fs::write(&pending, b"pending sentinel").unwrap();
        }

        assert!(generic::data::replace(root, policy, &output).is_err());
        assert_eq!(
            std::fs::read(&output).ok(),
            (state & 1 != 0).then(|| b"prior output".to_vec())
        );
        assert_eq!(
            std::fs::read(&pending).ok(),
            (state & 2 != 0).then(|| b"pending sentinel".to_vec())
        );
    }
}
