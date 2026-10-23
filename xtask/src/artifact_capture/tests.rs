//! Negative controls check real files and observable child suppression.

use super::*;
use serde_json::json;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);

/// Gives parallel tests independent persistent evidence rather than shared filenames.
fn directory() -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "artifact-capture-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&path).unwrap();
    path.canonicalize().unwrap()
}

/// Supplies the minimum real Cargo JSON fields without deriving executable paths.
fn row(path: Option<&Path>, test: bool) -> String {
    json!({"reason":"compiler-artifact", "package_id":"private-fixture",
        "target":{"name":"fixture"}, "profile":{"test":test}, "executable":path})
    .to_string()
}

/// Initializes a receipt for direct stream boundary controls.
fn manifest(target: &Path) -> Manifest {
    Manifest {
        source: target.to_owned(),
        source_files: Vec::new(),
        producers: Vec::new(),
        command: vec!["test".into()],
        target: target.to_owned(),
        artifacts: Vec::new(),
        cargo_status: Some(0),
        complete: true,
    }
}

#[test]
fn mixed_output_null_library_and_same_basename_preserve_real_bytes() {
    let root = directory();
    let target = root.join("target");
    let archive = root.join("archive");
    fs::create_dir_all(target.join("debug/build/a/out")).unwrap();
    fs::create_dir_all(target.join("debug/build/b/out")).unwrap();
    fs::create_dir(&archive).unwrap();
    let first = target.join("debug/build/a/out/space λ");
    let second = target.join("debug/build/b/out/space λ");
    fs::write(&first, b"first mode").unwrap();
    fs::write(&second, b"second mode").unwrap();
    let rows = format!(
        concat!(
            "compiler-artifact starts an ordinary row\n",
            "ordinary compiler-artifact in the middle\n",
            "ordinary row ends compiler-artifact\n",
            "\"compiler-artifact\"\n",
            "{{\"message\":\"compiler-artifact\"}}\n",
            "{{\"message\":\"\\\"reason\\\":\\\"compiler-artifact\\\"\"}}\n",
            "ordinary test output\n{}\n{}\n{}\nfinished\n"
        ),
        row(None, false),
        row(Some(&first), false),
        row(Some(&second), false)
    );
    let mut receipt = manifest(&target);
    stream::capture(rows.as_bytes(), &target, &archive, &mut receipt).unwrap();
    assert_eq!(receipt.artifacts.len(), 2);
    assert_eq!(
        fs::read(archive.join("0.executable")).unwrap(),
        b"first mode"
    );
    assert_eq!(
        fs::read(archive.join("1.executable")).unwrap(),
        b"second mode"
    );
    fs::write(&first, b"overwritten mode").unwrap();
    archive::atomic_json(&archive.join("manifest.json"), &receipt).unwrap();
    archive::verify(&archive).unwrap();
    assert_eq!(
        fs::read(archive.join("0.executable")).unwrap(),
        b"first mode"
    );
    assert_eq!(
        fs::read(archive.join("cargo.stdout")).unwrap(),
        rows.as_bytes()
    );
}

#[test]
fn missing_outside_and_malformed_artifacts_reject_but_keep_full_stream() {
    let root = directory();
    let target = root.join("target");
    fs::create_dir(&target).unwrap();
    let outside = root.join("outside");
    fs::write(&outside, b"outside").unwrap();
    let inputs = [
        row(Some(&target.join("missing")), true),
        row(Some(&outside), true),
        "{\"reason\":\"compiler-artifact\",invalid}".into(),
        "{\"message\":\"ordinary\",\"reason\":\"compiler-artifact\",invalid}".into(),
        "{\"re\\u0061son\":\"compiler-artifact\",invalid}".into(),
        "{\"reason\":\"compiler-artifact\",\"executable\":null}".into(),
    ];

    for (index, input) in inputs.iter().enumerate() {
        let archive = root.join(index.to_string());
        fs::create_dir(&archive).unwrap();
        let raw = format!("{input}\nlast ordinary row\n");
        assert!(
            stream::capture(raw.as_bytes(), &target, &archive, &mut manifest(&target)).is_err()
        );
        assert_eq!(
            fs::read(archive.join("cargo.stdout")).unwrap(),
            raw.as_bytes()
        );
    }
}

