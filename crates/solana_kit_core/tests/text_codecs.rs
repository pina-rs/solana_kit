use solana_kit_core::codecs::{
    Base16, BaseX, BaseXReslice, Decoder, Encoder, Error, Limits, Reader, Utf8, Utf8Config, Writer,
    pad_null_characters, remove_null_characters,
};

const MAX_BYTES: usize = 10000;
const MAX_ITEMS: usize = 100000;
const TOO_MANY_DIGITS: usize = 4097;

fn bytes(text: &str) -> Vec<u8> {
    if text.is_empty() || text == "-" {
        return vec![];
    }

    text.split(',').map(|s| s.parse().unwrap()).collect()
}

fn check(codec: &(impl Encoder<str> + Decoder<String>), input: &[u8], text: &str, wire: &[u8]) {
    let limits = Limits::new(MAX_BYTES, MAX_ITEMS);
    assert_eq!(codec.decode(input, limits).unwrap(), text);
    assert_eq!(codec.encode(text, MAX_BYTES).unwrap(), wire);
    assert_eq!(codec.encoded_len(text).unwrap(), wire.len());
    let mut storage = vec![99; wire.len() + 2];
    let mut writer = Writer::new(&mut storage[1..]);
    codec.write(text, &mut writer).unwrap();
    assert_eq!(writer.position(), wire.len());
    assert_eq!(storage[0], 99);
    assert_eq!(storage[wire.len() + 1], 99);
    let prefixed = [vec![123], input.to_vec()].concat();
    assert_eq!(
        codec.decode_prefix(&prefixed, 1, limits).unwrap(),
        (text.to_owned(), input.len() + 1)
    );
}

#[test]
fn actual_pinned_dart_fixtures() {
    let mut count = 0;

    for line in include_str!("fixtures/text-dart.tsv").lines() {
        let fields: Vec<_> = line.split('\t').collect();
        let input = bytes(fields[2]);
        let wire = bytes(fields[4]);

        match fields[1] {
            "0" => check(&BaseX::base10(), &input, fields[3], &wire),
            "1" => check(&Base16, &input, fields[3], &wire),
            "2" => check(&BaseX::base58(), &input, fields[3], &wire),
            "3" => {
                #[cfg(feature = "text-codecs")]
                check(&solana_kit_core::codecs::Base64, &input, fields[3], &wire);
            }
            "4" => check(&BaseX::new("abcdefghij").unwrap(), &input, fields[3], &wire),
            "5" => check(
                &BaseXReslice::new("01234567", 3).unwrap(),
                &input,
                fields[3],
                &wire,
            ),
            _ => panic!("unknown fixture"),
        }

        count += 1;
    }

    assert_eq!(count, 1536);
}

#[test]
fn utf8_all_configurations_match_actual_dart() {
    let mut count = 0;

    for line in include_str!("fixtures/numeric-dart.tsv")
        .lines()
        .filter(|line| line.starts_with("utf8\t"))
    {
        let fields: Vec<_> = line.split('\t').collect();
        let config: u8 = fields[1].parse().unwrap();
        let codec = Utf8Config {
            fatal: config & 1 == 0,
            ignore_bom: config & 2 != 0,
            remove_null_characters: config & 4 != 0,
        };
        let input = bytes(fields[2]);
        let result = codec.decode(&input, Limits::new(100, 100));

        if fields[3] == "INVALID" {
            assert_eq!(result, Err(Error::InvalidUtf8));
        } else {
            assert_eq!(result.unwrap().as_bytes(), bytes(fields[3]));
        }

        count += 1;
    }
    assert_eq!(count, 88);
    assert_eq!(
        Utf8.decode(&[239, 187, 191], Limits::new(3, 3)).unwrap(),
        "\u{feff}"
    );
    assert_eq!(pad_null_characters("😀", 3, 5).unwrap(), "😀\0");
    assert_eq!(pad_null_characters("abc", 2, 3).unwrap(), "abc");
    assert_eq!(
        pad_null_characters("a", usize::MAX, 4),
        Err(Error::LimitExceeded)
    );
    assert_eq!(remove_null_characters("\0a\0b", 2).unwrap(), "ab");
}

