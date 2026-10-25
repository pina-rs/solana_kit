//! All seven complete primary event headers and nonzero payloads.
use super::support::*;
use solana_kit_core::codecs::{Decoder, Encoder};
use solana_kit_subscriptions::*;

const EVENT_DOMAIN: [u8; 8] = [228, 69, 165, 46, 81, 203, 154, 29];

fn event<T: PartialEq + core::fmt::Debug + core::fmt::Display + core::hash::Hash>(
    codec: impl Encoder<T> + Decoder<T>,
    value: T,
    tag: u8,
    payload: Vec<u8>,
) {
    let mut bytes = EVENT_DOMAIN.to_vec();
    bytes.push(tag);
    bytes.extend(payload);
    check(&codec, &value, &bytes);

    for offset in [0, EVENT_DOMAIN.len()] {
        let mut wrong = bytes.clone();
        wrong[offset] ^= 0x80;
        assert!(codec.decode(&wrong, LIMITS).is_err());
    }
}

fn transfer_payload(words_count: usize, final_addresses: usize) -> Vec<u8> {
    let mut bytes = vec![77; 128];
    bytes.extend(words(words_count));
    bytes.extend([77; 32].repeat(final_addresses));
    bytes
}

#[cfg_attr(test, test)]
pub fn fixed_recurring_and_subscription_transfer_events() {
    event(
        FixedTransferEventCodec,
        FixedTransferEvent {
            delegation: ADDRESS,
            delegator: ADDRESS,
            delegatee: ADDRESS,
            mint: ADDRESS,
            amount: NUMBER,
            remaining_amount: NUMBER,
            receiver: ADDRESS,
            receiver_token_account: ADDRESS,
        },
        3,
        transfer_payload(2, 2),
    );
    event(
        RecurringTransferEventCodec,
        RecurringTransferEvent {
            delegation: ADDRESS,
            delegator: ADDRESS,
            delegatee: ADDRESS,
            mint: ADDRESS,
            amount: NUMBER,
            period_start_ts: SIGNED,
            period_end_ts: SIGNED,
            amount_pulled_in_period: NUMBER,
            receiver: ADDRESS,
            receiver_token_account: ADDRESS,
        },
        4,
        transfer_payload(4, 2),
    );
    event(
        SubscriptionTransferEventCodec,
        SubscriptionTransferEvent {
            subscription: ADDRESS,
            plan: ADDRESS,
            delegator: ADDRESS,
            mint: ADDRESS,
            amount: NUMBER,
            period_start_ts: SIGNED,
            period_end_ts: SIGNED,
            amount_pulled_in_period: NUMBER,
            receiver: ADDRESS,
            receiver_token_account: ADDRESS,
            puller: ADDRESS,
        },
        2,
        transfer_payload(4, 3),
    );
}

#[cfg_attr(test, test)]
pub fn create_cancel_resume_and_plan_update_events() {
    let mut bytes = vec![77; 96];
    bytes.extend(WORD);
    bytes.extend([77; 32]);
    event(
        SubscriptionCreatedEventCodec,
        SubscriptionCreatedEvent {
            plan: ADDRESS,
            subscriber: ADDRESS,
            mint: ADDRESS,
            created_ts: SIGNED,
            payer: ADDRESS,
        },
        0,
        bytes,
    );
    let mut bytes = vec![77; 64];
    bytes.extend(WORD);
    event(
        SubscriptionCancelledEventCodec,
        SubscriptionCancelledEvent {
            plan: ADDRESS,
            subscriber: ADDRESS,
            expires_at_ts: SIGNED,
        },
        1,
        bytes.clone(),
    );
    event(
        SubscriptionResumedEventCodec,
        SubscriptionResumedEvent {
            plan: ADDRESS,
            subscriber: ADDRESS,
            resumed_ts: SIGNED,
        },
        5,
        bytes,
    );

    let mut bytes = vec![77; 64];
    bytes.push(1);
    bytes.extend(WORD);
    bytes.extend([77; 128]);
    event(
        PlanUpdatedEventCodec,
        PlanUpdatedEvent {
            plan: ADDRESS,
            owner: ADDRESS,
            status: 1,
            end_ts: SIGNED,
            pullers: vec![ADDRESS; ADDRESS_COUNT],
        },
        6,
        bytes,
    );
}
