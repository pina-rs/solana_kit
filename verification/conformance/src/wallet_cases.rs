//! Append-only registration of the isolated wallet assertions.

use solana_kit_wallet_verification as wallet;

pub(super) fn cases() -> [crate::CaseResult; 5] {
    [
        crate::case("wallet-metadata-contracts", wallet::metadata()),
        crate::case("wallet-capability-contracts", wallet::capabilities()),
        crate::case("wallet-registry-revision", wallet::registry_revision()),
        crate::case("wallet-registry-lifecycle", wallet::registry_lifecycle()),
        crate::case("wallet-registry-exhaustion", wallet::registry_exhaustion()),
    ]
}
