//! Pinned primary endpoint contracts execute through the bounded shared REST client.

use super::service::Mock;
use solana_kit_helius::{Field, service::*, webhooks::*};

const HTTP_OK: u16 = 200;
const HTTP_NO_CONTENT: u16 = 204;
const WEBHOOK: &str = concat!(
    r#"{"webhookID":"id","wallet":"w","webhookURL":"https://example.invalid","transactionTypes":[],"#,
    r#""accountAddresses":[],"webhookType":"enhanced","authHeader":"","active":false}"#,
);

fn call<O: Operation>(operation: O, bytes: &[u8]) -> (Result<O::Response, Error>, ServiceRequest) {
    let (transport, requests) = Mock::new(HTTP_OK, bytes);
    let client = ServiceClient::new(transport, Budgets::default());
    let result = futures::executor::block_on(client.call(operation));
    assert_eq!(requests.borrow().len(), 1);
    let request = requests.borrow_mut().pop().expect("one request");
    (result, request)
}

fn request(actual: ServiceRequest, verb: Verb, path: &[&str], body: Option<serde_json::Value>) {
    assert_eq!(actual.service, Service::Webhooks);
    assert_eq!(actual.method, verb);
    assert_eq!(actual.path_segments, path);
    assert!(actual.query.is_empty());

    if let Some(expected) = body {
        assert_eq!(actual.body.content_type(), Some("application/json"));
        let actual: serde_json::Value =
            serde_json::from_slice(actual.body.as_deref().expect("JSON body")).expect("JSON");
        assert_eq!(actual, expected);
    } else {
        assert!(actual.body.as_deref().is_none());
        assert_eq!(actual.body.content_type(), None);
    }
}

pub fn vectors() {
    create();
    update();
    parameter_contracts();
    read_and_toggle();
    deletions();
    provider_errors();
    response_shapes();
}

fn create() {
    let input =
        r#"{"webhookURL":"https://example.invalid","transactionTypes":[],"accountAddresses":[]}"#;
    let operation: CreateWebhookRequest = serde_json::from_str(input).expect("optional policy");
    assert_eq!(operation.webhook_type, None);
    let (result, actual) = call(operation, WEBHOOK.as_bytes());
    let webhook = result.expect("primary response");
    assert_eq!(webhook.webhook_id, "id");
    assert_eq!(webhook.auth_header.expose(), "");
    assert_eq!(webhook.active, Field::Value(false));

    request(
        actual,
        Verb::Post,
        &["v0", "webhooks"],
        Some(serde_json::from_str(input).expect("wire")),
    );
}

fn update() {
    let changes = r#"{"webhookURL":"https://example.invalid/new","encoding":"json","authHeader":"secret-header","webhookType":"rawDevnet","txnStatus":"success","accountAddresses":[]}"#;
    let operation = UpdateWebhookRequest {
        webhook_id: "id/with?reserved&characters".into(),
        changes: serde_json::from_str(changes).expect("partial configuration"),
    };
    assert!(!format!("{operation:?}").contains("secret-header"));
    let (result, actual) = call(operation, WEBHOOK.as_bytes());
    result.expect("updated webhook");
    request(
        actual,
        Verb::Put,
        &["v0", "webhooks", "id/with?reserved&characters"],
        Some(serde_json::from_str(changes).expect("literal changes")),
    );
}

