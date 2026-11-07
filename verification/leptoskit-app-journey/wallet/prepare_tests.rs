//! Exercises preparation against minimal copies of the public generator layout.
use super::{NativeRuntime, guest_entry, plan, write};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);
const PLATFORMS: [&str; 5] = ["ios", "android", "macos", "windows", "linux"];
const ENTRY: &str = concat!(
    "pub extern \"C\" fn leptoskit_main() {\n",
    "    let state = Counter::default();\n",
    "\tleptoskit::guest::mount(move || screen(&state));\n",
    "}\n\n",
    "pub extern \"C\" fn leptoskit_event(node: u32, kind: u32, ptr: u32, len: u32) {\n",
    "    leptoskit::guest::dispatch(node, kind, ptr, len)\n",
    "}\n\n",
    "pub extern \"C\" fn leptoskit_poll() {\n",
    "\tleptoskit::guest::poll();\n",
    "}\n",
);

struct Sandbox {
    root: PathBuf,
}

impl Sandbox {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!("solana-wallet-prepare-{nonce}-{sequence}"));
        fs::create_dir(&root).unwrap();
        Self { root }
    }

    fn scaffold(&self, name: &str) -> &Self {
        let app = format!("{name}_app");
        self.put("leptoskit.toml", &config(name));
        self.put("Cargo.toml", &root_manifest(name));
        self.put(&format!("{app}/Cargo.toml"), &app_manifest(name));
        self.put(&format!("{app}/src/lib.rs"), ENTRY);
        self.put(
            &format!("{name}_backend/Cargo.toml"),
            &backend_manifest(name),
        );
        self.put(&format!("{name}_shared/Cargo.toml"), &shared_manifest(name));

        for platform in PLATFORMS {
            self.put(
                &format!("{platform}/Cargo.toml"),
                &host_manifest(name, platform),
            );
        }

        self
    }

    fn prepare(&self, runtime: NativeRuntime) {
        let edits = plan(&sdk(), &self.root, runtime).unwrap();
        write(&self.root, edits).unwrap();
    }

    fn put(&self, relative: &str, contents: &str) {
        let path = self.root.join(relative);

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        fs::write(path, contents).unwrap();
    }

    fn read(&self, relative: &str) -> String {
        fs::read_to_string(self.root.join(relative)).unwrap()
    }

    fn snapshot(&self) -> BTreeMap<PathBuf, Vec<u8>> {
        snapshot(&self.root)
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root).unwrap();
    }
}

fn sdk() -> PathBuf {
    fs::canonicalize(Path::new(env!("CARGO_MANIFEST_DIR")).join("..")).unwrap()
}

fn snapshot(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut files = BTreeMap::new();
    visit(root, root, &mut files);
    files
}

fn visit(root: &Path, directory: &Path, files: &mut BTreeMap<PathBuf, Vec<u8>>) {
    for entry in fs::read_dir(directory).unwrap() {
        let path = entry.unwrap().path();

        if path.is_dir() {
            visit(root, &path, files);
        } else {
            let relative = path.strip_prefix(root).unwrap().to_path_buf();
            files.insert(relative, fs::read(path).unwrap());
        }
    }
}

fn assert_prepared(sandbox: &Sandbox, name: &str, runtime: NativeRuntime) {
    let marker = match runtime {
        NativeRuntime::Wallet => "solana_kit_leptoskit",
        NativeRuntime::Journey => "solana_kit_app_journey_runtime",
    };

    for platform in PLATFORMS {
        let manifest = sandbox.read(&format!("{platform}/Cargo.toml"));
        let dependency = format!("{marker} = ");
        assert_eq!(
            manifest.matches(&dependency).count(),
            1,
            "{platform} dependency"
        );
        assert!(manifest.contains(&format!("version = {:?}", super::framework::VERSION)));
    }

    let config = sandbox.read("leptoskit.toml");
    let plugin = format!("plugins = [{:?}]", runtime.plugin());

    assert_eq!(config.matches(&plugin).count(), 1);
    assert!(config.contains("package_id = \"com.leptoskit.fixture\""));
    assert!(
        sandbox
            .root
            .join(format!("{name}_app/src/wallet_fixture/mod.rs"))
            .is_file()
    );

    for manifest in [
        "Cargo.toml".to_owned(),
        format!("{name}_app/Cargo.toml"),
        format!("{name}_backend/Cargo.toml"),
    ] {
        assert!(
            sandbox
                .read(&manifest)
                .contains(&format!("version = {:?}", super::framework::VERSION))
        );
    }
}

