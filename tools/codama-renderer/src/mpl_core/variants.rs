//! Explicit sum types retain every source discriminator and payload shape.
use super::declarations::{enumeration, named, tuple};
use crate::schema::Error;
use serde_json::{Value, json};

fn empty(name: &str) -> Value {
    json!({"kind":"enumEmptyVariantTypeNode","name":name})
}

pub(super) fn append(values: &mut Vec<Value>) -> Result<(), Error> {
    values.push(enumeration(
        "Authority",
        vec![
            empty("None"),
            empty("Owner"),
            empty("UpdateAuthority"),
            named("Address", "address:address")?,
        ],
    ));
    values.push(enumeration(
        "UpdateAuthority",
        vec![
            empty("None"),
            tuple("Address", "address")?,
            tuple("Collection", "address")?,
        ],
    ));
    values.push(enumeration(
        "RuleSet",
        vec![
            empty("None"),
            tuple("ProgramAllowList", "[]address")?,
            tuple("ProgramDenyList", "[]address")?,
        ],
    ));
    seed_variants(values)?;
    extra_accounts(values)?;
    plugin(values)?;
    external(values)?;
    Ok(())
}

fn seed_variants(values: &mut Vec<Value>) -> Result<(), Error> {
    values.push(enumeration(
        "Seed",
        vec![
            empty("Collection"),
            empty("Owner"),
            empty("Recipient"),
            empty("Asset"),
            tuple("Address", "address")?,
            tuple("Bytes", "bytes")?,
        ],
    ));
    values.push(enumeration(
        "ValidationResultsOffset",
        vec![empty("NoOffset"), empty("Anchor"), tuple("Custom", "u64")?],
    ));
    values.push(enumeration(
        "LinkedDataKey",
        vec![
            tuple("LinkedLifecycleHook", "address")?,
            tuple("LinkedAppData", "Authority")?,
        ],
    ));
    values.push(enumeration(
        "OracleValidation",
        vec![
            empty("Uninitialized"),
            named(
                "V1",
                concat!(
                    "create:ExternalValidationResult,transfer:ExternalValidationResult,",
                    "burn:ExternalValidationResult,update:ExternalValidationResult"
                ),
            )?,
        ],
    ));
    Ok(())
}

fn extra_accounts(values: &mut Vec<Value>) -> Result<(), Error> {
    let flags = "isSigner:bool,isWritable:bool";
    let mut variants = [
        "PreconfiguredProgram",
        "PreconfiguredCollection",
        "PreconfiguredOwner",
        "PreconfiguredRecipient",
        "PreconfiguredAsset",
    ]
    .iter()
    .map(|name| named(name, flags))
    .collect::<Result<Vec<_>, _>>()?;
    variants.push(named(
        "CustomPda",
        "seeds:[]Seed,customProgramId:?address,isSigner:bool,isWritable:bool",
    )?);
    variants.push(named(
        "Address",
        "address:address,isSigner:bool,isWritable:bool",
    )?);
    values.push(enumeration("ExtraAccount", variants));
    Ok(())
}

fn plugin(values: &mut Vec<Value>) -> Result<(), Error> {
    let names = [
        "Royalties",
        "FreezeDelegate",
        "BurnDelegate",
        "TransferDelegate",
        "UpdateDelegate",
        "PermanentFreezeDelegate",
        "Attributes",
        "PermanentTransferDelegate",
        "PermanentBurnDelegate",
        "Edition",
        "MasterEdition",
        "AddBlocker",
        "ImmutableMetadata",
        "VerifiedCreators",
        "Autograph",
        "BubblegumV2",
        "FreezeExecute",
        "PermanentFreezeExecute",
        "Groups",
    ];
    values.push(enumeration(
        "Plugin",
        names
            .iter()
            .map(|name| tuple(name, name))
            .collect::<Result<Vec<_>, _>>()?,
    ));
    Ok(())
}

/// Update variants omit DataSection exactly as the pinned source does, preserving all other tags.
fn external(values: &mut Vec<Value>) -> Result<(), Error> {
    let names = [
        "LifecycleHook",
        "Oracle",
        "AppData",
        "LinkedLifecycleHook",
        "LinkedAppData",
        "DataSection",
        "AgentIdentity",
    ];

    for suffix in ["", "InitInfo", "UpdateInfo"] {
        let mut variants = Vec::new();

        for name in names {
            if suffix == "UpdateInfo" && name == "DataSection" {
                continue;
            }

            variants.push(tuple(name, &format!("{name}{suffix}"))?);
        }

        values.push(enumeration(
            &format!("ExternalPluginAdapter{suffix}"),
            variants,
        ));
    }

    values.push(enumeration(
        "ExternalPluginAdapterKey",
        vec![
            tuple("LifecycleHook", "address")?,
            tuple("Oracle", "address")?,
            tuple("AppData", "Authority")?,
            tuple("LinkedLifecycleHook", "address")?,
            tuple("LinkedAppData", "Authority")?,
            tuple("DataSection", "LinkedDataKey")?,
            empty("AgentIdentity"),
        ],
    ));
    Ok(())
}
