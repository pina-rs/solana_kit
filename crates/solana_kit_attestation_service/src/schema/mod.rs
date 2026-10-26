//! Bounded schema-defined data encoding. Opaque account data remains caller-owned.
mod scalars;
mod values;

use crate::{Schema, SchemaDataType, SchemaDataTypeCodec};
use alloc::{string::String, vec::Vec};
use solana_kit_core::codecs::{Decoder, Encoder, Error, Limits, Reader, SizePrefix, Utf8, Writer};
pub use values::FieldValue;

/// One named schema field in its wire order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchemaField {
    /// Strict UTF-8 field name.
    pub name: String,
    /// Exact source-defined wire type.
    pub data_type: SchemaDataType,
}

/// One named value. Encoding accepts any input order and rejects duplicate names.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttestationField {
    /// Name of a declared field.
    pub name: String,
    /// Value with the exact declared type.
    pub value: FieldValue,
}

/// Owned attestation values. No arbitrary JSON or numeric coercion is involved.
pub type AttestationData = Vec<AttestationField>;

/// A validated schema and its exact field order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttestationDataCodec {
    fields: Vec<SchemaField>,
}

impl AttestationDataCodec {
    /// Parses the actual schema blobs and rejects ambiguous duplicate field names.
    /// # Errors
    /// Preserves UTF-8, unknown-tag, truncation and shared allocation-budget errors.
    pub fn from_schema(schema: &Schema, limits: Limits) -> Result<Self, Error> {
        let size = schema
            .field_names
            .len()
            .checked_add(schema.layout.len())
            .ok_or(Error::Overflow)?;

        if size > limits.max_bytes {
            return Err(Error::LimitExceeded);
        }

        let mut reader = Reader::new(&schema.field_names, limits)?;
        let mut fields = Vec::new();

        for tag in &schema.layout {
            reader.claim(1)?;
            let data_type = SchemaDataTypeCodec.decode(&[*tag], Limits::new(1, 0))?;
            let name = SizePrefix(Utf8).read(&mut reader)?;

            if fields.iter().any(|field: &SchemaField| field.name == name) {
                return Err(Error::InvalidValue);
            }

            fields.try_reserve(1).map_err(|_| Error::AllocationFailed)?;
            fields.push(SchemaField { name, data_type });
        }

        reader.finish()?;
        Ok(Self { fields })
    }

    /// Returns validated field declarations in their wire order.
    #[must_use]
    pub fn fields(&self) -> &[SchemaField] {
        &self.fields
    }

    fn field<'a>(
        &self,
        data: &'a [AttestationField],
        field: &SchemaField,
    ) -> Result<&'a FieldValue, Error> {
        let mut matching = data.iter().filter(|value| value.name == field.name);
        let value = &matching.next().ok_or(Error::InvalidValue)?.value;

        if matching.next().is_some() || value.data_type() != field.data_type {
            return Err(Error::InvalidValue);
        }

        Ok(value)
    }
}

impl Encoder<[AttestationField]> for AttestationDataCodec {
    fn encoded_len(&self, data: &[AttestationField]) -> Result<usize, Error> {
        if data.len() != self.fields.len() {
            return Err(Error::InvalidValue);
        }

        self.fields.iter().try_fold(0usize, |size, field| {
            size.checked_add(self.field(data, field)?.encoded_len()?)
                .ok_or(Error::Overflow)
        })
    }

    fn write(&self, data: &[AttestationField], writer: &mut Writer<'_>) -> Result<(), Error> {
        self.encoded_len(data)?;

        for field in &self.fields {
            self.field(data, field)?.write(writer)?;
        }

        Ok(())
    }
}

impl Decoder<AttestationData> for AttestationDataCodec {
    fn read(&self, reader: &mut Reader<'_>) -> Result<AttestationData, Error> {
        reader.claim(self.fields.len())?;
        let mut output = Vec::new();
        output
            .try_reserve_exact(self.fields.len())
            .map_err(|_| Error::AllocationFailed)?;

        for field in &self.fields {
            reader.claim(field.name.len())?;
            let mut name = String::new();
            name.try_reserve_exact(field.name.len())
                .map_err(|_| Error::AllocationFailed)?;
            name.push_str(&field.name);
            output.push(AttestationField {
                name,
                value: FieldValue::read(field.data_type.clone(), reader)?,
            });
        }

        Ok(output)
    }
}

/// Displays a schema name or description with the reference's replacement policy.
/// The input and allocation are bounded by the caller before this presentation helper.
#[must_use]
pub fn decode_schema_text(bytes: &[u8]) -> alloc::borrow::Cow<'_, str> {
    String::from_utf8_lossy(bytes)
}

/// Decodes the unwrapped field-names blob as strict u32-length-prefixed UTF-8.
/// # Errors
/// Rejects malformed UTF-8, truncation and exhausted budgets.
pub fn decode_schema_field_names(bytes: &[u8], limits: Limits) -> Result<Vec<String>, Error> {
    use solana_kit_core::codecs::{Count, Sequence};
    Sequence {
        item: SizePrefix(Utf8),
        count: Count::Remainder,
    }
    .decode(bytes, limits)
}

/// Decodes every schema tag without accepting unknown future variants.
/// # Errors
/// Rejects unknown tags and exhausted budgets.
pub fn decode_schema_layout(bytes: &[u8], limits: Limits) -> Result<Vec<SchemaDataType>, Error> {
    use solana_kit_core::codecs::{Count, Sequence};
    Sequence {
        item: SchemaDataTypeCodec,
        count: Count::Remainder,
    }
    .decode(bytes, limits)
}