fn prepare_twice(sandbox: &Sandbox, runtime: NativeRuntime) {
    sandbox.prepare(runtime);
    let first = sandbox.snapshot();
    sandbox.prepare(runtime);
    assert_eq!(sandbox.snapshot(), first);
}

#[test]
fn prepares_default_generator_name_in_both_modes_idempotently() {
    for runtime in [NativeRuntime::Wallet, NativeRuntime::Journey] {
        let name = "solana_wallet_app";
        let sandbox = Sandbox::new();
        sandbox.scaffold(name);
        prepare_twice(&sandbox, runtime);
        assert_prepared(&sandbox, name, runtime);
    }
}

#[test]
fn prepares_custom_generator_name_in_both_modes_idempotently() {
    for runtime in [NativeRuntime::Wallet, NativeRuntime::Journey] {
        let name = "solana-kit-wallet-check";
        let sandbox = Sandbox::new();
        sandbox.scaffold(name);
        prepare_twice(&sandbox, runtime);
        assert_prepared(&sandbox, name, runtime);
        assert!(!sandbox.root.join("solana_wallet_app_app").exists());
    }
}

#[test]
fn rejects_mismatched_workspace_member_before_writing() {
    let sandbox = Sandbox::new();
    sandbox.scaffold("fixture");
    sandbox.put("fixture_backend/Cargo.toml", &backend_manifest("other"));
    let before = sandbox.snapshot();

    let error = plan(&sdk(), &sandbox.root, NativeRuntime::Wallet).unwrap_err();

    assert!(error.to_string().contains("lacks fixture_backend"));
    assert_eq!(sandbox.snapshot(), before);
}

#[test]
fn rejects_conflicting_client_package_before_writing() {
    let sandbox = Sandbox::new();
    sandbox.scaffold("fixture");
    sandbox.put(
        "leptoskit.toml",
        &config("fixture").replace(
            "client_package = \"fixture_app\"",
            "client_package = \"other_app\"",
        ),
    );
    let before = sandbox.snapshot();

    let error = plan(&sdk(), &sandbox.root, NativeRuntime::Wallet).unwrap_err();

    assert!(error.to_string().contains("conflicts with the app name"));
    assert_eq!(sandbox.snapshot(), before);
}

#[test]
fn rejects_unsafe_member_and_dependency_paths_before_writing() {
    for member in ["../outside", "/outside"] {
        let sandbox = Sandbox::new();
        sandbox.scaffold("fixture");
        let root =
            root_manifest("fixture").replace("\"linux\"]", &format!("\"linux\", {member:?}]"));
        sandbox.put("Cargo.toml", &root);
        let before = sandbox.snapshot();

        let error = plan(&sdk(), &sandbox.root, NativeRuntime::Wallet).unwrap_err();

        assert!(error.to_string().contains("relative directory"));
        assert_eq!(sandbox.snapshot(), before);
    }

    let sandbox = Sandbox::new();
    sandbox.scaffold("fixture");
    sandbox.put(
        "fixture_app/Cargo.toml",
        &app_manifest("fixture").replace("../fixture_shared", "../../outside"),
    );
    let before = sandbox.snapshot();

    let error = plan(&sdk(), &sandbox.root, NativeRuntime::Wallet).unwrap_err();

    assert!(error.to_string().contains("escaped the project"));
    assert_eq!(sandbox.snapshot(), before);
}

