//! Enhanced REST history, distinct from JSON-RPC transaction reads.

mod amount;
mod models;
pub use amount::TokenAmount;
pub use models::*;

use crate::{
    enums::{EnhancedCommitment, SortOrder},
    service::{Body, Operation, Service, Verb, pair},
};

request!(
    "Decode enhanced transactions by signature.",
    GetTransactionsRequest {
        transactions: Vec<String> => "transactions",
    } optional { commitment: EnhancedCommitment => "commitment" }
);
request!(
    "Read enhanced transactions for an address.",
    GetTransactionsByAddressRequest {
        address: String => "address",
    } optional {
        before_signature: String => "beforeSignature",
        after_signature: String => "afterSignature",
        commitment: EnhancedCommitment => "commitment",
        source: String => "source",
        transaction_type: String => "type",
        limit: u64 => "limit",
        gt_slot: u64 => "gtSlot",
        gte_slot: u64 => "gteSlot",
        lt_slot: u64 => "ltSlot",
        lte_slot: u64 => "lteSlot",
        gt_time: i64 => "gtTime",
        gte_time: i64 => "gteTime",
        lt_time: i64 => "ltTime",
        lte_time: i64 => "lteTime",
        sort_order: SortOrder => "sortOrder",
    }
);

rest!(
    GetTransactionsRequest,
    Vec<EnhancedTransaction>,
    Api,
    Post,
    Json,
    |_request| vec!["v0".into(), "transactions".into()]
);

impl Operation for GetTransactionsByAddressRequest {
    type Response = Vec<EnhancedTransaction>;
    const SERVICE: Service = Service::Api;
    const VERB: Verb = Verb::Get;
    const BODY: Body = Body::Absent;

    fn path(&self) -> Vec<String> {
        vec![
            "v0".into(),
            "addresses".into(),
            self.address.clone(),
            "transactions".into(),
        ]
    }

    fn query(&self) -> Vec<(String, String)> {
        let mut query = Vec::new();
        pair(
            &mut query,
            "before-signature",
            self.before_signature.as_ref(),
        );
        pair(&mut query, "after-signature", self.after_signature.as_ref());
        pair(
            &mut query,
            "commitment",
            self.commitment.map(EnhancedCommitment::as_str),
        );
        pair(&mut query, "source", self.source.as_ref());
        pair(&mut query, "type", self.transaction_type.as_ref());
        pair(&mut query, "limit", self.limit);
        pair(&mut query, "gt-slot", self.gt_slot);
        pair(&mut query, "gte-slot", self.gte_slot);
        pair(&mut query, "lt-slot", self.lt_slot);
        pair(&mut query, "lte-slot", self.lte_slot);
        pair(&mut query, "gt-time", self.gt_time);
        pair(&mut query, "gte-time", self.gte_time);
        pair(&mut query, "lt-time", self.lt_time);
        pair(&mut query, "lte-time", self.lte_time);
        pair(
            &mut query,
            "sort-order",
            self.sort_order.map(SortOrder::as_str),
        );
        query
    }
}
