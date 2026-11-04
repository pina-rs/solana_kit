//! Caller-resolved decompilation back to an uncompiled instruction view.

use alloc::{collections::BTreeSet, vec::Vec};

use crate::{
    AccountLookupMeta, AccountMeta, AccountRole, Address, Instruction, InstructionAccount,
    InstructionWithAccounts, LookupRole,
};

use super::{
    CompiledInstruction, Error, Lifetime, Lookup, LookupTable, Message, V1Config, Version,
    compile::MAX_ACCOUNTS,
};

/// An uncompiled message rebuilt from a compiled snapshot.
///
/// Compilation is lossy. Rebuilding through [`Self::compile`] cannot restore a
/// narrower per-instruction privilege that compilation merged globally, and a
/// received message with noncanonical account ordering need not produce
/// identical bytes. Canonical messages produced by the Kit compiler do roundtrip
/// to their exact signed bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecompiledMessage {
    version: Version,
    payer: Address,
    lifetime: Lifetime,
    instructions: Vec<InstructionWithAccounts<InstructionAccount>>,
    config: Option<V1Config>,
}

impl DecompiledMessage {
    /// Message version preserved from the compiled snapshot.
    #[must_use]
    pub const fn version(&self) -> Version {
        self.version
    }

    /// Fee payer, which is the first static account.
    #[must_use]
    pub const fn payer(&self) -> Address {
        self.payer
    }

    /// Exact lifetime copied from the compiled message.
    #[must_use]
    pub const fn lifetime(&self) -> Lifetime {
        self.lifetime
    }

    /// Rebuilt instructions in execution order, with positions and data preserved.
    #[must_use]
    pub fn instructions(&self) -> &[InstructionWithAccounts<InstructionAccount>] {
        &self.instructions
    }

    /// Inline version-one configuration, present exactly when the message is V1.
    #[must_use]
    pub const fn config(&self) -> Option<V1Config> {
        self.config
    }

    /// Replaces the copied lifetime with caller-supplied metadata.
    ///
    /// Use this after establishing blockhash expiry or nonce identities. The
    /// decompiler never invents expiry height or authenticates nonce state.
    #[must_use]
    pub fn with_lifetime(mut self, lifetime: Lifetime) -> Self {
        self.lifetime = lifetime;
        self
    }

    /// Recompiles through the existing deterministic compiler.
    ///
    /// Version one uses its preserved inline configuration and ignores lookup
    /// snapshots. Older versions pass every instruction account through
    /// [`Message::compile_with_lookup_accounts`], so exact lookup provenance is
    /// validated rather than selected again. Compression is that same compiler
    /// selection against the supplied snapshots.
    ///
    /// # Errors
    /// Returns the compiler failure for an unresolved lifetime, conflicting
    /// lookup snapshot, invalid privileges or packet limit.
    pub fn compile(&self, tables: &[LookupTable]) -> Result<Message, Error> {
        if self.version == Version::V1 {
            return Message::compile_v1(
                self.payer,
                self.lifetime,
                &self.ordinary_instructions(),
                self.config.unwrap_or_default(),
            );
        }

        Message::compile_with_lookup_accounts(
            self.version,
            self.payer,
            self.lifetime,
            &self.instructions,
            tables,
        )
    }

    fn ordinary_instructions(&self) -> Vec<Instruction> {
        self.instructions
            .iter()
            .map(|instruction| Instruction {
                program_address: instruction.program_address,
                accounts: instruction
                    .accounts
                    .iter()
                    .map(|account| account.account_meta())
                    .collect(),
                data: instruction.data.clone(),
            })
            .collect()
    }
}

