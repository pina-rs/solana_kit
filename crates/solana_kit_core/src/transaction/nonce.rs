//! Deterministic nonce preparation and structural inspection without account trust.

use alloc::vec::Vec;

use crate::{
    AccountMeta, AccountRole, Address, Hash, Instruction, InstructionAccount,
    InstructionWithAccounts,
};

use super::{Error, Message};

const ADVANCE_DATA: [u8; 4] = [4, 0, 0, 0];
const SYSTEM_PROGRAM: Address = Address::from_bytes([0; 32]);

/// The exact account identities in a structurally recognized nonce advance.
///
/// This proves instruction shape only. It does not prove account ownership,
/// initialization, authority, balance, stored nonce value, or validator acceptance.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NonceAdvance {
    /// Writable account whose nonce the instruction consumes.
    pub nonce_account: Address,
    /// Signer authorized by the caller to advance the nonce.
    pub nonce_authority: Address,
}

impl NonceAdvance {
    /// Constructs the canonical System tag-four instruction.
    #[must_use]
    pub fn instruction(self) -> Instruction {
        Instruction {
            program_address: SYSTEM_PROGRAM,
            accounts: alloc::vec![
                AccountMeta {
                    address: self.nonce_account,
                    role: AccountRole::Writable
                },
                AccountMeta {
                    address: crate::sysvars::RECENT_BLOCKHASHES_ADDRESS,
                    role: AccountRole::Readonly,
                },
                AccountMeta {
                    address: self.nonce_authority,
                    role: AccountRole::ReadonlySigner,
                },
            ],
            data: ADVANCE_DATA.to_vec(),
        }
    }

    /// Recognizes the pinned Dart acceptance set, including a writable authority.
    #[must_use]
    pub fn recognize(instruction: &Instruction) -> Option<Self> {
        Self::recognize_accounts(instruction)
    }

    /// The compiler also recognizes heterogeneous instruction account inputs.
    pub(super) fn recognize_accounts<A: Copy + Into<InstructionAccount>>(
        instruction: &InstructionWithAccounts<A>,
    ) -> Option<Self> {
        let [nonce, sysvar, authority] = instruction.accounts.as_slice() else {
            return None;
        };
        let nonce = (*nonce).into().account_meta();
        let sysvar = (*sysvar).into().account_meta();
        let authority = (*authority).into().account_meta();

        (instruction.program_address == SYSTEM_PROGRAM
            && instruction.data == ADVANCE_DATA
            && nonce.role == AccountRole::Writable
            && sysvar.address == crate::sysvars::RECENT_BLOCKHASHES_ADDRESS
            && sysvar.role == AccountRole::Readonly
            && authority.role.is_signer())
        .then_some(Self {
            nonce_account: nonce.address,
            nonce_authority: authority.address,
        })
    }
}

/// Caller-provided nonce token and identities, with no inferred block-height expiry.
///
/// Parse textual tokens through [`struct@Hash`]'s strict 32-byte base58 parser. These
/// values are caller assertions, never authenticated account state. Compilation
/// checks the exact prerequisite instruction before freezing signed bytes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DurableNonce {
    token: Hash,
    advance: NonceAdvance,
}

impl DurableNonce {
    /// Records explicit caller metadata without fetching or validating account state.
    #[must_use]
    pub const fn new(token: Hash, nonce_account: Address, nonce_authority: Address) -> Self {
        Self {
            token,
            advance: NonceAdvance {
                nonce_account,
                nonce_authority,
            },
        }
    }

    /// Exact nonce placed in the signed lifetime field.
    #[must_use]
    pub const fn token(self) -> Hash {
        self.token
    }

    /// Identities required by the first instruction.
    #[must_use]
    pub const fn advance(self) -> NonceAdvance {
        self.advance
    }

    /// Prepends, replaces, or retains the first advance without changing the input.
    ///
    /// A recognized matching first instruction is retained byte-for-byte, including
    /// a writable authority. A different recognized advance is replaced. Otherwise
    /// a canonical advance is prepended. Repeating this operation is idempotent.
    #[must_use]
    pub fn prepare_instructions(self, instructions: &[Instruction]) -> Vec<Instruction> {
        let first = instructions.first().and_then(NonceAdvance::recognize);

        if first == Some(self.advance) {
            return instructions.to_vec();
        }

        let skip = usize::from(first.is_some());
        let mut result = Vec::with_capacity(instructions.len() + 1 - skip);
        result.push(self.advance.instruction());
        result.extend_from_slice(&instructions[skip..]);

        result
    }

    /// Asserts the first instruction's exact identities and source-supported roles.
    ///
    /// # Errors
    /// Returns [`Error::InvalidNonceAdvance`] for a missing, malformed, misplaced,
    /// or different first advance. No account-state verification is implied.
    pub fn validate_instructions(self, instructions: &[Instruction]) -> Result<(), Error> {
        if instructions.first().and_then(NonceAdvance::recognize) != Some(self.advance) {
            return Err(Error::InvalidNonceAdvance);
        }

        Ok(())
    }
}

impl Message {
    /// Inspects the first instruction using merged static account privileges.
    ///
    /// Lifetime metadata stays unresolved after decoding. Loaded lookup addresses
    /// require caller resolution and return [`Error::InvalidLookup`]. A nonce that
    /// also signs after privilege merging is not recognized, matching pinned Dart.
    /// This is structural evidence only, never authentication of account state.
    ///
    /// # Errors
    /// Returns [`Error::InvalidLookup`] when the first instruction references loaded
    /// accounts whose concrete identities are absent from the wire message.
    pub fn nonce_advance(&self) -> Result<Option<NonceAdvance>, Error> {
        let Some(first) = self.instructions.first() else {
            return Ok(None);
        };

        if first.data != ADVANCE_DATA
            || self.static_accounts[usize::from(first.program_index)] != SYSTEM_PROGRAM
            || first.accounts.len() != 3
        {
            return Ok(None);
        }

        let accounts = first
            .accounts
            .iter()
            .map(|index| self.static_meta(*index))
            .collect::<Result<_, _>>()?;

        Ok(NonceAdvance::recognize(&Instruction {
            program_address: SYSTEM_PROGRAM,
            accounts,
            data: first.data.clone(),
        }))
    }

    fn static_meta(&self, index: u8) -> Result<AccountMeta, Error> {
        let index = usize::from(index);
        let address = *self
            .static_accounts
            .get(index)
            .ok_or(Error::InvalidLookup)?;
        let signers = usize::from(self.header.required_signatures);
        let signer = index < signers;
        let writable = if signer {
            index < signers - usize::from(self.header.readonly_signers)
        } else {
            index < self.static_accounts.len() - usize::from(self.header.readonly_unsigned)
        };

        Ok(AccountMeta {
            address,
            role: AccountRole::from_privileges(signer, writable),
        })
    }
}
