//! Adds wallet source to a real public-generator workspace, retaining its hosts.
#[path = "framework.rs"]
mod framework;
#[path = "native_config.rs"]
mod native_config;
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};
type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

/// Gather and validate every expected generated member before changing files.
/// A new public template must fail visibly rather than receiving partial hooks.
pub(crate) fn run(project: &Path) -> Result<()> {
    prepare(project, NativeRuntime::Wallet)
}

/// Selects one composed Rust service for local-wallet RPC verification.
pub(crate) fn run_rpc(project: &Path) -> Result<()> {
    prepare(project, NativeRuntime::Journey)
}

#[derive(Clone, Copy)]
enum NativeRuntime {
    Wallet,
    Journey,
}

impl NativeRuntime {
    fn plugin(self) -> &'static str {
        match self {
            Self::Wallet => "solana-wallet",
            Self::Journey => "solana-journey",
        }
    }

    fn dependency(self, sdk: &Path) -> String {
        match self {
            Self::Wallet => format!(
                "solana_kit_leptoskit = {{ path = {:?}, features = [\"wallet-host\"] }}\n",
                sdk.join("crates/solana_kit_leptoskit")
            ),
            Self::Journey => format!(
                "solana_kit_app_journey_runtime = {{ path = {:?} }}\n",
                sdk.join("verification/leptoskit-app-journey/runtime")
            ),
        }
    }
}

fn prepare(project: &Path, runtime: NativeRuntime) -> Result<()> {
    let sdk = fs::canonicalize(".")?;
    framework::verify_sdk(&sdk)?;
    let project = fs::canonicalize(project)?;
    let edits = plan(&sdk, &project, runtime)?;
    write(&project, edits)
}

/// Hold every edit until the generated guest, hosts and dependency shapes agree.
/// A changed template must fail before any member receives a partial adaptation.
fn plan(sdk: &Path, project: &Path, runtime: NativeRuntime) -> Result<Vec<(PathBuf, String)>> {
    let app = project.join("solana_wallet_app_app");
    let mut edits = guest(sdk, &app)?;

    for platform in ["macos", "android", "ios", "windows", "linux"] {
        let path = project.join(platform).join("Cargo.toml");
        let contents = fs::read_to_string(&path)?;
        let dependency = runtime.dependency(sdk);
        let section = format!("target.'cfg(target_os = {platform:?})'.dependencies");
        edits.push((path, dependency_entry(&contents, &dependency, &section)?));
    }

    let config = project.join("leptoskit.toml");
    let contents = fs::read_to_string(&config)?;
    edits.push((config, native_config::select(&contents, runtime.plugin())?));

    for member in [".", "solana_wallet_app_backend", "solana_wallet_app_shared"] {
        let path = project.join(member).join("Cargo.toml");
        edits.push((path.clone(), fs::read_to_string(path)?));
    }

    for (path, contents) in &mut edits {
        if path.file_name().is_some_and(|name| name == "Cargo.toml") {
            *contents = framework::pin(contents)?;
        }
    }

    Ok(edits)
}

fn write(project: &Path, edits: Vec<(PathBuf, String)>) -> Result<()> {
    for (path, _) in &edits {
        destination(project, path)?;
    }

    for (path, contents) in edits {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, contents)?;
    }

    Ok(())
}

fn destination(project: &Path, path: &Path) -> Result<()> {
    if !path.starts_with(project) {
        return Err("fixture destination escaped generated project".into());
    }

    for member in path.ancestors().take_while(|member| *member != project) {
        regular_destination(member, path)?;
    }

    Ok(())
}

/// Missing entries are new generated modules. Existing parents must remain
/// directories and existing destinations must remain regular files, never links.
fn regular_destination(member: &Path, destination: &Path) -> Result<()> {
    let metadata = match fs::symlink_metadata(member) {
        Ok(metadata) => metadata,

        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error.into()),
    };
    let regular = if member == destination {
        metadata.is_file()
    } else {
        metadata.is_dir()
    };

    if metadata.file_type().is_symlink() || !regular {
        return Err("fixture destination must not redirect through a symlink".into());
    }

    Ok(())
}

