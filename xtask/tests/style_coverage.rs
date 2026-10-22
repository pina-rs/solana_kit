//! Exercises the real analyzer through the public gate in an isolated authored repository.

use std::{fs, path::PathBuf, process::Command, time::SystemTime};

struct Repository(PathBuf);

impl Repository {
    /// Keeps analyzer discovery independent of this checkout and its build caches.
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("kit-style-{}-{nonce}", std::process::id()));
        fs::create_dir_all(path.join("src/bin")).expect("fixture directory");
        let repository = Self(path);
        repository.git(&["init", "--quiet"]);
        repository.write(".gitignore", b"target/\n.monochange/local/\nignored.rs\n");
        repository.write("monostyle.toml", include_bytes!("../../monostyle.toml"));
        repository.write("src/lib.rs", b"pub fn library() {}\n");
        repository.write("src/bin/catalog.rs", b"fn main() {}\n");
        repository.write(
            "src/generated.rs",
            b"// @generated\npub fn generated() {}\n",
        );
        repository.write("src/space name.rs", b"pub fn spaced() {}\n");
        repository.write(
            "--lenient",
            b"A filename must never become an analyzer option.\n",
        );
        repository.write("ignored.rs", b"pub fn tracked_ignored() {}\n");
        fs::create_dir_all(repository.0.join("src/target")).expect("authored target directory");
        repository.write("src/target/platform.rs", b"fn main() {}\n");
        fs::create_dir_all(repository.0.join(".monochange/local")).expect("local directory");
        repository.write(".monochange/local/tracked.rs", b"fn main() {}\n");
        repository.git(&["add", "."]);
        repository.git(&[
            "add",
            "--force",
            "ignored.rs",
            "src/target/platform.rs",
            ".monochange/local/tracked.rs",
        ]);
        repository.add_global_ignore();
        repository
    }

    fn git(&self, arguments: &[&str]) {
        let status = Command::new("git")
            .current_dir(&self.0)
            .args(arguments)
            .status()
            .expect("execute Git");
        assert!(status.success());
    }

    fn write(&self, path: &str, bytes: &[u8]) {
        fs::write(self.0.join(path), bytes).expect("write fixture");
    }

    /// A repository-local setting simulates a global exclude without editing user configuration.
    fn add_global_ignore(&self) {
        self.write("global-excludes", b"global.rs\n");
        let excludes = self.0.join("global-excludes");
        self.git(&[
            "config",
            "core.excludesFile",
            excludes.to_str().expect("path"),
        ]);
        self.write("global.rs", b"pub fn tracked_global() {}\n");
        self.git(&["add", "--force", "global.rs"]);
    }

    /// Uses a child process because style selects its repository working directory.
    fn check(&self, expected: Option<&str>) {
        let output = Command::new(env!("CARGO_BIN_EXE_solana_kit_xtask"))
            .arg("style")
            .arg(&self.0)
            .output()
            .expect("execute canonical style gate");
        let stderr = String::from_utf8_lossy(&output.stderr);

        if let Some(diagnostic) = expected {
            assert!(!output.status.success(), "gate unexpectedly passed");
            assert!(stderr.contains(diagnostic), "{stderr}");
        } else {
            assert!(output.status.success(), "{stderr}");
        }
    }

    /// A tiny deeply nested source must fail even when a larger library hides its aggregate.
    fn check_file_floor(&self, path: &str) {
        self.write(
            "src/lib.rs",
            "pub fn library() {}\n".repeat(1_000).as_bytes(),
        );
        let deficient = concat!(
            "fn main() {\n    if true {\n        if true {\n            if true {\n",
            "                if true {\n                    if true {\n",
            "                        println!(\"test\");\n                    }\n",
            "                }\n            }\n        }\n    }\n}\n"
        );
        self.write(path, deficient.as_bytes());
        self.check(Some(&format!("{path}: readability")));
        let report = self.report();
        assert!(report["readability"]["value"].as_f64().expect("score") >= 90.0);
        assert!(report["complexity"]["value"].as_f64().expect("score") >= 90.0);
        self.write("src/lib.rs", b"pub fn library() {}\n");
        self.write(path, b"fn main() {}\n");
    }

    fn report(&self) -> serde_json::Value {
        serde_json::from_slice(
            &fs::read(self.0.join("target/style-evidence/report.json")).expect("report"),
        )
        .expect("JSON report")
    }
}

impl Drop for Repository {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).expect("remove isolated fixture");
    }
}

#[test]
#[ignore = "requires monostyle 0.3.26; executed explicitly by the style CI job"]
fn authored_files_cannot_escape_the_actual_analyzer() {
    let repository = Repository::new();
    fs::create_dir(repository.0.join("target")).expect("ignored cache directory");
    repository.write("target/bad.rs", b"\xff");
    repository.write(".monochange/local/bad.rs", b"\xff");
    repository.check_file_floor("src/target/platform.rs");
    repository.check_file_floor(".monochange/local/tracked.rs");
    repository.check(None);
    let report = repository.report();
    let paths: Vec<_> = report["files"]
        .as_array()
        .expect("files")
        .iter()
        .map(|file| file["path"].as_str().expect("path"))
        .collect();

    for path in [
        "src/bin/catalog.rs",
        "src/generated.rs",
        "ignored.rs",
        "global.rs",
        "src/target/platform.rs",
        ".monochange/local/tracked.rs",
    ] {
        assert!(paths.contains(&path), "missing {path}");
    }

    repository.write("src/new.rs", b"pub fn new_file() {}\n");
    repository.check(None);
    repository.check_file_floor("src/bin/catalog.rs");
    repository.write("src/new.rs", b"\xff");
    repository.check(Some("authored Rust file was not scored: src/new.rs"));
    repository.write("src/new.rs", b"pub fn new_file() {}\n");
    fs::remove_file(repository.0.join("src/bin/catalog.rs")).expect("remove tracked input");
    repository.check(Some("inspect authored file: src/bin/catalog.rs"));
}
