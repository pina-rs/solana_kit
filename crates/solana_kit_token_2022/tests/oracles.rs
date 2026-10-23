use solana_kit_core::codecs::{Decoder, Encoder, Limits};
use solana_kit_token_2022::*;
const ORACLE_ITEMS: usize = 4096;
const HEX_RADIX: u32 = 16;

fn limits() -> Limits {
    Limits::new(1_000_000, ORACLE_ITEMS)
}
fn hex(text: &str) -> Vec<u8> {
    text.as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), HEX_RADIX).unwrap())
        .collect()
}
fn rows(prefix: &str) -> Vec<(&'static str, Vec<u8>)> {
    include_str!("../../../verification/codegen-design/fixtures/token-2022-oracle.tsv")
        .lines()
        .filter_map(|row| {
            let (name, rest) = row.split_once('\t').unwrap();
            name.starts_with(prefix)
                .then(|| (name, hex(rest.split('\t').next().unwrap())))
        })
        .collect()
}
fn account_frame(name: &str, bytes: &[u8]) -> Vec<u8> {
    if name.starts_with("account-mint-") {
        let value = MintCodec.decode(bytes, limits()).unwrap();

        return MintCodec.encode(&value, limits().max_bytes).unwrap();
    }

    if name.starts_with("account-token-") {
        let value = TokenCodec.decode(bytes, limits()).unwrap();

        return TokenCodec.encode(&value, limits().max_bytes).unwrap();
    }

    let value = MultisigCodec.decode(bytes, limits()).unwrap();
    MultisigCodec.encode(&value, limits().max_bytes).unwrap()
}
#[test]
fn all_twelve_structural_base_frames_match_independent_dart() {
    let rows = rows("account-");
    assert_eq!(rows.len(), 12);

    for (name, bytes) in rows {
        assert_eq!(account_frame(name, &bytes), bytes, "{name}");
    }
}
#[test]
fn all_116_bare_payloads_match_independent_dart() {
    let rows = rows("extension-");
    assert_eq!(rows.len(), 116);

    for (name, bytes) in rows {
        let mut canonical = bytes[..2].to_vec();

        if canonical != [0, 0] {
            canonical.extend_from_slice(&u16::try_from(bytes.len() - 2).unwrap().to_le_bytes());
            canonical.extend_from_slice(&bytes[2..]);
        }

        let value = ExtensionCodec.decode(&canonical, limits()).unwrap();
        assert_eq!(
            ExtensionCodec.encode(&value, limits().max_bytes).unwrap(),
            canonical,
            "{name}"
        );
    }
}
