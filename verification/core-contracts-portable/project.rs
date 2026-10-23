//! Project unchanged accepted public test bodies into a host-neutral library.

use std::{env, fs, io, path::Path};

fn main() -> io::Result<()> {
    let root = env::args().nth(1).expect("repository path");
    let root = Path::new(&root);
    let output = root.join("verification/core-contracts-portable/src");

    fs::create_dir_all(output.join("accounts"))?;
    fs::create_dir_all(output.join("errors"))?;
    let mut entries = Vec::new();

    accounts(root, &output, &mut entries)?;
    errors(root, &output, &mut entries)?;
    registry(&output, entries)
}

fn accounts(root: &Path, output: &Path, entries: &mut Vec<(String, String)>) -> io::Result<()> {
    let mut modules = String::new();

    for name in [
        "original",
        "probe",
        "gaps",
        "contracts",
        "idioms",
        "independent",
        "assertion_gap",
    ] {
        let input = root.join(format!(
            "verification/account-contracts-independent/tests/{name}.rs"
        ));
        let text = fs::read_to_string(input)?;
        let projected = project(&text, &format!("accounts::{name}"), entries);
        fs::write(output.join(format!("accounts/{name}.rs")), projected)?;
        modules.push_str(&format!("pub mod {name};\n"));
    }

    fs::copy(
        root.join("verification/account-contracts-independent/oracle.tsv"),
        output.join("oracle.tsv"),
    )?;
    fs::write(output.join("accounts/mod.rs"), modules)
}

fn errors(root: &Path, output: &Path, entries: &mut Vec<(String, String)>) -> io::Result<()> {
    fs::copy(
        root.join("verification/shared-errors-independent/allocation.rs"),
        output.join("errors/allocation.rs"),
    )?;

    for name in ["consumer", "bounds", "more"] {
        error_file(root, output, entries, name)?;
    }

    Ok(())
}

fn error_file(
    root: &Path,
    output: &Path,
    entries: &mut Vec<(String, String)>,
    name: &str,
) -> io::Result<()> {
    let text =
        fs::read_to_string(root.join(format!("verification/shared-errors-independent/{name}.rs")))?;
    let (scope, file) = if name == "consumer" {
        ("errors".into(), "mod")
    } else {
        (format!("errors::{name}"), name)
    };

    let projected = project(&text, &scope, entries);
    let projected = projected
        .replace("mod bounds;", "pub mod bounds;")
        .replace("mod more;", "pub mod more;")
        .replace("fn main() {}\n", "");
    fs::write(output.join(format!("errors/{file}.rs")), projected)
}

fn registry(output: &Path, entries: Vec<(String, String)>) -> io::Result<()> {
    let groups = ordered_groups(entries);

    let mut library =
        String::from("//! Accepted public account and error contracts shared by all hosts.\n");
    library.push_str("pub mod accounts;\npub mod errors;\n\n");
    library.push_str("pub type Contract = (&'static str, fn());\n\n");
    library.push_str("pub fn cases() -> Vec<Contract> {\nlet mut cases = Vec::new();\n");

    for index in 0..groups.len() {
        library.push_str(&format!("cases.extend(group_{index}());\n"));
    }

    library.push_str("cases\n}\n");
    let mut mapping = String::from("case\taccepted_function\n");

    for (index, (scope, names)) in groups.into_iter().enumerate() {
        group(&mut library, &mut mapping, index, &scope, names);
    }

    fs::write(output.join("../runtime-map.tsv"), mapping)?;
    fs::write(output.join("lib.rs"), library)
}

fn ordered_groups(entries: Vec<(String, String)>) -> Vec<(String, Vec<String>)> {
    let mut groups: Vec<(String, Vec<String>)> = Vec::new();

    for (scope, name) in entries {
        if let Some((last, names)) = groups.last_mut() {
            if last == &scope {
                names.push(name);
                continue;
            }
        }

        groups.push((scope, vec![name]));
    }

    groups
}

fn group(
    library: &mut String,
    mapping: &mut String,
    index: usize,
    scope: &str,
    names: Vec<String>,
) {
    library.push_str(&format!(
        "\nfn group_{index}() -> Vec<Contract> {{\nuse crate::{scope}::*;\nvec![\n"
    ));

    for (position, name) in names.into_iter().enumerate() {
        let label = format!("core-contract-{index}-{position}");
        library.push_str(&format!("(\"{label}\", {name}),\n"));
        mapping.push_str(&format!("{label}\t{scope}::{name}\n"));
    }

    library.push_str("]\n}\n");
}

fn project(text: &str, scope: &str, entries: &mut Vec<(String, String)>) -> String {
    let mut output = String::new();
    let mut test = false;

    for line in text.lines() {
        if line == "#[test]" {
            test = true;
            continue;
        }

        if test {
            let name = line
                .strip_prefix("fn ")
                .expect("test function")
                .split('(')
                .next()
                .unwrap();
            entries.push((scope.into(), name.into()));
            output.push_str("pub ");
            test = false;
        }

        output.push_str(line);
        output.push('\n');
    }

    output
}
