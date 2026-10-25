//! Exact PDA seed widths and explicit message validation, without inferred bank state.
use solana_kit_core::codecs::{Decoder, Limits};
#[cfg(feature = "pda")]
use solana_kit_core::{Address, crypto::find_program_address};
use solana_kit_squads::*;

#[cfg(feature = "pda")]
const EPHEMERAL_INDEX: u8 = 254;

#[cfg(feature = "pda")]
#[cfg_attr(test, test)]
pub fn every_pda_recipe_uses_distinct_nonzero_addresses_and_complete_integer_seeds() {
    let multisig = Address::from_bytes([7; 32]);
    let create_key = Address::from_bytes([9; 32]);
    let transaction = Address::from_bytes([11; 32]);
    let index = 0x0102_0304_0506_0708u64;

    for program in [PROGRAM, Address::from_bytes([99; 32])] {
        explicit_recipes(program, multisig, create_key, transaction);
    }

    canonical_recipes(multisig, create_key, transaction);
    assert_ne!(
        find_spending_limit(multisig, create_key).unwrap(),
        find_spending_limit(create_key, multisig).unwrap()
    );
    const DIFFERENT_INDEX: u64 = 8;
    assert_ne!(
        find_transaction(multisig, index).unwrap(),
        find_transaction(multisig, DIFFERENT_INDEX).unwrap()
    );
}

#[cfg(feature = "pda")]
fn explicit_recipes(
    program: Address,
    multisig: Address,
    create_key: Address,
    transaction: Address,
) {
    let expected = |seeds: &[&[u8]]| find_program_address(seeds, &program).unwrap();
    assert_eq!(
        find_program_config_with_program(program).unwrap(),
        expected(&[b"multisig", b"program_config"])
    );
    assert_eq!(
        find_multisig_with_program(program, create_key).unwrap(),
        expected(&[b"multisig", b"multisig", create_key.as_bytes()])
    );
    assert_eq!(
        find_vault_with_program(program, multisig, 255).unwrap(),
        expected(&[b"multisig", multisig.as_bytes(), b"vault", &[255]])
    );
    indexed_transaction_recipes(program, multisig);
    assert_eq!(
        find_ephemeral_signer_with_program(program, transaction, EPHEMERAL_INDEX).unwrap(),
        expected(&[
            b"multisig",
            transaction.as_bytes(),
            b"ephemeral_signer",
            &[EPHEMERAL_INDEX]
        ])
    );
    assert_eq!(
        find_spending_limit_with_program(program, multisig, create_key).unwrap(),
        expected(&[
            b"multisig",
            multisig.as_bytes(),
            b"spending_limit",
            create_key.as_bytes()
        ])
    );
}

#[cfg(feature = "pda")]
fn indexed_transaction_recipes(program: Address, multisig: Address) {
    let index = 0x0102_0304_0506_0708u64;
    let transaction_index = 0x1122_3344u32;
    let expected = |seeds: &[&[u8]]| find_program_address(seeds, &program).unwrap();
    assert_eq!(
        find_transaction_with_program(program, multisig, index).unwrap(),
        expected(&[
            b"multisig",
            multisig.as_bytes(),
            b"transaction",
            &index.to_le_bytes()
        ])
    );
    assert_eq!(
        find_proposal_with_program(program, multisig, index).unwrap(),
        expected(&[
            b"multisig",
            multisig.as_bytes(),
            b"transaction",
            &index.to_le_bytes(),
            b"proposal"
        ])
    );
    assert_eq!(
        find_batch_transaction_with_program(program, multisig, index, transaction_index).unwrap(),
        expected(&[
            b"multisig",
            multisig.as_bytes(),
            b"transaction",
            &index.to_le_bytes(),
            b"batch_transaction",
            &transaction_index.to_le_bytes()
        ])
    );
}

#[cfg(feature = "pda")]
fn canonical_recipes(multisig: Address, create_key: Address, transaction: Address) {
    let index = 0x0102_0304_0506_0708u64;
    let transaction_index = 0x1122_3344u32;
    assert_eq!(
        find_program_config().unwrap(),
        find_program_config_with_program(PROGRAM).unwrap()
    );
    assert_eq!(
        find_multisig(create_key).unwrap(),
        find_multisig_with_program(PROGRAM, create_key).unwrap()
    );
    assert_eq!(
        find_vault(multisig, 255).unwrap(),
        find_vault_with_program(PROGRAM, multisig, 255).unwrap()
    );
    assert_eq!(
        find_transaction(multisig, index).unwrap(),
        find_transaction_with_program(PROGRAM, multisig, index).unwrap()
    );
    assert_eq!(
        find_proposal(multisig, index).unwrap(),
        find_proposal_with_program(PROGRAM, multisig, index).unwrap()
    );
    assert_eq!(
        find_batch_transaction(multisig, index, transaction_index).unwrap(),
        find_batch_transaction_with_program(PROGRAM, multisig, index, transaction_index).unwrap()
    );
    assert_eq!(
        find_ephemeral_signer(transaction, EPHEMERAL_INDEX).unwrap(),
        find_ephemeral_signer_with_program(PROGRAM, transaction, EPHEMERAL_INDEX).unwrap()
    );
    assert_eq!(
        find_spending_limit(multisig, create_key).unwrap(),
        find_spending_limit_with_program(PROGRAM, multisig, create_key).unwrap()
    );
}

#[cfg_attr(test, test)]
pub fn vault_message_checks_each_header_partition_index_and_budget() {
    const BYTE_BUDGET: usize = 8192;
    const ITEM_BUDGET: usize = 4096;
    let bytes = [
        vec![1, 1, 1, 3, 0, 0, 0],
        vec![7; 32],
        vec![8; 32],
        vec![9; 32],
        vec![
            1, 0, 0, 0, 2, 2, 0, 0, 0, 0, 1, 2, 0, 0, 0, 0xa1, 0xb2, 0, 0, 0, 0,
        ],
    ]
    .concat();
    let limits = Limits::new(BYTE_BUDGET, ITEM_BUDGET);
    let message = VaultTransactionMessageCodec.decode(&bytes, limits).unwrap();
    assert!(validate_vault_message(&message, limits).is_ok());

    for header in [(4, 1, 1), (1, 2, 1), (1, 1, 3)] {
        let mut changed = message.clone();
        changed.num_signers = header.0;
        changed.num_writable_signers = header.1;
        changed.num_writable_non_signers = header.2;
        assert!(validate_vault_message(&changed, limits).is_err());
    }

    let mut changed = message.clone();
    changed.instructions[0].program_id_index = 3;
    assert!(validate_vault_message(&changed, limits).is_err());
    changed = message.clone();
    changed.instructions[0].account_indexes[0] = 3;
    assert!(validate_vault_message(&changed, limits).is_err());
    assert!(validate_vault_message(&message, Limits::new(bytes.len() - 1, 4096)).is_err());
    assert!(validate_vault_message(&message, Limits::new(8192, 2)).is_err());

    assert!(validate_vault_message(&message, Limits::new(8192, 7)).is_err());
    assert!(validate_vault_message(&message, Limits::new(8192, 8)).is_ok());
}
