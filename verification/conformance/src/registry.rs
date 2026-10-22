//! Stable primitive case groups, kept separate from report validation.

use crate::{CaseResult, FixtureMode, addresses, case, cryptography};

/// Primitive values precede the cryptographic failure control in every report.
pub(crate) fn values_and_crypto(mode: FixtureMode) -> [CaseResult; 12] {
    [
        case("address-base58-golden", addresses::golden()),
        case("address-invalid-alphabet", addresses::invalid_alphabet()),
        case("address-invalid-length", addresses::invalid_length()),
        case("address-bounded-input", addresses::bounded_input()),
        case("hash-base58-golden", addresses::hash_value()),
        case("signature-base58-golden", addresses::signature_value()),
        case("sha256-fips-abc", cryptography::sha256(mode)),
        case("ed25519-rfc8032-test1", cryptography::rfc8032_empty()),
        case("ed25519-rfc8032-test2", cryptography::rfc8032_one_byte()),
        case(
            "ed25519-altered-signature",
            cryptography::altered_signature(),
        ),
        case("ed25519-altered-message", cryptography::altered_message()),
        case("ed25519-weak-key", cryptography::weak_key()),
    ]
}

/// Address derivation retains its original stable report order.
pub(crate) fn derivation() -> [CaseResult; 9] {
    [
        case("pda-no-seeds", addresses::pda_no_seeds()),
        case("pda-bump-search", addresses::pda_bump_search()),
        case("pda-byte-seed", addresses::pda_byte_seed()),
        case("pda-utf8-seed", addresses::pda_utf8_seed()),
        case("pda-seed-count", addresses::seed_count()),
        case("pda-seed-length", addresses::seed_length()),
        case("pda-on-curve", addresses::on_curve()),
        case("seeded-address-golden", addresses::seeded_address()),
        case("seeded-address-illegal-owner", addresses::illegal_owner()),
    ]
}

/// Instruction ordering and value sizes follow address derivation.
pub(crate) fn instructions() -> [CaseResult; 4] {
    use crate::instructions;

    [
        case("account-role-merge-matrix", instructions::role_matrix()),
        case(
            "instruction-ordered-accounts",
            instructions::ordered_accounts(),
        ),
        case("instruction-preserved-data", instructions::preserved_data()),
        case("fixed-value-byte-lengths", addresses::byte_lengths()),
    ]
}

/// Bounded codec vectors complete the original 31-case suite.
pub(crate) fn codecs() -> [CaseResult; 6] {
    use crate::codecs;

    [
        case("codec-integer-byte-fixtures", codecs::numbers()),
        case("codec-short-u16-canonical", codecs::short_u16()),
        case("codec-string-length-widths", codecs::strings()),
        case("codec-bounded-sequences", codecs::bounded_sequences()),
        case("codec-distinct-options", codecs::options()),
        case("codec-exact-framing", codecs::framing()),
    ]
}
