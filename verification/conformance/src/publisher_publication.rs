//! Every original normalized-projection body runs through four appended portable groups.

/// Reports a changed projection comparison as an error instead of unwinding a guest.
///
/// Native builds keep the standard assertion; portable execution receives the
/// first failed comparison as a static report entry so later groups still run.
macro_rules! assert_eq {
    ($left:expr, $right:expr $(,)?) => {
        if $left != $right {
            return Err(concat!(
                "projection assertion failed: ",
                stringify!($left),
                " != ",
                stringify!($right)
            ));
        }
    };
    ($left:expr, $right:expr, $($arguments:tt)+) => {
        if $left != $right {
            return Err(concat!(
                "projection assertion failed: ",
                stringify!($left),
                " != ",
                stringify!($right)
            ));
        }
    };
}

/// Reports a false projection predicate as an error instead of unwinding a guest.
macro_rules! assert {
    ($condition:expr $(,)?) => {
        if !$condition {
            return Err(concat!(
                "projection assertion failed: ",
                stringify!($condition)
            ));
        }
    };
    ($condition:expr, $($arguments:tt)+) => {
        if !$condition {
            return Err(concat!(
                "projection assertion failed: ",
                stringify!($condition)
            ));
        }
    };
}

#[path = "../../../crates/solana_kit_dapp_publisher/tests/publication/bundle.rs"]
mod bundle;
#[path = "../../../crates/solana_kit_dapp_publisher/tests/publication/diagnostics.rs"]
mod diagnostics;
#[path = "../../../crates/solana_kit_dapp_publisher/tests/publication/ingestion.rs"]
mod ingestion;
#[path = "../../../crates/solana_kit_dapp_publisher/tests/publication/session.rs"]
mod session;
#[path = "../../../crates/solana_kit_dapp_publisher/tests/publication/support.rs"]
mod support;

use super::{checks, run_groups};
use crate::CaseResult;

pub(super) use checks::Outcome;

/// Appends the four projection groups after every preserved and appended case.
pub(crate) fn cases() -> [CaseResult; 4] {
    run_groups([
        (
            "publisher-publication-bundle-source-projection",
            bundle_group,
        ),
        (
            "publisher-publication-diagnostic-redaction",
            diagnostics_group,
        ),
        (
            "publisher-publication-ingestion-projection",
            ingestion_group,
        ),
        (
            "publisher-publication-session-checkpoint-precedence",
            session_group,
        ),
    ])
}

/// Twelve source-bundle bodies retain every literal projection and fallback.
fn bundle_group() -> Outcome {
    bundle::maps_the_full_source_bundle()?;
    bundle::maps_an_empty_bundle_with_source_defaults()?;
    bundle::publisher_kind_preserves_individual_and_unknown_values()?;
    bundle::description_and_contact_precedence_matches_the_source()?;
    bundle::blank_contact_fields_remain_distinct_from_missing_values()?;
    bundle::description_fallback_uses_a_unicode_safe_utf16_bound()?;
    bundle::media_is_projected_in_deterministic_source_order()?;
    bundle::install_and_release_file_fallbacks_are_independent_literals()?;
    bundle::direct_install_file_details_infer_name_and_mime()?;
    bundle::install_file_size_and_url_supply_release_fallbacks()?;
    bundle::signer_authority_preserves_booleans_roles_and_fallbacks()?;
    bundle::numbers_and_version_names_keep_exact_source_text()
}

/// Three diagnostics bodies retain static progress text and redacted debug output.
fn diagnostics_group() -> Outcome {
    diagnostics::progress_events_use_static_source_messages()?;
    diagnostics::ingestion_progress_never_promotes_provider_text()?;
    diagnostics::debug_output_omits_provider_and_application_text()
}

/// Eight ingestion bodies retain source-kind, attachment and metadata precedence.
fn ingestion_group() -> Outcome {
    ingestion::source_kind_inference_preserves_unknown_values()?;
    ingestion::translates_ready_ingestion_identity_and_progress()?;
    ingestion::translates_ready_ingestion_bundle_and_source()?;
    ingestion::ingestion_status_and_source_fallbacks_are_distinct()?;
    ingestion::existing_release_source_uses_the_translated_identity()?;
    ingestion::unknown_and_portal_upload_sources_preserve_reported_fields()?;
    ingestion::explicit_attachments_replace_embedded_records()?;
    ingestion::metadata_uri_prefers_the_session_then_the_release_record()
}

