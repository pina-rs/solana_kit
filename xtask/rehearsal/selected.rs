//! Selected-feature external behavior checks for exact local-registry archives.
//!
//! This file is compiled only with the consumer's `selected` feature. It exercises
//! crypto, PDA derivation, MWA association metadata and the Token transfer plan
//! against the same offline registry as the default suite.
#![cfg(feature = "selected")]
#![forbid(unsafe_code)]

use solana_kit_core::{Address, codecs::Limits as CodecLimits};

const PLAN_LIMITS: CodecLimits = CodecLimits::new(256, 16);

mod selected {
    use super::{Address, PLAN_LIMITS};

    /// The selected crypto feature signs and verifies independent bytes.
    #[test]
    fn core_crypto_signs_and_rejects_changes() {
        use solana_kit_core::crypto::{
            Keypair, find_program_address, is_on_curve, verify,
        };

        let key = Keypair::from_seed(&[7; 32]);
        let signature = key.sign(b"downstream package");
        assert_eq!(
            verify(&key.address(), b"downstream package", &signature),
            Ok(())
        );
        assert!(verify(&key.address(), b"changed message", &signature).is_err());
        let derived =
            find_program_address(&[b"seed"], &Address::from([2; 32])).expect("off-curve PDA");
        assert!(!is_on_curve(&derived.address()));
    }

    /// The selected MWA feature parses its own association URI exactly.
    #[test]
    fn wallet_mwa_association_round_trips() {
        use solana_kit_wallet::mwa::{
            Association, AssociationKey, AssociationPort, association_uri, parse_association_uri,
        };

        // A deterministic private scalar exercises the real P-256 association path.
        let mut scalar = [0; 32];
        scalar[31] = 1;
        let association = Association::Local {
            key: AssociationKey::from_scalar(&scalar)
                .expect("scalar")
                .public_key(),
            protocol: "v1".into(),
            port: AssociationPort::new(49_152).expect("private port"),
        };
        let uri = association_uri(&association, None).expect("association URI");
        assert!(
            uri.starts_with("solana-wallet:/v1/associate/local?association="),
            "{uri}"
        );
        assert_eq!(parse_association_uri(&uri).expect("parsed URI"), association);
        assert!(AssociationPort::new(80).is_err());
    }

    /// The selected Token plan feature builds the two exact invocation shapes.
    #[test]
    fn token_plan_has_create_then_transfer_checked() {
        use solana_kit_core::plans::Plan;
        use solana_kit_token::{
            TransferToAtaConfig, TransferToAtaInput, transfer_to_ata_plan,
        };

        // The plan creates the destination account first, then transfers base units.
        let address = |byte| Address::from([byte; 32]);
        let plan = transfer_to_ata_plan(
            TransferToAtaInput {
                payer: address(1),
                mint: address(2),
                source: address(3),
                authority: address(4),
                destination: address(5),
                recipient: address(6),
                amount: 123,
                decimals: 6,
            },
            TransferToAtaConfig::default(),
            PLAN_LIMITS,
        )
        .expect("valid plan");
        let Plan::Sequential { plans, divisible } = plan else {
            panic!("expected a sequential plan");
        };
        assert!(divisible);
        assert_eq!(plans.len(), 2);

        let Plan::Single(create) = &plans[0] else {
            panic!("expected create leaf");
        };
        let Plan::Single(transfer) = &plans[1] else {
            panic!("expected transfer leaf");
        };
        assert_eq!(create.data, [1]);
        assert_eq!(transfer.data.len(), 10);
        assert_eq!(transfer.data[0], 12);
        assert_eq!(&transfer.data[1..9], &123_u64.to_le_bytes());
        assert_eq!(transfer.data[9], 6);
    }
}
