//! All six source account layouts, fetched metadata and explicit future-tail contracts.
use super::support::*;
use solana_kit_core::{
    Address,
    accounts::{Account, AccountError, EncodedAccount, MaybeEncodedAccount},
    codecs::{Decoder, Encoder, Error, Limits},
};
use solana_kit_subscriptions::*;

const FIXED_TAG: u8 = 2;
const RECURRING_TAG: u8 = 3;
const SUBSCRIPTION_TAG: u8 = 4;
const FUTURE_TAIL: &[u8] = &[211, 212, 213];

fn snapshot(bytes: &[u8]) -> EncodedAccount {
    Account {
        address: ADDRESS,
        owner: PROGRAM,
        lamports: NUMBER,
        executable: false,
        space: u64::try_from(bytes.len()).unwrap(),
        rent_epoch: Some(NUMBER),
        data: bytes.to_vec(),
    }
}

fn fetched<T: PartialEq + core::fmt::Debug + core::fmt::Display + core::hash::Hash>(
    codec: impl Encoder<T> + Decoder<T>,
    value: T,
    bytes: Vec<u8>,
    decode: fn(EncodedAccount, Limits) -> Result<Account<T>, AccountError>,
) {
    check(codec, &value, &bytes);
    let account = decode(snapshot(&bytes), LIMITS).unwrap();
    assert_eq!(account.data, value);
    assert_eq!(account.address, ADDRESS);
    assert_eq!(account.owner, PROGRAM);
    assert_eq!(account.lamports, NUMBER);
    assert_eq!(account.space, u64::try_from(bytes.len()).unwrap());
    assert_eq!(account.rent_epoch, Some(NUMBER));

    assert!(!account.executable);

    let mut wrong = snapshot(&bytes);
    wrong.owner = Address::from_bytes([99; 32]);
    assert!(matches!(
        decode(wrong, LIMITS),
        Err(AccountError::OwnerMismatch { .. })
    ));
    let mut wrong = snapshot(&bytes);
    wrong.space += 1;
    assert!(matches!(
        decode(wrong, LIMITS),
        Err(AccountError::DataSizeMismatch(_))
    ));
    assert!(decode(snapshot(&bytes), Limits::new(0, 0)).is_err() || bytes.is_empty());
    assert_eq!(
        MaybeEncodedAccount::Missing(ADDRESS).require(),
        Err(AccountError::Missing(ADDRESS))
    );

    if !bytes.is_empty() {
        let mut wrong = bytes;
        wrong[0] = u8::MAX;
        assert!(matches!(
            decode(snapshot(&wrong), LIMITS),
            Err(AccountError::Decode { .. })
        ));
    }
}

fn fixed() -> FixedDelegation {
    FixedDelegation {
        header: header(FIXED_TAG),
        subscription_authority: ADDRESS,
        mint: ADDRESS,
        amount: NUMBER,
        expiry_ts: SIGNED,
    }
}

fn recurring() -> RecurringDelegation {
    RecurringDelegation {
        header: header(RECURRING_TAG),
        subscription_authority: ADDRESS,
        mint: ADDRESS,
        current_period_start_ts: SIGNED,
        period_length_s: NUMBER,
        expiry_ts: SIGNED,
        amount_per_period: NUMBER,
        amount_pulled_in_period: NUMBER,
    }
}

fn subscription() -> SubscriptionDelegation {
    SubscriptionDelegation {
        header: header(SUBSCRIPTION_TAG),
        terms: terms(),
        amount_pulled_in_period: NUMBER,
        current_period_start_ts: SIGNED,
        expires_at_ts: SIGNED,
    }
}

#[cfg_attr(test, test)]
pub fn all_delegation_account_layouts_and_fetched_metadata() {
    let mut bytes = header_bytes(FIXED_TAG);
    bytes.extend([77; 64]);
    bytes.extend(words(2));
    fetched(
        FixedDelegationCodec,
        fixed(),
        bytes,
        decode_fixed_delegation,
    );

    let mut bytes = header_bytes(RECURRING_TAG);
    bytes.extend([77; 64]);
    bytes.extend(words(5));
    fetched(
        RecurringDelegationCodec,
        recurring(),
        bytes,
        decode_recurring_delegation,
    );

    let mut bytes = header_bytes(SUBSCRIPTION_TAG);
    bytes.extend(words(6));
    fetched(
        SubscriptionDelegationCodec,
        subscription(),
        bytes,
        decode_subscription_delegation,
    );
}

