use base64::{Engine, engine::general_purpose::STANDARD};
use serde_json::json;
use solana_kit_anchor::{Coder, Error, Idl, Limits, Value};

const PROGRAM: &str = "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS";
const FOREIGN: &str = "11111111111111111111111111111111";
const LIMITS: Limits = Limits::new(1_000_000, 100_000, 64);

fn idl() -> Idl {
    Idl::from_value(&json!({"address": PROGRAM, "events": [{"name": "Credit", "discriminator": [1,2,3,4,5,6,7,8]}],
        "types": [{"name": "Credit", "type": {"kind": "struct", "fields": [{"name": "amount", "type": "u8"}]}}]
    }), LIMITS).unwrap()
}

fn payload() -> String {
    format!(
        "Program data: {}",
        STANDARD.encode([1, 2, 3, 4, 5, 6, 7, 8, 99])
    )
}

fn invoke(program: &str, depth: usize) -> String {
    format!("Program {program} invoke [{depth}]")
}
fn success(program: &str) -> String {
    format!("Program {program} success")
}

#[test]
fn foreign_data_and_user_messages_cannot_forge_events() {
    let idl = idl();

    let coder = Coder::new(&idl, LIMITS);

    assert!(
        coder
            .decode_event_logs(&[invoke(FOREIGN, 1), payload(), success(FOREIGN)])
            .unwrap()
            .is_empty()
    );

    assert!(
        coder
            .decode_event_logs(&[
                invoke(PROGRAM, 1),
                format!("Program log: {}", payload()),
                success(PROGRAM)
            ])
            .unwrap()
            .is_empty()
    );

    assert!(coder.decode_event_logs(&[payload()]).unwrap().is_empty());

    assert!(
        coder
            .decode_event_logs(&[invoke(PROGRAM, 1), success(PROGRAM), payload()])
            .unwrap()
            .is_empty()
    );
}

#[test]
fn valid_event_keeps_name_and_decoded_fields() {
    let idl = idl();

    let events = Coder::new(&idl, LIMITS)
        .decode_event_logs(&[invoke(PROGRAM, 1), payload(), success(PROGRAM)])
        .unwrap();

    assert_eq!(events.len(), 1);

    assert_eq!(events[0].name, "Credit");

    let Value::Struct(fields) = &events[0].data else {
        panic!()
    };

    assert_eq!(fields["amount"], Value::Unsigned(99));
}

#[test]
fn foreign_cpi_is_ignored_and_the_caller_is_restored() {
    let idl = idl();

    let events = Coder::new(&idl, LIMITS)
        .decode_event_logs(&[
            invoke(PROGRAM, 1),
            invoke(FOREIGN, 2),
            payload(),
            success(FOREIGN),
            payload(),
            success(PROGRAM),
        ])
        .unwrap();

    assert_eq!(events.len(), 1);
}

#[test]
fn target_cpi_is_recognized_inside_a_foreign_root() {
    let idl = idl();

    let events = Coder::new(&idl, LIMITS)
        .decode_event_logs(&[
            invoke(FOREIGN, 1),
            invoke(PROGRAM, 2),
            payload(),
            success(PROGRAM),
            payload(),
            success(FOREIGN),
        ])
        .unwrap();

    assert_eq!(events.len(), 1);
}

#[test]
fn failed_nested_invocations_restore_the_caller() {
    let idl = idl();

    let events = Coder::new(&idl, LIMITS)
        .decode_event_logs(&[
            invoke(PROGRAM, 1),
            invoke(FOREIGN, 2),
            payload(),
            format!("Program {FOREIGN} failed: custom program error: 0x1"),
            payload(),
            success(PROGRAM),
        ])
        .unwrap();

    assert_eq!(events.len(), 1);
}

