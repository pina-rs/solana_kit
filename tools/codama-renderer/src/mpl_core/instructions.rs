//! All 42 MIT Dart instruction payloads and ordered privilege contracts.
use super::schema::{fields, number, roles};
use crate::schema::Error;
use serde_json::{Value, json};

const ASSET: &str = "asset:w,collection:w?,payer:ws,authority:s?,systemProgram:r,logWrapper:r?";
const COLLECTION: &str = "collection:w,payer:ws,authority:s?,systemProgram:r,logWrapper:r?";
const CREATE: &str = concat!(
    "asset:ws,collection:w?,authority:s?,payer:ws,owner:r?,",
    "updateAuthority:r?,systemProgram:r,logWrapper:r?"
);
const CREATE_COLLECTION: &str = "collection:ws,updateAuthority:r?,payer:ws,systemProgram:r";
const GROUP: &str = "group:w,payer:ws,authority:s?,systemProgram:r";
const PARENT: &str = "parentGroup:w,payer:ws,authority:s?,systemProgram:r";
const CREATE_DATA: &str = "dataState:DataState,name:text,uri:text,plugins:?[]PluginAuthorityPair";
const UPDATE_DATA: &str = "newName:?text,newUri:?text,newUpdateAuthority:?UpdateAuthority";

const UPDATE_PLUGIN_TAG: u8 = 6;
const UPDATE_COLLECTION_PLUGIN_TAG: u8 = 7;
const COLLECT_TAG: u8 = 19;
const EXECUTE_TAG: u8 = 31;
const ADD_COLLECTIONS_TAG: u8 = 33;
const REMOVE_COLLECTIONS_TAG: u8 = 34;
const ADD_ASSETS_TAG: u8 = 35;
const REMOVE_ASSETS_TAG: u8 = 36;
const ADD_GROUPS_TAG: u8 = 37;
const REMOVE_GROUPS_TAG: u8 = 38;

const CREATE_GROUP_TAG: u8 = 39;
const CLOSE_GROUP_TAG: u8 = 40;

