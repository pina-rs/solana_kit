# These docs

This book is plain Markdown in `docs/book`. The `docs/site` crate builds it into a static site with [leptoskit_docs](https://github.com/ifiokjr/leptoskit), which handles pages, navigation, search, highlighting and link checks. The hand-drawn look comes from the rough engine in [skribble](https://github.com/openbudgetfun/skribble) (`skribble_rough`). To fix a typo or add a page, you only edit Markdown.

## Build and preview

The site crate has its own `[workspace]` and lockfile, so its Leptos and tree-sitter dependencies never enter the SDK's `Cargo.lock`. Run it with `--manifest-path` from the repository root:

```sh
# Write the static site to docs/dist
cargo run --manifest-path docs/site/Cargo.toml -- build

# Build, serve on 127.0.0.1:4410, and rebuild and reload on every edit
cargo run --manifest-path docs/site/Cargo.toml -- serve
```

Both commands accept `--out <DIR>` to write somewhere other than `docs/dist`. `serve` also takes `--port <PORT>`. Once it is running, open `http://127.0.0.1:4410/`.

The build is strict. An unknown component, an unknown or missing attribute, an unclosed tag, a broken link or a missing anchor fails it with a diagnostic. Run `build` before you push.

## Layout

| Path                           | Contents                                                                            |
| ------------------------------ | ----------------------------------------------------------------------------------- |
| `docs/book/SUMMARY.md`         | The table of contents. Only pages listed here are built and linkable.               |
| `docs/book/readme.md`          | The home page                                                                       |
| `docs/book/<section>/*.md`     | Pages, grouped by sidebar section                                                   |
| `docs/book/theme/skribble.css` | The paper-and-ink stylesheet                                                        |
| `docs/book/theme/fonts/`       | Skribble Noto Sketch and Skribble Mono Playful (`.woff`), each with its OFL license |
| `docs/book/theme/ink/`         | Generated SVG ink. Gitignored.                                                      |
| `docs/site/src/lib.rs`         | Site configuration, custom components and the CLI                                   |
| `docs/site/src/ink.rs`         | The rough-ink generator                                                             |
| `docs/dist/`                   | Build output. Gitignored.                                                           |

To add a page, create the Markdown file and add a line for it under the right heading in `SUMMARY.md`.

## How the ink works

leptoskit_docs renders the chrome (header, sidebar, code blocks, tables, callouts, cards and the pager), so the hand-drawn strokes can't be drawn into that markup directly. Instead, every `build` or `serve` first runs `ink::write`. It draws a small set of rough shapes (boxes, cards, circles, rules, edges, underlines, loops, hatching, frames and the brand mark) into SVG files in `docs/book/theme/ink`. `skribble.css` then uses those files as CSS masks. A pseudo-element is painted in a palette color, and the mask cuts the wobbly stroke out of it. The ink therefore follows the light and dark themes with no JavaScript.

Each shape's seed is fixed, so the same seed always draws the same wobble, and the files are byte-identical between builds. `ink::write` only rewrites a file when its content changes. Don't edit the generated files. Change `ink.rs` instead.

## Writing pages

The dialect is Markdown with MDX-style components:

- Start each page with an H1 that matches its `SUMMARY.md` title, then one lead paragraph.
- Use fenced code blocks with a language (`rust`, `toml`, `sh`, `text`, `markdown`). Indented code blocks are disabled.
- Link to other pages with relative `.md` paths, for example `[Crates](../reference/crates.md)`. Link to repository files with absolute `https://github.com/pina-rs/solana_kit/...` URLs.
- A capitalized tag is a component, and every component must be closed. Lowercase tags are plain HTML. If you mention a tag in prose, put it in inline code.
- GitHub alerts (`> [!NOTE]`, `> [!TIP]`, `> [!IMPORTANT]`, `> [!WARNING]`, `> [!CAUTION]`) render as callouts.

Leave blank lines around Markdown inside a component, so it is parsed as Markdown.

## Components

leptoskit_docs provides `Callout`, `Steps`, `Cards`, `Card`, `Tabs` and `Tab`. `docs/site/src/lib.rs` adds `Hero`, `Highlight`, `Platforms` and `Platform`. Nothing else is accepted.

### Callout

`kind` is one of `note` (the default), `tip`, `important`, `warning` or `caution`. `title` is optional.

```markdown
<Callout kind="warning" title="Off-chain only">

Use Pina or the official Solana program libraries for on-chain programs.

</Callout>
```

### Steps

Wrap `###` headings. Each heading becomes a numbered step.

```markdown
<Steps>

### Install

Add the crate.

### Call

Make the first request.

</Steps>
```

### Cards and Card

`title` is required. `href` is optional, and when set it must be a valid link.

```markdown
<Cards>

<Card title="RPC" href="../core/rpc.md">Typed JSON-RPC calls.</Card>

<Card title="Signers" href="../core/signers.md">Asynchronous signing.</Card>

</Cards>
```

### Tabs and Tab

Each `Tab` needs a `label`.

```markdown
<Tabs>

<Tab label="Native">

Use `solana_kit_native`.

</Tab>

<Tab label="Browser">

Use `solana_kit_browser`.

</Tab>

</Tabs>
```

### Hero

Opens the home page. It shows the wrapped Markdown beside a large hand-drawn mark, with a "Get started" button that links to `start` (required) and a link to the repository.

```markdown
<Hero start="getting-started/installation.md">

# solana_kit

A portable Rust Solana client.

</Hero>
```

### Highlight

Draws a hand-made underline, loop or marker behind a word or phrase.

| Attribute | Values                                                | Default                               |
| --------- | ----------------------------------------------------- | ------------------------------------- |
| `kind`    | `underline`, `loop`, `marker`                         | `underline`                           |
| `tone`    | `ink`, `coral`, `sun`, `lilac`, `mint`, `sky`, `pink` | `sun` for `marker`, otherwise `coral` |
| `seed`    | An integer that picks the wobble                      | `7`                                   |

```markdown
Runs on <Highlight kind="loop">every</Highlight> host, and <Highlight kind="marker" tone="mint" seed="3">stays bounded</Highlight>.
```

### Platforms and Platform

A row of sketched cards, one for each place the SDK runs. `name` and `crate` are both required.

```markdown
<Platforms>

<Platform name="Native" crate="solana_kit_native">

Tokio HTTP/1, TLS, and WebSockets.

</Platform>

</Platforms>
```

## Site settings

`site()` in `docs/site/src/lib.rs` sets the title, the description, the published URL (`https://kit.pina.rs/`), the repository and the "edit this page" links (which open `docs/book/<path>` on `main`), the favicon and the stylesheet. The [`docs` workflow](https://github.com/pina-rs/solana_kit/blob/main/.github/workflows/docs.yml) runs `fmt`, `clippy`, the tests, and the build on every pull request that touches `docs/book` or `docs/site`, and deploys the site to GitHub Pages from `main`. `docs/book/CNAME` holds the custom domain, `kit.pina.rs`; GitHub redirects `https://pina-rs.github.io/solana_kit/` there.
