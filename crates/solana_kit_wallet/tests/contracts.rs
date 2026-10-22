mod common;
use common::*;
use solana_kit_wallet::*;
use std::{
    future::Future,
    pin::pin,
    task::{Context, Poll, Waker},
};

fn done<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);

    match future
        .as_mut()
        .poll(&mut Context::from_waker(Waker::noop()))
    {
        Poll::Ready(output) => output,
        Poll::Pending => panic!("deterministic backend unexpectedly pending"),
    }
}

#[test]
fn literal_identifiers() {
    assert_eq!(WALLET_STANDARD_VERSION, "1.0.0");
    assert_eq!(
        [CONNECT, DISCONNECT, EVENTS],
        ["standard:connect", "standard:disconnect", "standard:events"]
    );
    assert_eq!(
        SOLANA_CHAINS,
        [
            "solana:mainnet",
            "solana:devnet",
            "solana:testnet",
            "solana:localnet"
        ]
    );
    assert_eq!(
        [
            SIGN_TRANSACTION,
            SIGN_AND_SEND_TRANSACTION,
            SIGN_AND_SEND_ALL_TRANSACTIONS,
            SIGN_MESSAGE,
            SIGN_IN,
            SIGN_OFFCHAIN_MESSAGE
        ],
        [
            "solana:signTransaction",
            "solana:signAndSendTransaction",
            "solana:signAndSendAllTransactions",
            "solana:signMessage",
            "solana:signIn",
            "solana:signOffchainMessage"
        ]
    );
}

#[test]
fn identifier_and_version_validation() {
    for value in ["x:y", "a:b:c", " : "] {
        assert!(is_identifier(value));
    }

    for value in ["x", ":x", "x:", ""] {
        assert!(!is_identifier(value));
    }

    for (value, version) in [
        ("legacy", TransactionVersion::Legacy),
        ("0", TransactionVersion::V0),
        ("1", TransactionVersion::V1),
    ] {
        assert_eq!(version.wire_value(), value);
        assert_eq!(TransactionVersion::from_wire_value(value), Some(version));
    }

    assert_eq!(TransactionVersion::from_wire_value("2"), None);
    assert_eq!(TransactionVersion::from_wire_value("01"), None);
    assert!(
        !SignAndSendTransaction::supported_transaction_versions(&Backend)
            .contains(&TransactionVersion::V0)
    );
    assert!(
        SignTransaction::supported_transaction_versions(&Backend).contains(&TransactionVersion::V1)
    );
}

#[test]
fn immutable_metadata_and_icon_literal_oracles() {
    let icon = WalletIcon::parse("data:image/png;base64,AA==").unwrap();
    assert_eq!(icon.bytes(), [0]);
    assert_eq!(icon.mime_type(), "image/png");
    assert_eq!(icon.mime_subtype(), "png");
    assert_eq!(icon.to_string(), "data:image/png;base64,AA==");

    for subtype in ["svg+xml", "webp", "gif"] {
        assert!(WalletIcon::parse(format!("data:image/{subtype};base64,AA==")).is_ok());
    }

    for value in [
        "https://wallet.test/icon.png",
        "data:image/jpeg;base64,AA==",
        "data:image/png;base64,!",
        "data:image/png;base64,A",
        "data:image/png;base64,AA===",
        "data:image/png;base64,A A=",
    ] {
        assert_eq!(WalletIcon::parse(value), Err(MetadataError::InvalidIcon));
    }

    assert!(
        WalletIcon::parse("data:image/png;base64,")
            .unwrap()
            .bytes()
            .is_empty()
    );
}

