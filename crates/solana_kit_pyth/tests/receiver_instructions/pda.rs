//! PDA fixtures come from actual pinned Dart derivation, including custom programs.
use super::*;

#[test]
fn receiver_and_treasury_seeds_match_all_reference_boundaries() {
    assert_eq!(
        config_address(PYTH_SOLANA_RECEIVER_PROGRAM)
            .unwrap()
            .address()
            .to_string(),
        "DaWUKXCyXsnzcvLUyeJRWou8KTn7XtadgTsdhJ6RHS7b"
    );
    assert_eq!(
        config_address(Address::from_bytes([8; 32]))
            .unwrap()
            .address()
            .to_string(),
        "4Qtq9wXyB89UDsULihx8myhMqc6AV9XSTrJ96chhiXrZ"
    );

    for (id, expected) in [
        (0, "8hQfT7SVhkCrzUSgBq6u2wYEt1sH3xmofZ5ss3YaydZW"),
        (u8::MAX, "BBDPTkYigxggxLnQZWZWdTmt7T9KHmLdWUthrw4ZCJaT"),
    ] {
        assert_eq!(
            treasury_address(id, PYTH_SOLANA_RECEIVER_PROGRAM)
                .unwrap()
                .address()
                .to_string(),
            expected
        );
    }

    assert_eq!(
        treasury_address(u8::MAX, Address::from_bytes([8; 32]))
            .unwrap()
            .address()
            .to_string(),
        "4GiNq5oS9bdVzA6CpaGpe9YVBoDKtaxX6qg1NBD8VVth"
    );
}

#[test]
fn guardian_set_indices_are_big_endian_under_default_and_custom_programs() {
    for (index, expected) in [
        (0, "C7RmcKdjeFscYSyekkmCjvHcnBQd7qJDkeB9RmRtuB3L"),
        (2, "izDSKeLVBe4fT3HawjGfnwbMwUQSw1LLoFG9QvZ9Asz"),
        (u32::MAX, "Gjah6v9yf6eLjMbUHkg5ndERrvEf7boCYho9HdEcaLec"),
    ] {
        assert_eq!(
            guardian_set_address(index, PYTH_WORMHOLE_PROGRAM)
                .unwrap()
                .address()
                .to_string(),
            expected
        );
    }

    assert_eq!(
        guardian_set_address(u32::MAX, Address::from_bytes([9; 32]))
            .unwrap()
            .address()
            .to_string(),
        "5PNGNcCjt6Pmudfz33o7kRoREuKUg51jLvvfKM1x38eR"
    );
}

#[test]
fn derived_defaults_match_complete_reference_instructions() {
    let update = update();
    let payer = Address::from_bytes([1; 32]);
    let context = ReceiverAccounts::derive(
        payer,
        Address::from_bytes([7; 32]),
        DEFAULT_TREASURY_ID,
        PYTH_SOLANA_RECEIVER_PROGRAM,
    )
    .unwrap();
    let guardian_index = get_guardian_set_index(VAA).unwrap();
    let guardian = guardian_set_address(guardian_index, PYTH_WORMHOLE_PROGRAM).unwrap();
    let atomic = ReceiverInstruction::PostUpdateAtomic {
        accounts: context,
        guardian_set: guardian.address(),
        params: PostUpdateAtomicParams::new(VAA, &update),
    }
    .instruction(LIMITS)
    .unwrap();
    let posted = ReceiverInstruction::PostUpdate {
        accounts: context,
        encoded_vaa: Address::from_bytes([6; 32]),
        params: PostUpdateParams::new(&update),
    }
    .instruction(LIMITS)
    .unwrap();
    assert_eq!(atomic.data, ATOMIC);
    assert_eq!(posted.data, POSTED);
    assert_eq!(atomic.program_address, PYTH_SOLANA_RECEIVER_PROGRAM);

    check_defaults(&atomic, &posted);
}

/// Compares the complete seven-account reference, including the duplicate payer.
fn check_defaults(atomic: &Instruction, posted: &Instruction) {
    let expected = [
        (
            "4vJ9JU1bJJE96FWSJKvHsmmFADCg4gpZQff4P3bkLKi",
            AccountRole::WritableSigner,
        ),
        (
            "5gxPdahvSzcKySxXxPuRXZZ9s6h8hZ88XDVKavWpaQGn",
            AccountRole::Readonly,
        ),
        (
            "DaWUKXCyXsnzcvLUyeJRWou8KTn7XtadgTsdhJ6RHS7b",
            AccountRole::Readonly,
        ),
        (
            "8hQfT7SVhkCrzUSgBq6u2wYEt1sH3xmofZ5ss3YaydZW",
            AccountRole::Writable,
        ),
        (
            "US517G5965aydkZ46HS38QLi7UQiSojurfbQfKCELFx",
            AccountRole::WritableSigner,
        ),
        ("11111111111111111111111111111111", AccountRole::Readonly),
        (
            "4vJ9JU1bJJE96FWSJKvHsmmFADCg4gpZQff4P3bkLKi",
            AccountRole::ReadonlySigner,
        ),
    ];

    assert_eq!(atomic.accounts.len(), expected.len());

    for (actual, (address, role)) in atomic.accounts.iter().zip(expected) {
        assert_eq!(
            (actual.address.to_string(), actual.role),
            (address.to_string(), role)
        );
    }

    assert_eq!(posted.accounts.len(), ACCOUNT_COUNT);
    assert_eq!(posted.accounts[1].address, Address::from_bytes([6; 32]));
    assert_eq!(posted.accounts[1].role, AccountRole::Readonly);
    assert_eq!(&posted.accounts[2..], &atomic.accounts[2..]);
}

#[test]
fn discriminators_match_the_primary_anchor_hash_domain() {
    let atomic = solana_kit_core::crypto::hash(&[b"global:post_update_atomic"]).to_bytes();
    let posted = solana_kit_core::crypto::hash(&[b"global:post_update"]).to_bytes();
    assert_eq!(POST_UPDATE_ATOMIC_DISCRIMINATOR, atomic[..PREFIX_BYTES]);
    assert_eq!(POST_UPDATE_DISCRIMINATOR, posted[..PREFIX_BYTES]);
}