/// Nine session bodies retain checkpoint, status, timestamp and marker precedence.
fn session_group() -> Outcome {
    session::checkpoint_stage_precedence_matches_the_pinned_translator()?;
    session::checkpoint_receipt_precedence_falls_through_failed_and_unknown_stages()?;
    session::checkpoint_receipt_precedence_orders_ticket_attestation_and_signatures()?;
    session::absent_null_blank_and_unknown_stages_remain_distinct()?;
    session::status_precedence_is_failed_then_completed_then_running()?;
    session::publication_session_timestamps_never_sample_a_clock()?;
    session::publication_session_retains_error_text_only_as_explicit_data()?;
    session::checkpoint_wire_names_and_ordering_are_stable()?;
    session::markers_can_be_normalized_directly()
}

#[cfg(test)]
mod continuation {
    /// The continuation controls must keep the standard test assertions.
    macro_rules! assert_eq {
        ($($arguments:tt)*) => {
            std::assert_eq!($($arguments)*)
        };
    }

    /// The continuation controls must keep the standard test assertions.
    macro_rules! assert {
        ($($arguments:tt)*) => {
            std::assert!($($arguments)*)
        };
    }

    use super::checks;
    use super::*;

    const CAPTURED: &str = include_str!("../evidence/native-success.txt");
    const APPENDED: [&str; 4] = [
        "publisher-publication-bundle-source-projection",
        "publisher-publication-diagnostic-redaction",
        "publisher-publication-ingestion-projection",
        "publisher-publication-session-checkpoint-precedence",
    ];

    /// The portable continuation control changes only this independent expected literal.
    fn changed_expected_release_version() -> Outcome {
        let projected = support::map_backend_bundle_to_publication_bundle(
            &support::bundle(support::source_bundle()),
            None,
            support::PortalSourceKind::PortalUpload,
        );

        checks::require(
            projected.release.version_name == "deliberately incorrect version",
            "release version name mismatch",
        )
    }

    #[test]
    fn appended_groups_match_the_committed_and_live_reports() {
        let captured: alloc::vec::Vec<_> = CAPTURED
            .lines()
            .filter_map(|line| line.strip_suffix(": Ok(())"))
            .collect();
        let committed = &captured[captured.len() - APPENDED.len()..];

        assert_eq!(
            committed,
            &APPENDED[..],
            "committed report does not end with the appended publication groups"
        );

        let live = cases();
        let names: alloc::vec::Vec<_> = live.iter().map(|case| case.name).collect();

        assert_eq!(
            names.as_slice(),
            &APPENDED[..],
            "live publication group order changed"
        );
        assert!(
            live.iter().all(|case| case.outcome.is_ok()),
            "live publication group failed: {live:#?}"
        );

        let report = crate::run(crate::FixtureMode::Canonical);
        let tail: alloc::vec::Vec<_> = report.cases[report.cases.len() - APPENDED.len()..]
            .iter()
            .map(|case| case.name)
            .collect();

        assert_eq!(
            tail.as_slice(),
            &APPENDED[..],
            "registered suite tail changed"
        );
    }

    #[test]
    fn a_real_projection_mismatch_is_reported_and_later_groups_still_execute() {
        let cases = run_groups([
            (
                "changed-release-version-oracle",
                changed_expected_release_version,
            ),
            ("later-bundle-assertions", bundle_group),
            ("later-diagnostics-assertions", diagnostics_group),
            ("later-ingestion-assertions", ingestion_group),
            ("later-session-assertions", session_group),
        ]);

        assert_eq!(
            cases[0].outcome,
            Err("release version name mismatch"),
            "the changed expected literal did not report its own failure"
        );
        assert_eq!(cases[1].name, "later-bundle-assertions");
        assert!(cases[1..].iter().all(|case| case.outcome.is_ok()));
    }

    #[test]
    fn a_failed_middle_group_does_not_erase_earlier_or_later_results() {
        let cases = run_groups([
            ("earlier-bundle-assertions", bundle_group),
            (
                "changed-release-version-oracle",
                changed_expected_release_version,
            ),
            ("later-session-assertions", session_group),
        ]);

        assert_eq!(cases.len(), 3);
        assert_eq!(cases[0].name, "earlier-bundle-assertions");
        assert_eq!(cases[0].outcome, Ok(()));
        assert_eq!(
            cases[1].outcome,
            Err("release version name mismatch"),
            "the changed expected literal did not report its own failure"
        );
        assert_eq!(cases[2].name, "later-session-assertions");
        assert_eq!(cases[2].outcome, Ok(()));
    }
}
