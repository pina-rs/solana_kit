//! Account resolution uses signed lookup counts to preserve privilege partitions.

use crate::Presence;
use crate::methods::transaction_results::{self as rpc, TransactionResult, TransactionView};
use solana_kit_core::{AccountMeta, AccountRole, Address, transaction};

use super::Error;

const MAX_ACCOUNTS: usize = 256;
const LEGACY_VERSION_PREFIX: usize = 128;
const V1_MAX_SIGNERS: usize = 12;
const V1_MAX_ACCOUNTS: usize = 64;
const V1_MAX_INSTRUCTIONS: usize = 64;

pub(super) fn version(
    source: &TransactionResult,
    binary: Option<&transaction::Transaction>,
) -> Result<transaction::Version, Error> {
    let reported = match &source.version {
        Presence::Missing => None,
        Presence::Present(rpc::TransactionVersion::Legacy(_)) => Some(transaction::Version::Legacy),
        Presence::Present(rpc::TransactionVersion::Number(0)) => Some(transaction::Version::V0),
        Presence::Present(rpc::TransactionVersion::Number(1)) => Some(transaction::Version::V1),
        _ => return Err(Error::Version),
    };

    if let Some(binary) = binary {
        let actual = binary.message().version();

        if reported.is_some_and(|version| version != actual) {
            return Err(Error::Version);
        }

        return Ok(actual);
    }

    Ok(reported.unwrap_or(transaction::Version::Legacy))
}

pub(super) fn resolve(
    source: &TransactionResult,
    binary: Option<&transaction::Transaction>,
    version: transaction::Version,
) -> Result<Vec<AccountMeta>, Error> {
    match &source.transaction {
        TransactionView::Json(value) => raw(source, value, version),
        TransactionView::JsonParsed(value) => parsed(source, value, version),
        TransactionView::Binary(_) => {
            let message = binary.ok_or(Error::Header)?.message();
            from_message(message, loaded(source))
        }
        TransactionView::Accounts(_) => Err(Error::AccountsOnly),
    }
}

pub(super) fn from_message(
    message: &transaction::Message,
    loaded: Option<&rpc::LoadedAddresses>,
) -> Result<Vec<AccountMeta>, Error> {
    let header = message.header();
    let accounts = static_accounts(
        message.static_accounts(),
        header.required_signatures,
        header.readonly_signers,
        header.readonly_unsigned,
        message.version(),
    )?;
    let counts = message
        .lookups()
        .iter()
        .fold((0, 0), |(writable, readonly), lookup| {
            (
                writable + lookup.writable_indices().len(),
                readonly + lookup.readonly_indices().len(),
            )
        });

    append_loaded(accounts, loaded, counts)
}

fn loaded(source: &TransactionResult) -> Option<&rpc::LoadedAddresses> {
    match source.meta.as_ref().map(|meta| &meta.loaded_addresses) {
        Some(Presence::Present(Some(loaded))) => Some(loaded),
        _ => None,
    }
}

fn raw(
    source: &TransactionResult,
    value: &rpc::JsonTransaction<rpc::RawMessage>,
    version: transaction::Version,
) -> Result<Vec<AccountMeta>, Error> {
    let header = &value.message.header;
    let accounts = static_accounts(
        &value.message.account_keys,
        header.num_required_signatures,
        header.num_readonly_signed_accounts,
        header.num_readonly_unsigned_accounts,
        version,
    )?;

    if value.signatures.len() != usize::from(header.num_required_signatures) {
        return Err(Error::Signatures);
    }

    if value.message.instructions.iter().any(|instruction| {
        instruction.program_id_index == 0
            || usize::from(instruction.program_id_index) >= value.message.account_keys.len()
    }) {
        return Err(Error::Index);
    }

    let counts = lookup_counts(&value.message.address_table_lookups, version)?;
    let accounts = append_loaded(accounts, loaded(source), counts)?;
    validate_v1(&accounts, version, value.message.instructions.len())?;

    Ok(accounts)
}

fn static_accounts(
    addresses: &[Address],
    required: u8,
    readonly_signed: u8,
    readonly_unsigned: u8,
    version: transaction::Version,
) -> Result<Vec<AccountMeta>, Error> {
    let required = usize::from(required);
    let readonly_signed = usize::from(readonly_signed);
    let readonly_unsigned = usize::from(readonly_unsigned);

    if required == 0
        || readonly_signed >= required
        || required + readonly_unsigned > addresses.len()
        || addresses.len() > MAX_ACCOUNTS
        || (version == transaction::Version::Legacy && required >= LEGACY_VERSION_PREFIX)
    {
        return Err(Error::Header);
    }

    Ok(addresses
        .iter()
        .enumerate()
        .map(|(index, address)| AccountMeta {
            address: *address,
            role: AccountRole::from_privileges(
                index < required,
                (0..required - readonly_signed).contains(&index)
                    || (required..addresses.len() - readonly_unsigned).contains(&index),
            ),
        })
        .collect())
}

