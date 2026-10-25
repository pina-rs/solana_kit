//! Closed node vocabulary for the data projection, including retained wrapper metadata.
use crate::schema::Error;
use serde_json::Value;

const KINDS: &[&str] = &[
    "rootNode",
    "programNode",
    "accountNode",
    "instructionNode",
    "definedTypeNode",
    "instructionAccountNode",
    "instructionArgumentNode",
    "structTypeNode",
    "structFieldTypeNode",
    "numberTypeNode",
    "publicKeyTypeNode",
    "booleanTypeNode",
    "bytesTypeNode",
    "optionTypeNode",
    "arrayTypeNode",
    "tupleTypeNode",
    "definedTypeLinkNode",
    "prefixedCountNode",
    "remainderCountNode",
    "fieldDiscriminatorNode",
    "numberValueNode",
    "accountBumpValueNode",
    "accountValueNode",
    "argumentValueNode",
    "identityValueNode",
    "payerValueNode",
    "publicKeyValueNode",
    "pdaNode",
    "pdaLinkNode",
    "variablePdaSeedNode",
    "pdaValueNode",
    "pdaSeedValueNode",
    "resolverValueNode",
    "instructionByteDeltaNode",
    "instructionRemainingAccountsNode",
    "instructionAccountDisplayNode",
    "instructionDisplayNode",
    "structFieldDisplayNode",
    "enumTypeNode",
    "enumEmptyVariantTypeNode",
    "enumStructVariantTypeNode",
    "enumTupleVariantTypeNode",
    "amountTypeNode",
    "amountNumberDisplayNode",
    "dateTimeNumberDisplayNode",
    "fixedSizeTypeNode",
    "bytesValueNode",
    "sizePrefixTypeNode",
    "stringTypeNode",
    "mapTypeNode",
    "hiddenPrefixTypeNode",
    "preOffsetTypeNode",
    "constantValueNode",
    "remainderOptionTypeNode",
    "zeroableOptionTypeNode",
    "fixedCountNode",
    "sizeDiscriminatorNode",
    "errorNode",
    "accountFieldValueNode",
    "accountLinkNode",
    "booleanValueNode",
    "injectedValueNode",
    "noneValueNode",
    "providedNode",
    "stringValueNode",
    "eventNode",
    "constantDiscriminatorNode",
    "constantPdaSeedNode",
];

pub(super) fn validate(value: &Value) -> Result<(), Error> {
    // Wrapper metadata is retained, but unfamiliar node kinds cannot cross the boundary.
    match value {
        Value::Object(object) => object_nodes(object)?,
        Value::Array(items) => {
            for item in items {
                validate(item)?;
            }
        }
        _ => {}
    }

    Ok(())
}

fn object_nodes(object: &serde_json::Map<String, Value>) -> Result<(), Error> {
    let kind = object
        .get("kind")
        .and_then(Value::as_str)
        .ok_or("schema node kind required")?;

    if !KINDS.contains(&kind) {
        return Err(format!("unsupported data projection node {kind}").into());
    }

    for child in object.values() {
        validate(child)?;
    }

    Ok(())
}
