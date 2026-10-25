//! Complete MIT Dart value contracts and keyed account layouts.
use super::schema::{constant_header, number, record, wire};
use crate::schema::Error;
use serde_json::{Value, json};

const RECORDS: &[(&str, &str)] = &[
    ("AddBlocker", ""),
    ("BubblegumV2", ""),
    ("BurnDelegate", ""),
    ("TransferDelegate", ""),
    ("PermanentTransferDelegate", ""),
    ("PermanentBurnDelegate", ""),
    ("ImmutableMetadata", ""),
    ("DataSectionUpdateInfo", ""),
    ("AgentIdentity", "uri:text"),
    (
        "AgentIdentityInitInfo",
        "uri:text,initPluginAuthority:?Authority,lifecycleChecks:[]LifecycleCheck",
    ),
    (
        "AgentIdentityUpdateInfo",
        "uri:?text,lifecycleChecks:?[]LifecycleCheck",
    ),
    (
        "AppData",
        "dataAuthority:Authority,schema:ExternalPluginAdapterSchema",
    ),
    (
        "AppDataInitInfo",
        "dataAuthority:Authority,initPluginAuthority:?Authority,schema:?ExternalPluginAdapterSchema",
    ),
    ("AppDataUpdateInfo", "schema:?ExternalPluginAdapterSchema"),
    ("Attribute", "key:text,value:text"),
    ("Attributes", "attributeList:[]Attribute"),
    ("Autograph", "signatures:[]AutographSignature"),
    ("AutographSignature", "address:address,message:text"),
    (
        "CompressionProof",
        "owner:address,updateAuthority:UpdateAuthority,name:text,uri:text,seq:u64,plugins:[]HashablePluginSchema",
    ),
    ("Creator", "address:address,percentage:u8"),
    (
        "DataSection",
        "parentKey:LinkedDataKey,schema:ExternalPluginAdapterSchema",
    ),
    (
        "DataSectionInitInfo",
        "parentKey:LinkedDataKey,schema:ExternalPluginAdapterSchema",
    ),
    ("Edition", "number:u32"),
    ("ExternalCheckResult", "flags:u32"),
    (
        "ExternalRegistryRecord",
        concat!(
            "pluginType:ExternalPluginAdapterType,authority:Authority,",
            "lifecycleChecks:?[]LifecycleCheck,offset:u64,dataOffset:?u64,dataLen:?u64"
        ),
    ),
    ("FreezeDelegate", "frozen:bool"),
    ("FreezeExecute", "frozen:bool"),
    ("Groups", "groups:[]address"),
    (
        "HashablePluginSchema",
        "index:u64,authority:Authority,plugin:Plugin",
    ),
    (
        "HashedAssetSchema",
        "assetHash:fixed32,pluginHashes:[]fixed32",
    ),
    (
        "LifecycleCheck",
        "event:HookableLifecycleEvent,result:ExternalCheckResult",
    ),
    (
        "LifecycleHook",
        "hookedProgram:address,extraAccounts:?[]ExtraAccount,dataAuthority:?Authority,schema:ExternalPluginAdapterSchema",
    ),
    (
        "LifecycleHookInitInfo",
        concat!(
            "hookedProgram:address,initPluginAuthority:?Authority,",
            "lifecycleChecks:[]LifecycleCheck,extraAccounts:?[]ExtraAccount,",
            "dataAuthority:?Authority,schema:?ExternalPluginAdapterSchema"
        ),
    ),
    (
        "LifecycleHookUpdateInfo",
        "lifecycleChecks:?[]LifecycleCheck,extraAccounts:?[]ExtraAccount,schema:?ExternalPluginAdapterSchema",
    ),
    (
        "LinkedAppData",
        "dataAuthority:Authority,schema:ExternalPluginAdapterSchema",
    ),
    (
        "LinkedAppDataInitInfo",
        "dataAuthority:Authority,initPluginAuthority:?Authority,schema:?ExternalPluginAdapterSchema",
    ),
    (
        "LinkedAppDataUpdateInfo",
        "schema:?ExternalPluginAdapterSchema",
    ),
    (
        "LinkedLifecycleHook",
        "hookedProgram:address,extraAccounts:?[]ExtraAccount,dataAuthority:?Authority,schema:ExternalPluginAdapterSchema",
    ),
    (
        "LinkedLifecycleHookInitInfo",
        concat!(
            "hookedProgram:address,initPluginAuthority:?Authority,",
            "lifecycleChecks:[]LifecycleCheck,extraAccounts:?[]ExtraAccount,",
            "dataAuthority:?Authority,schema:?ExternalPluginAdapterSchema"
        ),
    ),
    (
        "LinkedLifecycleHookUpdateInfo",
        "lifecycleChecks:?[]LifecycleCheck,extraAccounts:?[]ExtraAccount,schema:?ExternalPluginAdapterSchema",
    ),
    ("MasterEdition", "maxSupply:?u32,name:?text,uri:?text"),
    (
        "Oracle",
        "baseAddress:address,baseAddressConfig:?ExtraAccount,resultsOffset:ValidationResultsOffset",
    ),
    (
        "OracleInitInfo",
        concat!(
            "baseAddress:address,initPluginAuthority:?Authority,",
            "lifecycleChecks:[]LifecycleCheck,baseAddressConfig:?ExtraAccount,resultsOffset:?ValidationResultsOffset"
        ),
    ),
    (
        "OracleUpdateInfo",
        "lifecycleChecks:?[]LifecycleCheck,baseAddressConfig:?ExtraAccount,resultsOffset:?ValidationResultsOffset",
    ),
    ("PermanentFreezeDelegate", "frozen:bool"),
    ("PermanentFreezeExecute", "frozen:bool"),
    ("PluginAuthorityPair", "plugin:Plugin,authority:?Authority"),
    (
        "RegistryRecord",
        "pluginType:PluginType,authority:Authority,offset:u64",
    ),
    ("RelationshipEntry", "kind:RelationshipKind,key:address"),
    (
        "Royalties",
        "basisPoints:u16,creators:[]Creator,ruleSet:RuleSet",
    ),
    ("UpdateDelegate", "additionalDelegates:[]address"),
    ("VerifiedCreators", "signatures:[]VerifiedCreatorsSignature"),
    ("VerifiedCreatorsSignature", "address:address,verified:bool"),
];