#[test]
fn immutable_account_metadata() {
    let icon = WalletIcon::parse("data:image/png;base64,AA==").unwrap();
    let value = account();
    assert_eq!(value.address(), "display-address");
    assert_eq!(value.public_key(), &[7; 32]);
    assert_eq!(value.label(), Some("Primary"));
    assert_eq!(value.icon(), Some(&icon));

    assert_eq!(
        WalletAccount::new(String::new(), [0; 32], vec![], vec![], None, None),
        Err(MetadataError::EmptyAddress)
    );
    assert!(
        WalletAccount::new("a".into(), [0; 32], vec!["bad".into()], vec![], None, None).is_err()
    );
    assert!(
        WalletAccount::new("a".into(), [0; 32], vec![], vec!["bad".into()], None, None).is_err()
    );
}

#[test]
fn typed_extensible_features_preserve_versions() {
    struct FutureFeature;
    impl WalletFeature for FutureFeature {
        fn version(&self) -> &str {
            "9.2.1"
        }
    }
    let mut features = WalletFeatures::new();
    features.insert("future:unknown", FutureFeature).unwrap();
    assert!(features.supports("future:unknown"));
    assert!(features.feature::<Backend>("future:unknown").is_none());
    assert!(
        features
            .feature::<FutureFeature>("future:unknown")
            .is_some()
    );
    assert_eq!(features.version("future:unknown"), Some("9.2.1"));
    assert_eq!(
        features.identifiers().collect::<Vec<_>>(),
        ["future:unknown"]
    );
    assert!(features.insert("invalid", Backend).is_err());

    let wallet = TestWallet::named("Test");
    assert_eq!(wallet.version(), "1.0.0");
    assert_eq!(wallet.icon().mime_type(), "image/png");
    assert_eq!(wallet.chains(), ["solana:localnet"]);
    assert_eq!(wallet.accounts().len(), 1);
    let feature = wallet.features().feature::<Backend>(CONNECT).unwrap();
    assert_eq!(
        done(feature.connect(ConnectInput::default()))
            .unwrap()
            .accounts,
        [account()]
    );
    assert_eq!(
        done(feature.connect(ConnectInput { silent: true }))
            .unwrap_err()
            .code,
        WalletErrorCode::UserRejected
    );

    done(feature.disconnect()).unwrap();
}

#[test]
fn complete_feature_operations_preserve_order_and_partial_failure() {
    let backend = Backend;
    let options = SignTransactionOptions {
        preflight_commitment: Some(TransactionCommitment::Confirmed),
        min_context_slot: Some(-4),
    };
    let tx = SignTransactionInput {
        account: account(),
        transaction: vec![1, 2],
        chain: None,
        options: Some(options.clone()),
    };
    assert_eq!(
        done(backend.sign_transaction(vec![tx])).unwrap()[0].signed_transaction,
        [1, 2]
    );

    let send_options = SignAndSendTransactionOptions {
        simulation: options,
        commitment: Some(TransactionCommitment::Finalized),
        skip_preflight: Some(true),
        max_retries: Some(-2),
    };
    let input = SignAndSendTransactionInput {
        account: account(),
        transaction: vec![4],
        chain: "arbitrary".into(),
        options: Some(send_options),
    };
    assert_eq!(
        done(backend.sign_and_send_transaction(vec![input.clone()])).unwrap()[0].signature,
        [4]
    );
    let mut rejected_input = input.clone();
    rejected_input.transaction.clear();

    for (mode, marker) in [
        (SignAndSendAllMode::Parallel, 0),
        (SignAndSendAllMode::Serial, 1),
    ] {
        let result = done(backend.sign_and_send_all_transactions(
            vec![input.clone(), rejected_input.clone(), input.clone()],
            mode,
        ))
        .unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].as_ref().unwrap().signature, [4, marker]);
        assert_eq!(
            result[1].as_ref().unwrap_err().code,
            WalletErrorCode::UserRejected
        );
        assert_eq!(result[2].as_ref().unwrap().signature, [4, marker]);
    }

    assert_eq!(SignAndSendAllMode::default(), SignAndSendAllMode::Parallel);
}

