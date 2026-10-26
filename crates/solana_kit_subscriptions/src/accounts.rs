//! Already fetched snapshots retain core ownership, existence and allocation checks.
use crate::generated::*;
use solana_kit_core::{
    Address,
    accounts::{Account, AccountError, EncodedAccount},
    codecs::{Decoder, Error, Limits, Reader},
};

/// Typed account selection from the primary program's explicit first-byte table.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccountKind {
    /// Subscription-authority state.
    SubscriptionAuthority,
    /// Billing-plan state.
    Plan,
    /// Fixed allowance state.
    FixedDelegation,
    /// Recurring allowance state.
    RecurringDelegation,
    /// Plan subscription state.
    SubscriptionDelegation,
}

/// Identifies a data account without treating an empty EventAuthority PDA as allocated state.
/// # Errors
/// Rejects empty bytes or unknown account discriminants.
pub fn identify_account(bytes: &[u8]) -> Result<AccountKind, Error> {
    match bytes.first() {
        Some(0) => Ok(AccountKind::SubscriptionAuthority),
        Some(1) => Ok(AccountKind::Plan),
        Some(2) => Ok(AccountKind::FixedDelegation),
        Some(3) => Ok(AccountKind::RecurringDelegation),
        Some(4) => Ok(AccountKind::SubscriptionDelegation),
        None => Err(Error::Truncated),
        _ => Err(Error::InvalidDiscriminator),
    }
}

struct Tagged<C> {
    kind: AccountKind,
    codec: C,
}

impl<T, C: Decoder<T>> Decoder<T> for Tagged<C> {
    fn read(&self, reader: &mut Reader<'_>) -> Result<T, Error> {
        if identify_account(reader.remaining())? != self.kind {
            return Err(Error::InvalidDiscriminator);
        }

        self.codec.read(reader)
    }
}

macro_rules! account_decoder {
    ($function:ident, $with_program:ident, $value:ident, $codec:ident, $kind:ident) => {
        #[doc = concat!("Decodes `", stringify!($value), "` with the canonical owner.")]
        #[doc = "# Errors"]
        #[doc = "Preserves owner, allocation, discriminator, wire and caller-limit failures."]
        pub fn $function(
            account: EncodedAccount,
            limits: Limits,
        ) -> Result<Account<$value>, AccountError> {
            $with_program(account, PROGRAM, limits)
        }

        #[doc = concat!("Decodes `", stringify!($value), "` against an explicit owner.")]
        #[doc = "# Errors"]
        #[doc = "Preserves owner, allocation, discriminator, wire and caller-limit failures."]
        pub fn $with_program(
            account: EncodedAccount,
            program: Address,
            limits: Limits,
        ) -> Result<Account<$value>, AccountError> {
            account.decode(
                &Tagged {
                    kind: AccountKind::$kind,
                    codec: $codec,
                },
                program,
                limits,
            )
        }
    };
}

account_decoder!(
    decode_fixed_delegation,
    decode_fixed_delegation_with_program,
    FixedDelegation,
    FixedDelegationCodec,
    FixedDelegation
);
account_decoder!(
    decode_recurring_delegation,
    decode_recurring_delegation_with_program,
    RecurringDelegation,
    RecurringDelegationCodec,
    RecurringDelegation
);
account_decoder!(
    decode_subscription_authority,
    decode_subscription_authority_with_program,
    SubscriptionAuthority,
    SubscriptionAuthorityCodec,
    SubscriptionAuthority
);
account_decoder!(decode_plan, decode_plan_with_program, Plan, PlanCodec, Plan);
account_decoder!(
    decode_subscription_delegation,
    decode_subscription_delegation_with_program,
    SubscriptionDelegation,
    SubscriptionDelegationCodec,
    SubscriptionDelegation
);

/// Decodes the empty EventAuthority representation against the caller's expected owner.
/// The PDA is an event-signing identity, not an allocated program-data account.
/// # Errors
/// Preserves the core owner's exact validation and strict empty-codec failures.
pub fn decode_event_authority(
    account: EncodedAccount,
    expected_owner: Address,
    limits: Limits,
) -> Result<Account<EventAuthority>, AccountError> {
    account.decode(&EventAuthorityCodec, expected_owner, limits)
}
