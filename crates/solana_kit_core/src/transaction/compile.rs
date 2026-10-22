//! Deterministic local construction; protocol decoding has a separate path.

use alloc::{collections::BTreeMap, vec::Vec};

use crate::{AccountRole, Address, Instruction};

use super::{
    CompiledInstruction, Error, Header, Lifetime, Lookup, LookupTable, Message, PACKET_SIZE,
    V1Config, Version, wire,
};

mod collation;
use collation::address_key;

const MAX_ACCOUNTS: usize = 256;

#[derive(Clone, Copy)]
struct Entry {
    role: AccountRole,
    invoked: bool,
}

// Compilation merges privileges before selecting lookups so signers and invoked
// programs cannot accidentally move out of the static account partition.
pub(super) fn compile(
    version: Version,
    payer: Address,
    lifetime: Lifetime,
    instructions: &[Instruction],
    tables: &[LookupTable],
    config: Option<V1Config>,
) -> Result<Message, Error> {
    match lifetime {
        Lifetime::Unresolved(_) => return Err(Error::UnresolvedLifetime),
        Lifetime::DurableNonce(nonce) => nonce.validate_instructions(instructions)?,
        Lifetime::Blockhash { .. } => {}
    }

    if instructions.len() > PACKET_SIZE || tables.len() > MAX_ACCOUNTS {
        return Err(Error::PacketTooLarge);
    }

    if version == Version::V1 && instructions.len() > super::v1::MAX_INSTRUCTIONS {
        return Err(Error::InvalidV1Limits);
    }

    let accounts = collect_accounts(payer, instructions, version)?;
    let mut tables: Vec<_> = tables.iter().collect();
    tables.sort_by_key(|table| address_key(table.address));
    validate_tables(version, &tables)?;
    let mut static_entries = Vec::new();
    let mut loaded = Vec::new();

    partition(accounts, &tables, &mut static_entries, &mut loaded);

    static_entries.sort_by_key(|(address, role)| {
        (
            *address != payer,
            !role.is_signer(),
            !role.is_writable(),
            address_key(*address),
        )
    });
    loaded.sort_by_key(|(address, role, table, _)| {
        (
            !role.is_writable(),
            address_key(*table),
            address_key(*address),
        )
    });
    let header = header(&static_entries)?;
    let static_accounts: Vec<_> = static_entries.iter().map(|(address, _)| *address).collect();
    let mut ordered = static_accounts.clone();
    ordered.extend(loaded.iter().map(|(address, _, _, _)| *address));
    let instructions = compile_instructions(instructions, &ordered)?;
    let lookups = compile_lookups(&loaded, &tables);
    let mut message = Message {
        version,
        header,
        static_accounts,
        instructions,
        lookups,
        lifetime,
        config,
        bytes: Vec::new(),
    };
    message.bytes = wire::encode_message(&message)?;
    wire::validate(&message)?;

    Ok(message)
}

fn partition(
    accounts: BTreeMap<Address, Entry>,
    tables: &[&LookupTable],
    static_entries: &mut Vec<(Address, AccountRole)>,
    loaded: &mut Vec<(Address, AccountRole, Address, u8)>,
) {
    for (address, entry) in accounts {
        if let Some((table, index)) = lookup(address, entry, tables) {
            loaded.push((address, entry.role, table, index));
        } else {
            static_entries.push((address, entry.role));
        }
    }
}

// Privileges merge before partitioning so a later signer/writable use cannot
// accidentally remain a readonly lookup entry. Bound input before cloning data.
fn collect_accounts(
    payer: Address,
    instructions: &[Instruction],
    version: Version,
) -> Result<BTreeMap<Address, Entry>, Error> {
    let mut accounts = BTreeMap::new();
    insert(&mut accounts, payer, AccountRole::WritableSigner, false)?;

    for instruction in instructions {
        if instruction.data.len() > wire::packet_limit(version)
            || instruction.accounts.len() > PACKET_SIZE
        {
            return Err(Error::PacketTooLarge);
        }

        insert(
            &mut accounts,
            instruction.program_address,
            AccountRole::Readonly,
            true,
        )?;

        for account in &instruction.accounts {
            insert(&mut accounts, account.address, account.role, false)?;
        }
    }

    Ok(accounts)
}