#[test]
fn message_signing_preserves_order_and_raw_signatures() {
    let backend = Backend;
    let messages = done(backend.sign_message(vec![
        SignMessageInput {
            account: account(),
            message: vec![1],
        },
        SignMessageInput {
            account: account(),
            message: vec![2],
        },
    ]))
    .unwrap();
    assert_eq!(
        messages
            .iter()
            .map(|result| result.signed_message.clone())
            .collect::<Vec<_>>(),
        [vec![1], vec![2]]
    );
    assert_eq!(messages[0].signature, [9]);
    assert_eq!(messages[0].signature_type.as_deref(), Some("ed25519"));
}

#[test]
fn sign_in_fields_and_backend_selected_account() {
    let backend = Backend;
    let sign_in = SignInInput {
        domain: Some("example.test".into()),
        address: Some("display-address".into()),
        statement: Some("Sign in".into()),
        uri: Some("https://example.test".into()),
        version: Some("1".into()),
        chain_id: Some("1".into()),
        nonce: Some("nonce".into()),
        issued_at: Some("now".into()),
        expiration_time: Some("later".into()),
        not_before: Some("now".into()),
        request_id: Some("request".into()),
        resources: Some(vec!["resource".into()]),
    };
    assert_eq!(sign_in.resources.as_ref().unwrap(), &["resource"]);
    let signed_in = done(backend.sign_in(vec![sign_in])).unwrap();
    assert_eq!(signed_in[0].account, account());
    assert_eq!(signed_in[0].message.signed_message, b"example.test");
}

#[test]
fn offchain_signing_preserves_future_versions_and_unvalidated_signer_bytes() {
    let backend = Backend;
    assert_eq!(backend.supported_message_versions(), [1, 2]);
    let offchain = SignOffchainMessageInput {
        account: account(),
        message: "hello".into(),
        required_signers: vec![vec![0; 31]],
    };
    assert_eq!(offchain.required_signers[0].len(), 31);
    let result = done(backend.sign_offchain_message(vec![offchain])).unwrap();
    assert_eq!(result[0].signed_offchain_message, b"hello");
    assert_eq!(result[0].signature, [6]);
}

#[test]
fn stable_error_categories_retain_concrete_causes() {
    let codes = [
        WalletErrorCode::Unavailable,
        WalletErrorCode::UnsupportedFeature,
        WalletErrorCode::UserRejected,
        WalletErrorCode::InvalidRequest,
        WalletErrorCode::InvalidResponse,
        WalletErrorCode::Disconnected,
        WalletErrorCode::Transport,
        WalletErrorCode::Timeout,
    ];

    for code in codes {
        let error = WalletError {
            code,
            message: "failed".into(),
            cause: Some(42_u32),
        };
        assert_eq!(error.cause, Some(42));
        assert!(error.to_string().contains("failed"));
    }
}

#[test]
fn logos_follow_literal_name_resolution_and_canonical_image_metadata() {
    assert_eq!(generic_wallet_logo().unwrap().mime_subtype(), "svg+xml");
    for name in [
        "Phantom",
        "SOLFLARE",
        "metamask",
        "Trust Wallet",
        "XDEFI Wallet",
        "\u{feff}Phantom\u{feff}",
    ] {
        assert_eq!(
            wallet_logo_fallback(name).unwrap().unwrap().mime_subtype(),
            "svg+xml"
        );
    }
    assert_eq!(wallet_logo_fallback("Unknown Wallet").unwrap(), None);
    assert_eq!(wallet_logo_fallback("").unwrap(), None);
    assert_eq!(
        wallet_logo_fallback("trust").unwrap(),
        wallet_logo_fallback("Trust Wallet").unwrap()
    );
}

#[test]
fn icon_padding_matches_executed_dart_literal_oracle() {
    for payload in ["AA", "AA=", "AB==", "AAA", "AAB=", "AA===", "===="] {
        assert_eq!(
            WalletIcon::parse(format!("data:image/png;base64,{payload}")),
            Err(MetadataError::InvalidIcon)
        );
    }

    assert_eq!(
        WalletIcon::parse("data:image/png;base64,AA==")
            .unwrap()
            .bytes(),
        [0]
    );
}
