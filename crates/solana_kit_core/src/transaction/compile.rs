//! Deterministic local construction; protocol decoding has a separate path.

use alloc::{collections::BTreeMap, vec::Vec};

use crate::{AccountRole, Address, InstructionAccount, InstructionWithAccounts};

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
    origin: Option<(Address, u8)>,
}

// Compilation merges privileges before selecting lookups so signers and invoked
// programs cannot accidentally move out of the static account partition.
pub(super) fn compile<A: Copy + Into<InstructionAccount>>(
    version: Version,
    payer: Address,
    lifetime: Lifetime,
    instructions: &[InstructionWithAccounts<A>],
    tables: &[LookupTable],
    config: Option<V1Config>,
) -> Result<Message, Error> {
    match lifetime {
        Lifetime::Unresolved(_) => return Err(Error::UnresolvedLifetime),
        Lifetime::DurableNonce(nonce) => {
            if instructions
                .first()
                .and_then(super::NonceAdvance::recognize_accounts)
                != Some(nonce.advance())
            {
                return Err(Error::InvalidNonceAdvance);
            }
        }
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

    partition(accounts, version, &tables, &mut static_entries, &mut loaded)?;

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
    version: Version,
    tables: &[&LookupTable],
    static_entries: &mut Vec<(Address, AccountRole)>,
    loaded: &mut Vec<(Address, AccountRole, Address, u8)>,
) -> Result<(), Error> {
    for (address, entry) in accounts {
        let selected = match entry.origin {
            Some(origin) => Some(validate_origin(address, entry, version, origin, tables)?),
            None => lookup(address, entry, tables),
        };

        if let Some((table, index)) = selected {
            loaded.push((address, entry.role, table, index));
        } else {
            static_entries.push((address, entry.role));
        }
    }

    Ok(())
}

fn validate_origin(
    address: Address,
    entry: Entry,
    version: Version,
    origin: (Address, u8),
    tables: &[&LookupTable],
) -> Result<(Address, u8), Error> {
    if version != Version::V0 || entry.role.is_signer() || entry.invoked {
        return Err(Error::InvalidLookup);
    }

    let table = tables
        .iter()
        .find(|table| table.address == origin.0)
        .ok_or(Error::InvalidLookup)?;

    if table.addresses.get(usize::from(origin.1)) != Some(&address) {
        return Err(Error::InvalidLookup);
    }

    Ok(origin)
}

// Privileges merge before partitioning so a later signer/writable use cannot
// accidentally remain a readonly lookup entry. Bound input before cloning data.
fn collect_accounts<A: Copy + Into<InstructionAccount>>(
    payer: Address,
    instructions: &[InstructionWithAccounts<A>],
    version: Version,
) -> Result<BTreeMap<Address, Entry>, Error> {
    let mut accounts = BTreeMap::new();
    insert(
        &mut accounts,
        payer,
        AccountRole::WritableSigner,
        false,
        None,
    )?;

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
            None,
        )?;

        for account in &instruction.accounts {
            let account = (*account).into();
            let meta = account.account_meta();
            let origin = match account {
                InstructionAccount::Account(_) => None,
                InstructionAccount::Lookup(lookup) => {
                    Some((lookup.lookup_table_address, lookup.address_index))
                }
            };
            insert(&mut accounts, meta.address, meta.role, false, origin)?;
        }
    }

    Ok(accounts)
}

fn insert(
    accounts: &mut BTreeMap<Address, Entry>,
    address: Address,
    role: AccountRole,
    invoked: bool,
    origin: Option<(Address, u8)>,
) -> Result<(), Error> {
    let entry = accounts.entry(address).or_insert(Entry {
        role,
        invoked,
        origin,
    });

    if origin.is_some() && entry.origin.is_some() && origin != entry.origin {
        return Err(Error::InvalidLookup);
    }

    entry.origin = entry.origin.or(origin);
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

fn compile_instructions<A: Copy + Into<InstructionAccount>>(
    instructions: &[InstructionWithAccounts<A>],
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
                    .map(|account| index((*account).into().account_meta().address))
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