#[test]
fn malformed_text_and_shared_budgets() {
    for value in ["g", "abcg", "é", "0x00", " "] {
        assert_eq!(Base16.encode(value, 100), Err(Error::InvalidValue));
    }

    assert_eq!(Base16.encode("AbCdE", 3).unwrap(), [171, 205, 14]);
    assert_eq!(Base16.encode("abc", 1), Err(Error::LimitExceeded));
    assert_eq!(
        Base16.decode(&[1], Limits::new(1, 1)),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        Utf8Config {
            fatal: false,
            ..Utf8Config::default()
        }
        .decode(&[255], Limits::new(1, 2)),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        BaseX::base10().encode(&"0".repeat(TOO_MANY_DIGITS), MAX_BYTES),
        Err(Error::LimitExceeded)
    );
    assert_eq!(BaseX::base58().encode("0OIl", 10), Err(Error::InvalidValue));

    assert!(BaseX::new("").is_err());
    assert!(BaseX::new("x").is_err());
    assert!(BaseX::new("a😀").is_err());
    assert!(BaseXReslice::new("abc", 2).is_err());
    assert!(BaseXReslice::new("01", 0).is_err());
    assert!(BaseXReslice::new("01", 17).is_err());
    assert_eq!(
        BaseXReslice::new("01", 1).unwrap().encode("1", 0).unwrap(),
        []
    );
    let mut reader = Reader::new(&[255], Limits::new(1, 5)).unwrap();

    assert_eq!(BaseX::base10().read(&mut reader), Err(Error::LimitExceeded));
    assert_eq!(BaseX::base10().decode(&[], Limits::new(0, 0)).unwrap(), "");
}

#[test]
fn unicode_and_wide_alphabets() {
    for count in [2, 10, 120, 300] {
        let alphabet: String = (0..count)
            .map(|i| char::from_u32(0x100 + i).unwrap())
            .collect();
        let codec = BaseX::new(&alphabet).unwrap();

        for n in 0..256u16 {
            let input = [0, n.to_le_bytes()[0], 255];
            let text = codec.decode(&input, Limits::new(3, 1000)).unwrap();
            assert_eq!(codec.encode(&text, 100).unwrap(), input);
        }
    }

    let duplicate = BaseX::new("001").unwrap();

    assert_eq!(duplicate.encode("1", 1).unwrap(), [2]);
    assert_eq!(duplicate.decode(&[1], Limits::new(1, 100)).unwrap(), "0");
}

#[cfg(feature = "text-codecs")]
#[test]
fn actual_dart_base64_leniency_and_rejections() {
    use solana_kit_core::codecs::Base64;

    for text in ["AB", "AC", "AAB", "AAC", "é", "_A", "A-", "AA\n"] {
        assert_eq!(Base64.encode(text, 100), Err(Error::InvalidValue));
    }

    for text in ["A=Q", "A===Q===", "AQ", "AQ=", "AQ=="] {
        assert_eq!(Base64.encode(text, 1).unwrap(), [1]);
    }

    for text in ["=", "====", "a", ""] {
        assert!(Base64.encode(text, 0).unwrap().is_empty());
    }

    assert_eq!(Base64.encode("AABCA", 3).unwrap(), [0, 0, 66]);
    assert_eq!(Base64.encode("/////", 3).unwrap(), [255, 255, 255]);

    assert_eq!(
        Base64.decode(&[1], Limits::new(1, 3)),
        Err(Error::LimitExceeded)
    );
}

#[path = "../../../verification/conformance/src/text_numbers.rs"]
mod portable;

#[test]
fn isolated_portable_cases_execute() {
    assert_eq!(portable::text(), Ok(()));
    assert_eq!(portable::wide(), Ok(()));
    assert_eq!(portable::malformed(), Ok(()));
}

fn bounded_failure(codec: &(impl Encoder<str> + Decoder<String>), text: &str) {
    assert_eq!(
        codec.decode(&[42], Limits::new(0, MAX_ITEMS)),
        Err(Error::LimitExceeded)
    );
    assert_eq!(
        codec.decode(&[42], Limits::new(1, 0)),
        Err(Error::LimitExceeded)
    );
    let length = codec.encoded_len(text).unwrap();
    assert!(length > 0);
    let mut output = vec![99; length - 1];
    assert_eq!(
        codec.write(text, &mut Writer::new(&mut output)),
        Err(Error::Truncated)
    );
}

#[test]
fn each_codec_enforces_cursor_and_allocation_limits() {
    bounded_failure(&Base16, "1234");
    bounded_failure(&BaseX::base10(), "65535");
    bounded_failure(&BaseX::base58(), "LUv");
    bounded_failure(&BaseXReslice::new("01234567", 3).unwrap(), "77777777");
    bounded_failure(&Utf8Config::default(), "abc");
    #[cfg(feature = "text-codecs")]
    bounded_failure(&solana_kit_core::codecs::Base64, "AQID");
}
