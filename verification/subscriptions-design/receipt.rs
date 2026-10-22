//! Extract exact canonical IDs without changing the capability inventory.
use std::{env, error::Error, fs};

const PACKAGES: [&str; 3] = [
    "solana_kit_rpc_subscriptions",
    "solana_kit_rpc_subscriptions_api",
    "solana_kit_rpc_subscriptions_channel_websocket",
];

fn entry(line: &str) -> Option<(String, bool)> {
    let fields: Vec<_> = line.split('\t').collect();

    if fields.len() < 4 || !PACKAGES.contains(&fields[1]) {
        return None;
    }

    let covered = fields[1] == "solana_kit_rpc_subscriptions_api"
        || fields[2].ends_with("rpc_subscriptions_execute.dart");

    let (status, evidence, limit) = match covered {
        true => (
            "partial",
            "crates/solana_kit_client/tests/subscriptions.rs;crates/solana_kit_client/tests/subscription_families.rs",
            "typed equivalent; no Dart API identity or host networking parity claim",
        ),
        false => (
            "pending",
            "-",
            "pooling/coalescing/autoping/convenience/host adapter outside this unit",
        ),
    };

    let output = format!(
        "{}\t{}\t{}\t{}\t{status}\t{evidence}\t{limit}\n",
        fields[0], fields[1], fields[2], fields[3],
    );
    Some((output, covered))
}

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<_> = env::args().collect();
    let ledger = fs::read_to_string(&arguments[1])?;
    let mut output = String::from("id\tpackage\tsource\tsymbol\tstatus\tevidence\tlimit\n");
    let mut partial = 0;
    let mut pending = 0;

    for (row, covered) in ledger.lines().filter_map(entry) {
        partial += usize::from(covered);
        pending += usize::from(!covered);
        output.push_str(&row);
    }

    fs::write(&arguments[2], output)?;
    println!(
        "canonical IDs {}, partial {partial}, pending {pending}, complete 0",
        partial + pending
    );
    Ok(())
}
