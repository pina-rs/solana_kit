//! Exact source default and override matrices for six instruction families.
use super::*;

pub(super) fn default_init_subscription_authority() {
    let input = InitSubscriptionAuthorityAccounts {
        owner: FIRST_SEED,
        subscription_authority: None,
        token_mint: SECOND_SEED,
        user_ata: ADDRESS,
        system_program: None,
        token_program: ADDRESS,
        payer: None,
    };

    for program in [PROGRAM, OVERRIDE] {
        verify(
            input.resolve(program).unwrap(),
            vec![
                FIRST_SEED,
                expected(
                    &[
                        b"SubscriptionAuthority",
                        FIRST_SEED.as_bytes(),
                        SECOND_SEED.as_bytes(),
                    ],
                    program,
                )
                .address(),
                SECOND_SEED,
                ADDRESS,
                fixed("11111111111111111111111111111111"),
                ADDRESS,
            ],
            "Swrwrr",
        );
    }

    let mut overrides = input;
    overrides.subscription_authority = Some(OVERRIDE);
    overrides.system_program = Some(OVERRIDE);
    overrides.payer = Some(OVERRIDE);
    verify(
        overrides.resolve(OVERRIDE).unwrap(),
        vec![
            FIRST_SEED,
            OVERRIDE,
            SECOND_SEED,
            ADDRESS,
            OVERRIDE,
            ADDRESS,
            OVERRIDE,
        ],
        "SwrwrrS",
    );
}

pub(super) fn default_create_fixed_delegation() {
    let input = CreateFixedDelegationAccounts {
        delegator: ADDRESS,
        subscription_authority: ADDRESS,
        delegation_account: ADDRESS,
        delegatee: ADDRESS,
        system_program: None,
        payer: None,
    };

    for program in [PROGRAM, OVERRIDE] {
        verify(
            input.resolve(program).unwrap(),
            vec![
                ADDRESS,
                ADDRESS,
                ADDRESS,
                ADDRESS,
                fixed("11111111111111111111111111111111"),
            ],
            "Srwrr",
        );
    }

    let mut overrides = input;
    overrides.system_program = Some(OVERRIDE);
    overrides.payer = Some(OVERRIDE);
    verify(
        overrides.resolve(OVERRIDE).unwrap(),
        vec![ADDRESS, ADDRESS, ADDRESS, ADDRESS, OVERRIDE, OVERRIDE],
        "SrwrrS",
    );
}

pub(super) fn default_create_recurring_delegation() {
    let input = CreateRecurringDelegationAccounts {
        delegator: ADDRESS,
        subscription_authority: ADDRESS,
        delegation_account: ADDRESS,
        delegatee: ADDRESS,
        system_program: None,
        payer: None,
    };

    for program in [PROGRAM, OVERRIDE] {
        verify(
            input.resolve(program).unwrap(),
            vec![
                ADDRESS,
                ADDRESS,
                ADDRESS,
                ADDRESS,
                fixed("11111111111111111111111111111111"),
            ],
            "Srwrr",
        );
    }

    let mut overrides = input;
    overrides.system_program = Some(OVERRIDE);
    overrides.payer = Some(OVERRIDE);
    verify(
        overrides.resolve(OVERRIDE).unwrap(),
        vec![ADDRESS, ADDRESS, ADDRESS, ADDRESS, OVERRIDE, OVERRIDE],
        "SrwrrS",
    );
}

pub(super) fn default_revoke_delegation() {
    let input = RevokeDelegationAccounts {
        authority: ADDRESS,
        delegation_account: ADDRESS,
    };

    for program in [PROGRAM, OVERRIDE] {
        verify(
            input.resolve(program).unwrap(),
            vec![ADDRESS, ADDRESS],
            "Sw",
        );
    }

    let overrides = input;

    verify(
        overrides.resolve(OVERRIDE).unwrap(),
        vec![ADDRESS, ADDRESS],
        "Sw",
    );
}

pub(super) fn default_transfer_fixed() {
    let input = TransferFixedAccounts {
        delegation_pda: ADDRESS,
        subscription_authority: ADDRESS,
        delegator_ata: ADDRESS,
        receiver_ata: ADDRESS,
        token_mint: ADDRESS,
        token_program: ADDRESS,
        delegatee: ADDRESS,
        event_authority: None,
        self_program: None,
    };

    for program in [PROGRAM, OVERRIDE] {
        verify(
            input.resolve(program).unwrap(),
            vec![
                ADDRESS,
                ADDRESS,
                ADDRESS,
                ADDRESS,
                ADDRESS,
                ADDRESS,
                ADDRESS,
                event_authority(program),
                fixed("De1egAFMkMWZSN5rYXRj9CAdheBamobVNubTsi9avR44"),
            ],
            "wrwwrrsrr",
        );
    }

    let mut overrides = input;
    overrides.event_authority = Some(OVERRIDE);
    overrides.self_program = Some(OVERRIDE);
    verify(
        overrides.resolve(OVERRIDE).unwrap(),
        vec![
            ADDRESS, ADDRESS, ADDRESS, ADDRESS, ADDRESS, ADDRESS, ADDRESS, OVERRIDE, OVERRIDE,
        ],
        "wrwwrrsrr",
    );
}

pub(super) fn default_transfer_recurring() {
    let input = TransferRecurringAccounts {
        delegation_pda: ADDRESS,
        subscription_authority: ADDRESS,
        delegator_ata: ADDRESS,
        receiver_ata: ADDRESS,
        token_mint: ADDRESS,
        token_program: ADDRESS,
        delegatee: ADDRESS,
        event_authority: None,
        self_program: None,
    };

    for program in [PROGRAM, OVERRIDE] {
        verify(
            input.resolve(program).unwrap(),
            vec![
                ADDRESS,
                ADDRESS,
                ADDRESS,
                ADDRESS,
                ADDRESS,
                ADDRESS,
                ADDRESS,
                event_authority(program),
                fixed("De1egAFMkMWZSN5rYXRj9CAdheBamobVNubTsi9avR44"),
            ],
            "wrwwrrsrr",
        );
    }

    let mut overrides = input;
    overrides.event_authority = Some(OVERRIDE);
    overrides.self_program = Some(OVERRIDE);
    verify(
        overrides.resolve(OVERRIDE).unwrap(),
        vec![
            ADDRESS, ADDRESS, ADDRESS, ADDRESS, ADDRESS, ADDRESS, ADDRESS, OVERRIDE, OVERRIDE,
        ],
        "wrwwrrsrr",
    );
}
