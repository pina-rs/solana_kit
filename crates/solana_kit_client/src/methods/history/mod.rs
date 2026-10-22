//! Typed block ranges and signature history for off-chain applications.
mod requests;
mod responses;
pub use requests::*;
pub(crate) use responses::signature;
pub use responses::*;

use crate::Error;
use serde::de::DeserializeOwned;

pub(crate) fn parse<T: DeserializeOwned>(json: &str) -> Result<T, Error> {
    serde_json::from_str(json).map_err(Error::Schema)
}
