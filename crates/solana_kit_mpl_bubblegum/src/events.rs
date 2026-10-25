//! Event validation keeps the transport record distinct from a valid leaf-schema event.
use crate::{BubblegumEventType, LeafSchema, LeafSchemaEvent, LeafSchemaEventCodec, Version};
use solana_kit_core::codecs::{Decoder, Error, Limits};

/// Decodes the Borsh event and rejects inconsistent marker/version/schema combinations.
/// This record has no Anchor eight-byte event discriminator.
/// # Errors
/// Preserves framing, branch, budget and event-domain failures.
pub fn decode_leaf_schema_event(bytes: &[u8], limits: Limits) -> Result<LeafSchemaEvent, Error> {
    let value = LeafSchemaEventCodec.decode(bytes, limits)?;
    let correct_version = matches!(
        (&value.version, &value.schema),
        (Version::V1, LeafSchema::V1(_)) | (Version::V2, LeafSchema::V2(_))
    );

    if value.event_type != BubblegumEventType::LeafSchemaEvent || !correct_version {
        return Err(Error::InvalidValue);
    }

    Ok(value)
}