#[test]
fn incomplete_and_inconsistent_log_context_fails_closed() {
    let idl = idl();

    let coder = Coder::new(&idl, LIMITS);

    assert!(
        coder
            .decode_event_logs(&[invoke(PROGRAM, 2), payload(), success(PROGRAM)])
            .unwrap()
            .is_empty()
    );

    assert!(
        coder
            .decode_event_logs(&[invoke(PROGRAM, 1), success(FOREIGN), payload()])
            .unwrap()
            .is_empty()
    );

    assert!(
        coder
            .decode_event_logs(&[invoke(PROGRAM, 1), invoke(PROGRAM, 3), payload()])
            .unwrap()
            .is_empty()
    );

    assert!(
        coder
            .decode_event_logs(&[
                format!("Program {PROGRAM} invoke [999999999999999999999999999999]"),
                payload()
            ])
            .unwrap()
            .is_empty()
    );
}

#[test]
fn eligible_invalid_payloads_error_but_foreign_ones_are_never_decoded() {
    let idl = idl();

    let coder = Coder::new(&idl, LIMITS);

    let bad = "Program data: %not base64%".to_string();

    assert!(
        coder
            .decode_event_logs(&[invoke(FOREIGN, 1), bad.clone(), success(FOREIGN)])
            .unwrap()
            .is_empty()
    );

    assert_eq!(
        coder.decode_event_logs(&[invoke(PROGRAM, 1), bad]),
        Err(Error::Base64)
    );

    let short = format!(
        "Program data: {}",
        STANDARD.encode([1, 2, 3, 4, 5, 6, 7, 8])
    );

    assert!(
        coder
            .decode_event_logs(&[invoke(PROGRAM, 1), short])
            .is_err()
    );
}

#[test]
fn roots_reset_state_and_unknown_discriminators_are_ignored() {
    let idl = idl();

    let unknown = format!(
        "Program data: {}",
        STANDARD.encode([8, 7, 6, 5, 4, 3, 2, 1])
    );

    let logs = [
        invoke(FOREIGN, 1),
        invoke(PROGRAM, 1),
        unknown,
        payload(),
        success(PROGRAM),
        invoke(PROGRAM, 1),
        payload(),
    ];

    assert_eq!(
        Coder::new(&idl, LIMITS)
            .decode_event_logs(&logs)
            .unwrap()
            .len(),
        2
    );

    assert!(
        Coder::new(&idl, LIMITS)
            .decode_event_logs(&Vec::<String>::new())
            .unwrap()
            .is_empty()
    );
}

#[test]
fn event_budgets_cover_total_input_invocation_depth_and_entry_counts() {
    let idl = idl();

    assert_eq!(
        Coder::new(&idl, Limits::new(1, 10, 10)).decode_event_logs(&[invoke(PROGRAM, 1)]),
        Err(Error::Limit)
    );

    assert_eq!(
        Coder::new(&idl, Limits::new(1000, 1, 10))
            .decode_event_logs(&[invoke(PROGRAM, 1), payload()]),
        Err(Error::Limit)
    );

    assert_eq!(
        Coder::new(&idl, Limits::new(1000, 10, 0)).decode_event_logs(&[invoke(PROGRAM, 1)]),
        Err(Error::Limit)
    );
}

#[test]
fn decoded_event_allocations_share_one_aggregate_budget() {
    const AGGREGATE_ITEMS: usize = 20;
    let idl = idl();
    let coder = Coder::new(&idl, Limits::new(1000, AGGREGATE_ITEMS, 10));

    assert_eq!(
        coder
            .decode_event_logs(&[invoke(PROGRAM, 1), payload()])
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        coder.decode_event_logs(&[invoke(PROGRAM, 1), payload(), payload()]),
        Err(Error::Limit)
    );
}

#[test]
fn url_safe_base64_matches_dart_without_relaxing_padding_or_unused_bits() {
    let idl = idl();
    let coder = Coder::new(&idl, LIMITS);
    let payload = STANDARD
        .encode([1, 2, 3, 4, 5, 6, 7, 8, 255])
        .replace('/', "_");
    let log = format!("Program data: {payload}");
    let events = coder.decode_event_logs(&[invoke(PROGRAM, 1), log]).unwrap();
    let Value::Struct(fields) = &events[0].data else {
        panic!()
    };

    assert_eq!(fields["amount"], Value::Unsigned(255));

    for invalid in ["/x==", "/y==", "/w"] {
        let log = format!("Program data: {invalid}");
        assert_eq!(
            coder.decode_event_logs(&[invoke(PROGRAM, 1), log]),
            Err(Error::Base64)
        );
    }
}
