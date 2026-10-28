//! Native tasks may require Send without imposing that bound on local Wasm adapters.

use solana_kit_client::{Reply, TransportError, rest};
use solana_kit_jupiter::{Category, Client, Interval, Service};
use std::future::{Ready, ready};

struct Native;

impl rest::Transport<Service> for Native {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, _: rest::Request<Service>) -> Self::Exchange<'_> {
        ready(Ok(Reply {
            status: 200,
            body: b"[]".to_vec(),
        }))
    }
}

fn requires_send(_: impl Send) {}

#[test]
fn native_call_futures_satisfy_send() {
    let client = Client::new(Native, rest::Budgets::default());
    requires_send(client.get_prices(&[]));
    requires_send(client.search_tokens("SOL", None));
    requires_send(client.tagged_tokens("verified"));
    requires_send(client.category_tokens(Category::TopTrending, Interval::OneHour, None));
    requires_send(client.recent_tokens());
}
