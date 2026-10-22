//! Reconcile the exact pinned catalog with source-backed request planning rows.

use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::error::Error;
use std::fs;
use std::path::Path;

const METHOD_COUNT: usize = 54;
const HASH_COLUMN: usize = 12;
const PIN: &str = "0138bb956d2d81ccc0c56edc602401c0df2f2b46";
const PLANS: &str = "getAccountInfo|accounts|Context<Option<AccountByEncoding>>
getAgGenesisCert|node|Option<AlpenglowGenesisCertificate>
getBalance|accounts|Context<u64>
getBlock|history|Option<BlockByDetailsAndEncoding>
getBlockCommitment|history|BlockCommitment
getBlockHeight|node|u64
getBlockProduction|node|Context<BlockProduction>
getBlocks|history|Vec<Slot>
getBlocksWithLimit|history|Vec<Slot>
getBlockTime|history|Option<i64>
getClusterNodes|node|Vec<ClusterNode>
getEpochInfo|node|EpochInfo
getEpochSchedule|node|EpochSchedule
getFeeForMessage|submission|Context<Option<u64>>
getFirstAvailableBlock|history|Slot
getGenesisHash|node|Hash
getHealth|node|HealthOk
getHighestSnapshotSlot|node|SnapshotSlots
getIdentity|node|NodeIdentity
getInflationGovernor|economics|InflationGovernor
getInflationRate|economics|InflationRate
getInflationReward|economics|Vec<Option<InflationReward>>
getLargestAccounts|economics|Context<Vec<LargestAccount>>
getLatestBlockhash|accounts|Context<LatestBlockhash>
getLeaderSchedule|node|Option<LeaderSchedule>
getMaxRetransmitSlot|node|Slot
getMaxShredInsertSlot|node|Slot
getMinimumBalanceForRentExemption|accounts|u64
getMultipleAccounts|accounts|Context<Vec<Option<AccountByEncoding>>>
getProgramAccounts|accounts|AccountsOrContextByConfig
getRecentPerformanceSamples|node|Vec<PerformanceSample>
getRecentPrioritizationFees|economics|Vec<PrioritizationFee>
getSignaturesForAddress|history|Vec<SignatureStatus>
getSignatureStatuses|submission|Context<Vec<Option<SignatureStatus>>>
getSlot|node|Slot
getSlotLeader|node|Address
getSlotLeaders|node|Vec<Address>
getStakeMinimumDelegation|economics|Context<u64>
getSupply|economics|Context<Supply>
getTokenAccountBalance|tokens|Context<TokenAmount>
getTokenAccountsByDelegate|tokens|Context<Vec<KeyedAccountByEncoding>>
getTokenAccountsByOwner|tokens|Context<Vec<KeyedAccountByEncoding>>
getTokenLargestAccounts|tokens|Context<Vec<TokenLargestAccount>>
getTokenSupply|tokens|Context<TokenAmount>
getTransaction|history|Option<TransactionByEncoding>
getTransactionCount|node|u64
getTransactionsForAddress|history|TransactionPageByDetailsAndEncoding
getVersion|node|NodeVersion
getVoteAccounts|economics|VoteAccounts
isBlockhashValid|accounts|Context<bool>
minimumLedgerSlot|history|Slot
requestAirdrop|submission|Signature
sendTransaction|submission|Signature
simulateTransaction|submission|Context<Simulation>";

/// Reconcile membership, source hashes and exact IDs before writing any worklist.
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().collect();
    let reference = Path::new(args.get(1).ok_or("expected pinned reference directory")?);
    verify_reference(reference)?;
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let source = reference.join("packages/solana_kit_rpc_api/lib/src");
    let catalog = fs::read_to_string(source.join("solana_rpc_api.dart"))?;
    let literals: Vec<_> = catalog.lines().filter_map(literal).collect();
    let methods: BTreeSet<_> = literals.iter().copied().collect();

    let rows: Vec<_> = PLANS
        .lines()
        .map(|line| line.split('|').collect::<Vec<_>>())
        .collect();

    let planned: BTreeSet<_> = rows.iter().map(|row| row[0]).collect();

    if methods != planned
        || methods.len() != METHOD_COUNT
        || rows.len() != METHOD_COUNT
        || literals.len() != METHOD_COUNT
    {
        return Err("literal catalog membership differs from the 54-row plan".into());
    }

    let ledger = fs::read_to_string(root.join("docs/parity/capabilities.tsv"))?;
    let conveniences =
        fs::read_to_string(reference.join("packages/solana_kit_rpc/lib/src/rpc_methods.dart"))?;
    let mut output = String::from(
        "method\tfamily\tplanned_response\tparams_id\tcanonical_ids\tsource_sha256\trequest_signature\tdeclared_fields\tconvenience\tstatus\tprovenance\n",
    );

    for row in rows {
        output.push_str(&render_row(&row, &source, &ledger, &conveniences)?);
    }

    let destination = root.join("docs/parity/rpc-catalog.tsv");

    if args.get(2).is_some_and(|arg| arg == "--write") {
        fs::write(destination, output)?;
    } else if fs::read_to_string(destination)? != output {
        return Err("RPC catalog differs from deterministic source-backed plan".into());
    }

    println!("54/54 exact catalog methods; canonical IDs and pinned source hashes matched");
    Ok(())
}