#[test]
fn rejects_conflicting_runtime_without_partial_writes() {
    let sandbox = Sandbox::new();
    sandbox.scaffold("fixture");
    sandbox.put(
        "leptoskit.toml",
        &config("fixture").replace("[native]\n", "[native]\nplugins = [\"other\"]\n"),
    );
    let before = sandbox.snapshot();

    let error = plan(&sdk(), &sandbox.root, NativeRuntime::Wallet).unwrap_err();

    assert!(error.to_string().contains("existing native runtime"));
    assert_eq!(sandbox.snapshot(), before);
}

#[test]
fn missing_template_leaves_every_generated_file_untouched() {
    let sandbox = Sandbox::new();
    sandbox.scaffold("fixture");
    let before = sandbox.snapshot();

    assert!(
        plan(
            &sandbox.root.join("missing-sdk"),
            &sandbox.root,
            NativeRuntime::Wallet
        )
        .is_err()
    );
    assert_eq!(sandbox.snapshot(), before);
}

#[test]
fn upgrades_previous_guest_event_and_is_idempotent() {
    let previous = "wallet_fixture::main();\nlet result = leptoskit::guest::dispatch(node, kind, ptr, len);\n\twallet_fixture::poll();\n\tresult\n}\nleptoskit::guest::poll();\n}";
    let upgraded = guest_entry(previous).unwrap();

    assert!(upgraded.contains("wallet_fixture::event(node, kind, result);"));
    assert!(upgraded.contains("leptoskit::guest::poll();\n\twallet_fixture::poll();\n}"));
    assert_eq!(guest_entry(&upgraded).unwrap(), upgraded);
    assert!(guest_entry("wallet_fixture::main();\nwallet_fixture::poll();").is_err());

    let without_poll = previous.replace("leptoskit::guest::poll();\n}", "");
    assert!(
        guest_entry(&without_poll).is_err(),
        "accepted an entry without the poll export"
    );
}

fn config(name: &str) -> String {
    format!(
        r#"schema_version = 0

[app]
name = {name:?}
default_platform = "macos"

[build]
client_package = "{name}_app"

[native]
package_id = "com.leptoskit.fixture"
"#
    )
}

fn root_manifest(name: &str) -> String {
    format!(
        r#"[package]
name = {name:?}
version = "0.1.0"
edition = "2024"
publish = false

[workspace]
resolver = "2"
members = ["{name}_app", "{name}_backend", "{name}_shared", "ios", "android", "macos", "windows", "linux"]
default-members = ["."]

[dependencies]
{name}_app = {{ path = "{name}_app", default-features = false }}
{name}_backend = {{ path = "{name}_backend" }}
leptoskit = {{ version = "=0.3.0", default-features = false, features = ["server"] }}
"#
    )
}

fn app_manifest(name: &str) -> String {
    format!(
        r#"[package]
name = "{name}_app"
version = "0.1.0"
edition = "2024"
publish = false

[dependencies]
{name}_shared = {{ path = "../{name}_shared" }}
leptoskit = {{ version = "=0.3.0", default-features = false }}

[build-dependencies]
leptoskit_build = {{ version = "=0.3.0", default-features = false }}
"#
    )
}

fn backend_manifest(name: &str) -> String {
    format!(
        r#"[package]
name = "{name}_backend"
version = "0.1.0"
edition = "2024"
publish = false

[dependencies]
{name}_shared = {{ path = "../{name}_shared" }}
leptoskit = {{ version = "=0.3.0", default-features = false, features = ["server"] }}
"#
    )
}

fn shared_manifest(name: &str) -> String {
    format!(
        r#"[package]
name = "{name}_shared"
version = "0.1.0"
edition = "2024"
publish = false
"#
    )
}

fn host_manifest(name: &str, platform: &str) -> String {
    format!(
        r#"[package]
name = "{name}_{platform}"
version = "0.1.0"
edition = "2024"
publish = false

[lib]
name = "leptoskit_app"
crate-type = ["staticlib"]

[target.'cfg(target_os = "{platform}")'.dependencies]
leptoskit_bridge_ffi = {{ version = "=0.3.0", default-features = false, features = ["app"] }}
"#
    )
}
