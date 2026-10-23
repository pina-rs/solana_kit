//! Rejects all imports in the plain Wasm consumer, including injected host RNG.

use std::{env, fs};

fn unsigned(bytes: &mut &[u8]) -> u32 {
    let mut value = 0;
    let mut shift = 0;

    loop {
        let (&byte, rest) = bytes.split_first().expect("truncated Wasm LEB128");
        *bytes = rest;
        value |= u32::from(byte & 0x7f) << shift;

        if byte & 0x80 == 0 {
            return value;
        }

        shift += 7;
        assert!(shift < 35, "oversized Wasm LEB128");
    }
}

fn check(bytes: &[u8]) {
    assert!(
        bytes.starts_with(b"\0asm\x01\0\0\0"),
        "expected core Wasm module"
    );
    let mut sections = &bytes[8..];

    while let Some((&kind, rest)) = sections.split_first() {
        sections = rest;
        let length = usize::try_from(unsigned(&mut sections)).unwrap();
        let (mut section, rest) = sections
            .split_at_checked(length)
            .expect("truncated section");
        sections = rest;

        if kind == 2 {
            assert_eq!(unsigned(&mut section), 0, "forbidden plain guest import");
        }
    }
}

fn main() {
    let arguments: Vec<_> = env::args().skip(1).collect();
    assert_eq!(arguments.len(), 1, "expected compiled plain Wasm consumer");
    check(&fs::read(&arguments[0]).unwrap());
    let forbidden = b"\0asm\x01\0\0\0\x01\x04\x01\x60\x00\x00\x02\x0c\x01\x04host\x03rng\x00\x00";
    assert!(
        std::panic::catch_unwind(|| check(forbidden)).is_err(),
        "host RNG negative accepted"
    );
    println!("compiled plain consumer has zero imports; injected host RNG rejected");
}
