//! MIT Dart accounts, named types and custom errors. Primary IDL bytes stay private.
use super::schema::{header, number, record};
use crate::schema::Error;
use serde_json::{Value, json};

pub(super) const ACCOUNTS: &[(&str, u8, &str)] = &[
    (
        "Credential",
        0,
        "authority:address,name:bytes,authorizedSigners:[]address",
    ),
    (
        "Schema",
        1,
        concat!(
            "credential:address,name:bytes,description:bytes,layout:bytes,",
            "fieldNames:bytes,isPaused:bool,version:u8"
        ),
    ),
    (
        "Attestation",
        2,
        concat!(
            "nonce:address,credential:address,schema:address,data:bytes,",
            "signer:address,expiry:i64,tokenAccount:address"
        ),
    ),
];

pub(super) const TYPES: &[&str] = &[
    "U8",
    "U16",
    "U32",
    "U64",
    "U128",
    "I8",
    "I16",
    "I32",
    "I64",
    "I128",
    "Bool",
    "Char",
    "String",
    "VecU8",
    "VecU16",
    "VecU32",
    "VecU64",
    "VecU128",
    "VecI8",
    "VecI16",
    "VecI32",
    "VecI64",
    "VecI128",
    "VecBool",
    "VecChar",
    "VecString",
];

pub(super) fn accounts() -> Result<Vec<Value>, Error> {
    ACCOUNTS
        .iter()
        .map(|(name, tag, fields)| {
            Ok(json!({"kind":"accountNode","name":name,
        "data":record(fields)?,"discriminators":header(*tag)}))
        })
        .collect()
}

pub(super) fn definitions() -> Result<Vec<Value>, Error> {
    let variants = TYPES
        .iter()
        .map(|name| {
            json!({"kind":"enumEmptyVariantTypeNode",
        "name":name})
        })
        .collect::<Vec<_>>();
    Ok(vec![
        json!({"kind":"definedTypeNode","name":"SchemaDataType",
        "type":{"kind":"enumTypeNode","size":number("u8"),"variants":variants}}),
        json!({"kind":"definedTypeNode","name":"CloseAttestationEvent",
            "type":record("discriminator:u8,schema:address,attestationData:bytes")?}),
    ])
}

pub(super) fn errors() -> Vec<Value> {
    [
        "InvalidCredential",
        "InvalidSchema",
        "InvalidAttestation",
        "InvalidAuthority",
        "InvalidSchemaDataType",
        "SignerNotAuthorized",
        "InvalidAttestationData",
        "InvalidEventAuthority",
        "InvalidMint",
        "InvalidProgramSigner",
        "InvalidTokenAccount",
        "SchemaPaused",
    ]
    .iter()
    .enumerate()
    .map(|(code, name)| json!({"kind":"errorNode","name":name,"code":code,"message":name}))
    .collect()
}
