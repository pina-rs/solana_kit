use solana_kit_core::codecs::{Error, remove_null_characters};

#[test]
fn removing_nulls_preserves_every_bom_and_unicode_character() {
    for count in 0..=8 {
        let boms = "\u{feff}".repeat(count);
        let cases = [
            (format!("{boms}\0a😀é\0z"), format!("{boms}a😀éz")),
            (format!("\0a😀\0{boms}é\0z"), format!("a😀{boms}éz")),
            (format!("\0a😀é\0z{boms}\0"), format!("a😀éz{boms}")),
            (format!("{boms}\0"), boms),
        ];

        for (input, expected) in cases {
            assert_eq!(
                remove_null_characters(&input, expected.len()).unwrap(),
                expected
            );

            if !expected.is_empty() {
                assert_eq!(
                    remove_null_characters(&input, expected.len() - 1),
                    Err(Error::LimitExceeded)
                );
            }
        }
    }
}

#[test]
fn empty_and_all_null_inputs_need_no_output_budget() {
    for input in ["", "\0", "\0\0\0\0", "\0\0\0\0\0\0\0\0"] {
        assert_eq!(remove_null_characters(input, 0).unwrap(), "");
    }
}