#[test]
fn corrupt_prior_bytes_prevent_even_producer_child_start() {
    let root = directory();
    let target = root.join("target");
    let prior = root.join("prior");
    let toolchain = root.join("marker toolchain");
    fs::create_dir(&target).unwrap();
    fs::create_dir(&prior).unwrap();
    fs::create_dir(&toolchain).unwrap();
    let mut receipt = manifest(&target);
    fs::write(prior.join("0.executable"), b"original").unwrap();
    let mut copy = FileHash::inspect(&prior.join("0.executable")).unwrap();
    copy.path = "0.executable".into();
    receipt.artifacts.push(archive::Artifact {
        original: target.join("old"),
        package: "fixture".into(),
        target: "fixture".into(),
        is_test: false,
        copy,
        tests: None,
    });
    archive::atomic_json(&prior.join("manifest.json"), &receipt).unwrap();
    archive::atomic_json(
        &target.join(".artifact-capture-history.json"),
        &vec![prior.clone()],
    )
    .unwrap();
    fs::write(prior.join("0.executable"), b"corrupt").unwrap();
    let config = Config {
        source: root.clone(),
        source_files: vec!["source.rs".into()],
        target,
        archive: root.join("new archive"),
        toolchain,
        expected_release: "1.99.1".into(),
        args: vec!["test".into()],
        required_tests: true,
    };
    let error = capture(config).unwrap_err().to_string();
    assert!(error.contains("corrupt"), "{error}");
    assert!(!root.join("new archive").exists());
}

#[test]
fn ordinary_binary_is_not_executed_as_a_test() {
    let root = directory();
    let mut receipt = manifest(&root);
    receipt.artifacts.push(archive::Artifact {
        original: root.join("unlaunchable"),
        package: "fixture".into(),
        target: "fixture".into(),
        is_test: false,
        copy: FileHash {
            path: "unlaunchable".into(),
            sha256: String::new(),
        },
        tests: None,
    });
    archive::list_tests(&root, &mut receipt).unwrap();
    assert!(receipt.artifacts[0].tests.is_none());
}

#[test]
fn duplicate_reported_executable_rejects_instead_of_overwriting_copy() {
    let root = directory();
    let target = root.join("target");
    let archive = root.join("archive");
    fs::create_dir(&target).unwrap();
    fs::create_dir(&archive).unwrap();
    let executable = target.join("same");
    fs::write(&executable, b"one").unwrap();
    let input = format!(
        "{}\n{}\n",
        row(Some(&executable), false),
        row(Some(&executable), false)
    );
    let mut receipt = manifest(&target);
    assert!(stream::capture(input.as_bytes(), &target, &archive, &mut receipt).is_err());
    assert_eq!(receipt.artifacts.len(), 1);
}

#[cfg(unix)]
#[test]
fn corrupt_preflight_does_not_launch_a_real_marker_producer() {
    use std::os::unix::fs::PermissionsExt;

    let root = directory();
    let target = root.join("target");
    let prior = root.join("prior");
    fs::create_dir(&target).unwrap();
    fs::create_dir(&prior).unwrap();
    let producer = root.join("cargo");
    fs::write(
        &producer,
        b"#!/bin/sh\nprintf started > \"$0.marker\"\nprintf 'cargo 1.99.1\\n'\n",
    )
    .unwrap();
    fs::set_permissions(&producer, fs::Permissions::from_mode(0o755)).unwrap();
    assert!(
        Command::new(&producer)
            .arg("--version")
            .status()
            .unwrap()
            .success()
    );
    let marker = root.join("cargo.marker");
    assert!(marker.exists());
    fs::rename(&marker, root.join("positive.marker")).unwrap();
    let mut receipt = manifest(&target);
    fs::write(prior.join("0.executable"), b"good").unwrap();
    let mut copy = FileHash::inspect(&prior.join("0.executable")).unwrap();
    copy.path = "0.executable".into();
    receipt.artifacts.push(archive::Artifact {
        original: target.join("old"),
        package: "fixture".into(),
        target: "fixture".into(),
        is_test: false,
        copy,
        tests: None,
    });
    archive::atomic_json(&prior.join("manifest.json"), &receipt).unwrap();
    archive::atomic_json(
        &target.join(".artifact-capture-history.json"),
        &vec![prior.clone()],
    )
    .unwrap();
    fs::write(prior.join("0.executable"), b"bad").unwrap();
    let config = Config {
        source: root.clone(),
        source_files: vec!["source.rs".into()],
        target,
        archive: root.join("next"),
        toolchain: root.clone(),
        expected_release: "1.99.1".into(),
        args: vec!["test".into()],
        required_tests: true,
    };
    assert!(capture(config).unwrap_err().to_string().contains("corrupt"));
    assert!(!root.join("cargo.marker").exists());
    assert!(!root.join("next").exists());
}

#[test]
fn missing_copy_and_manifest_path_escape_reject() {
    let root = directory();
    let archive = root.join("archive");
    fs::create_dir(&archive).unwrap();
    fs::write(root.join("outside"), b"outside").unwrap();

    for path in ["missing", "../outside"] {
        let mut receipt = manifest(&root);
        receipt.artifacts.push(archive::Artifact {
            original: root.join("original"),
            package: "fixture".into(),
            target: "fixture".into(),
            is_test: false,
            copy: FileHash {
                path: path.into(),
                sha256: String::new(),
            },
            tests: None,
        });
        archive::atomic_json(&archive.join("manifest.json"), &receipt).unwrap();
        assert!(archive::verify(&archive).is_err());
    }
}
