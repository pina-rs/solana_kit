use solana_kit_mpl_core::MplCoreError;

#[cfg_attr(all(test, not(target_arch = "wasm32")), test)]
pub fn all_57_known_error_names_messages_and_unknown_codes_remain_exact() {
    const FIRST_UNKNOWN_CODE: u32 = 57;
    const UNRELATED_CODE: u32 = 999;
    let rows: Vec<_> = include_str!("fixtures/errors.tsv").lines().collect();
    assert_eq!(rows.len(), 57);

    for (code, row) in rows.iter().enumerate() {
        let (name, message) = row.split_once('\t').unwrap();
        let code = u32::try_from(code).unwrap();
        let error = MplCoreError::from_code(code);
        assert_eq!(format!("{error:?}"), name);
        assert_eq!(error.message(), Some(message));
        assert_eq!(error.to_string(), message);
        assert_eq!(error.code(), code);
    }

    for code in [FIRST_UNKNOWN_CODE, UNRELATED_CODE, u32::MAX] {
        let unknown = MplCoreError::from_code(code);
        assert_eq!(unknown, MplCoreError::Unknown(code));
        assert_eq!(unknown.code(), code);
        assert_eq!(unknown.message(), None);
    }
}
