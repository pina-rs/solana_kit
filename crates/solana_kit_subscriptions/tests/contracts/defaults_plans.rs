//! Exact source default and override matrices for six instruction families.
use super::*;

pub(super) fn default_close_subscription_authority() {
    let input = CloseSubscriptionAuthorityAccounts {
        user: ADDRESS,
        subscription_authority: ADDRESS,
        receiver: None,
    };

    for program in [PROGRAM, OVERRIDE] {
        verify(
            input.resolve(program).unwrap(),
            vec![ADDRESS, ADDRESS],
            "Sw",
        );
    }

    let mut overrides = input;
    overrides.receiver = Some(OVERRIDE);
    verify(
        overrides.resolve(OVERRIDE).unwrap(),
        vec![ADDRESS, ADDRESS, OVERRIDE],
        "Sww",
    );
}

pub(super) fn default_create_plan() {
    let input = CreatePlanAccounts {
        merchant: ADDRESS,
        plan_pda: ADDRESS,
        token_mint: ADDRESS,
        system_program: None,
        token_program: None,
        payer: None,
    };

    for program in [PROGRAM, OVERRIDE] {
        verify(
            input.resolve(program).unwrap(),
            vec![
                ADDRESS,
                ADDRESS,
                ADDRESS,
                fixed("11111111111111111111111111111111"),
                fixed("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            ],
            "Swrrr",
        );
    }

    let mut overrides = input;
    overrides.system_program = Some(OVERRIDE);
    overrides.token_program = Some(OVERRIDE);
    overrides.payer = Some(OVERRIDE);
    verify(
        overrides.resolve(OVERRIDE).unwrap(),
        vec![ADDRESS, ADDRESS, ADDRESS, OVERRIDE, OVERRIDE, OVERRIDE],
        "SwrrrS",
    );
}

pub(super) fn default_update_plan() {
    let input = UpdatePlanAccounts {
        owner: ADDRESS,
        plan_pda: ADDRESS,
        event_authority: None,
        self_program: None,
    };

    for program in [PROGRAM, OVERRIDE] {
        verify(
            input.resolve(program).unwrap(),
            vec![
                ADDRESS,
                ADDRESS,
                event_authority(program),
                fixed("De1egAFMkMWZSN5rYXRj9CAdheBamobVNubTsi9avR44"),
            ],
            "swrr",
        );
    }

    let mut overrides = input;
    overrides.event_authority = Some(OVERRIDE);
    overrides.self_program = Some(OVERRIDE);
    verify(
        overrides.resolve(OVERRIDE).unwrap(),
        vec![ADDRESS, ADDRESS, OVERRIDE, OVERRIDE],
        "swrr",
    );
}

pub(super) fn default_delete_plan() {
    let input = DeletePlanAccounts {
        owner: ADDRESS,
        plan_pda: ADDRESS,
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

pub(super) fn default_transfer_subscription() {
    let input = TransferSubscriptionAccounts {
        subscription_pda: ADDRESS,
        plan_pda: ADDRESS,
        subscription_authority: ADDRESS,
        delegator_ata: ADDRESS,
        receiver_ata: ADDRESS,
        caller: ADDRESS,
        token_mint: ADDRESS,
        token_program: ADDRESS,
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
                ADDRESS,
                event_authority(program),
                fixed("De1egAFMkMWZSN5rYXRj9CAdheBamobVNubTsi9avR44"),
            ],
            "wrrwwsrrrr",
        );
    }

    let mut overrides = input;
    overrides.event_authority = Some(OVERRIDE);
    overrides.self_program = Some(OVERRIDE);
    verify(
        overrides.resolve(OVERRIDE).unwrap(),
        vec![
            ADDRESS, ADDRESS, ADDRESS, ADDRESS, ADDRESS, ADDRESS, ADDRESS, ADDRESS, OVERRIDE,
            OVERRIDE,
        ],
        "wrrwwsrrrr",
    );
}

pub(super) fn default_subscribe() {
    let input = SubscribeAccounts {
        subscriber: ADDRESS,
        merchant: ADDRESS,
        plan_pda: ADDRESS,
        subscription_pda: None,
        subscription_authority_pda: ADDRESS,
        system_program: None,
        event_authority: None,
        self_program: None,
        payer: None,
    };

    for program in [PROGRAM, OVERRIDE] {
        verify(
            input.resolve(program).unwrap(),
            vec![
                ADDRESS,
                ADDRESS,
                ADDRESS,
                subscription(program),
                ADDRESS,
                fixed("11111111111111111111111111111111"),
                event_authority(program),
                fixed("De1egAFMkMWZSN5rYXRj9CAdheBamobVNubTsi9avR44"),
            ],
            "Srrwrrrr",
        );
    }

    let mut overrides = input;
    overrides.subscription_pda = Some(OVERRIDE);
    overrides.system_program = Some(OVERRIDE);
    overrides.event_authority = Some(OVERRIDE);
    overrides.self_program = Some(OVERRIDE);

    overrides.payer = Some(OVERRIDE);
    verify(
        overrides.resolve(OVERRIDE).unwrap(),
        vec![
            ADDRESS, ADDRESS, ADDRESS, OVERRIDE, ADDRESS, OVERRIDE, OVERRIDE, OVERRIDE, OVERRIDE,
        ],
        "SrrwrrrrS",
    );
}
