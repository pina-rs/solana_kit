#![cfg(feature = "pda")]
use solana_kit_core::{Address, crypto::PdaError};
use solana_kit_mpl_core::*;
fn address(text: &str) -> Address {
    text.parse().unwrap()
}

#[cfg_attr(all(test, not(target_arch = "wasm32")), test)]
pub fn independent_dart_vectors_cover_default_custom_and_on_curve_bump_search() {
    let asset = address("So11111111111111111111111111111111111111112");
    let owner = address("9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM");
    let base = address("G5VeVfFCLYP2WwXTb4kTWhfHecDnEuVF2qGBd9Qv7aEJ");
    let signer = find_asset_signer(&asset).unwrap();
    assert_eq!(
        signer.address(),
        address("BScC4A8jz6F4ZJKHrwWs7KJetuuAzy7bLKjauJsgbsXQ")
    );
    assert_eq!(signer.bump(), 255);

    for (key, program, expected, bump) in [
        (
            asset,
            PROGRAM,
            "FcCyJhf11qot76FbAyFaKAPWqxJwiBH3pfmMZpj2ULxi",
            254,
        ),
        (
            owner,
            PROGRAM,
            "Cv1iCaujHHL8DmJmxAPJ2eFoMVpLcnRvrxt7PtrUQ3F6",
            255,
        ),
        (
            asset,
            base,
            "2GKyLK8Fi8tUNgLKpMagr2vUCF5ff4J7ogpBVoyvyrZs",
            255,
        ),
    ] {
        let value = find_preconfigured(&key, &program).unwrap();
        assert_eq!(value.address(), address(expected));
        assert_eq!(value.bump(), bump);
    }

    oracle_literals(base);
}

fn oracle_literals(base: Address) {
    let program = find_preconfigured_program(&PROGRAM).unwrap();
    assert_eq!(
        program.address(),
        address("CKMh9MdJJqMb163H8UznSQziqNGiBhEe6cdUjyzMR7Ry")
    );
    assert_eq!(program.bump(), 251);
    let oracle = ExtraAccount::PreconfiguredProgram(ExtraAccountPreconfiguredProgramPayload {
        is_signer: false,
        is_writable: false,
    });
    let context = ExtraAccountContext::default();
    assert_eq!(find_oracle_account(&base, None, &context).unwrap(), base);
    assert_eq!(
        find_oracle_account(&base, Some(&oracle), &context).unwrap(),
        address("FfBScmC6jUVRdv3Qrk1sSRRSg2KdS7MsdfcncDxoeyGx")
    );
    let custom = ExtraAccount::CustomPda(ExtraAccountCustomPdaPayload {
        seeds: vec![Seed::Bytes(b"oracle".to_vec())],
        custom_program_id: None,
        is_signer: false,
        is_writable: false,
    });

    assert_eq!(
        find_oracle_account(&base, Some(&custom), &context).unwrap(),
        address("B4kNGsN3F9VwSTBA7VJ1x6MqwBsPSN5VjyCVsHad7bTC")
    );
}

#[cfg_attr(all(test, not(target_arch = "wasm32")), test)]
pub fn context_and_custom_seed_failures_are_explicit() {
    let context = ExtraAccountContext::default();
    let collection =
        ExtraAccount::PreconfiguredCollection(ExtraAccountPreconfiguredCollectionPayload {
            is_signer: false,
            is_writable: true,
        });
    assert_eq!(
        derive_extra_account(&collection, &PROGRAM, &context),
        Err(ExtraAccountError::MissingContext("collection"))
    );

    for (seeds, expected) in [
        (vec![Seed::Bytes(vec![1; 33])], PdaError::SeedTooLong),
        (vec![Seed::Bytes(vec![]); 16], PdaError::TooManySeeds),
    ] {
        let custom = ExtraAccount::CustomPda(ExtraAccountCustomPdaPayload {
            seeds,
            custom_program_id: None,
            is_signer: false,
            is_writable: false,
        });
        assert_eq!(
            derive_extra_account(&custom, &PROGRAM, &context),
            Err(ExtraAccountError::Derivation(expected))
        );
    }
}

#[cfg_attr(all(test, not(target_arch = "wasm32")), test)]
pub fn custom_program_precedence_all_seed_contexts_and_direct_addresses_are_explicit() {
    let asset = address("So11111111111111111111111111111111111111112");
    let custom = ExtraAccount::CustomPda(ExtraAccountCustomPdaPayload {
        seeds: vec![Seed::Bytes(b"oracle".to_vec()), Seed::Asset],
        custom_program_id: Some(Address::from_bytes([0; 32])),
        is_signer: false,
        is_writable: false,
    });
    let context = ExtraAccountContext {
        asset: Some(asset),
        ..ExtraAccountContext::default()
    };
    assert_eq!(
        derive_extra_account(&custom, &PROGRAM, &context).unwrap(),
        address("D2TDeihn1232WNT1NoDXEhqrcnCzXsi58Q51X3j9MU27")
    );
    let direct = ExtraAccount::Address(ExtraAccountAddressPayload {
        address: asset,
        is_signer: true,
        is_writable: true,
    });
    assert_eq!(
        derive_extra_account(&direct, &PROGRAM, &ExtraAccountContext::default()).unwrap(),
        asset
    );
    all_context_failures(asset);
}

fn all_context_failures(asset: Address) {
    let all = ExtraAccount::CustomPda(ExtraAccountCustomPdaPayload {
        seeds: vec![Seed::Collection, Seed::Owner, Seed::Recipient, Seed::Asset],
        custom_program_id: None,
        is_signer: false,
        is_writable: false,
    });

    for (context, name) in [
        (ExtraAccountContext::default(), "collection"),
        (
            ExtraAccountContext {
                collection: Some(asset),
                ..ExtraAccountContext::default()
            },
            "owner",
        ),
        (
            ExtraAccountContext {
                collection: Some(asset),
                owner: Some(asset),
                ..ExtraAccountContext::default()
            },
            "recipient",
        ),
        (
            ExtraAccountContext {
                collection: Some(asset),
                owner: Some(asset),
                recipient: Some(asset),
                asset: None,
            },
            "asset",
        ),
    ] {
        assert_eq!(
            derive_extra_account(&all, &PROGRAM, &context),
            Err(ExtraAccountError::MissingContext(name))
        );
    }
}
