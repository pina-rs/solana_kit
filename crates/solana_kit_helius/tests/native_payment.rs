#![cfg(all(not(target_arch = "wasm32"), feature = "checkout"))]

use solana_kit_client::{Reply, TransportError};
use solana_kit_helius::{auth::*, service::*};
use std::future::{Ready, ready};

struct Native;

impl ServiceTransport<Service> for Native {
    type Exchange<'a> = Ready<Result<Reply, TransportError>>;

    fn exchange(&self, request: ServiceRequest) -> Self::Exchange<'_> {
        let body = if request.path_segments == ["checkout", "preview"] {
            concat!(
                r#"{"planName":"Provider","period":"monthly","baseAmount":100,"subtotal":100,"#,
                r#""appliedCredits":0,"proratedCredits":0,"discounts":0,"dueToday":100,"#,
                r#""destinationWallet":"destination","note":"preview"}"#,
            )
        } else if request.path_segments == ["checkout", "initialize"]
            || request.path_segments == ["checkout", "intent"]
        {
            concat!(
                r#"{"id":"intent","status":"pending","destinationWallet":"destination","#,
                r#""amount":100,"solanaPayUrl":"solana:fixture","expiresAt":"later"}"#,
            )
        } else {
            panic!("unexpected checkout operation");
        };

        ready(Ok(Reply {
            status: 200,
            body: body.as_bytes().to_vec(),
        }))
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn checkout_preparation_retains_send() {
    let link = tokio::spawn(async move {
        let client = ServiceClient::new(Native, Budgets::default());
        let request = CreatePaymentRequest::new("project", PaymentPrice::Id("price".into()));
        create_payment(&client, request).await
    })
    .await
    .expect("native task joined")
    .expect("payment prepared");
    assert_eq!(link.memo, "intent");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn renewal_preparation_retains_send() {
    let link = tokio::spawn(async move {
        let client = ServiceClient::new(Native, Budgets::default());
        pay_renewal(&client, "intent", None).await
    })
    .await
    .expect("native task joined")
    .expect("renewal prepared");
    assert_eq!(link.plan_name, "Subscription renewal");
}