fn parameter_contracts() {
    assert!(serde_json::from_str::<UpdateWebhookParameters>(r#"{"webhookID":"id"}"#).is_err());
    assert_eq!(
        serde_json::to_string(&UpdateWebhookParameters::default()).expect("empty changes"),
        "{}"
    );
    let old_casing = concat!(
        r#"{"webhookUrl":"https://example.invalid","#,
        r#""transactionTypes":[],"accountAddresses":[]}"#,
    );
    assert!(serde_json::from_str::<CreateWebhookRequest>(old_casing).is_err());
}

fn read_and_toggle() {
    let id = "id/with?reserved&characters";
    let (result, actual) = call(
        GetWebhookRequest {
            webhook_id: id.into(),
        },
        WEBHOOK.as_bytes(),
    );
    result.expect("read webhook");
    request(actual, Verb::Get, &["v0", "webhooks", id], None);

    let list = format!("[{WEBHOOK}]");
    let (result, actual) = call(GetAllWebhooks {}, list.as_bytes());
    assert_eq!(result.expect("webhooks").len(), 1);
    request(actual, Verb::Get, &["v0", "webhooks"], None);

    let (result, actual) = call(
        ToggleWebhook {
            webhook_id: id.into(),
            active: false,
        },
        WEBHOOK.as_bytes(),
    );
    result.expect("toggled webhook");
    request(
        actual,
        Verb::Patch,
        &["v0", "webhooks", id],
        Some(serde_json::json!({"active":false})),
    );
}

fn deletions() {
    for body in ["", "null", "false", r#"{"deleted":true}"#] {
        let (result, actual) = call(
            DeleteWebhookRequest {
                webhook_id: "id".into(),
            },
            body.as_bytes(),
        );
        assert!(result.expect("HTTP acknowledgment"));
        request(actual, Verb::Delete, &["v0", "webhooks", "id"], None);
    }

    let (transport, _) = Mock::new(HTTP_NO_CONTENT, b"");
    let client = ServiceClient::new(transport, Budgets::default());
    assert!(
        futures::executor::block_on(client.call(DeleteWebhookRequest {
            webhook_id: "id".into()
        }))
        .expect("no content")
    );
    let (result, _) = call(
        DeleteWebhookRequest {
            webhook_id: "id".into(),
        },
        b"invalid JSON",
    );
    assert!(matches!(result, Err(Error::Schema(_))));
}

fn rejected<O: Operation>(operation: O)
where
    O::Response: core::fmt::Debug,
{
    let detail = r#"{ "code":18446744073709551615, "message":"private-provider-detail" }"#;
    let body = format!(r#"{{"error":{detail}}}"#);
    let (result, _) = call(operation, body.as_bytes());
    let error = result.expect_err("provider failure cannot become success");

    assert!(!format!("{error:?}").contains("private-provider-detail"));
    assert!(!error.to_string().contains("private-provider-detail"));
    let Error::Provider(raw) = error else {
        panic!("provider failure")
    };
    assert_eq!(raw.get(), detail);
}

fn provider_errors() {
    rejected(GetAllWebhooks {});
    rejected(GetWebhookRequest {
        webhook_id: "id".into(),
    });
    rejected(ToggleWebhook {
        webhook_id: "id".into(),
        active: true,
    });
    rejected(UpdateWebhookRequest {
        webhook_id: "id".into(),
        changes: UpdateWebhookParameters::default(),
    });
    rejected(CreateWebhookRequest {
        webhook_url: "https://example.invalid".into(),
        transaction_types: vec![],
        account_addresses: vec![],
        webhook_type: None,
        auth_header: None,
        encoding: None,
        txn_status: None,
    });
    rejected(DeleteWebhookRequest {
        webhook_id: "id".into(),
    });
}

fn response_shapes() {
    let mut value: serde_json::Value = serde_json::from_str(WEBHOOK).expect("fixture");
    value["webhookType"] = serde_json::json!("future-provider-format");
    value["error"] = serde_json::Value::Null;
    let (result, _) = call(
        GetWebhookRequest {
            webhook_id: "id".into(),
        },
        value.to_string().as_bytes(),
    );
    assert_eq!(
        result.expect("open string format").webhook_type,
        "future-provider-format"
    );
    value.as_object_mut().expect("object").remove("authHeader");
    let (result, _) = call(
        GetWebhookRequest {
            webhook_id: "id".into(),
        },
        value.to_string().as_bytes(),
    );
    assert!(matches!(result, Err(Error::Schema(_))));

    let tuple = r#"["id","w","url",[],[],"enhanced",""]"#;
    let (result, _) = call(
        GetWebhookRequest {
            webhook_id: "id".into(),
        },
        tuple.as_bytes(),
    );
    assert!(matches!(result, Err(Error::Schema(_))));
    let (result, _) = call(GetAllWebhooks {}, format!("[{tuple}]").as_bytes());
    assert!(matches!(result, Err(Error::Schema(_))));
}
