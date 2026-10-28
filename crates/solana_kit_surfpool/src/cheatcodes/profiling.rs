//! Transaction profiling has a required null placeholder when config skips tag.

use crate::Object;
use serde::{Serialize, Serializer, ser::SerializeSeq};
use serde_json::Value;

/// Profiles an encoded transaction and optionally associates it with a tag.
#[derive(Clone, Debug, PartialEq)]
pub struct ProfileTransaction {
    /// Provider-encoded transaction data.
    pub transaction_data: String,
    /// Optional profile tag.
    pub tag: Option<String>,
    /// Optional trailing provider configuration.
    pub config: Option<Object>,
}

impl Serialize for ProfileTransaction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let has_tag_position = self.tag.is_some() || self.config.is_some();
        let count = 1 + usize::from(has_tag_position) + usize::from(self.config.is_some());
        let mut sequence = serializer.serialize_seq(Some(count))?;
        sequence.serialize_element(&self.transaction_data)?;

        if has_tag_position {
            sequence.serialize_element(&self.tag)?;
        }

        if let Some(config) = &self.config {
            sequence.serialize_element(config)?;
        }

        sequence.end()
    }
}

method!(ProfileTransaction, "surfnet_profileTransaction", Value);

macro_rules! lookup {
    ($name:ident, $wire:literal, $field:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Debug, PartialEq)]
        pub struct $name {
            /// Exact lookup value.
            pub $field: String,
            /// Optional trailing provider configuration.
            pub config: Option<Object>,
        }

        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                crate::wire::trailing(&self.$field, self.config.as_ref(), serializer)
            }
        }

        method!($name, $wire, Value);
    };
}

lookup!(
    GetTransactionProfile,
    "surfnet_getTransactionProfile",
    signature_or_uuid,
    "Read one transaction profile by signature or UUID."
);
lookup!(
    GetProfileResultsByTag,
    "surfnet_getProfileResultsByTag",
    tag,
    "Read transaction profiles recorded under one tag."
);