#[cfg_attr(test, test)]
pub fn authority_plan_and_empty_event_authority_accounts() {
    let value = SubscriptionAuthority {
        discriminator: 0,
        user: ADDRESS,
        token_mint: ADDRESS,
        payer: ADDRESS,
        bump: BUMP,
        init_id: SIGNED,
    };
    let mut bytes = vec![0];
    bytes.extend([77; 96]);
    bytes.push(BUMP);
    bytes.extend(WORD);
    fetched(
        SubscriptionAuthorityCodec,
        value,
        bytes,
        decode_subscription_authority,
    );

    let value = Plan {
        discriminator: 1,
        owner: ADDRESS,
        bump: BUMP,
        status: 1,
        data: plan_data(),
    };
    let mut bytes = vec![1];
    bytes.extend([77; 32]);
    bytes.extend([BUMP, 1]);
    bytes.extend(plan_bytes());
    fetched(PlanCodec, value, bytes, decode_plan);
    fetched(
        EventAuthorityCodec,
        EventAuthority {},
        vec![],
        event_authority_snapshot,
    );
    let event_identity = EncodedAccount {
        owner: ADDRESS,
        ..snapshot(&[])
    };

    assert_eq!(
        decode_event_authority(event_identity, ADDRESS, LIMITS)
            .unwrap()
            .owner,
        ADDRESS
    );
}

fn event_authority_snapshot(
    account: EncodedAccount,
    limits: Limits,
) -> Result<Account<EventAuthority>, AccountError> {
    decode_event_authority(account, PROGRAM, limits)
}

#[cfg_attr(test, test)]
pub fn explicit_account_tags_fixed_utf8_and_revoke_tails() {
    for (tag, value) in [
        (0, AccountDiscriminator::SubscriptionAuthority),
        (1, AccountDiscriminator::Plan),
        (FIXED_TAG, AccountDiscriminator::FixedDelegation),
        (RECURRING_TAG, AccountDiscriminator::RecurringDelegation),
        (
            SUBSCRIPTION_TAG,
            AccountDiscriminator::SubscriptionDelegation,
        ),
    ] {
        check(AccountDiscriminatorCodec, &value, &[tag]);
    }

    check(PlanStatusCodec, &PlanStatus::Sunset, &[0]);
    check(PlanStatusCodec, &PlanStatus::Active, &[1]);
    assert!(
        AccountDiscriminatorCodec
            .decode(&[u8::MAX], LIMITS)
            .is_err()
    );
    assert!(PlanStatusCodec.decode(&[u8::MAX], LIMITS).is_err());
    assert_eq!(identify_account(&[]), Err(Error::Truncated));
    text_fields();
    future_tails();
}

fn text_fields() {
    let mut value = plan_data();
    value.metadata_uri = "é".to_owned();
    let bytes = PlanDataCodec.encode(&value, LIMITS.max_bytes).unwrap();
    let decoded = PlanDataCodec.decode(&bytes, LIMITS).unwrap();
    assert_eq!(
        decoded.metadata_uri,
        format!("é{}", "\0".repeat(TEXT_WIDTH - 2))
    );
    value.metadata_uri = "x".repeat(TEXT_WIDTH + 1);
    assert!(PlanDataCodec.encode(&value, LIMITS.max_bytes).is_err());
    let mut invalid = bytes;

    let offset = invalid.len() - TEXT_WIDTH;
    invalid[offset] = u8::MAX;
    assert_eq!(
        PlanDataCodec.decode(&invalid, LIMITS),
        Err(Error::InvalidUtf8)
    );
}

fn future_tails() {
    let fixed_bytes = FixedDelegationCodec
        .encode(&fixed(), LIMITS.max_bytes)
        .unwrap();
    let recurring_bytes = RecurringDelegationCodec
        .encode(&recurring(), LIMITS.max_bytes)
        .unwrap();
    let subscription_bytes = SubscriptionDelegationCodec
        .encode(&subscription(), LIMITS.max_bytes)
        .unwrap();
    let mut bytes = fixed_bytes.clone();
    bytes.extend(FUTURE_TAIL);
    assert_eq!(
        fixed_delegation_for_revoke(&bytes, LIMITS).unwrap(),
        WithTail {
            value: fixed(),
            trailing: FUTURE_TAIL.to_vec()
        }
    );
    assert!(decode_fixed_delegation(snapshot(&bytes), LIMITS).is_err());
    let mut bytes = recurring_bytes.clone();

    bytes.extend(FUTURE_TAIL);
    assert_eq!(
        recurring_delegation_for_revoke(&bytes, LIMITS).unwrap(),
        WithTail {
            value: recurring(),
            trailing: FUTURE_TAIL.to_vec()
        }
    );
    let mut bytes = subscription_bytes.clone();
    bytes.extend(FUTURE_TAIL);
    assert_eq!(
        subscription_delegation_for_revoke(&bytes, LIMITS).unwrap(),
        WithTail {
            value: subscription(),
            trailing: FUTURE_TAIL.to_vec()
        }
    );
    assert!(fixed_delegation_for_revoke(&fixed_bytes[..fixed_bytes.len() - 1], LIMITS).is_err());
    assert!(
        recurring_delegation_for_revoke(&recurring_bytes[..recurring_bytes.len() - 1], LIMITS)
            .is_err()
    );
    assert!(
        subscription_delegation_for_revoke(
            &subscription_bytes[..subscription_bytes.len() - 1],
            LIMITS
        )
        .is_err()
    );
}
