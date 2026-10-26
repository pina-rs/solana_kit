//! Exact source default and override matrices for six instruction families.
use super::*;

pub(super) fn default_cancel_subscription() {
    let input = CancelSubscriptionAccounts {
        subscriber: ADDRESS,
        plan_pda: ADDRESS,
        subscription_pda: None,
        event_authority: None,
        self_program: None,
    };

    for program in [PROGRAM, OVERRIDE] {
        verify(
            input.resolve(program).unwrap(),
            vec![
                ADDRESS,
                ADDRESS,
                subscription(program),
                event_authority(program),
                fixed("De1egAFMkMWZSN5rYXRj9CAdheBamobVNubTsi9avR44"),
            ],
            "srwrr",
        );
    }

    let mut overrides = input;
    overrides.subscription_pda = Some(OVERRIDE);
    overrides.event_authority = Some(OVERRIDE);
    overrides.self_program = Some(OVERRIDE);
    verify(
        overrides.resolve(OVERRIDE).unwrap(),
        vec![ADDRESS, ADDRESS, OVERRIDE, OVERRIDE, OVERRIDE],
        "srwrr",
    );
}

pub(super) fn default_resume_subscription() {
    let input = ResumeSubscriptionAccounts {
        subscriber: ADDRESS,
        plan_pda: ADDRESS,
        subscription_pda: None,
        subscription_authority: ADDRESS,
        event_authority: None,
        self_program: None,
    };

    for program in [PROGRAM, OVERRIDE] {
        verify(
            input.resolve(program).unwrap(),
            vec![
                ADDRESS,
                ADDRESS,
                subscription(program),
                ADDRESS,
                event_authority(program),
                fixed("De1egAFMkMWZSN5rYXRj9CAdheBamobVNubTsi9avR44"),
            ],
            "srwrrr",
        );
    }

    let mut overrides = input;
    overrides.subscription_pda = Some(OVERRIDE);
    overrides.event_authority = Some(OVERRIDE);
    overrides.self_program = Some(OVERRIDE);
    verify(
        overrides.resolve(OVERRIDE).unwrap(),
        vec![ADDRESS, ADDRESS, OVERRIDE, ADDRESS, OVERRIDE, OVERRIDE],
        "srwrrr",
    );
}

pub(super) fn default_revoke_subscription_authority() {
    let input = RevokeSubscriptionAuthorityAccounts {
        user: ADDRESS,
        user_ata: ADDRESS,
        token_mint: ADDRESS,
        token_program: ADDRESS,
        subscription_authority: None,
        receiver: None,
    };

    for program in [PROGRAM, OVERRIDE] {
        verify(
            input.resolve(program).unwrap(),
            vec![ADDRESS, ADDRESS, ADDRESS, ADDRESS, authority(program)],
            "Swrrw",
        );
    }

    let mut overrides = input;
    overrides.subscription_authority = Some(OVERRIDE);
    overrides.receiver = Some(OVERRIDE);
    verify(
        overrides.resolve(OVERRIDE).unwrap(),
        vec![ADDRESS, ADDRESS, ADDRESS, ADDRESS, OVERRIDE, OVERRIDE],
        "Swrrww",
    );
}

pub(super) fn default_revoke_abandoned_delegation() {
    let input = RevokeAbandonedDelegationAccounts {
        payer: ADDRESS,
        delegation_account: ADDRESS,
        subscription_authority: ADDRESS,
    };

    for program in [PROGRAM, OVERRIDE] {
        verify(
            input.resolve(program).unwrap(),
            vec![ADDRESS, ADDRESS, ADDRESS],
            "Swr",
        );
    }

    let overrides = input;

    verify(
        overrides.resolve(OVERRIDE).unwrap(),
        vec![ADDRESS, ADDRESS, ADDRESS],
        "Swr",
    );
}

pub(super) fn default_revoke_abandoned_subscription() {
    let input = RevokeAbandonedSubscriptionAccounts {
        payer: ADDRESS,
        subscription_account: ADDRESS,
        subscription_authority: ADDRESS,
        plan_pda: ADDRESS,
    };

    for program in [PROGRAM, OVERRIDE] {
        verify(
            input.resolve(program).unwrap(),
            vec![ADDRESS, ADDRESS, ADDRESS, ADDRESS],
            "Swrr",
        );
    }

    let overrides = input;

    verify(
        overrides.resolve(OVERRIDE).unwrap(),
        vec![ADDRESS, ADDRESS, ADDRESS, ADDRESS],
        "Swrr",
    );
}

pub(super) fn default_cancel_subscription_now() {
    let input = CancelSubscriptionNowAccounts {
        subscriber: ADDRESS,
        merchant: ADDRESS,
        plan_pda: ADDRESS,
        subscription_pda: None,
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
                subscription(program),
                event_authority(program),
                fixed("De1egAFMkMWZSN5rYXRj9CAdheBamobVNubTsi9avR44"),
            ],
            "ssrwrr",
        );
    }

    let mut overrides = input;
    overrides.subscription_pda = Some(OVERRIDE);
    overrides.event_authority = Some(OVERRIDE);
    overrides.self_program = Some(OVERRIDE);
    verify(
        overrides.resolve(OVERRIDE).unwrap(),
        vec![ADDRESS, ADDRESS, ADDRESS, OVERRIDE, OVERRIDE, OVERRIDE],
        "ssrwrr",
    );
}
