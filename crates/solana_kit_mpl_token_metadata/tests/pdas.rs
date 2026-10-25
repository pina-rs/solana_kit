#![cfg(feature = "pda")]
//! Exact MIT Dart literals and explicit-program seed-order controls.
use solana_kit_core::{
    Address,
    crypto::{ProgramDerivedAddress, find_program_address},
};
use solana_kit_mpl_token_metadata::*;

fn inputs() -> (Address, Address, Address, Address) {
    (
        "So11111111111111111111111111111111111111112"
            .parse()
            .unwrap(),
        "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin"
            .parse()
            .unwrap(),
        "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"
            .parse()
            .unwrap(),
        "4Nd1mBQtrMJVYVfKf2PJy9NZUZdTAsp7D4xWLs4gDB4T"
            .parse()
            .unwrap(),
    )
}

fn literal(actual: ProgramDerivedAddress, expected: &str, bump: u8) {
    assert_eq!(actual.address(), expected.parse::<Address>().unwrap());
    assert_eq!(actual.bump(), bump);
}

#[cfg_attr(test, test)]
pub fn all_canonical_pda_literals() {
    let (mint, authority, owner, delegate) = inputs();
    canonical_mint_literals(mint, delegate);
    canonical_authority_literals(mint, authority, owner, delegate);
}

fn canonical_mint_literals(mint: Address, delegate: Address) {
    literal(
        find_metadata(mint).unwrap(),
        "6dM4TqWyWJsbx7obrdLcviBkTafD5E8av61zfU6jq57X",
        255,
    );
    literal(
        find_master_edition(mint).unwrap(),
        "7r1W5yu5i7ev1wPNGsNuRLcdKW1sCy2x4rwyQkdi9ew2",
        254,
    );
    literal(
        find_edition_marker(mint, 247).unwrap(),
        "4Y1mQRDnq8F8vrvtkgte6Pi1LFgRSqVRXtqh2eMsRyBs",
        253,
    );
    literal(
        find_edition_marker(mint, 761).unwrap(),
        "DnAhDZ8svTpU2fJ9ZNkJtKJmeMLnbUG2rJLbRdArM2Si",
        254,
    );
    literal(
        find_edition_marker_v2(mint).unwrap(),
        "AtKgUUJsLTPt73NBSpAyJEpth4D9Ra5aby8osTexGZZh",
        255,
    );
    literal(
        find_token_record(mint, delegate).unwrap(),
        "9nRgosXsTzVLb9P11px73AcFgyNxgngh4LF6BBEF1H71",
        254,
    );
}

fn canonical_authority_literals(
    mint: Address,
    authority: Address,
    owner: Address,
    delegate: Address,
) {
    literal(
        find_collection_authority_record(mint, authority).unwrap(),
        "8bs6pySUMXhFqxfhExMGdvR4M9rASv8F4qyrw8Ld5zJw",
        255,
    );

    literal(
        find_use_authority_record(mint, authority).unwrap(),
        "524rRBfxrFVGfVF91wQydFtNNAeV3GVEgKB83XTfZjMz",
        255,
    );
    literal(
        find_holder_delegate_record(mint, owner, delegate).unwrap(),
        "CrURBYkCru4UmTRqzgxrt88orMy3i8evfHYCJv2ry8cG",
        255,
    );
    literal(
        find_program_as_burner().unwrap(),
        "GKv5PeCxKBCDezo4FMVjjRbkUfoou9PRvPKdzaFEwjXi",
        255,
    );

    for (role, expected, bump) in [
        (
            MetadataDelegateRole::Collection,
            "8QJbxizyAHwNcoj5EEA1cbBgGVrkdFFsH8jV7s2yx36R",
            255,
        ),
        (
            MetadataDelegateRole::Use,
            "7CQdCkTkenFpasatq7biRqU7nFHmiJieMndMwoj56ova",
            255,
        ),
        (
            MetadataDelegateRole::AuthorityItem,
            "8effUicKAdhWdH1KGefqXHUGjrKNCpy65vpoygNGJEaQ",
            255,
        ),
        (
            MetadataDelegateRole::DataItem,
            "BHPMwdpRcizfiFBW6gnh5UpGtaMJf27RX4ESXrDx2nYr",
            255,
        ),
        (
            MetadataDelegateRole::ProgrammableConfigItem,
            "DpFbGapttDXTqPHVis1iGZUmoiNszKr76gGtnahVGrkX",
            254,
        ),
    ] {
        literal(
            find_metadata_delegate_record(mint, &role, owner, delegate).unwrap(),
            expected,
            bump,
        );
    }
}

