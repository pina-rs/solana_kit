//! Complete original Anchor test bodies, adapted only for host-neutral reporting.
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
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

pub(super) fn foreign_data_and_user_messages_cannot_forge_events() -> Result<(), &'static str> {
    let idl = idl();

    let coder = Coder::new(&idl, LIMITS);

    super::assert!(
        coder
            .decode_event_logs(&[invoke(FOREIGN, 1), payload(), success(FOREIGN)])
            .unwrap()
            .is_empty()
    );

    super::assert!(
        coder
            .decode_event_logs(&[
                invoke(PROGRAM, 1),
                format!("Program log: {}", payload()),
                success(PROGRAM)
            ])
            .unwrap()
            .is_empty()
    );

    super::assert!(coder.decode_event_logs(&[payload()]).unwrap().is_empty());

    super::assert!(
        coder
            .decode_event_logs(&[invoke(PROGRAM, 1), success(PROGRAM), payload()])
            .unwrap()
            .is_empty()
    );
    Ok(())
}

pub(super) fn valid_event_keeps_name_and_decoded_fields() -> Result<(), &'static str> {
    let idl = idl();

    let events = Coder::new(&idl, LIMITS)
        .decode_event_logs(&[invoke(PROGRAM, 1), payload(), success(PROGRAM)])
        .unwrap();

    super::assert_eq!(events.len(), 1);

    super::assert_eq!(events[0].name, "Credit");

    let Value::Struct(fields) = &events[0].data else {
        panic!()
    };

    super::assert_eq!(fields["amount"], Value::Unsigned(99));
    Ok(())
}

pub(super) fn foreign_cpi_is_ignored_and_the_caller_is_restored() -> Result<(), &'static str> {
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

    super::assert_eq!(events.len(), 1);
    Ok(())
}

pub(super) fn target_cpi_is_recognized_inside_a_foreign_root() -> Result<(), &'static str> {
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

    super::assert_eq!(events.len(), 1);
    Ok(())
}

pub(super) fn failed_nested_invocations_restore_the_caller() -> Result<(), &'static str> {
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

    super::assert_eq!(events.len(), 1);
    Ok(())
}

pub(super) fn incomplete_and_inconsistent_log_context_fails_closed() -> Result<(), &'static str> {
    let idl = idl();

    let coder = Coder::new(&idl, LIMITS);

    super::assert!(
        coder
            .decode_event_logs(&[invoke(PROGRAM, 2), payload(), success(PROGRAM)])
            .unwrap()
            .is_empty()
    );

    super::assert!(
        coder
            .decode_event_logs(&[invoke(PROGRAM, 1), success(FOREIGN), payload()])
            .unwrap()
            .is_empty()
    );

    super::assert!(
        coder
            .decode_event_logs(&[invoke(PROGRAM, 1), invoke(PROGRAM, 3), payload()])
            .unwrap()
            .is_empty()
    );

    super::assert!(
        coder
            .decode_event_logs(&[
                format!("Program {PROGRAM} invoke [999999999999999999999999999999]"),
                payload()
            ])
            .unwrap()
            .is_empty()
    );
    Ok(())
}

pub(super) fn eligible_invalid_payloads_error_but_foreign_ones_are_never_decoded()
-> Result<(), &'static str> {
    let idl = idl();

    let coder = Coder::new(&idl, LIMITS);

    let bad = "Program data: %not base64%".to_string();

    super::assert!(
        coder
            .decode_event_logs(&[invoke(FOREIGN, 1), bad.clone(), success(FOREIGN)])
            .unwrap()
            .is_empty()
    );

    super::assert_eq!(
        coder.decode_event_logs(&[invoke(PROGRAM, 1), bad]),
        Err(Error::Base64)
    );

    let short = format!(
        "Program data: {}",
        STANDARD.encode([1, 2, 3, 4, 5, 6, 7, 8])
    );

    super::assert!(
        coder
            .decode_event_logs(&[invoke(PROGRAM, 1), short])
            .is_err()
    );
    Ok(())
}

pub(super) fn roots_reset_state_and_unknown_discriminators_are_ignored() -> Result<(), &'static str>
{
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

    super::assert_eq!(
        Coder::new(&idl, LIMITS)
            .decode_event_logs(&logs)
            .unwrap()
            .len(),
        2
    );

    super::assert!(
        Coder::new(&idl, LIMITS)
            .decode_event_logs(&Vec::<String>::new())
            .unwrap()
            .is_empty()
    );
    Ok(())
}

pub(super) fn event_budgets_cover_total_input_invocation_depth_and_entry_counts()
-> Result<(), &'static str> {
    let idl = idl();

    super::assert_eq!(
        Coder::new(&idl, Limits::new(1, 10, 10)).decode_event_logs(&[invoke(PROGRAM, 1)]),
        Err(Error::Limit)
    );

    super::assert_eq!(
        Coder::new(&idl, Limits::new(1000, 1, 10))
            .decode_event_logs(&[invoke(PROGRAM, 1), payload()]),
        Err(Error::Limit)
    );

    super::assert_eq!(
        Coder::new(&idl, Limits::new(1000, 10, 0)).decode_event_logs(&[invoke(PROGRAM, 1)]),
        Err(Error::Limit)
    );
    Ok(())
}

pub(super) fn decoded_event_allocations_share_one_aggregate_budget() -> Result<(), &'static str> {
    const AGGREGATE_ITEMS: usize = 20;
    let idl = idl();
    let coder = Coder::new(&idl, Limits::new(1000, AGGREGATE_ITEMS, 10));

    super::assert_eq!(
        coder
            .decode_event_logs(&[invoke(PROGRAM, 1), payload()])
            .unwrap()
            .len(),
        1
    );
    super::assert_eq!(
        coder.decode_event_logs(&[invoke(PROGRAM, 1), payload(), payload()]),
        Err(Error::Limit)
    );
    Ok(())
}

pub(super) fn url_safe_base64_matches_dart_without_relaxing_padding_or_unused_bits()
-> Result<(), &'static str> {
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

    super::assert_eq!(fields["amount"], Value::Unsigned(255));

    for invalid in ["/x==", "/y==", "/w"] {
        let log = format!("Program data: {invalid}");
        super::assert_eq!(
            coder.decode_event_logs(&[invoke(PROGRAM, 1), log]),
            Err(Error::Base64)
        );
    }

    Ok(())
}
