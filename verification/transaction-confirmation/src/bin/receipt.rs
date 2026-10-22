//! Exact declaration/source audit; arguments are owned checkout/archive/live roots.
use std::{
    collections::BTreeSet,
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};
const PACKAGE: &str = "solana_kit_transaction_confirmation";
const SOURCE_ROOT: &str = "packages/solana_kit_transaction_confirmation";

fn digest(path: &Path) -> String {
    let output = Command::new("shasum")
        .arg("-a")
        .arg("256")
        .arg(path)
        .output()
        .unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout)
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .to_owned()
}
fn classification(symbol: &str) -> (&'static str, &'static str) {
    if symbol.starts_with("SlotNotification")
        || symbol.contains("Config")
            && !symbol.starts_with("SendAndConfirmTransactionConfig")
            && !matches!(
                symbol,
                "RpcTransactionConfirmationConfig.commitment"
                    | "RpcTransactionConfirmationConfig.abortSignal"
            )
    {
        return (
            "pending",
            "async subscription registration/reconciliation or polling adapter absent; no placeholder implementation",
        );
    }

    if matches!(
        symbol,
        "defaultTransactionConfirmationPollInterval"
            | "SignatureStatus.hashCode"
            | "SignatureStatus.operator=="
            | "SignatureStatus.toString"
    ) {
        return (
            "pending",
            "no polling scheduler or Dart structural equality/hash/display equivalent claimed",
        );
    }

    (
        "partial",
        "typed policy/status/identity/expiry/nonce assertion and explicit cleanup evidence; higher factory/polling/nonce-authentication contract remains pending",
    )
}
fn write_declarations(root: &Path, archive: &Path) {
    let canonical = fs::read_to_string(root.join("docs/parity/capabilities.tsv")).unwrap();
    let mut lines = canonical.lines();
    let mut output = format!("{}\n", lines.next().unwrap());
    let mut ids = BTreeSet::new();
    let mut partial = 0;
    let mut pending = 0;

    for line in lines {
        let mut row: Vec<String> = line.split('\t').map(str::to_owned).collect();

        if row[1] != PACKAGE {
            continue;
        }

        assert!(ids.insert(row[0].clone()), "duplicate canonical ID");
        assert_eq!(row[12], digest(&archive.join(&row[2])));
        let (status, note) = classification(&row[3]);
        row[8] = status.into();
        row[9] = "crates/solana_kit_client/tests/confirmation.rs;verification/transaction-confirmation/tests;docs/verification/transaction-confirmation.md".into();
        row[10] = note.into();
        row[13].push_str(";unit185:pinned-original116+literal-events+public-native-current/MSRV;portable-host-execution-pending");
        partial += usize::from(status == "partial");
        pending += usize::from(status == "pending");
        output.push_str(&row.join("\t"));
        output.push('\n');
    }

    assert_eq!(ids.len(), 59, "exact canonical package declaration count");
    fs::write(
        root.join("docs/parity/transaction-confirmation.tsv"),
        output,
    )
    .unwrap();
    println!(
        "canonical={} partial={partial} pending={pending} complete=0",
        ids.len()
    );
}
fn files(directory: &Path, paths: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(directory).unwrap() {
        let path = entry.unwrap().path();

        if path.is_dir() {
            files(&path, paths);
        } else {
            paths.push(path);
        }
    }
}
fn write_sources(root: &Path, archive: &Path, live: &Path) {
    let mut paths = Vec::new();
    files(&archive.join(SOURCE_ROOT), &mut paths);
    paths.extend([
        archive.join("packages/solana_kit_rpc_types/lib/src/commitment.dart"),
        archive.join("packages/solana_kit_rpc_types/lib/solana_kit_rpc_types.dart"),
        archive.join("packages/solana_kit_rpc_types/LICENSE"),
    ]);
    paths.sort();
    let mut output = String::from("path\tsha256\tpin\tlive_identity\n");

    for path in paths {
        let relative = path.strip_prefix(archive).unwrap();
        let hash = digest(&path);
        let exact = fs::read(&path).unwrap() == fs::read(live.join(relative)).unwrap();
        let source = relative.to_str().unwrap();

        if source.contains("/lib/") || source.contains("/test/") || source.ends_with("/LICENSE") {
            assert!(exact, "live defining source/test/license differs: {source}");
        }

        output.push_str(&format!(
            "{source}\t{hash}\t0138bb956d2d81ccc0c56edc602401c0df2f2b46\t{exact}\n"
        ));
    }

    fs::write(
        root.join("docs/parity/transaction-confirmation-sources.tsv"),
        output,
    )
    .unwrap();
}
fn main() {
    let args: Vec<_> = env::args_os().collect();
    assert_eq!(args.len(), 4);
    let root = Path::new(&args[1]);
    let archive = Path::new(&args[2]);
    let live = Path::new(&args[3]);
    check_lock(root);
    write_sources(root, archive, live);
    write_declarations(root, archive);
}

fn registry_tuples(path: &Path) -> BTreeSet<Vec<String>> {
    let lock = fs::read_to_string(path).unwrap();
    lock.split("[[package]]")
        .filter_map(|block| {
            let fields: Vec<_> = block
                .lines()
                .filter(|line| {
                    line.starts_with("name =")
                        || line.starts_with("version =")
                        || line.starts_with("source =")
                        || line.starts_with("checksum =")
                })
                .map(str::to_owned)
                .collect();

            fields
                .iter()
                .any(|line| line.starts_with("source ="))
                .then_some(fields)
        })
        .collect()
}

fn check_lock(root: &Path) {
    let accepted = registry_tuples(&root.join("Cargo.lock"));
    let consumer = registry_tuples(&root.join("verification/transaction-confirmation/Cargo.lock"));
    assert!(
        consumer.is_subset(&accepted),
        "consumer selected a registry tuple absent from accepted root"
    );
    println!(
        "consumer registry tuples={}; all exact accepted-root matches",
        consumer.len()
    );
}