/// Tag, public operation name, role flags, and complete payload in source wire order.
pub(super) const INSTRUCTIONS: &[(u8, &str, &str, &str)] = &[
    (0, "createV1", CREATE, CREATE_DATA),
    (
        1,
        "createCollectionV1",
        CREATE_COLLECTION,
        "name:text,uri:text,plugins:?[]PluginAuthorityPair",
    ),
    (
        2,
        "addPluginV1",
        ASSET,
        "plugin:Plugin,initAuthority:?Authority",
    ),
    (
        3,
        "addCollectionPluginV1",
        COLLECTION,
        "plugin:Plugin,initAuthority:?Authority",
    ),
    (4, "removePluginV1", ASSET, "pluginType:PluginType"),
    (
        5,
        "removeCollectionPluginV1",
        COLLECTION,
        "pluginType:PluginType",
    ),
    (UPDATE_PLUGIN_TAG, "updatePluginV1", ASSET, "plugin:Plugin"),
    (
        UPDATE_COLLECTION_PLUGIN_TAG,
        "updateCollectionPluginV1",
        COLLECTION,
        "plugin:Plugin",
    ),
    (
        8,
        "approvePluginAuthorityV1",
        ASSET,
        "pluginType:PluginType,newAuthority:Authority",
    ),
    (
        9,
        "approveCollectionPluginAuthorityV1",
        COLLECTION,
        "pluginType:PluginType,newAuthority:Authority",
    ),
    (
        10,
        "revokePluginAuthorityV1",
        ASSET,
        "pluginType:PluginType",
    ),
    (
        11,
        "revokeCollectionPluginAuthorityV1",
        COLLECTION,
        "pluginType:PluginType",
    ),
    (
        12,
        "burnV1",
        "asset:w,collection:w?,payer:ws,authority:s?,systemProgram:r?,logWrapper:r?",
        "compressionProof:?CompressionProof",
    ),
    (
        13,
        "burnCollectionV1",
        "collection:w,payer:ws,authority:ws?,logWrapper:r?",
        "compressionProof:?CompressionProof",
    ),
    (
        14,
        "transferV1",
        concat!(
            "asset:w,collection:r?,payer:ws,authority:s?,newOwner:r,",
            "systemProgram:r?,logWrapper:r?"
        ),
        "compressionProof:?CompressionProof",
    ),
    (
        15,
        "updateV1",
        "asset:w,collection:r?,payer:ws,authority:s?,systemProgram:r,logWrapper:r?",
        UPDATE_DATA,
    ),
    (
        16,
        "updateCollectionV1",
        concat!(
            "collection:w,payer:ws,authority:s?,",
            "newUpdateAuthority:r?,systemProgram:r,logWrapper:r?"
        ),
        "newName:?text,newUri:?text",
    ),
    (
        17,
        "compressV1",
        "asset:w,collection:r?,payer:ws,authority:s?,systemProgram:r,logWrapper:r?",
        "",
    ),
    (
        18,
        "decompressV1",
        "asset:w,collection:r?,payer:ws,authority:s?,systemProgram:r,logWrapper:r?",
        "compressionProof:CompressionProof",
    ),
    (COLLECT_TAG, "collect", "recipient1:w,recipient2:w", ""),
    (
        20,
        "createV2",
        CREATE,
        concat!(
            "dataState:DataState,name:text,uri:text,",
            "plugins:?[]PluginAuthorityPair,externalPluginAdapters:?[]ExternalPluginAdapterInitInfo"
        ),
    ),
    (
        21,
        "createCollectionV2",
        CREATE_COLLECTION,
        concat!(
            "name:text,uri:text,",
            "plugins:?[]PluginAuthorityPair,externalPluginAdapters:?[]ExternalPluginAdapterInitInfo"
        ),
    ),
    (
        22,
        "addExternalPluginAdapterV1",
        ASSET,
        "initInfo:ExternalPluginAdapterInitInfo",
    ),
    (
        23,
        "addCollectionExternalPluginAdapterV1",
        COLLECTION,
        "initInfo:ExternalPluginAdapterInitInfo",
    ),
    (
        24,
        "removeExternalPluginAdapterV1",
        ASSET,
        "key:ExternalPluginAdapterKey",
    ),
    (
        25,
        "removeCollectionExternalPluginAdapterV1",
        COLLECTION,
        "key:ExternalPluginAdapterKey",
    ),
    (
        26,
        "updateExternalPluginAdapterV1",
        ASSET,
        "key:ExternalPluginAdapterKey,updateInfo:ExternalPluginAdapterUpdateInfo",
    ),
    (
        27,
        "updateCollectionExternalPluginAdapterV1",
        COLLECTION,
        "key:ExternalPluginAdapterKey,updateInfo:ExternalPluginAdapterUpdateInfo",
    ),
    (
        28,
        "writeExternalPluginAdapterDataV1",
        concat!(
            "asset:w,collection:w?,payer:ws,",
            "authority:s?,buffer:r?,systemProgram:r,logWrapper:r?"
        ),
        "key:ExternalPluginAdapterKey,data:?bytes",
    ),
    (
        29,
        "writeCollectionExternalPluginAdapterDataV1",
        concat!(
            "collection:w,payer:ws,",
            "authority:s?,buffer:r?,systemProgram:r,logWrapper:r?"
        ),
        "key:ExternalPluginAdapterKey,data:?bytes",
    ),
    (
        30,
        "updateV2",
        concat!(
            "asset:w,collection:w?,payer:ws,authority:s?,",
            "newCollection:w?,systemProgram:r,logWrapper:r?"
        ),
        UPDATE_DATA,
    ),
    (
        EXECUTE_TAG,
        "executeV1",
        concat!(
            "asset:w,collection:w?,assetSigner:r,payer:ws,",
            "authority:s?,systemProgram:r,programId:r"
        ),
        "instructionData:bytes",
    ),
    (
        32,
        "updateCollectionInfoV1",
        "collection:w,bubblegumSigner:s",
        "updateType:UpdateType,amount:u32",
    ),
    (ADD_COLLECTIONS_TAG, "addCollectionsToGroupV1", GROUP, ""),
    (
        REMOVE_COLLECTIONS_TAG,
        "removeCollectionsFromGroupV1",
        GROUP,
        "collections:[]address",
    ),
    (ADD_ASSETS_TAG, "addAssetsToGroupV1", GROUP, ""),
    (
        REMOVE_ASSETS_TAG,
        "removeAssetsFromGroupV1",
        GROUP,
        "assets:[]address",
    ),
    (
        ADD_GROUPS_TAG,
        "addGroupsToGroupV1",
        PARENT,
        "groups:[]address",
    ),
    (
        REMOVE_GROUPS_TAG,
        "removeGroupsFromGroupV1",
        PARENT,
        "groups:[]address",
    ),
    (
        CREATE_GROUP_TAG,
        "createGroupV1",
        "group:ws,updateAuthority:s?,payer:ws,systemProgram:r",
        "name:text,uri:text,relationships:[]RelationshipEntry",
    ),
    (
        CLOSE_GROUP_TAG,
        "closeGroupV1",
        "group:w,payer:ws,authority:s?",
        "",
    ),
    (
        41,
        "updateGroupV1",
        "group:w,payer:ws,authority:s?,newUpdateAuthority:r?,systemProgram:r",
        "newName:?text,newUri:?text",
    ),
];

pub(super) fn instructions() -> Result<Vec<Value>, Error> {
    INSTRUCTIONS.iter().map(|(tag, name, accounts, payload)| {
        let mut arguments = vec![json!({"kind":"instructionArgumentNode","name":"discriminator",
            "type":number("u8"),"defaultValue":{"kind":"numberValueNode","number":tag},
            "defaultValueStrategy":"omitted"})];
        arguments.extend(fields(payload, "instructionArgumentNode")?);
        let mut instruction = json!({
            "kind":"instructionNode","name":name,"accounts":roles(accounts)?,
            "arguments":arguments,"optionalAccountStrategy":"programId",
            "discriminators":[{
                "kind":"fieldDiscriminatorNode","name":"discriminator","offset":0
            }]
        });

        if [
            COLLECT_TAG, EXECUTE_TAG, ADD_COLLECTIONS_TAG, REMOVE_COLLECTIONS_TAG, ADD_ASSETS_TAG,
            REMOVE_ASSETS_TAG, ADD_GROUPS_TAG, REMOVE_GROUPS_TAG, CREATE_GROUP_TAG,
        ].contains(tag) {
            instruction["remainingAccounts"] = json!([{"kind":"instructionRemainingAccountsNode",
                "value":{"kind":"argumentValueNode","name":"remaining"},"isOptional":true,
                "privilegePolicy":"callerProvided"}]);
        }

        Ok(instruction)
    }).collect()
}
