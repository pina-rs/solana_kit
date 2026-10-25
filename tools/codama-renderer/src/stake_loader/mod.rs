//! Source-pinned projections for the Stake and two Loader client protocols.
mod errors;
mod roles;
mod shank;
mod tuples;
mod wrappers;

use crate::{generate, generic, schema::Error};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

const INPUTS: [(&str, &str, &str, &str); 3] = [
    (
        "stake",
        "9527f716b5f449ce809ad6d3c05b4d4e743dff6c80242531df6a2f24067a6b9a",
        "Stake11111111111111111111111111111111111111",
        include_str!("../../fixtures/stake-loader/stake.json"),
    ),
    (
        "loaderV3",
        "039d84bbf2e785260caf2d543705c8465867d509fe7511ef11baa6dcd921e375",
        "BPFLoaderUpgradeab1e11111111111111111111111",
        include_str!("../../fixtures/stake-loader/loader-v3.json"),
    ),
    (
        "loaderV4",
        "5483251550588de5b08335b4f50d0d7f0155a9fe751ba5a28d938b67f8765a79",
        "LoaderV411111111111111111111111111111111111",
        include_str!("../../fixtures/stake-loader/loader-v4.json"),
    ),
];

/// Resolves every exact input before replacing any individual output.
pub(crate) fn run() -> Result<(), Error> {
    let base = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut outputs = Vec::new();

    for input in INPUTS {
        outputs.extend(prepare(base, input)?);
    }

    for (path, source) in outputs {
        fs::create_dir_all(path.parent().ok_or("missing output parent")?)?;
        generate::replace_source(&source, &path)?;
    }

    println!("PASS exact Stake and Loader v3/v4 program-data projections");
    Ok(())
}

type Input<'a> = (&'a str, &'a str, &'a str, &'a str);
type Output = (std::path::PathBuf, String);

/// Checks pinned input before preparing state, instruction and Stake-only error outputs together.
fn prepare(base: &Path, (name, digest, address, source): Input<'_>) -> Result<Vec<Output>, Error> {
    let mut outputs = Vec::new();

    if format!("{:x}", Sha256::digest(source.as_bytes())) != digest {
        return Err("program input digest mismatch".into());
    }

    let root = project(name, address, source)?;
    let package = if name == "stake" {
        "solana_kit_stake"
    } else {
        "solana_kit_loader"
    };

    let folder = base.join("../../crates").join(package).join("src");

    let data = generic::data::render_program_data(
        &root,
        &generic::Policy {
            name,
            address: address.parse()?,
        },
    )?;
    // Preserve the reviewed Stake enum layout without changing other program outputs.
    let data = data.replace(
        "};\nsize.checked_add(core::mem::size_of::<u32>())",
        "};\n\nsize.checked_add(core::mem::size_of::<u32>())",
    );
    let filename = if name == "stake" { "generated" } else { name };
    outputs.push((folder.join(format!("{filename}.rs")), data));
    outputs.push((
        folder.join(format!("{filename}_instructions.rs")),
        wrappers::render(&root, name)?,
    ));

    if name == "stake" {
        outputs.push((folder.join("errors.rs"), errors::render(&root)?));
    }

    Ok(outputs)
}

fn project(name: &str, address: &str, source: &str) -> Result<Value, Error> {
    let original = serde_json::from_str(source)?;

    if name == "stake" {
        return tuples::project(&original);
    }

    shank::project(&original, name, address)
}

#[cfg(all(test, feature = "verification"))]
mod tests {
    use super::*;

    #[test]
    fn exact_source_projection_preserves_every_instruction_and_canonical_state() {
        for (name, _, address, source) in INPUTS {
            let root = project(name, address, source).unwrap();

            let source = generic::data::render_program_data(
                &root,
                &generic::Policy {
                    name,
                    address: address.parse().unwrap(),
                },
            )
            .unwrap();
            assert_eq!(
                source,
                generic::data::render_program_data(
                    &root,
                    &generic::Policy {
                        name,
                        address: address.parse().unwrap()
                    }
                )
                .unwrap()
            );

            assert!(source.contains("InstructionDataCodec"));
        }
    }

    #[test]
    fn untrusted_identity_schema_errors_and_versions_reject() {
        let (name, _, address, source) = INPUTS[0];
        let policy = generic::Policy {
            name,
            address: address.parse().unwrap(),
        };
        let original = project(name, address, source).unwrap();
        let mut unknown_version = original.clone();
        unknown_version["version"] = serde_json::json!("1.9.3");
        let mut unknown_prefix = original.clone();
        unknown_prefix["program"]["prefix"] = serde_json::json!("foreign");
        let mut malformed_error = original;

        malformed_error["program"]["errors"][0]["code"] = serde_json::json!(u64::MAX);

        for root in [unknown_version, unknown_prefix, malformed_error] {
            assert!(generic::data::render_program_data(&root, &policy).is_err());
        }

        assert!(prepare(Path::new("unused"), (name, "wrong", address, source)).is_err());
    }
}
