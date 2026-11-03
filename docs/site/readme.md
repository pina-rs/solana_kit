# solana_kit_docs_site

The binary that builds and serves the solana_kit documentation site at <https://kit.pina.rs/>.

The book itself is Markdown in [`docs/book`](../book), listed by `SUMMARY.md`. [`leptoskit_docs`](https://github.com/ifiokjr/leptoskit/tree/main/crates/leptoskit_docs) turns it into a static site: MDX-style Markdown with leptos components, navigation, search, and build-time highlighting. This crate adds the [skribble](https://github.com/openbudgetfun/skribble) look, the same hand-drawn paper and ink as the Dart solana_kit site:

- `src/ink.rs` draws rough rectangles, ellipses, rules, underlines, loops, hatching, and the brand mark with [`skribble_rough`](https://github.com/openbudgetfun/skribble/tree/main/rust/rough). Every build writes them into `docs/book/theme/ink/` (ignored by git). The same seed always draws the same wobble, so the files are stable between builds.
- `docs/book/theme/skribble.css` restyles leptoskit_docs' page with the skribble palette for light and dark paper, the Skribble Noto Sketch and Skribble Mono Playful fonts (OFL, in `theme/fonts`), and the ink files as CSS masks, so every stroke follows the theme without JavaScript.
- `src/lib.rs` registers the components only these docs use: `<Hero>`, `<Highlight>`, and `<Platforms>` with `<Platform>`. leptoskit_docs' own `<Callout>`, `<Steps>`, `<Cards>`, and `<Tabs>` work too.

The site builds outside the SDK workspace, with its own lockfile, so leptos never enters the SDK's dependency graph.

```sh
cargo run --manifest-path docs/site/Cargo.toml -- build              # into docs/dist
cargo run --manifest-path docs/site/Cargo.toml -- serve --port 4410  # rebuild and reload on every edit
cargo test --manifest-path docs/site/Cargo.toml                       # build the real book in a temporary directory
```

The [`docs` workflow](../../.github/workflows/docs.yml) checks, tests, and builds the site on every pull request that touches `docs/book` or `docs/site`, and deploys it to GitHub Pages from `main`. `docs/book/CNAME` serves it at `kit.pina.rs`; GitHub redirects <https://pina-rs.github.io/solana_kit/> there.

The build is strict: an unknown component or attribute, a tag that is never closed, or a link to a missing page or heading fails it with the file, line, and column. [These docs](../book/contributing/docs.md) covers writing pages.
