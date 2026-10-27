#![cfg(feature = "hermes")]
//! Public GET operations run with a local, non-Send transport and an explicit clock.
#[path = "hermes_http/clock.rs"]
mod clock;
#[path = "hermes_http/peer.rs"]
mod peer;
#[path = "hermes_http/requests.rs"]
mod requests;
#[path = "hermes_http/retries.rs"]
mod retries;

use solana_kit_client::rest::Budgets;
use solana_kit_pyth::hermes::{Client, Policy};

const UPDATE: &[u8] = br#"{"binary":{"encoding":"hex","data":[]}}"#;
const LISTING: &[u8] = br#"[{"id":"feed-one","attributes":{"symbol":"A"}}]"#;

/// Gives each test independent protocol state and a real monotonic sleep namespace.
fn client(
    peer: peer::Peer,
    clock: clock::Manual,
    policy: Policy,
) -> Client<peer::Peer, clock::Manual> {
    Client::new(peer, clock, policy, Budgets::default())
}
