//! Rust value hashing and diagnostic display for source-defined public data.
use crate::*;
use core::{
    fmt,
    hash::{Hash, Hasher},
};

/// Field-based hashing follows structural Rust equality rather than Dart list identity.
macro_rules! fields {
    ($($name:ident { $($field:ident),+ }),+ $(,)?) => {
        $(impl Hash for $name {
            fn hash<H: Hasher>(&self, state: &mut H) {
                $(self.$field.hash(state);)+
            }
        })+
    };
}

fields! {
    Epoch { value },
    UnixTimestamp { value },
    StakeFlags { bits },
    Authorized { staker, withdrawer },
    Lockup { unix_timestamp, epoch, custodian },
    Meta { rent_exempt_reserve, authorized, lockup },
    Delegation { voter_pubkey, stake, activation_epoch, deactivation_epoch, reserved },
    Stake { delegation, credits_observed },
    StakeStateStakePayload { item0, item1 },
    StakeStateV2StakePayload { item0, item1, item2 },
    StakeStateAccount { state },
    AuthorizeWithSeed { new_authorized_pubkey, stake_authorize, authority_seed, authority_owner },
    AuthorizeCheckedWithSeed { stake_authorize, authority_seed, authority_owner },
    SetLockup { unix_timestamp, epoch, custodian },
    SetLockupChecked { unix_timestamp, epoch },
}

impl Hash for StakeState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);

        match self {
            Self::Initialized(value) => value.hash(state),
            Self::Stake(value) => value.hash(state),
            Self::Uninitialized | Self::RewardsPool => {}
        }
    }
}

impl Hash for StakeStateV2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);

        match self {
            Self::Initialized(value) => value.hash(state),
            Self::Stake(value) => value.hash(state),
            Self::Uninitialized | Self::RewardsPool => {}
        }
    }
}

impl Hash for StakeAuthorize {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

/// Diagnostic display uses Rust field names; exact Dart formatting is not a wire contract.
macro_rules! display {
    ($($name:ident),+ $(,)?) => {
        $(impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Debug::fmt(self, formatter)
            }
        })+
    };
}

display!(
    Epoch,
    UnixTimestamp,
    StakeFlags,
    Authorized,
    Lockup,
    Meta,
    Delegation,
    Stake,
    StakeStateAccount,
    StakeState,
    StakeStateV2,
    StakeAuthorize,
    AuthorizeWithSeed,
    AuthorizeCheckedWithSeed,
    SetLockup,
    SetLockupChecked
);
