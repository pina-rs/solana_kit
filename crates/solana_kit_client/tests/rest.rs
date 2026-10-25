use solana_kit_client::{
    TransportFailureKind,
    rest::{Body, Budgets, JsonError, request_target},
};

#[test]
fn relative_segments_and_duplicate_query_pairs_keep_literal_boundaries() {
    let target = request_target(
        "/v0?api-key=approved",
        &["a/b?é".into(), "x%2Fy".into()],
        &[
            ("ids[]".into(), "a b".into()),
            ("ids[]".into(), "a+b".into()),
        ],
        256,
    )
    .expect("bounded request target");
    assert_eq!(
        target,
        "/v0/a%2Fb%3F%C3%A9/x%252Fy?api-key=approved&ids%5B%5D=a%20b&ids%5B%5D=a%2Bb"
    );
    assert_eq!(
        request_target("/", &[], &[("k".into(), "v".into())], 5).expect("exact target limit"),
        "/?k=v"
    );
    assert_eq!(
        request_target("/", &[], &[("k".into(), "v".into())], 4)
            .expect_err("encoded target limit")
            .kind,
        TransportFailureKind::BodyLimit
    );

    for segment in [".", ".."] {
        assert_eq!(
            request_target("/root", &[segment.into()], &[], 100)
                .expect_err("URL parsers normalize dot segments")
                .kind,
            TransportFailureKind::Denied
        );
    }
}

#[test]
fn absent_json_null_and_form_own_their_media_types() {
    let absent = Body::Absent;
    let null = Body::Json(b"null".to_vec());
    let form = Body::Form(b"code=a%2Bb".to_vec());
    assert_eq!((absent.as_deref(), absent.content_type()), (None, None));
    assert_eq!(
        (null.as_deref(), null.content_type()),
        (Some(b"null".as_slice()), Some("application/json"))
    );
    assert_eq!(
        (form.as_deref(), form.content_type()),
        (
            Some(b"code=a%2Bb".as_slice()),
            Some("application/x-www-form-urlencoded")
        )
    );
    assert!(!format!("{form:?}").contains("a%2Bb"));
}

#[test]
fn lexical_admission_precedes_typed_allocation_and_rejects_nested_duplicate_keys() {
    let budgets = Budgets {
        bytes: 128,
        depth: 2,
        items: 8,
        string_bytes: 8,
    };
    assert!(budgets.validate_json(br#"{"n":9007199254740993}"#).is_ok());
    assert!(matches!(
        budgets.validate_json(br#"{"a":{"x":1,"x":2}}"#),
        Err(JsonError::Schema(_))
    ));
    assert!(matches!(
        budgets.validate_json(br#"{"a":{"\u0078":1,"x":2}}"#),
        Err(JsonError::Schema(_))
    ));
    assert!(matches!(
        budgets.validate_json(b"[[[]]]"),
        Err(JsonError::Depth)
    ));
    assert!(matches!(
        budgets.validate_json(br#""123456789""#),
        Err(JsonError::String)
    ));
    assert!(matches!(
        budgets.validate_json(b"[1,2,3,4,5,6,7,8]"),
        Err(JsonError::Items)
    ));
    assert!(matches!(
        Budgets {
            bytes: 3,
            ..budgets
        }
        .serialize(&vec![0_u8; 128]),
        Err(JsonError::Bytes)
    ));
}

#[test]
fn redacted_transport_formatting_retains_the_original_error_for_explicit_inspection() {
    use solana_kit_client::{TransportError, rest::redact_error};
    use std::error::Error;
    let original = TransportError::with_source(
        TransportFailureKind::Denied,
        std::io::Error::other("fixture-private-value"),
    );
    let error = redact_error(original);
    assert_eq!(error.kind, TransportFailureKind::Denied);
    assert!(!format!("{error:?} {error}").contains("fixture-private-value"));
    let cause = error
        .source()
        .expect("redacted retained cause")
        .source()
        .expect("original transport error")
        .source()
        .expect("original I/O error");
    assert_eq!(cause.to_string(), "fixture-private-value");
}
