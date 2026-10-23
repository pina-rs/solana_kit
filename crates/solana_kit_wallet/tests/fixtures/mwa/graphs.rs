//! Applies a closed normal/build package policy to the optional protocol graph.

use std::{collections::BTreeSet, env, fs};

fn packages(input: &str) -> BTreeSet<&str> {
    input
        .lines()
        .filter_map(|line| line.split_whitespace().next())
        .collect()
}

fn check(graph: &str, policy: &str) {
    let actual = packages(graph);
    let allowed = packages(policy);
    assert!(
        actual.contains("solana_kit_wallet"),
        "wallet package missing"
    );
    let forbidden: Vec<_> = actual.difference(&allowed).collect();
    assert!(
        forbidden.is_empty(),
        "forbidden normal/build package: {forbidden:?}"
    );
}

fn main() {
    let arguments: Vec<_> = env::args().skip(1).collect();
    assert_eq!(
        arguments.len(),
        2,
        "expected policy and prefix-free Cargo graph"
    );
    let policy = fs::read_to_string(&arguments[0]).unwrap();
    let graph = fs::read_to_string(&arguments[1]).unwrap();
    check(&graph, &policy);
    let injected = format!("{graph}\ngetrandom v0.3.0\n");
    assert!(
        std::panic::catch_unwind(|| check(&injected, &policy)).is_err(),
        "host RNG dependency negative accepted"
    );
    println!("closed protocol normal/build graph accepted; injected host RNG dependency rejected");
}
