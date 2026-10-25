//! Every pinned MIT Dart operation and its complete ordered account policy.
use super::schema::{fields, number, roles};
use crate::schema::Error;
use serde_json::{Value, json};

const CREATE: &str = "payer:ws,authority:s,credential:r,schema:r,attestation:w,systemProgram:r";
const CLOSE: &str = "payer:ws,authority:s,credential:r,attestation:w,eventAuthority:r,systemProgram:r,attestationProgram:r";
const CLOSE_ATTESTATION_TAG: u8 = 7;
const EMIT_EVENT_TAG: u8 = 228;

pub(super) const INSTRUCTIONS: &[(&str, u8, &str, &str)] = &[
    (
        "createCredential",
        0,
        "payer:ws,credential:w,authority:s,systemProgram:r",
        "name:text,signers:[]address",
    ),
    (
        "createSchema",
        1,
        "payer:ws,authority:s,credential:r,schema:w,systemProgram:r",
        "name:text,description:text,layout:[]SchemaDataType,fieldNames:[]text",
    ),
    (
        "changeSchemaStatus",
        2,
        "authority:s,credential:r,schema:w",
        "isPaused:bool",
    ),
    (
        "changeAuthorizedSigners",
        3,
        "payer:ws,authority:s,credential:w,systemProgram:r",
        "signers:[]address",
    ),
    (
        "changeSchemaDescription",
        4,
        "payer:ws,authority:s,credential:r,schema:w,systemProgram:r",
        "description:text",
    ),
    (
        "changeSchemaVersion",
        5,
        "payer:ws,authority:s,credential:r,existingSchema:r,newSchema:w,systemProgram:r",
        "layout:[]SchemaDataType,fieldNames:[]text",
    ),
    (
        "createAttestation",
        6,
        CREATE,
        "nonce:address,data:bytes,expiry:i64",
    ),
    ("closeAttestation", CLOSE_ATTESTATION_TAG, CLOSE, ""),
    (
        "tokenizeSchema",
        9,
        "payer:ws,authority:s,credential:r,schema:r,mint:w,sasPda:r,systemProgram:r,tokenProgram:r",
        "maxSize:u64",
    ),
    (
        "createTokenizedAttestation",
        10,
        concat!(
            "payer:ws,authority:s,credential:r,schema:r,attestation:w,systemProgram:r,",
            "schemaMint:w,attestationMint:w,sasPda:r,recipientTokenAccount:w,recipient:r,",
            "tokenProgram:r,associatedTokenProgram:r"
        ),
        concat!(
            "nonce:address,data:bytes,expiry:i64,name:text,uri:text,symbol:text,",
            "mintAccountSpace:u16"
        ),
    ),
    (
        "closeTokenizedAttestation",
        11,
        concat!(
            "payer:ws,authority:s,credential:r,attestation:w,eventAuthority:r,",
            "systemProgram:r,attestationProgram:r,attestationMint:w,sasPda:r,",
            "attestationTokenAccount:w,tokenProgram:r"
        ),
        "",
    ),
    ("emitEvent", EMIT_EVENT_TAG, "eventAuthority:s", ""),
];

pub(super) fn instructions() -> Result<Vec<Value>, Error> {
    INSTRUCTIONS.iter().map(|(name, tag, accounts, payload)| {
        let mut arguments = vec![json!({"kind":"instructionArgumentNode","name":"discriminator",
            "type":number("u8"),"defaultValue":{"kind":"numberValueNode","number":tag},
            "defaultValueStrategy":"omitted"})];
        arguments.extend(fields(payload,"instructionArgumentNode")?);
        Ok(json!({"kind":"instructionNode","name":name,"accounts":roles(accounts)?,
            "arguments":arguments,"optionalAccountStrategy":"programId",
            "discriminators":[{"kind":"fieldDiscriminatorNode","name":"discriminator","offset":0}]}))
    }).collect()
}