fn literal(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    trimmed.strip_prefix('\'')?.strip_suffix("',")
}

fn snake(method: &str) -> String {
    let mut result = String::new();

    for character in method.chars() {
        if character.is_uppercase() {
            result.push('_');
        }

        result.extend(character.to_lowercase());
    }

    result
}

fn render_row(
    row: &[&str],
    source: &Path,
    ledger: &str,
    conveniences: &str,
) -> Result<String, Box<dyn Error>> {
    let method = row[0];
    let file = format!("{}.dart", snake(method));
    let text = fs::read_to_string(source.join(&file))?;
    let path = format!("packages/solana_kit_rpc_api/lib/src/{file}");
    let (params_id, ids, hash) = identity(ledger, &path, method, &text)?;
    let (signature, fields) = declarations(&text)?;

    let convenience = if conveniences.contains(&format!(" {method}(")) {
        "yes"
    } else {
        "missing"
    };
    let status = if ["getBalance", "getLatestBlockhash", "getAccountInfo"].contains(&method) {
        "private-pilot-subset"
    } else {
        "planned-schema-unverified"
    };

    let provenance = format!("{PIN}:{path}");
    let columns = [
        method,
        row[1],
        row[2],
        &params_id,
        &ids,
        &hash,
        &signature,
        &fields,
        convenience,
        status,
        &provenance,
    ];

    Ok(columns.join("\t") + "\n")
}

fn declarations(text: &str) -> Result<(String, String), Box<dyn Error>> {
    let start = text
        .find("List<Object?> ")
        .ok_or("params signature missing")?;
    let signature = text[start..]
        .split('{')
        .next()
        .ok_or("params body missing")?
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let fields = text
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("final ") && !line.contains(" = ") && line.ends_with(';'))
        .collect::<Vec<_>>()
        .join(" ");
    Ok((signature, fields))
}

fn identity(
    ledger: &str,
    path: &str,
    method: &str,
    text: &str,
) -> Result<(String, String, String), Box<dyn Error>> {
    let entries: Vec<Vec<&str>> = ledger
        .lines()
        .skip(1)
        .map(|line| line.split('\t').collect())
        .filter(|entry: &Vec<&str>| entry[2] == path)
        .collect();
    let params = format!("{method}Params");
    let params_entry = entries
        .iter()
        .find(|entry| entry[3] == params)
        .ok_or("params canonical ID missing")?;
    let actual_hash = format!("{:x}", Sha256::digest(text.as_bytes()));

    if actual_hash != params_entry[HASH_COLUMN] {
        return Err("source SHA-256 differs from canonical inventory".into());
    }

    let ids = entries
        .iter()
        .map(|entry| entry[0])
        .collect::<Vec<_>>()
        .join(";");
    Ok((params_entry[0].to_owned(), ids, actual_hash))
}

fn verify_reference(reference: &Path) -> Result<(), Box<dyn Error>> {
    let revision = std::process::Command::new("git")
        .arg("-C")
        .arg(reference)
        .args(["rev-parse", "HEAD"])
        .output()?;
    let status = std::process::Command::new("git")
        .arg("-C")
        .arg(reference)
        .args(["status", "--porcelain"])
        .output()?;

    if !revision.status.success() || String::from_utf8(revision.stdout)?.trim() != PIN {
        return Err("reference revision differs from immutable source pin".into());
    }

    if !status.status.success() || !status.stdout.is_empty() {
        return Err("reference checkout must be clean".into());
    }

    Ok(())
}
