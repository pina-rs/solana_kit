//! Complete bounded public MWA contracts with their original assertion order.

macro_rules! check {
    ($condition:expr $(, $($message:tt)*)?) => {
        crate::require($condition, stringify!($condition))?
    };
}
macro_rules! check_eq {
    ($left:expr, $right:expr $(,)?) => {
        crate::require($left == $right, stringify!($left == $right))?
    };
}

mod independent;
mod public;
mod uri;

type Check = Result<(), &'static str>;

mod metering;
pub use metering::BACKEND_WORK;

pub(super) fn cases() -> [crate::CaseResult; 7] {
    [
        crate::case("mwa-crypto-vectors", crypto_vectors()),
        crate::case("mwa-hello-session", hello_session()),
        crate::case("mwa-authenticated-frames", authenticated_frames()),
        crate::case("mwa-association-uris", association_uris()),
        crate::case("mwa-rpc-models", rpc_models()),
        crate::case("mwa-siws-contracts", siws_contracts()),
        crate::case(
            "mwa-jws-boundaries",
            independent::jws_rejects_correctly_signed_wrong_algorithm_shape_and_key_binding(),
        ),
    ]
}

fn crypto_vectors() -> Check {
    public::dart_crypto_vectors_and_rust_signature_output()?;
    public::canonical_point_scalar_and_signature_boundaries()
}

fn hello_session() -> Check {
    public::hello_response_and_properties_require_authenticated_frames()?;
    public::encrypted_memory_session_handshake_authorization_and_close()?;
    independent::hello_authenticates_association_and_ephemeral_point_before_agreement()
}

fn authenticated_frames() -> Check {
    public::authenticated_ordering_replay_tampering_and_exhaustion()?;
    independent::every_tampered_receive_preserves_state_for_successful_retry()?;
    independent::gap_does_not_advance_and_receive_counter_cannot_wrap()?;
    independent::directional_ordering_does_not_bind_the_sender_or_prevent_reflection()?;
    independent::cross_direction_iv_reuse_is_a_caller_obligation_separate_from_sequence()
}

fn association_uris() -> Check {
    public::associations_preserve_wallet_prefix()?;
    public::association_port_boundaries()?;
    public::reflector_ids_and_query_escaping_round_trip()?;
    public::associations_reject_wrong_bases_paths_and_ambiguous_parameters()?;
    uri::reflector_authorities_preserve_canonical_spelling()?;
    uri::wallet_prefix_and_absolute_uri_validation_keep_original_rules()?;
    independent::uri_decoded_duplicates_and_authority_redirection_are_rejected()
}

fn rpc_models() -> Check {
    public::typed_rpc_legacy_and_v1_request_mapping()?;
    public::typed_rpc_rejects_wrong_response_ids_and_invalid_envelopes()?;
    public::typed_capabilities_map_both_protocol_versions()?;
    public::json_models_preserve_omission_null_and_solana_address_validation()?;
    independent::response_id_and_outcome_boundaries_are_strict()
}

fn siws_contracts() -> Check {
    public::siws_matches_dart_and_verifies_exact_signer_message_and_algorithm()?;
    public::siws_rejects_injection_in_every_field_and_resource()?;
    independent::siws_checks_expected_signer_claim_context_and_strict_signature()?;
    public::siws_wire_keys_preserve_full_payload_message_and_authorize_envelope()
}
