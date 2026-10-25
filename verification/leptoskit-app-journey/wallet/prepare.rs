//! Adds wallet source to a real public-generator workspace, retaining its hosts.
mod framework;
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};
type Result<T> = std::result::Result<T, Box<dyn Error>>;

/// Gather and validate every expected generated member before changing files.
/// A new public template must fail visibly rather than receiving partial hooks.
fn main() -> Result<()> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();

    if arguments.len() != 4 {
        return Err("expected SDK_ROOT FRAMEWORK_ROOT FRAMEWORK_HEAD GENERATED_PROJECT".into());
    }

    let sdk = fs::canonicalize(&arguments[0])?;
    let framework = fs::canonicalize(&arguments[1])?;
    let patches = framework::patches(&framework, &arguments[2])?;
    let project = fs::canonicalize(&arguments[3])?;
    let app = project.join("solana_wallet_app");
    let mut edits = guest(&sdk, &app)?;

    for platform in ["macos", "android", "ios"] {
        let path = project.join(platform).join("Cargo.toml");
        let contents = fs::read_to_string(&path)?;
        let dependency = format!(
            "solana_kit_leptoskit = {{ path = {:?}, features = [\"wallet-host\"] }}\n",
            sdk.join("crates/solana_kit_leptoskit")
        );
        edits.push((path, dependency_entry(&contents, &dependency)?));
    }

    let config = project.join("leptoskit.toml");
    let contents = fs::read_to_string(&config)?;
    let source = contents.replace("plugins = []", "plugins = [\"solana-wallet\"]");

    if !source.contains("plugins = [\"solana-wallet\"]") {
        return Err("expected the generated empty native plugin selection".into());
    }

    edits.push((config, source));
    let manifest = project.join("Cargo.toml");
    let contents = fs::read_to_string(&manifest)?;
    edits.push((manifest, framework::workspace(&contents, &patches)?));

    for (path, _) in &edits {
        destination(&project, path)?;
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
        ("app.rs.txt", "mod.rs"),
        ("flow.rs.txt", "flow.rs"),
        ("local.rs.txt", "flow/local.rs"),
        ("mobile.rs.txt", "flow/mobile.rs"),
        ("approval.rs.txt", "flow/approval.rs"),
    ] {
        edits.push((
            app.join("src/wallet_fixture").join(destination),
            fs::read_to_string(templates.join(template))?,
        ));
    }

    let manifest = app.join("Cargo.toml");
    let mut contents = fs::read_to_string(&manifest)?;

    for dependency in guest_dependencies(sdk) {
        contents = dependency_entry(&contents, &dependency)?;
    }

    edits.push((manifest, contents));
    Ok(edits)
}

fn guest_dependencies(sdk: &Path) -> Vec<String> {
    let adapter = sdk.join("crates/solana_kit_leptoskit");
    let core = sdk.join("crates/solana_kit_core");
    let wallet = sdk.join("crates/solana_kit_wallet");
    vec![
        format!("solana_kit_leptoskit = {{ path = {adapter:?}, features = [\"wallet-session\", \"wallet-ui\"] }}\n"),
        format!("solana_kit_core = {{ path = {core:?}, features = [\"crypto\"] }}\n"),
        format!("solana_kit_wallet = {{ path = {wallet:?}, features = [\"mwa\"] }}\n"),
        "leptoskit_bridge = { version = \"=0.1.0\", git = \"https://github.com/ifiokjr/leptoskit\", rev = \"3c6383a17f786172d696e1e83d8026cba2085fa4\", features = [\"guest\"] }\n".into(),
        "futures = { version = \"=0.3.34\", default-features = false, features = [\"executor\", \"std\"] }\n".into(),
        "base64 = \"=0.22.1\"\n".into(),
        "serde_json = \"=1.0.154\"\n".into(),
    ]
}

fn dependency_entry(contents: &str, dependency: &str) -> Result<String> {
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

    let marker = "[dependencies]\n";

    if contents.matches(marker).count() != 1 {
        return Err("expected one generated dependency section".into());
    }

    Ok(contents.replacen(marker, &format!("{marker}{dependency}"), 1))
}
