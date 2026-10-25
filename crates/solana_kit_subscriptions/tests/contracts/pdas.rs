//! Real PDA derivations and every declared default/override use independent seed recipes.
#![cfg(feature = "pda")]
use super::support::*;
use solana_kit_core::{
    AccountMeta, Address,
    crypto::{ProgramDerivedAddress, find_program_address},
};
use solana_kit_subscriptions::*;

const OVERRIDE: Address = Address::from_bytes([91; 32]);
const FIRST_SEED: Address = Address::from_bytes([31; 32]);
const SECOND_SEED: Address = Address::from_bytes([73; 32]);
const THIRD_SEED: Address = Address::from_bytes([117; 32]);

fn expected(seeds: &[&[u8]], program: Address) -> ProgramDerivedAddress {
    find_program_address(seeds, &program).unwrap()
}

fn authority(program: Address) -> Address {
    expected(
        &[
            b"SubscriptionAuthority",
            ADDRESS.as_bytes(),
            ADDRESS.as_bytes(),
        ],
        program,
    )
    .address()
}

fn event_authority(program: Address) -> Address {
    expected(&[b"event_authority"], program).address()
}

fn subscription(program: Address) -> Address {
    expected(
        &[b"subscription", ADDRESS.as_bytes(), ADDRESS.as_bytes()],
        program,
    )
    .address()
}

fn fixed(text: &str) -> Address {
    text.parse().unwrap()
}

fn verify(actual: Vec<AccountMeta>, addresses: Vec<Address>, codes: &str) {
    let expected: Vec<_> = roles(codes)
        .into_iter()
        .zip(addresses)
        .map(|(mut meta, address)| {
            meta.address = address;
            meta
        })
        .collect();
    assert_eq!(actual, expected);
}

#[cfg_attr(test, test)]
pub fn all_six_primary_pda_recipes_use_full_nonzero_seed_bytes() {
    let nonce = NUMBER.to_le_bytes();
    let delegation = expected(
        &[
            b"delegation",
            FIRST_SEED.as_bytes(),
            SECOND_SEED.as_bytes(),
            THIRD_SEED.as_bytes(),
            &nonce,
        ],
        PROGRAM,
    );
    assert_eq!(
        find_fixed_delegation(FIRST_SEED, SECOND_SEED, THIRD_SEED, NUMBER).unwrap(),
        delegation
    );
    assert_eq!(
        find_recurring_delegation(FIRST_SEED, SECOND_SEED, THIRD_SEED, NUMBER).unwrap(),
        delegation
    );
    assert_eq!(
        find_plan(FIRST_SEED, NUMBER).unwrap(),
        expected(&[b"plan", FIRST_SEED.as_bytes(), &nonce], PROGRAM)
    );
    assert_eq!(
        find_subscription_authority(FIRST_SEED, SECOND_SEED).unwrap(),
        expected(
            &[
                b"SubscriptionAuthority",
                FIRST_SEED.as_bytes(),
                SECOND_SEED.as_bytes()
            ],
            PROGRAM
        )
    );
    assert_eq!(
        find_subscription_delegation(FIRST_SEED, SECOND_SEED).unwrap(),
        expected(
            &[
                b"subscription",
                FIRST_SEED.as_bytes(),
                SECOND_SEED.as_bytes()
            ],
            PROGRAM
        )
    );
    assert_eq!(
        find_event_authority().unwrap().address(),
        event_authority(PROGRAM)
    );

    explicit_program_recipes();
}

fn explicit_program_recipes() {
    let nonce = NUMBER.to_le_bytes();
    let delegation = expected(
        &[
            b"delegation",
            FIRST_SEED.as_bytes(),
            SECOND_SEED.as_bytes(),
            THIRD_SEED.as_bytes(),
            &nonce,
        ],
        OVERRIDE,
    );
    assert_eq!(
        find_fixed_delegation_with_program(OVERRIDE, FIRST_SEED, SECOND_SEED, THIRD_SEED, NUMBER)
            .unwrap(),
        delegation
    );
    assert_eq!(
        find_recurring_delegation_with_program(
            OVERRIDE,
            FIRST_SEED,
            SECOND_SEED,
            THIRD_SEED,
            NUMBER
        )
        .unwrap(),
        delegation
    );
    assert_eq!(
        find_plan_with_program(OVERRIDE, FIRST_SEED, NUMBER).unwrap(),
        expected(&[b"plan", FIRST_SEED.as_bytes(), &nonce], OVERRIDE)
    );
    assert_eq!(
        find_subscription_authority_with_program(OVERRIDE, FIRST_SEED, SECOND_SEED).unwrap(),
        expected(
            &[
                b"SubscriptionAuthority",
                FIRST_SEED.as_bytes(),
                SECOND_SEED.as_bytes()
            ],
            OVERRIDE,
        )
    );
    assert_eq!(
        find_subscription_delegation_with_program(OVERRIDE, FIRST_SEED, SECOND_SEED).unwrap(),
        expected(
            &[
                b"subscription",
                FIRST_SEED.as_bytes(),
                SECOND_SEED.as_bytes()
            ],
            OVERRIDE,
        )
    );
    assert_eq!(
        find_event_authority_with_program(OVERRIDE).unwrap(),
        expected(&[b"event_authority"], OVERRIDE)
    );
}

#[cfg_attr(test, test)]
pub fn all_declared_defaults_and_explicit_overrides_resolve() {
    default_init_subscription_authority();
    default_create_fixed_delegation();
    default_create_recurring_delegation();
    default_revoke_delegation();
    default_transfer_fixed();
    default_transfer_recurring();
    default_close_subscription_authority();
    default_create_plan();

    default_update_plan();
    default_delete_plan();
    default_transfer_subscription();
    default_subscribe();
    default_cancel_subscription();
    default_resume_subscription();
    default_revoke_subscription_authority();
    default_revoke_abandoned_delegation();

    default_revoke_abandoned_subscription();
    default_cancel_subscription_now();
}

#[path = "defaults_authority.rs"]
mod defaults_authority;
#[path = "defaults_plans.rs"]
mod defaults_plans;
#[path = "defaults_subscriptions.rs"]
mod defaults_subscriptions;
use defaults_authority::*;
use defaults_plans::*;
use defaults_subscriptions::*;