impl Message {
    /// Rebuilds an uncompiled instruction view using caller-owned table snapshots.
    ///
    /// Static partition roles come from the compiled header. Loaded accounts keep
    /// the runtime order: every writable index before every readonly index, in
    /// lookup descriptor order. Each instruction account retains its exact table
    /// address, index and non-signer role, and duplicate positions and data are
    /// preserved.
    ///
    /// The lifetime is copied exactly. A decoded [`Lifetime::Unresolved`] stays
    /// unresolved until the caller supplies established metadata through
    /// [`DecompiledMessage::with_lifetime`]; a nonce-shaped first instruction is
    /// never promoted to trusted nonce metadata.
    ///
    /// Snapshots and their indexed addresses are caller assertions. This method
    /// performs no RPC request and authenticates no table or account state.
    ///
    /// # Errors
    /// Rejects a missing or duplicate snapshot, a snapshot longer than 256
    /// addresses, any referenced table index outside its snapshot, and any
    /// instruction index outside the resolved account list. A structurally valid
    /// decoded message cannot contain the last case.
    pub fn decompile(&self, tables: &[LookupTable]) -> Result<DecompiledMessage, Error> {
        let payer = *self.static_accounts.first().ok_or(Error::InvalidHeader)?;
        let mut accounts = static_accounts(self);
        accounts.extend(loaded_accounts(self, tables)?);
        let instructions = self
            .instructions
            .iter()
            .map(|instruction| instruction_with(instruction, &accounts))
            .collect::<Result<_, _>>()?;

        Ok(DecompiledMessage {
            version: self.version,
            payer,
            lifetime: self.lifetime,
            instructions,
            config: self.config,
        })
    }
}

fn static_accounts(message: &Message) -> Vec<InstructionAccount> {
    let signers = usize::from(message.header.required_signatures);
    let writable_signers = signers - usize::from(message.header.readonly_signers);
    let writable_unsigned =
        message.static_accounts.len() - signers - usize::from(message.header.readonly_unsigned);

    message
        .static_accounts
        .iter()
        .enumerate()
        .map(|(index, address)| {
            let signer = index < signers;
            let writable = if signer {
                index < writable_signers
            } else {
                index < signers + writable_unsigned
            };

            InstructionAccount::Account(AccountMeta {
                address: *address,
                role: AccountRole::from_privileges(signer, writable),
            })
        })
        .collect()
}

fn loaded_accounts(
    message: &Message,
    tables: &[LookupTable],
) -> Result<Vec<InstructionAccount>, Error> {
    if message.lookups.is_empty() {
        return Ok(Vec::new());
    }

    validate_snapshots(tables)?;

    let mut writable = Vec::new();
    let mut readonly = Vec::new();

    for lookup in &message.lookups {
        let table = find_table(tables, lookup)?;
        push_loaded(
            &mut writable,
            table,
            lookup.writable_indices(),
            LookupRole::Writable,
        )?;
        push_loaded(
            &mut readonly,
            table,
            lookup.readonly_indices(),
            LookupRole::Readonly,
        )?;
    }

    writable.extend(readonly);
    Ok(writable)
}

fn validate_snapshots(tables: &[LookupTable]) -> Result<(), Error> {
    if tables
        .iter()
        .any(|table| table.addresses.len() > MAX_ACCOUNTS)
    {
        return Err(Error::InvalidLookup);
    }

    let mut seen = BTreeSet::new();

    if tables.iter().any(|table| !seen.insert(table.address)) {
        return Err(Error::InvalidLookup);
    }

    Ok(())
}

fn find_table<'a>(tables: &'a [LookupTable], lookup: &Lookup) -> Result<&'a LookupTable, Error> {
    tables
        .iter()
        .find(|table| table.address == lookup.address)
        .ok_or(Error::InvalidLookup)
}

fn push_loaded(
    output: &mut Vec<InstructionAccount>,
    table: &LookupTable,
    indices: &[u8],
    role: LookupRole,
) -> Result<(), Error> {
    for index in indices {
        let address = *table
            .addresses
            .get(usize::from(*index))
            .ok_or(Error::InvalidLookup)?;

        output.push(InstructionAccount::Lookup(AccountLookupMeta {
            address,
            lookup_table_address: table.address,
            address_index: *index,
            role,
        }));
    }

    Ok(())
}

fn instruction_with(
    instruction: &CompiledInstruction,
    accounts: &[InstructionAccount],
) -> Result<InstructionWithAccounts<InstructionAccount>, Error> {
    let program = *accounts
        .get(usize::from(instruction.program_index))
        .ok_or(Error::InvalidIndex)?;
    let accounts = instruction
        .accounts
        .iter()
        .map(|index| {
            accounts
                .get(usize::from(*index))
                .copied()
                .ok_or(Error::InvalidIndex)
        })
        .collect::<Result<_, _>>()?;

    Ok(InstructionWithAccounts {
        program_address: program.account_meta().address,
        accounts,
        data: instruction.data.clone(),
    })
}
