//! Every instruction's nonzero primary data and exact ordered privilege matrix.
use super::support;
use solana_kit_core::{
    AccountRole, Address, Instruction,
    codecs::{Decoder, Encoder},
};
use solana_kit_subscriptions::*;
use support::*;

fn verify(
    instruction: Instruction,
    value: SubscriptionsInstructionData,
    tag: u8,
    payload: Vec<u8>,
    optional: bool,
) {
    let mut expected = vec![tag];
    expected.extend(payload);
    assert_eq!(instruction.data, expected);
    assert_eq!(format!("{value}"), format!("{value:?}"));
    assert_eq!(
        hash(&value),
        hash(
            &SubscriptionsInstructionDataCodec
                .decode(&expected, LIMITS)
                .unwrap()
        )
    );
    assert_eq!(parse_instruction(&instruction, LIMITS).unwrap(), value);
    assert_eq!(
        SubscriptionsInstructionDataCodec
            .encode(&value, LIMITS.max_bytes)
            .unwrap(),
        expected
    );
    assert_eq!(
        SubscriptionsInstructionDataCodec
            .decode(&expected, LIMITS)
            .unwrap(),
        value
    );

    assert_eq!(identify_instruction(&expected).unwrap(), tag);
    malformed(&instruction);

    privileges(&instruction);

    let mut short = instruction.clone();
    short.accounts.pop();
    assert_eq!(parse_instruction(&short, LIMITS).is_ok(), optional);
    let mut excess = instruction.clone();
    excess.accounts.push(instruction.accounts[0]);
    assert!(parse_instruction(&excess, LIMITS).is_err());
    let override_program = Address::from_bytes([99; 32]);
    let mut overridden = instruction;

    overridden.program_address = override_program;
    assert!(parse_instruction(&overridden, LIMITS).is_err());
    assert_eq!(
        parse_instruction_with_program(override_program, &overridden, LIMITS).unwrap(),
        value
    );
}

fn malformed(instruction: &Instruction) {
    for end in 0..instruction.data.len() {
        let mut changed = instruction.clone();
        changed.data.truncate(end);
        assert!(parse_instruction(&changed, LIMITS).is_err());
    }

    let mut changed = instruction.clone();
    changed.data.push(0);
    assert!(parse_instruction(&changed, LIMITS).is_err());
    changed.data[0] = u8::MAX;
    assert!(parse_instruction(&changed, LIMITS).is_err());
}

fn privileges(instruction: &Instruction) {
    for index in 0..instruction.accounts.len() {
        for role in [
            AccountRole::Readonly,
            AccountRole::Writable,
            AccountRole::ReadonlySigner,
            AccountRole::WritableSigner,
        ] {
            let mut changed = instruction.clone();
            changed.accounts[index].role = role;
            assert_eq!(
                parse_instruction(&changed, LIMITS).is_ok(),
                role == instruction.accounts[index].role
            );
        }
    }
}

macro_rules! case {
    ($tag:expr, $value:expr, $variant:ident, $roles:expr, $optional:expr, $payload:expr) => {{
        let value = $value;
        let accounts = roles($roles);
        let instruction = value.instruction(&accounts, LIMITS).unwrap();
        let override_program = Address::from_bytes([99; 32]);
        let overridden = value
            .instruction_with_program(override_program, &accounts, LIMITS)
            .unwrap();
        assert_eq!(instruction.data, overridden.data);
        assert_eq!(overridden.program_address, override_program);
        verify(
            instruction,
            SubscriptionsInstructionData::$variant(value),
            $tag,
            $payload,
            $optional,
        );
    }};
}

#[cfg_attr(test, test)]
pub fn all_authority_and_revoke_instructions() {
    case!(
        0,
        InitSubscriptionAuthority {},
        InitSubscriptionAuthority,
        "SwrwrrS",
        true,
        vec![]
    );
    case!(
        3,
        RevokeDelegation {},
        RevokeDelegation,
        "Sw",
        false,
        vec![]
    );
    case!(
        6,
        CloseSubscriptionAuthority {},
        CloseSubscriptionAuthority,
        "Sww",
        true,
        vec![]
    );
    case!(
        14,
        RevokeSubscriptionAuthority {},
        RevokeSubscriptionAuthority,
        "Swrrww",
        true,
        vec![]
    );
    case!(
        15,
        RevokeAbandonedDelegation {},
        RevokeAbandonedDelegation,
        "Swr",
        false,
        vec![]
    );
    case!(
        16,
        RevokeAbandonedSubscription {},
        RevokeAbandonedSubscription,
        "Swrr",
        false,
        vec![]
    );
}

