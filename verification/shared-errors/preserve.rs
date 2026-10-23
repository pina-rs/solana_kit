//! Preserves actual Cargo test executable bytes and inventories before another mode.
use std::{collections::BTreeSet, env, fs, path::Path, process::Command};

const EXPECTED_EXECUTABLES: usize = 36;
const SHA_BITS: &str = "256";
const QUERY: &str = "fromjson? | select(.reason == \"compiler-artifact\" and .profile.test == true and .executable != null) | .executable";

fn main() {
    let arguments: Vec<_> = env::args().collect();
    let input = Path::new(&arguments[1]);
    let directory = Path::new(&arguments[2]);
    let result = Command::new("jq")
        .args(["-Rr", QUERY])
        .arg(input)
        .output()
        .expect("Cargo JSON parser");
    assert!(result.status.success(), "Cargo JSON parser failed");
    let paths = String::from_utf8(result.stdout).expect("UTF-8 executable paths");
    let paths: BTreeSet<_> = paths.lines().collect();
    assert_eq!(paths.len(), EXPECTED_EXECUTABLES);

    fs::create_dir(directory).expect("fresh preservation directory");
    let mut names = BTreeSet::new();
    let mut inventory = String::new();

    for path in paths {
        let source = Path::new(path);
        let name = source.file_name().expect("executable name");
        assert!(names.insert(name), "duplicate executable destination");
        let destination = directory.join(name);
        fs::copy(source, &destination).expect("preserved actual executable bytes");
        assert_eq!(
            fs::metadata(source).unwrap().len(),
            fs::metadata(&destination).unwrap().len()
        );
        let source_hash = digest(source);

        let copied_hash = digest(&destination);
        assert_eq!(source_hash, copied_hash);
        let listed = Command::new(&destination)
            .arg("--list")
            .output()
            .expect("preserved executable inventory");
        assert!(listed.status.success(), "test inventory failed");
        fs::write(destination.with_extension("list"), listed.stdout).expect("test inventory bytes");
        inventory.push_str(&format!(
            "{copied_hash}\t{}\t{path}\n",
            fs::metadata(&destination).unwrap().len()
        ));
    }

    fs::write(directory.join("inventory.tsv"), inventory).expect("artifact identity inventory");
    println!(
        "36 actual executable copies, source/copy hashes, byte sizes and executed lists validated"
    );
}

fn digest(path: &Path) -> String {
    let result = Command::new("shasum")
        .args(["-a", SHA_BITS])
        .arg(path)
        .output()
        .expect("SHA-256");
    assert!(result.status.success(), "SHA-256 failed");
    String::from_utf8(result.stdout)
        .expect("digest text")
        .split_whitespace()
        .next()
        .expect("digest")
        .to_owned()
}