fn lookup_counts(
    lookups: &Presence<Vec<rpc::AddressTableLookup>>,
    version: transaction::Version,
) -> Result<(usize, usize), Error> {
    let lookups = lookup_list(lookups, version)?;

    if version != transaction::Version::V0 && !lookups.is_empty() {
        return Err(Error::LoadedAddresses);
    }

    let mut seen = std::collections::BTreeSet::new();
    let mut counts = (0, 0);

    for lookup in lookups {
        if !seen.insert(lookup.account_key)
            || lookup.writable_indexes.len() + lookup.readonly_indexes.len() == 0
        {
            return Err(Error::LoadedAddresses);
        }

        counts.0 += lookup.writable_indexes.len();
        counts.1 += lookup.readonly_indexes.len();
    }

    Ok(counts)
}

fn lookup_list(
    lookups: &Presence<Vec<rpc::AddressTableLookup>>,
    version: transaction::Version,
) -> Result<&[rpc::AddressTableLookup], Error> {
    match lookups {
        Presence::Present(lookups) => Ok(lookups),

        Presence::Missing if version == transaction::Version::V0 => Err(Error::LoadedAddresses),
        Presence::Missing => Ok(&[]),
    }
}

fn append_loaded(
    mut accounts: Vec<AccountMeta>,
    loaded: Option<&rpc::LoadedAddresses>,
    counts: (usize, usize),
) -> Result<Vec<AccountMeta>, Error> {
    let Some(loaded) = loaded else {
        return if counts == (0, 0) {
            Ok(accounts)
        } else {
            Err(Error::MissingLoadedAddresses)
        };
    };

    if (loaded.writable.len(), loaded.readonly.len()) != counts
        || accounts.len() + counts.0 + counts.1 > MAX_ACCOUNTS
    {
        return Err(Error::LoadedAddresses);
    }

    accounts.extend(loaded.writable.iter().map(|address| AccountMeta {
        address: *address,
        role: AccountRole::Writable,
    }));
    accounts.extend(loaded.readonly.iter().map(|address| AccountMeta {
        address: *address,
        role: AccountRole::Readonly,
    }));

    Ok(accounts)
}

fn parsed(
    source: &TransactionResult,
    value: &rpc::JsonTransaction<rpc::ParsedMessage>,
    version: transaction::Version,
) -> Result<Vec<AccountMeta>, Error> {
    let accounts: Vec<_> = value
        .message
        .account_keys
        .iter()
        .map(|account| AccountMeta {
            address: account.pubkey,
            role: AccountRole::from_privileges(account.signer, account.writable),
        })
        .collect();
    let signers = accounts
        .iter()
        .take_while(|account| account.role.is_signer())
        .count();

    if !accounts
        .first()
        .is_some_and(|account| account.role == AccountRole::WritableSigner)
        || accounts.len() > MAX_ACCOUNTS
        || accounts[signers..]
            .iter()
            .any(|account| account.role.is_signer())
    {
        return Err(Error::Header);
    }

    if signers != value.signatures.len() {
        return Err(Error::Signatures);
    }

    let counts = lookup_counts(&value.message.address_table_lookups, version)?;
    validate_parsed_sources(&value.message.account_keys, counts)?;
    validate_parsed_loaded(source, &accounts, counts)?;
    validate_v1(&accounts, version, value.message.instructions.len())?;

    Ok(accounts)
}

fn validate_v1(
    accounts: &[AccountMeta],
    version: transaction::Version,
    instructions: usize,
) -> Result<(), Error> {
    if version != transaction::Version::V1 {
        return Ok(());
    }

    let unique: std::collections::BTreeSet<_> =
        accounts.iter().map(|account| account.address).collect();
    let signers = accounts
        .iter()
        .filter(|account| account.role.is_signer())
        .count();

    if accounts.len() > V1_MAX_ACCOUNTS
        || signers > V1_MAX_SIGNERS
        || instructions > V1_MAX_INSTRUCTIONS
        || unique.len() != accounts.len()
    {
        return Err(Error::Header);
    }

    Ok(())
}

fn validate_parsed_sources(
    accounts: &[rpc::ParsedAccount],
    counts: (usize, usize),
) -> Result<(), Error> {
    let loaded = counts.0 + counts.1;

    if loaded > accounts.len() {
        return Err(Error::LoadedAddresses);
    }

    let static_count = accounts.len() - loaded;

    for (index, account) in accounts.iter().enumerate() {
        let is_loaded = index >= static_count;

        validate_source(account, is_loaded, index < static_count + counts.0)?;
    }

    Ok(())
}

fn validate_source(
    account: &rpc::ParsedAccount,
    is_loaded: bool,
    writable: bool,
) -> Result<(), Error> {
    if let Presence::Present(Some(source)) = &account.source
        && matches!(source, rpc::AccountSource::LookupTable) != is_loaded
    {
        return Err(Error::LoadedAddresses);
    }

    if is_loaded && (account.signer || account.writable != writable) {
        return Err(Error::LoadedAddresses);
    }

    Ok(())
}

fn validate_parsed_loaded(
    source: &TransactionResult,
    accounts: &[AccountMeta],
    counts: (usize, usize),
) -> Result<(), Error> {
    let Some(meta) = &source.meta else {
        return Ok(());
    };
    let Presence::Present(Some(loaded)) = &meta.loaded_addresses else {
        return Ok(());
    };
    let expected = append_loaded(Vec::new(), Some(loaded), counts)?;
    let start = accounts.len() - counts.0 - counts.1;

    if expected != accounts[start..] || loaded.writable.len() != counts.0 {
        return Err(Error::LoadedAddresses);
    }

    Ok(())
}
