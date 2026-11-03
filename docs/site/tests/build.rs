//! Builds the real book, the way CI and the deploy do.

use solana_kit_docs_site::INK;
use solana_kit_docs_site::book;
use solana_kit_docs_site::ink;
use solana_kit_docs_site::site;

#[test]
fn every_chapter_in_the_summary_builds() {
    ink::write(&book().join(INK)).expect("the ink is drawn");
    let out = tempfile::tempdir().expect("a temporary directory");

    let build = site(book())
        .build(out.path())
        .unwrap_or_else(|error| panic!("{error}"));

    let summary = std::fs::read_to_string(book().join("SUMMARY.md")).expect("SUMMARY.md");
    let chapters = summary.matches(".md)").count();
    assert_eq!(build.pages, chapters);

    for (name, _) in ink::files() {
        assert!(
            out.path().join(INK).join(&name).is_file(),
            "{name} was not copied into the site"
        );
    }
    assert!(out.path().join("theme/skribble.css").is_file());
    assert!(
        out.path()
            .join("theme/fonts/SkribbleNotoSketch-Regular.woff")
            .is_file()
    );
}