#[cfg_attr(test, test)]
pub fn explicit_program_seed_domains_and_delegate_roles() {
    let program = Address::from_bytes([97; 32]);
    let mint = Address::from_bytes([17; 32]);
    let authority = Address::from_bytes([34; 32]);
    let delegate = Address::from_bytes([51; 32]);
    let domain: &[&[u8]] = &[b"metadata", program.as_bytes(), mint.as_bytes()];
    assert_eq!(
        find_metadata_with_program(program, mint).unwrap(),
        find_program_address(domain, &program).unwrap()
    );
    assert_ne!(
        find_metadata_with_program(program, mint).unwrap(),
        find_metadata(mint).unwrap()
    );

    for (actual, tail) in [
        (
            find_master_edition_with_program(program, mint).unwrap(),
            vec![&b"edition"[..]],
        ),
        (
            find_edition_marker_v2_with_program(program, mint).unwrap(),
            vec![&b"edition"[..], &b"marker"[..]],
        ),
        (
            find_token_record_with_program(program, mint, authority).unwrap(),
            vec![&b"token_record"[..], authority.as_bytes().as_slice()],
        ),
        (
            find_use_authority_record_with_program(program, mint, authority).unwrap(),
            vec![&b"user"[..], authority.as_bytes().as_slice()],
        ),
        (
            find_collection_authority_record_with_program(program, mint, authority).unwrap(),
            vec![
                &b"collection_authority"[..],
                authority.as_bytes().as_slice(),
            ],
        ),
        (
            find_holder_delegate_record_with_program(program, mint, authority, delegate).unwrap(),
            vec![
                &b"print_delegate"[..],
                authority.as_bytes().as_slice(),
                delegate.as_bytes().as_slice(),
            ],
        ),
    ] {
        assert_eq!(
            actual,
            find_program_address(&[domain.to_vec(), tail].concat(), &program).unwrap()
        );
    }

    assert_eq!(
        find_edition_marker_with_program(program, mint, u64::MAX).unwrap(),
        find_program_address(
            &[
                b"metadata",
                program.as_bytes(),
                mint.as_bytes(),
                b"edition",
                b"74382032555280450"
            ],
            &program
        )
        .unwrap()
    );
    assert_eq!(
        find_program_as_burner_with_program(program).unwrap(),
        find_program_address(&[b"metadata", program.as_bytes(), b"burn"], &program).unwrap()
    );
    delegate_seed_domains(program, mint, authority, delegate);
}

fn delegate_seed_domains(program: Address, mint: Address, authority: Address, delegate: Address) {
    for (role, seed) in [
        (
            MetadataDelegateRole::AuthorityItem,
            &b"authority_item_delegate"[..],
        ),
        (
            MetadataDelegateRole::Collection,
            &b"collection_delegate"[..],
        ),
        (MetadataDelegateRole::Use, &b"use_delegate"[..]),
        (MetadataDelegateRole::Data, &b"data_delegate"[..]),
        (
            MetadataDelegateRole::ProgrammableConfig,
            &b"programmable_config_delegate"[..],
        ),
        (MetadataDelegateRole::DataItem, &b"data_item_delegate"[..]),
        (
            MetadataDelegateRole::CollectionItem,
            &b"collection_item_delegate"[..],
        ),
        (
            MetadataDelegateRole::ProgrammableConfigItem,
            &b"prog_config_item_delegate"[..],
        ),
    ] {
        assert_eq!(metadata_delegate_role_seed(&role), seed);
        assert_eq!(
            find_metadata_delegate_record_with_program(program, mint, &role, authority, delegate)
                .unwrap(),
            find_program_address(
                &[
                    b"metadata",
                    program.as_bytes(),
                    mint.as_bytes(),
                    seed,
                    authority.as_bytes(),
                    delegate.as_bytes()
                ],
                &program
            )
            .unwrap()
        );
    }
}
