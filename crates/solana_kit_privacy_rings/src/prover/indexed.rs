//! `circuitType` strings the indexed prover expects inside a witness.
//!
//! The URL still names a proving key, such as `transfer_confidential_2_2`.
//! These names are the JSON field, not the key file.

/// A witness `circuitType` served by the indexed prover routes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndexedCircuit {
    /// Confidential transfer.
    TransferConfidential,
    /// Ring transfer.
    TransferRing,
    /// Ring-authority transfer.
    TransferRingAuthority,
    /// Anonymous ring EdDSA transfer.
    TransferAnonymousRingEddsa,
    /// Confidential EdDSA transfer.
    TransferConfidentialEddsa,
    /// Note merge.
    Merge,
    /// Ring merge.
    MergeRing,
}

impl IndexedCircuit {
    /// The kebab-case value written into `circuitType`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TransferConfidential => "transfer-confidential",
            Self::TransferRing => "transfer-ring",
            Self::TransferRingAuthority => "transfer-ring-authority",
            Self::TransferAnonymousRingEddsa => "transfer-eddsa-ring",
            Self::TransferConfidentialEddsa => "transfer-confidential-eddsa",
            Self::Merge => "merge",
            Self::MergeRing => "merge-ring",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_match_the_upstream_indexed_client() {
        assert_eq!(
            IndexedCircuit::TransferConfidential.as_str(),
            "transfer-confidential"
        );
        assert_eq!(IndexedCircuit::TransferRing.as_str(), "transfer-ring");
        assert_eq!(
            IndexedCircuit::TransferRingAuthority.as_str(),
            "transfer-ring-authority"
        );
        assert_eq!(
            IndexedCircuit::TransferAnonymousRingEddsa.as_str(),
            "transfer-eddsa-ring"
        );
        assert_eq!(
            IndexedCircuit::TransferConfidentialEddsa.as_str(),
            "transfer-confidential-eddsa"
        );
        assert_eq!(IndexedCircuit::Merge.as_str(), "merge");
        assert_eq!(IndexedCircuit::MergeRing.as_str(), "merge-ring");
    }
}