/// Keep the generated ABI exports and native shells; replace only their app
/// callbacks and install the ordinary source modules they invoke.
fn guest(sdk: &Path, app: &Path) -> Result<Vec<(PathBuf, String)>> {
    let entry = app.join("src/lib.rs");
    let original = fs::read_to_string(&entry)?;

    if !original.contains("pub extern \"C\" fn leptoskit_main()") {
        return Err("expected a real public-generator native app entry".into());
    }

    let original_main =
        "let state = Counter::default();\n\tleptoskit::guest::mount(move || screen(&state));";
    let original_event = "leptoskit::guest::dispatch(node, kind, ptr, len)\n}";
    let mut source = original.replace(original_main, "wallet_fixture::main();");
    source = source.replace(original_event,
        "let result = leptoskit::guest::dispatch(node, kind, ptr, len);\n\twallet_fixture::poll();\n\tresult\n}");

    if !source.contains("wallet_fixture::main();") || !source.contains("wallet_fixture::poll();") {
        return Err("generated entry contract changed; inspect the new public template".into());
    }

    if !source.contains("mod wallet_fixture;") {
        source.push_str(
            "\n#[cfg(all(target_arch = \"wasm32\", feature = \"native\"))]\nmod wallet_fixture;\n",
        );
    }

    let mut edits = vec![(entry, source)];
    let templates = sdk.join("verification/leptoskit-app-journey/wallet");

    for (template, destination) in [
        ("guest/mod.rs", "mod.rs"),
        ("guest/flow.rs", "flow.rs"),
        ("guest/flow/local.rs", "flow/local.rs"),
        ("guest/flow/mobile.rs", "flow/mobile.rs"),
        ("guest/flow/approval.rs", "flow/approval.rs"),
        ("guest/flow/rpc.rs", "flow/rpc.rs"),
        ("guest/flow/rpc/panel.rs", "flow/rpc/panel.rs"),
        ("guest/flow/rpc/transfer.rs", "flow/rpc/transfer.rs"),
        ("guest/flow/rpc/submission.rs", "flow/rpc/submission.rs"),
        ("guest/flow/rpc/error.rs", "flow/rpc/error.rs"),
    ] {
        edits.push((
            app.join("src/wallet_fixture").join(destination),
            fs::read_to_string(templates.join(template))?,
        ));
    }

    let manifest = app.join("Cargo.toml");
    let mut contents = fs::read_to_string(&manifest)?;

    for dependency in guest_dependencies(sdk) {
        contents = dependency_entry(&contents, &dependency, "dependencies")?;
    }

    edits.push((manifest, contents));
    Ok(edits)
}

fn guest_dependencies(sdk: &Path) -> Vec<String> {
    let adapter = sdk.join("crates/solana_kit_leptoskit");
    let core = sdk.join("crates/solana_kit_core");
    let wallet = sdk.join("crates/solana_kit_wallet");
    let client = sdk.join("crates/solana_kit_client");
    let system = sdk.join("crates/solana_kit_system");
    vec![
        format!("solana_kit_leptoskit = {{ path = {adapter:?}, features = [\"wallet-session\", \"wallet-ui\"] }}\n"),
        format!("solana_kit_core = {{ path = {core:?}, features = [\"crypto\"] }}\n"),
        format!("solana_kit_wallet = {{ path = {wallet:?}, features = [\"mwa\"] }}\n"),
        format!("solana_kit_client = {{ path = {client:?}, features = [\"confirmation-factories\"] }}\n"),
        format!("solana_kit_system = {{ path = {system:?} }}\n"),
        format!("leptoskit_bridge = {{ version = \"=0.1.0\", git = \"https://github.com/ifiokjr/leptoskit\", rev = {:?}, features = [\"guest\"] }}\n", framework::REVISION),
        "futures = { version = \"=0.3.34\", default-features = false, features = [\"executor\", \"std\"] }\n".into(),
        "base64 = \"=0.22.1\"\n".into(),
        "serde_json = \"=1.0.154\"\n".into(),
    ]
}

fn dependency_entry(contents: &str, dependency: &str, section: &str) -> Result<String> {
    if contents.contains(dependency) {
        return Ok(contents.to_owned());
    }

    let name = dependency
        .split_once('=')
        .ok_or("dependency needs a name")?
        .0
        .trim();

    if contents.lines().any(|line| {
        line.split_once('=')
            .is_some_and(|(key, _)| key.trim() == name)
    }) {
        return Err("existing dependency must be reviewed before fixture adaptation".into());
    }

    let marker = format!("[{section}]\n");

    if contents.matches(&marker).count() != 1 {
        return Err("expected one generated dependency section".into());
    }

    Ok(contents.replacen(&marker, &format!("{marker}{dependency}"), 1))
}
