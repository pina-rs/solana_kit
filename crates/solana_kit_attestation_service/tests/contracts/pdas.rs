//! Immutable MIT Dart golden PDA vectors and explicit-program seed controls.
use solana_kit_attestation_service::*;
use solana_kit_core::{
    Address,
    crypto::{ProgramDerivedAddress, find_program_address},
};

fn check(actual: ProgramDerivedAddress, address: &str, bump: u8) {
    assert_eq!(actual.address(), address.parse::<Address>().unwrap());
    assert_eq!(actual.bump(), bump);
}

#[cfg_attr(test, test)]
pub fn all_seven_pda_recipes_match_literal_vectors() {
    check(
        find_credential(
            "tbFevHibEdBNFJfZ7xKC8k1th8pt2YPEXTk4sGMxCGa"
                .parse()
                .unwrap(),
            "test",
        )
        .unwrap(),
        "CdUAYGvNc7NdtNgXmxTXoUWR5NjpcU4Za4vtoP2AVZD4",
        255,
    );
    check(
        find_schema(
            "2zHazqL3MVayNGkDrTmAC7VsvP5QrSYfiyA7uf29Usmd"
                .parse()
                .unwrap(),
            "test",
            1,
        )
        .unwrap(),
        "bD7cVGpuTHY43fxtRoqJYi58U6Yi3kMyVck2DyZZRKq",
        255,
    );
    check(
        find_attestation(
            "G6QmvUp3a1Kv9rX2LqHDH8AWcKD8yaufcoXEB1h6SzN8"
                .parse()
                .unwrap(),
            "GSwz99vWPKnePyeYTM5iionEfArVmfrufV4AaV4SecTH"
                .parse()
                .unwrap(),
            "Bdf3cgpzgboZq95T4AVYNxuYGDVE4pwLNQBhQ2ob8CoG"
                .parse()
                .unwrap(),
        )
        .unwrap(),
        "CnhgnrLiawRWitfjrrUfWdR2jpwKbKGDccbk3ne171iu",
        255,
    );
    check(
        find_schema_mint(
            "GCVt9SmgLF8bgEVwZAhQ9A2skwj5TvEnyn8Z7eUm583E"
                .parse()
                .unwrap(),
        )
        .unwrap(),
        "9JLQQK3zeEjiq2AJ1XPN765bYnLrBWJSFfyjDwdSMmyN",
        245,
    );
    check(
        find_attestation_mint(
            "3z8EuPHrzhfVWuDomSGjU13ABDgQC75DMHoDNBgxdzKR"
                .parse()
                .unwrap(),
        )
        .unwrap(),
        "61FeMtSXR8H22fodXNrTkwmrSmBuTNcAvKzZXvZRPMkX",
        253,
    );
    check(
        find_event_authority().unwrap(),
        "DzSpKpST2TSyrxokMXchFz3G2yn5WEGoxzpGEUDjCX4g",
        255,
    );
    check(
        find_sas_authority().unwrap(),
        "HngMQFF6Yoqj9VqA31r43HQsnuYZ6BxopRWQLQAS6zk",
        254,
    );
}

#[cfg_attr(test, test)]
pub fn custom_program_seed_order_utf8_and_version_boundaries() {
    let program = Address::from_bytes([90; 32]);
    let a = Address::from_bytes([31; 32]);
    let b = Address::from_bytes([32; 32]);
    let nonce = Address::from_bytes([33; 32]);
    assert_eq!(
        find_credential_with_program(program, a, "é").unwrap(),
        find_program_address(&[b"credential", a.as_bytes(), &[0xc3, 0xa9]], &program).unwrap()
    );

    for version in [0, 1, 255] {
        assert_eq!(
            find_schema_with_program(program, a, "test", version).unwrap(),
            find_program_address(&[b"schema", a.as_bytes(), b"test", &[version]], &program)
                .unwrap()
        );
    }

    assert_eq!(
        find_attestation_with_program(program, a, b, nonce).unwrap(),
        find_program_address(
            &[b"attestation", a.as_bytes(), b.as_bytes(), nonce.as_bytes()],
            &program
        )
        .unwrap()
    );
    assert_ne!(
        find_attestation_with_program(program, a, b, nonce).unwrap(),
        find_attestation_with_program(program, b, a, nonce).unwrap()
    );
    mint_and_authority_seed_domains(program, a);
    seed_length_boundaries(a);
}

fn mint_and_authority_seed_domains(program: Address, address: Address) {
    assert_eq!(
        find_schema_mint_with_program(program, address).unwrap(),
        find_program_address(&[b"schemaMint", address.as_bytes()], &program).unwrap()
    );
    assert_eq!(
        find_attestation_mint_with_program(program, address).unwrap(),
        find_program_address(&[b"attestationMint", address.as_bytes()], &program).unwrap()
    );
    assert_eq!(
        find_event_authority_with_program(program).unwrap(),
        find_program_address(&[b"__event_authority"], &program).unwrap()
    );
    assert_eq!(
        find_sas_authority_with_program(program).unwrap(),
        find_program_address(&[b"sas"], &program).unwrap()
    );
    assert_ne!(
        find_sas_authority_with_program(program).unwrap(),
        find_sas_authority().unwrap()
    );
}

fn seed_length_boundaries(credential: Address) {
    assert!(find_credential(credential, &"é".repeat(16)).is_ok());
    assert!(find_credential(credential, &"é".repeat(17)).is_err());
    assert!(find_schema(credential, &"x".repeat(33), 1).is_err());
}
