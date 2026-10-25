//! Checks supported command arities without executing expensive verification journeys.

use super::{Command, parse};
use anyhow::Result;
use std::ffi::OsString;

/// Converts textual test cases to the executable's operating-system argument format.
fn parse_command(arguments: &[&str]) -> Result<Command> {
    let arguments: Vec<_> = arguments.iter().map(OsString::from).collect();
    parse(&arguments)
}

#[test]
fn all_journeys_accept_only_the_command() {
    for name in [
        "native",
        "browser",
        "wasi",
        "leptoskit",
        "gates",
        "dependency-graphs",
        "consumer",
        "all",
    ] {
        assert!(parse_command(&[name]).is_ok(), "{name}");
        assert!(parse_command(&[name, "extra"]).is_err(), "{name}");
    }
}

#[test]
fn style_preserves_its_optional_root() {
    let default = parse_command(&["style"]).expect("default style root");
    assert!(matches!(default, Command::Style(None)));
    let command = parse_command(&["style", "root with spaces"]).expect("style root");
    let Command::Style(Some(path)) = command else {
        panic!("style must retain its root");
    };
    assert_eq!(path.as_os_str(), "root with spaces");
}

#[test]
fn file_commands_require_one_path() {
    for name in [
        "style-report",
        "imports",
        "dependency-fixture",
        "package-review",
        "package-report",
        "registry-fixture",
        "prepare-wallet",
    ] {
        assert!(parse_command(&[name]).is_err(), "{name}");
        assert!(parse_command(&[name, "fixture"]).is_ok(), "{name}");
        assert!(parse_command(&[name, "--offline"]).is_err(), "{name}");
        assert!(
            parse_command(&[name, "fixture", "extra"]).is_err(),
            "{name}"
        );
    }
}

#[test]
fn rejects_missing_and_unknown_commands() {
    assert!(parse_command(&[]).is_err());
    assert!(parse_command(&["unknown"]).is_err());
}
