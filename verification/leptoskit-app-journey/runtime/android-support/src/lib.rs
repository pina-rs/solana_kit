//! Empty Rust crate; the generator consumes this package's Android sources.
#![forbid(unsafe_code)]

#[cfg(test)]
mod tests {
    use std::fs;

    const SOURCES: [&str; 3] = [
        "AssociationUri.kt",
        "ForegroundActivity.kt",
        "MwaPlatform.kt",
    ];
    const SDK: &str = "../../../../crates/solana_kit_leptoskit/android";

    /// The composed runtime republishes the SDK host sources for the generator.
    /// Any difference is a second implementation, so it must fail here first.
    #[test]
    fn support_sources_match_the_sdk() {
        for source in SOURCES {
            let copy = fs::read(format!("android/{source}")).unwrap();
            let original = fs::read(format!("{SDK}/{source}")).unwrap();
            assert_eq!(copy, original, "Android support source drifted: {source}");
        }
    }
}
