//! Reproduce exact scoped canonical IDs and immutable source hashes.
use std::{env, error::Error, fs};

const FIELD_COUNT: usize = 14;
const HASH_FIELD: usize = 12;
const PACKAGES: [&str; 3] = [
    "solana_kit_rpc_subscriptions",
    "solana_kit_rpc_subscriptions_api",
    "solana_kit_rpc_subscriptions_channel_websocket",
];
const SOCKET: [&str; 8] = [
    "RpcSubscriptionsChannel",
    "RpcSubscriptionsChannel.send",
    "WebSocketChannelConfig",
    "WebSocketChannelConfig.new",
    "WebSocketChannelConfig.url",
    "WebSocketChannelConfig.sendBufferHighWatermark",
    "createWebSocketChannel",
    "validateWebSocketUrl",
];

fn coverage(
    package: &str,
    source: &str,
    symbol: &str,
) -> (&'static str, &'static str, &'static str) {
    if package == PACKAGES[2] && SOCKET.contains(&symbol) {
        return (
            "partial",
            "verification/browser-websocket/src/tests.rs",
            "Window text socket; Dart private-host/default-WSS/broadcast policy differs; trusted positive WSS unverified",
        );
    }

    if package == PACKAGES[1] || source.ends_with("rpc_subscriptions_execute.dart") {
        return (
            "partial",
            "verification/browser-websocket/src/tests/families.rs",
            "Window network only; workers/WASI/Leptos/mobile and complete Dart convenience contracts pending",
        );
    }

    (
        "pending",
        "-",
        "pooling/coalescing/autoping/reconnect/convenience/portable-host/device work pending",
    )
}

fn entry(line: &str) -> Option<String> {
    let fields: Vec<_> = line.split('\t').collect();

    if fields.len() != FIELD_COUNT || !PACKAGES.contains(&fields[1]) {
        return None;
    }

    let (status, evidence, limit) = coverage(fields[1], fields[2], fields[3]);
    Some(format!(
        "{}\t{}\t{}\t{}\t{}\t{status}\t{evidence}\t{limit}\n",
        fields[0], fields[1], fields[2], fields[3], fields[HASH_FIELD],
    ))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;
    let header = "id\tpackage\tsource\tsymbol\tsource_sha256\tstatus\tevidence\tlimit\n";
    let mut output = String::from(header);
    output.extend(input.lines().filter_map(entry));
    fs::write(&args[2], output)?;
    Ok(())
}
