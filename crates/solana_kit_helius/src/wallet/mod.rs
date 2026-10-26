//! The seven Wallet v1 beta operations use their own approved service origin.

mod balances;
mod history;
mod models;
mod point;
mod queries;

pub use balances::*;
pub use history::*;
pub use models::*;
pub use point::*;
pub use queries::*;

use crate::service::{Body, Error, Operation, Service, Verb};

request!(
    #[serde(deny_unknown_fields)]
    "Read a known wallet identity.",
    GetIdentityRequest { wallet: String => "wallet" } optional {}
);
request!(
    #[serde(deny_unknown_fields)]
    "Read known wallet identities in one exchange.",
    GetBatchIdentityRequest { addresses: Vec<String> => "addresses" } optional {}
);
request!(
    #[serde(deny_unknown_fields)]
    "Read the wallet's initial funding source.",
    GetFundedByRequest { wallet: String => "wallet" } optional {}
);

pub(super) fn path(wallet: &str, operation: &str) -> Vec<String> {
    vec![
        "v1".into(),
        "wallet".into(),
        wallet.into(),
        operation.into(),
    ]
}

impl Operation for GetIdentityRequest {
    type Response = Identity;
    const SERVICE: Service = Service::Wallet;
    const VERB: Verb = Verb::Get;
    const BODY: Body = Body::Absent;

    fn path(&self) -> Vec<String> {
        path(&self.wallet, "identity")
    }

    fn decode(&self, bytes: &[u8]) -> Result<Identity, Error> {
        crate::models::checked_json(bytes, |identity: &Identity| identity.address == self.wallet)
            .map_err(Error::Schema)
    }
}

rest!(
    GetBatchIdentityRequest,
    Vec<Identity>,
    Wallet,
    Post,
    Json,
    |_request| vec!["v1".into(), "wallet".into(), "batch-identity".into()]
);
rest!(
    GetFundedByRequest,
    FundedByResult,
    Wallet,
    Get,
    Absent,
    |request| path(&request.wallet, "funded-by")
);