fn insert(
    accounts: &mut BTreeMap<Address, Entry>,
    address: Address,
    role: AccountRole,
    invoked: bool,
) -> Result<(), Error> {
    let entry = accounts.entry(address).or_insert(Entry { role, invoked });
    entry.role = entry.role.merge(role);
    entry.invoked |= invoked;

    if entry.invoked && entry.role.is_writable() {
        return Err(Error::WritableProgram);
    }

    if accounts.len() > MAX_ACCOUNTS {
        return Err(Error::InvalidIndex);
    }

    Ok(())
}

fn validate_tables(version: Version, tables: &[&LookupTable]) -> Result<(), Error> {
    if version != Version::V0 && !tables.is_empty() {
        return Err(Error::InvalidLookup);
    }

    if tables
        .iter()
        .any(|table| table.addresses.len() > MAX_ACCOUNTS)
        || tables
            .windows(2)
            .any(|pair| pair[0].address == pair[1].address)
    {
        return Err(Error::InvalidLookup);
    }

    Ok(())
}

fn lookup(address: Address, entry: Entry, tables: &[&LookupTable]) -> Option<(Address, u8)> {
    if entry.role.is_signer() || entry.invoked {
        return None;
    }

    tables.iter().find_map(|table| {
        let index = table
            .addresses
            .iter()
            .position(|candidate| *candidate == address)?;
        u8::try_from(index).ok().map(|index| (table.address, index))
    })
}

fn header(entries: &[(Address, AccountRole)]) -> Result<Header, Error> {
    let count = |predicate: fn(AccountRole) -> bool| {
        u8::try_from(entries.iter().filter(|(_, role)| predicate(*role)).count())
            .map_err(|_| Error::InvalidHeader)
    };

    Ok(Header {
        required_signatures: count(AccountRole::is_signer)?,
        readonly_signers: count(|role| role.is_signer() && !role.is_writable())?,
        readonly_unsigned: count(|role| !role.is_signer() && !role.is_writable())?,
    })
}

fn compile_instructions(
    instructions: &[Instruction],
    ordered: &[Address],
) -> Result<Vec<CompiledInstruction>, Error> {
    let index = |address: Address| {
        ordered
            .iter()
            .position(|candidate| *candidate == address)
            .and_then(|index| u8::try_from(index).ok())
            .ok_or(Error::InvalidIndex)
    };
    instructions
        .iter()
        .map(|instruction| {
            Ok(CompiledInstruction {
                program_index: index(instruction.program_address)?,
                accounts: instruction
                    .accounts
                    .iter()
                    .map(|account| index(account.address))
                    .collect::<Result<_, _>>()?,
                data: instruction.data.clone(),
            })
        })
        .collect()
}

// Descriptor order follows table address; each privilege partition follows the
// same address ordering as the loaded account indices used by instructions.
fn compile_lookups(
    loaded: &[(Address, AccountRole, Address, u8)],
    tables: &[&LookupTable],
) -> Vec<Lookup> {
    tables
        .iter()
        .filter_map(|table| {
            let mut lookup = Lookup {
                address: table.address,
                writable: Vec::new(),
                readonly: Vec::new(),
            };

            for (_, role, address, index) in loaded {
                if *address != table.address {
                    continue;
                }

                push_lookup_index(&mut lookup, *role, *index);
            }

            (!lookup.writable.is_empty() || !lookup.readonly.is_empty()).then_some(lookup)
        })
        .collect()
}

fn push_lookup_index(lookup: &mut Lookup, role: AccountRole, index: u8) {
    if role.is_writable() {
        lookup.writable.push(index);
    } else {
        lookup.readonly.push(index);
    }
}
