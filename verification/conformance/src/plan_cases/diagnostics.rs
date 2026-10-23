extern crate std;
use solana_kit_client::plans::{FailureAction, FailureDetails, FailureReport};
use solana_kit_core::{
    Address, Hash, Signature,
    plans::{LeafResult, Plan, Status},
    transaction::{Lifetime, Message, Version},
};
use std::prelude::v1::*;
use std::vec;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Preflight {
    units: u64,
    return_data: Vec<u8>,
}

fn canceled() -> LeafResult<(), FailureDetails<&'static str, Preflight>> {
    let message = Message::compile(
        Version::V0,
        Address::from_bytes([1; 32]),
        Lifetime::Blockhash {
            blockhash: Hash::from_bytes([2; 32]),
            last_valid_block_height: 3,
        },
        &[],
        &[],
    )
    .unwrap();
    LeafResult {
        planned_message: message,
        context: (),
        status: Status::Canceled,
    }
}

pub(super) fn nested_diagnostics_keep_typed_payload_flat_indices_and_abort_reason()
-> Result<(), &'static str> {
    let canceled = canceled();
    let failed = LeafResult {
        planned_message: canceled.planned_message.clone(),
        context: (),
        status: Status::Failed(FailureDetails {
            cause: "denied",
            logs: Some(vec![]),
            preflight: Some(Preflight {
                units: u64::MAX,
                return_data: vec![0, 255],
            }),
            signature: Some(Signature::from_bytes([8; 64])),
        }),
    };
    let result = Plan::sequential(vec![
        Plan::Parallel(vec![Plan::Single(canceled.clone()), Plan::Single(failed)]),
        Plan::Single(canceled.clone()),
    ]);
    let report = FailureReport {
        result: &result,
        action: FailureAction::Send,
        abort_reason: Some("caller halted"),
    };
    super::assert_eq!(report.failures()[0].0, 1);
    super::assert_eq!(
        report.failures()[0].1.preflight.as_ref().unwrap(),
        &Preflight {
            units: u64::MAX,
            return_data: vec![0, 255]
        }
    );
    super::assert_eq!(
        report.to_string(),
        "Failed to send transactions.\n[Tx #2 (preflight)] denied\n"
    );
    let signing = FailureReport {
        action: FailureAction::Sign,
        ..report
    };

    super::assert_eq!(
        signing.to_string(),
        "Failed to sign transactions.\n[Tx #2] denied\n"
    );
    Ok(())
}

pub(super) fn all_canceled_report_keeps_explicit_caller_reason() -> Result<(), &'static str> {
    let canceled = canceled();
    let all_canceled = Plan::Parallel(vec![Plan::Single(canceled.clone()), Plan::Single(canceled)]);
    let canceled_report = FailureReport {
        result: &all_canceled,
        action: FailureAction::Send,
        abort_reason: Some("caller halted"),
    };
    super::assert_eq!(
        canceled_report.to_string(),
        "Failed to send transactions. Canceled with abort reason: caller halted"
    );
    Ok(())
}
