//! Provider object models enter through a map visitor, including when nested in arrays.

use core::{fmt, marker::PhantomData};
use serde::{
    Deserialize, Deserializer,
    de::{MapAccess, Visitor, value::MapAccessDeserializer},
};

struct Object<Output>(PhantomData<Output>);

impl<'de, Output: Deserialize<'de>> Visitor<'de> for Object<Output> {
    type Value = Output;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON object")
    }

    fn visit_map<Access: MapAccess<'de>>(self, map: Access) -> Result<Output, Access::Error> {
        Output::deserialize(MapAccessDeserializer::new(map))
    }
}

/// Delegates the existing field schema without admitting positional struct representations.
pub(crate) fn deserialize<'de, D, Output>(decoder: D) -> Result<Output, D::Error>
where
    D: Deserializer<'de>,
    Output: Deserialize<'de>,
{
    decoder.deserialize_map(Object(PhantomData))
}
