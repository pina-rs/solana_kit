//! Deterministic off-chain wallet public API assertions shared unchanged by all hosts.
#![forbid(unsafe_code)]

mod callbacks;
mod common;
mod contracts;
mod counterexamples;
mod disposal;
mod exhaustion;
mod lifecycle;

/// Checks literal metadata, identifiers, versions, typed features and failures.
pub fn metadata() -> Result<(), &'static str> {
    contracts::literal_identifiers();
    contracts::identifier_and_version_validation();
    contracts::immutable_metadata_and_icon_literal_oracles();
    contracts::immutable_account_metadata();
    contracts::typed_extensible_features_preserve_versions();
    contracts::stable_error_categories_retain_concrete_causes();
    contracts::icon_padding_matches_executed_dart_literal_oracle();
    contracts::logos_follow_literal_name_resolution_and_canonical_image_metadata();
    Ok(())
}

/// Checks concrete capability outputs, order, future versions and partial failures.
pub fn capabilities() -> Result<(), &'static str> {
    contracts::complete_feature_operations_preserve_order_and_partial_failure();
    contracts::message_signing_preserves_order_and_raw_signatures();
    contracts::sign_in_fields_and_backend_selected_account();
    contracts::offchain_signing_preserves_future_versions_and_unvalidated_signer_bytes();
    Ok(())
}

/// Checks stale observations, Rc identity and the independent mutation matrix.
pub fn registry_revision() -> Result<(), &'static str> {
    counterexamples::insertion();
    counterexamples::removal();
    counterexamples::identity();
    callbacks::run();
    Ok(())
}

/// Checks nested dispatch, destructor reentry and disposal ownership.
pub fn registry_lifecycle() -> Result<(), &'static str> {
    lifecycle::run();
    disposal::run();
    Ok(())
}

/// Checks31/32-pass success, typed exhaustion, reference bounds and recovery.
pub fn registry_exhaustion() -> Result<(), &'static str> {
    exhaustion::run();
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn portable_wallet_assertions() {
        super::metadata().unwrap();
        super::capabilities().unwrap();
        super::registry_revision().unwrap();
        super::registry_lifecycle().unwrap();
        super::registry_exhaustion().unwrap();
    }
}
