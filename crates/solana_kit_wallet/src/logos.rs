use crate::{MetadataError, WalletIcon, logo_assets::LOGOS};
use alloc::{format, string::ToString};

/// Parses the exact neutral glyph bundled by the pinned Dart package.
pub fn generic_wallet_logo() -> Result<WalletIcon, MetadataError> {
    WalletIcon::parse(include_str!("../assets/generic.uri"))
}

/// Resolves pinned logo data case-insensitively, ignoring a trailing Wallet suffix.
/// Bundled assets are returned as metadata. No network request occurs.
pub fn wallet_logo_fallback(wallet_name: &str) -> Result<Option<WalletIcon>, MetadataError> {
    let key = wallet_name
        .trim_matches(|value: char| value.is_whitespace() || value == '\u{feff}')
        .to_lowercase();
    let without_suffix = key.strip_suffix(" wallet").unwrap_or(&key);
    let with_suffix = format!("{key} wallet");
    let lookup = |name: &str| {
        LOGOS
            .iter()
            .find(|(key, _)| *key == name)
            .map(|(_, uri)| *uri)
    };
    let uri = lookup(&key)
        .or_else(|| lookup(without_suffix))
        .or_else(|| lookup(&with_suffix));
    uri.map(|uri| WalletIcon::parse(uri.to_string()))
        .transpose()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_pinned_logo_decodes() {
        assert_eq!(LOGOS.len(), 36);

        for (name, _) in LOGOS {
            assert!(wallet_logo_fallback(name).unwrap().is_some());
        }
    }
}
