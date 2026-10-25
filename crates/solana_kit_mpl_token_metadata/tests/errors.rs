//! Independent MIT Dart message-map oracle covers each typed public error and unknown codes.
use solana_kit_mpl_token_metadata::MplTokenMetadataError;

#[cfg_attr(test, test)]
pub fn every_error_name_code_and_message() {
    const KNOWN_ERRORS: u32 = 203;
    const UNRELATED_CODE: u32 = 4096;
    let source = include_str!("fixtures/errors.dart.txt");
    let map = source
        .split_once("const Map<int, String> _mplTokenMetadataErrorMessages = {")
        .unwrap()
        .1
        .split_once("};")
        .unwrap()
        .0;
    let mut count = 0;

    for (index, entry) in map.split("mplTokenMetadataError").skip(1).enumerate() {
        let (name, value) = entry.split_once(':').unwrap();
        let message = value
            .trim()
            .strip_prefix('\'')
            .unwrap()
            .strip_suffix("',")
            .unwrap()
            .replace("\\'", "'")
            .replace("\\\\", "\\");
        let code = u32::try_from(index).unwrap();
        let error = MplTokenMetadataError::from_code(code);
        assert_eq!(error.code(), code);
        assert_eq!(format!("{error:?}"), name.trim());
        assert_eq!(error.message(), Some(message.as_str()));

        count += 1;
    }

    assert_eq!(count, 203);
    assert_eq!(MplTokenMetadataError::NotRentExempt.code(), 2);
    assert_eq!(
        MplTokenMetadataError::ConditionsForClosingNotMet.code(),
        202
    );

    for code in [KNOWN_ERRORS, UNRELATED_CODE, u32::MAX] {
        let unknown = MplTokenMetadataError::from_code(code);
        assert_eq!(unknown, MplTokenMetadataError::Unknown(code));
        assert_eq!(unknown.code(), code);
        assert_eq!(unknown.message(), None);
    }
}