const UNITS: &[(&str, &[&str])] = &[
    ("DataState", &["AccountState", "LedgerState"]),
    (
        "ExternalPluginAdapterSchema",
        &["Binary", "Json", "MsgPack"],
    ),
    (
        "ExternalPluginAdapterType",
        &[
            "LifecycleHook",
            "Oracle",
            "AppData",
            "LinkedLifecycleHook",
            "LinkedAppData",
            "DataSection",
            "AgentIdentity",
        ],
    ),
    (
        "ExternalValidationResult",
        &["Approved", "Rejected", "Pass"],
    ),
    (
        "HookableLifecycleEvent",
        &["Create", "Transfer", "Burn", "Update", "Execute"],
    ),
    (
        "Key",
        &[
            "Uninitialized",
            "AssetV1",
            "HashedAssetV1",
            "PluginHeaderV1",
            "PluginRegistryV1",
            "CollectionV1",
            "GroupV1",
        ],
    ),
    (
        "PluginType",
        &[
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
        ],
    ),
    (
        "RelationshipKind",
        &["Collection", "ChildGroup", "ParentGroup", "Asset"],
    ),
    ("UpdateType", &["Mint", "Add", "Remove"]),
    (
        "ValidationResult",
        &["Approved", "Rejected", "Pass", "ForceApproved"],
    ),
];

pub(super) fn accounts() -> Result<Vec<Value>, Error> {
    [
        (1u8, "AssetV1", "owner:address,updateAuthority:UpdateAuthority,name:text,uri:text,seq:?u64"),
        (2, "HashedAssetV1", "hash:fixed32"),
        (3, "PluginHeaderV1", "pluginRegistryOffset:u64"),
        (4, "PluginRegistryV1", "registry:[]RegistryRecord,externalRegistry:[]ExternalRegistryRecord"),
        (5, "CollectionV1", "updateAuthority:address,name:text,uri:text,numMinted:u32,currentSize:u32"),
        (6, "GroupV1", "updateAuthority:address,name:text,uri:text,collections:[]address,groups:[]address,parentGroups:[]address,assets:[]address"),
    ].iter().map(|(key, name, fields)| Ok(json!({"kind":"accountNode","name":name,
        "data":record(fields)?,"discriminators":constant_header(&format!("{key:02x}"))}))).collect()
}

pub(super) fn definitions() -> Result<Vec<Value>, Error> {
    let mut values = RECORDS
        .iter()
        .map(|(name, fields)| {
            Ok(json!({
                "kind":"definedTypeNode","name":name,"type":record(fields)?,
            }))
        })
        .collect::<Result<Vec<_>, Error>>()?;

    for (name, variants) in UNITS {
        values.push(enumeration(
            name,
            variants
                .iter()
                .map(|name| json!({"kind":"enumEmptyVariantTypeNode","name":name}))
                .collect(),
        ));
    }

    super::variants::append(&mut values)?;
    Ok(values)
}

pub(super) fn enumeration(name: &str, variants: Vec<Value>) -> Value {
    json!({"kind":"definedTypeNode","name":name,"type":{"kind":"enumTypeNode",
        "size":number("u8"),"variants":variants}})
}

pub(super) fn tuple(name: &str, payload: &str) -> Result<Value, Error> {
    Ok(json!({"kind":"enumTupleVariantTypeNode","name":name,
        "tuple":{"kind":"tupleTypeNode","items":[wire(payload)?]}}))
}

pub(super) fn named(name: &str, fields: &str) -> Result<Value, Error> {
    Ok(json!({"kind":"enumStructVariantTypeNode","name":name,"struct":record(fields)?}))
}