#[cfg_attr(test, test)]
pub fn all_fixed_recurring_and_subscription_transfer_instructions() {
    let fixed = CreateFixedDelegationData {
        nonce: NUMBER,
        amount: NUMBER,
        expiry_ts: SIGNED,
        expected_subscription_authority_init_id: SIGNED,
    };
    case!(
        1,
        CreateFixedDelegation {
            fixed_delegation: fixed
        },
        CreateFixedDelegation,
        "SrwrrS",
        true,
        words(4)
    );
    let recurring = CreateRecurringDelegationData {
        nonce: NUMBER,
        amount_per_period: NUMBER,
        period_length_s: NUMBER,
        start_ts: SIGNED,
        expiry_ts: SIGNED,
        expected_subscription_authority_init_id: SIGNED,
    };
    case!(
        2,
        CreateRecurringDelegation {
            recurring_delegation: recurring
        },
        CreateRecurringDelegation,
        "SrwrrS",
        true,
        words(6)
    );
    case!(
        4,
        TransferFixed {
            transfer_data: transfer_data()
        },
        TransferFixed,
        "wrwwrrsrr",
        false,
        transfer_bytes()
    );
    case!(
        5,
        TransferRecurring {
            transfer_data: transfer_data()
        },
        TransferRecurring,
        "wrwwrrsrr",
        false,
        transfer_bytes()
    );
    case!(
        10,
        TransferSubscription {
            transfer_data: transfer_data()
        },
        TransferSubscription,
        "wrrwwsrrrr",
        false,
        transfer_bytes()
    );
}

#[cfg_attr(test, test)]
pub fn all_plan_and_subscription_lifecycle_instructions() {
    case!(
        7,
        CreatePlan {
            plan_data: plan_data()
        },
        CreatePlan,
        "SwrrrS",
        true,
        plan_bytes()
    );
    update_plan();
    case!(9, DeletePlan {}, DeletePlan, "Sw", false, vec![]);
    subscribe();
    case!(
        12,
        CancelSubscription {},
        CancelSubscription,
        "srwrr",
        false,
        vec![]
    );
    case!(
        13,
        ResumeSubscription {
            resume_data: ResumeData {
                expected_expires_at_ts: SIGNED
            }
        },
        ResumeSubscription,
        "srwrrr",
        false,
        words(1)
    );
    case!(
        17,
        CancelSubscriptionNow {
            cancel_subscription_now_data: CancelSubscriptionNowData {
                expected_current_period_start_ts: SIGNED
            }
        },
        CancelSubscriptionNow,
        "ssrwrr",
        false,
        words(1)
    );
}

fn update_plan() {
    let data = UpdatePlanData {
        status: 1,
        end_ts: SIGNED,
        pullers: vec![ADDRESS; ADDRESS_COUNT],
        metadata_uri: "x".repeat(TEXT_WIDTH),
        expected_created_at: SIGNED,
        expected_end_ts: SIGNED,
        expected_pullers: vec![ADDRESS; ADDRESS_COUNT],
        expected_metadata_uri: "x".repeat(TEXT_WIDTH),
    };
    let mut bytes = vec![1];
    bytes.extend(WORD);
    bytes.extend([77; 32].repeat(ADDRESS_COUNT));
    bytes.extend([b'x'; TEXT_WIDTH]);
    bytes.extend(words(2));
    bytes.extend([77; 32].repeat(ADDRESS_COUNT));
    bytes.extend([b'x'; TEXT_WIDTH]);

    case!(
        8,
        UpdatePlan {
            update_plan_data: data
        },
        UpdatePlan,
        "swrr",
        false,
        bytes
    );
}

fn subscribe() {
    let data = SubscribeData {
        plan_id: NUMBER,
        plan_bump: BUMP,
        expected_mint: ADDRESS,
        expected_amount: NUMBER,
        expected_period_hours: NUMBER,
        expected_created_at: SIGNED,
        expected_subscription_authority_init_id: SIGNED,
    };
    let mut bytes = words(1);
    bytes.push(BUMP);
    bytes.extend([77; 32]);
    bytes.extend(words(4));
    case!(
        11,
        Subscribe {
            subscribe_data: data
        },
        Subscribe,
        "SrrwrrrrS",
        true,
        bytes
    );
}
