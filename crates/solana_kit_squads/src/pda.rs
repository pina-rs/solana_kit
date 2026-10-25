//! Exact MIT Dart seed order and fixed-width integer encoding.
use crate::PROGRAM;
use solana_kit_core::{
    Address,
    crypto::{PdaError, ProgramDerivedAddress, find_program_address},
};

macro_rules! pda {
    ($name:ident, $with_program:ident, ($($argument:ident: $ty:ty),*), [$($seed:expr),*]) => {
        #[doc = concat!("Derives the canonical `", stringify!($name), "` PDA.")]
        #[doc = "# Errors"]
        #[doc = "Preserves the core seed and off-curve derivation errors."]
        pub fn $name($($argument: $ty),*) -> Result<ProgramDerivedAddress, PdaError> {
            $with_program(PROGRAM, $($argument),*)
        }

        #[doc = concat!("Derives `", stringify!($name), "` against an explicit program identity.")]
        #[doc = "# Errors"]
        #[doc = "Preserves exact seed order, full integer widths and core derivation errors."]
        pub fn $with_program(
            program: Address,
            $($argument: $ty),*
        ) -> Result<ProgramDerivedAddress, PdaError> {
            find_program_address(&[$($seed),*], &program)
        }
    };
}

pda!(
    find_program_config,
    find_program_config_with_program,
    (),
    [b"multisig", b"program_config"]
);
pda!(find_multisig, find_multisig_with_program, (create_key: Address),
    [b"multisig", b"multisig", create_key.as_bytes()]);
pda!(find_vault, find_vault_with_program, (multisig: Address, index: u8),
    [b"multisig", multisig.as_bytes(), b"vault", &[index]]);
pda!(find_transaction, find_transaction_with_program, (multisig: Address, index: u64),
    [b"multisig", multisig.as_bytes(), b"transaction", &index.to_le_bytes()]);
pda!(find_proposal, find_proposal_with_program, (multisig: Address, transaction_index: u64),
    [b"multisig", multisig.as_bytes(), b"transaction",
        &transaction_index.to_le_bytes(), b"proposal"]);
pda!(find_batch_transaction, find_batch_transaction_with_program,
    (multisig: Address, batch_index: u64, transaction_index: u32),
    [b"multisig", multisig.as_bytes(), b"transaction", &batch_index.to_le_bytes(),
        b"batch_transaction", &transaction_index.to_le_bytes()]);
pda!(find_ephemeral_signer, find_ephemeral_signer_with_program,
    (transaction: Address, ephemeral_signer_index: u8),
    [b"multisig", transaction.as_bytes(), b"ephemeral_signer", &[ephemeral_signer_index]]);
pda!(find_spending_limit, find_spending_limit_with_program,
    (multisig: Address, create_key: Address),
    [b"multisig", multisig.as_bytes(), b"spending_limit", create_key.as_bytes()]);
