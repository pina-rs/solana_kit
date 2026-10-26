//! Checkout preparation stops before side effects when public input or provider data fails.

use super::*;

const CONFIG: &str = r#"{"stripe":{"priceIds":{"Monthly":{"developer_v4":"price"},"Yearly":{}}}}"#;
const PREVIEW: &str = concat!(
    r#"{"planName":"Developer","period":"monthly","baseAmount":4900,"subtotal":4900,"#,
    r#""appliedCredits":0,"proratedCredits":0,"discounts":0,"dueToday":4900,"#,
    r#""destinationWallet":"destination","note":"preview"}"#,
);
const INTENT: &str = concat!(
    r#"{"id":"intent/opaque","status":"pending","destinationWallet":"destination","amount":4900,"#,
    r#""solanaPayUrl":"solana:fixture","expiresAt":"2026-01-01T00:00:00Z"}"#,
);

fn options() -> SignupRequest {
    let mut options = request(Plan::Developer);
    options.email = Some(SensitiveString::new("fixture@example.invalid"));
    options.first_name = Some(SensitiveString::new("fixture-first"));
    options.last_name = Some(SensitiveString::new("fixture-last"));
    options.coupon_code = Some("coupon".into());
    options.payment_host = Some("https://checkout.example.invalid/base/".into());
    options
}

fn replies(preview: &str, intent: &str) -> Vec<Reply> {
    vec![reply("[]"), reply(CONFIG), reply(preview), reply(intent)]
}

/// A new subscription freezes one price, forwards contact data and returns unsigned terms.
pub fn new_subscription() {
    let (result, calls) = execute(options(), replies(PREVIEW, INTENT));
    let SignupResult::PaymentRequired {
        identity,
        payment_link,
    } = result.expect("checkout prepared")
    else {
        panic!("new subscription needs payment");
    };
    assert_eq!(identity.jwt.expose(), "fixture-jwt");
    assert_eq!(calls.len(), 4);
    assert_eq!(calls[1].path_segments, ["dev-portal", "configs"]);
    assert_eq!(calls[2].path_segments, ["checkout", "preview"]);
    assert_eq!(
        calls[2].query,
        [
            ("priceId".into(), "price".into()),
            ("refId".into(), "reference".into()),
            ("couponCode".into(), "coupon".into())
        ]
    );
    assert_eq!(
        body(&calls[3]),
        serde_json::json!({
            "priceId":"price", "refId":"reference", "email":"fixture@example.invalid",
            "firstName":"fixture-first", "lastName":"fixture-last", "couponCode":"coupon",
            "walletAddress":identity.wallet_address.to_string()
        })
    );
    assert_eq!(
        payment_link.payment_url,
        "https://checkout.example.invalid/base/pay/intent%2Fopaque"
    );
    assert_eq!(payment_link.memo, "intent/opaque");
    assert_eq!(payment_link.amount_cents, 4900);
    assert_eq!(payment_link.plan_name, "Developer (Monthly)");
    assert_eq!(payment_link.solana_pay_url, "solana:fixture");

    let endpoints = build_endpoints(&SensitiveString::new("key&other=secret"));
    assert_eq!(
        endpoints.devnet.expose(),
        "https://devnet.helius-rpc.com/?api-key=key%26other%3Dsecret"
    );
    assert!(!format!("{endpoints:?}").contains("secret"));
}

/// Invalid inputs, zero totals and preview failures cannot silently initialize payment.
pub fn checkout_failures() {
    contact_failures();
    host_failures();
    amount_failures();
    preview_failure();
    identity_failure();
    #[cfg(not(feature = "crypto"))]
    crypto_unavailable();
}

#[cfg(not(feature = "crypto"))]
fn crypto_unavailable() {
    let (bootstrap, trace) = client(Vec::new());
    let request = SignupRequest::new(
        SignupAuthentication::SecretKey(SensitiveString::new("caller-key")),
        Plan::Developer,
    );
    let result = futures::executor::block_on(auth_signup(
        &bootstrap,
        request,
        0,
        |_| -> Result<ServiceClient<Replay>, Error> {
            panic!("crypto capability is unavailable before session binding")
        },
    ));
    assert!(matches!(result, Err(SignupError::CryptoUnavailable)));
    assert!(trace.borrow().is_empty());
}

fn contact_failures() {
    let (result, calls) = execute(request(Plan::Developer), vec![reply("[]")]);
    assert!(matches!(
        result,
        Err(SignupError::MissingContact(ContactField::Email))
    ));
    assert_eq!(calls.len(), 1);

    let mut blank = options();
    blank.first_name = Some(SensitiveString::new(" \u{feff}\t"));
    let (result, calls) = execute(blank, vec![reply("[]")]);
    assert!(matches!(
        result,
        Err(SignupError::MissingContact(ContactField::FirstName))
    ));
    assert_eq!(calls.len(), 1);
}

fn host_failures() {
    for host in [
        "/relative",
        "https://user@host",
        "https://host?query",
        "https://host#fragment",
        "file://host",
    ] {
        let mut invalid = options();
        invalid.payment_host = Some(host.into());
        let (result, calls) = execute(invalid, vec![reply("[]")]);
        assert!(
            matches!(
                result,
                Err(SignupError::Payment(PaymentError::InvalidPaymentHost))
            ),
            "{host}"
        );
        assert_eq!(calls.len(), 1);
    }
}

fn amount_failures() {
    let zero = PREVIEW.replace("\"dueToday\":4900", "\"dueToday\":0");
    let (result, calls) = execute(options(), replies(&zero, INTENT));
    assert!(matches!(
        result,
        Err(SignupError::Payment(PaymentError::ZeroAmount))
    ));
    assert_eq!(calls.len(), 3);
    let zero = INTENT.replace("\"amount\":4900", "\"amount\":0");
    let (result, _) = execute(options(), replies(PREVIEW, &zero));
    assert!(matches!(
        result,
        Err(SignupError::Payment(PaymentError::ZeroAmount))
    ));
}

fn preview_failure() {
    let mut failed = replies(PREVIEW, INTENT);
    failed[2] = Reply {
        status: 503,
        body: b"private-provider-message".to_vec(),
    };
    let (result, calls) = execute(options(), failed);
    let error = result.expect_err("preview errors propagate");
    assert!(matches!(
        error,
        SignupError::Service(Error::Http { status: 503, .. })
    ));
    assert!(!format!("{error:?}").contains("private-provider-message"));
    assert_eq!(calls.len(), 3);
}

fn identity_failure() {
    let (bootstrap, trace) = client(Vec::new());
    let options = SignupRequest::new(
        SignupAuthentication::Authenticated {
            jwt: SensitiveString::new(""),
            ref_id: "ref".into(),
            wallet_address: Address::from_bytes([1; 32]),
        },
        Plan::Developer,
    );
    let result = futures::executor::block_on(auth_signup(
        &bootstrap,
        options,
        0,
        |_| -> Result<ServiceClient<Replay>, Error> {
            panic!("invalid identity cannot bind a session")
        },
    ));
    assert!(matches!(result, Err(SignupError::InvalidIdentity)));
    assert!(trace.borrow().is_empty());
}
