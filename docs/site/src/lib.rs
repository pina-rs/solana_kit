//! The solana_kit documentation site: the book in `docs/book`, built with
//! `leptoskit_docs` and drawn by hand with skribble's rough engine.
//!
//! `leptoskit_docs` renders the pages, navigation, search, and highlighting.
//! This crate adds the skribble look: the paper-and-ink stylesheet in
//! `docs/book/theme`, the rough ink files it masks with (see [`ink`]), and the
//! few components only these docs use.

pub mod ink;

use std::path::Path;
use std::path::PathBuf;

use leptos::prelude::*;
use leptoskit_docs::Components;
use leptoskit_docs::PropError;
use leptoskit_docs::Props;
use leptoskit_docs::Server;
use leptoskit_docs::Site;
use skribble_rough::FillStyle;
use skribble_rough::Point;

use crate::ink::Canvas;
use crate::ink::Doodle;
use crate::ink::Mark;
use crate::ink::Palette;
use crate::ink::Shape;
use crate::ink::Tone;

/// Where the site is published.
pub const URL: &str = "https://kit.pina.rs/";

/// The repository the docs live in.
pub const REPOSITORY: &str = "https://github.com/pina-rs/solana_kit";

/// Where [`ink::write`] puts the ink, relative to the book.
pub const INK: &str = "theme/ink";

/// The `docs` directory this crate lives in.
fn docs() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the site crate lives in docs/")
}

/// The book: `docs/book`, beside this crate.
pub fn book() -> PathBuf {
    docs().join("book")
}

/// The solana_kit site for the book in `root`.
pub fn site(root: impl Into<PathBuf>) -> Site {
    Site::new(root)
        .title("solana_kit")
        .description(
            "A portable Rust Solana client for native applications, browser WebAssembly, WASI, \
             and embedded WebAssembly.",
        )
        .url(URL)
        .repository(REPOSITORY)
        .edit_url(format!("{REPOSITORY}/edit/main/docs/book/{{path}}"))
        .favicon(format!("{INK}/mark-light.svg"))
        .stylesheet("theme/skribble.css")
        .components(components())
}

/// The built-in components and the ones only these docs use:
///
/// - `<Hero start="…">` opens the home page: the Markdown it wraps beside a
///   large hand-drawn mark, and a call to action.
/// - `<Highlight kind="underline|loop|marker" tone="…" seed="…">`: a word
///   underlined, looped, or marked by hand.
/// - `<Platforms>` holding `<Platform name="…" crate="…">`: a row of
///   sketched cards, one per place the SDK runs.
pub fn components() -> Components {
    Components::new()
        .with("Hero", |props, children| {
            let start = props
                .url("start")
                .ok_or_else(|| PropError::new("<Hero> needs a `start` link"))??;
            Ok(view! { <Hero start>{children()}</Hero> }.into_any())
        })
        .with("Highlight", highlight)
        .with("Platforms", |_props, children| {
            Ok(view! { <div class="sk-platforms">{children()}</div> }.into_any())
        })
        .with("Platform", |props, children| {
            let name = props.require("name")?.to_owned();
            let krate = props.require("crate")?.to_owned();
            Ok(view! { <Platform name krate>{children()}</Platform> }.into_any())
        })
}

#[component]
fn Hero(start: String, children: Children) -> impl IntoView {
    let mark = Doodle::new(ink::BRAND_WIDTH, ink::BRAND_HEIGHT, Palette::Page)
        .class("sk-hero-mark")
        .render(&ink::brand_mark(ink::BRAND_SEED));
    let arrow = hero_arrow();

    view! {
        <section class="sk-hero">
            <div class="sk-hero-copy">
                {children()}
                <p class="sk-hero-actions">
                    <a class="sk-button sk-button-primary" href=start>
                        "Get started"
                    </a>
                    <a class="sk-button" href=REPOSITORY>
                        "Source on GitHub"
                    </a>
                </p>
            </div>
            <div class="sk-hero-art" aria-hidden="true">
                <span class="sk-hero-mark-wrap" inner_html=mark />
                <span class="sk-hero-arrow" inner_html=arrow />
                <span class="sk-hero-note">"one core, every runtime"</span>
            </div>
        </section>
    }
}

/// The arrow's intrinsic width.
const ARROW_WIDTH: f64 = 72.0;
/// The arrow's intrinsic height.
const ARROW_HEIGHT: f64 = 66.0;
/// The arrow's shaft, sweeping up from the note to the mark.
const ARROW_SHAFT: [(f64, f64); 4] = [(6.0, 58.0), (18.0, 30.0), (40.0, 22.0), (62.0, 8.0)];
/// The arrow's head, at the top of the shaft.
const ARROW_HEAD: [(f64, f64); 3] = [(49.0, 7.0), (63.0, 7.0), (59.0, 20.0)];
/// The shaft's seed; the head is drawn from the next one.
const ARROW_SEED: i64 = 31;
/// How far the head wobbles; the shaft uses the default roughness.
const ARROW_HEAD_ROUGHNESS: f64 = 0.8;
/// The arrow's stroke width.
const ARROW_STROKE: f64 = 2.4;
/// The hachure gap the arrow's pens are built with; the arrow has no
/// hachure, so it only keeps the random streams stable.
const ARROW_GAP: f64 = 5.0;

/// The arrow from the hero's note up to its mark.
fn hero_arrow() -> String {
    let points = |points: &[(f64, f64)]| {
        points
            .iter()
            .map(|&(x, y)| Point::new(x, y))
            .collect::<Vec<_>>()
    };
    let shaft = ink::pen(ARROW_SEED, 1.0, FillStyle::None, ARROW_GAP).curve(&points(&ARROW_SHAFT));
    let head = ink::pen(
        ARROW_SEED + 1,
        ARROW_HEAD_ROUGHNESS,
        FillStyle::None,
        ARROW_GAP,
    )
    .linear_path(&points(&ARROW_HEAD), false);

    Doodle::new(ARROW_WIDTH, ARROW_HEIGHT, Palette::Page).render(&[
        Mark::new(shaft, Tone::Coral).width(ARROW_STROKE),
        Mark::new(head, Tone::Coral).width(ARROW_STROKE),
    ])
}

/// The kinds of hand-drawn emphasis `<Highlight>` draws.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HighlightKind {
    Underline,
    Loop,
    Marker,
}

impl std::str::FromStr for HighlightKind {
    type Err = &'static str;

    fn from_str(kind: &str) -> Result<Self, Self::Err> {
        match kind {
            "underline" => Ok(Self::Underline),
            "loop" => Ok(Self::Loop),
            "marker" => Ok(Self::Marker),
            _ => Err("expected underline, loop, or marker"),
        }
    }
}

impl std::str::FromStr for ToneName {
    type Err = &'static str;

    fn from_str(tone: &str) -> Result<Self, Self::Err> {
        Ok(Self(match tone {
            "ink" => Tone::Ink,
            "coral" => Tone::Coral,
            "sun" => Tone::Sun,
            "lilac" => Tone::Lilac,
            "mint" => Tone::Mint,
            "sky" => Tone::Sky,
            "pink" => Tone::Pink,
            _ => return Err("expected ink, coral, sun, lilac, mint, sky, or pink"),
        }))
    }
}

/// A [`Tone`] read from a prop.
struct ToneName(Tone);

/// The seed `<Highlight>` draws with when it is not given one.
const HIGHLIGHT_SEED: i64 = 7;
/// How far a highlight wobbles.
const HIGHLIGHT_ROUGHNESS: f64 = 1.2;
/// How far a platform card's frame wobbles.
const PLATFORM_ROUGHNESS: f64 = 1.8;
/// The port `serve` listens on by default.
const DEFAULT_PORT: u16 = 4410;

/// Renders `<Highlight>`: `children` underlined, looped, or marked by hand.
fn highlight(props: &Props<'_>, children: Children) -> Result<AnyView, PropError> {
    let kind = props.parse("kind")?.unwrap_or(HighlightKind::Underline);

    let default_tone = match kind {
        HighlightKind::Marker => Tone::Sun,
        HighlightKind::Underline | HighlightKind::Loop => Tone::Coral,
    };

    let tone = props
        .parse::<ToneName>("tone")?
        .map_or(default_tone, |tone| tone.0);
    let seed = props.parse("seed")?.unwrap_or(HIGHLIGHT_SEED);

    let (shape, fill, class) = match kind {
        HighlightKind::Underline => (Shape::Underline, FillStyle::None, "sk-highlight-underline"),
        HighlightKind::Loop => (Shape::Loop, FillStyle::None, "sk-highlight-loop"),
        HighlightKind::Marker => (Shape::Rectangle, FillStyle::Hachure, "sk-highlight-marker"),
    };

    let mut paths = ink::draw(shape, Canvas::WORD, seed, HIGHLIGHT_ROUGHNESS, fill);

    if kind == HighlightKind::Marker {
        paths.outline.clear();
    }

    let svg = ink::overlay(&paths, Canvas::WORD);
    let style = format!("--sk-tone: var(--{})", tone.name());

    Ok(view! {
        <span class=format!("sk-highlight {class}") style=style>
            <span class="sk-highlight-ink" inner_html=svg />
            <span class="sk-highlight-text">{children()}</span>
        </span>
    }
    .into_any())
}

#[component]
fn Platform(name: String, krate: String, children: Children) -> impl IntoView {
    let seed = ink::seed_for(&name);
    let frame = ink::overlay(
        &ink::draw(
            Shape::Rectangle,
            Canvas::CARD,
            seed,
            PLATFORM_ROUGHNESS,
            FillStyle::None,
        ),
        Canvas::CARD,
    );

    view! {
        <div class="sk-platform">
            <span class="sk-platform-ink" inner_html=frame />
            <p class="sk-platform-name">{name}</p>
            <p class="sk-platform-crate">
                <code>{krate}</code>
            </p>
            <div class="sk-platform-body">{children()}</div>
        </div>
    }
}

/// Build or serve the solana_kit documentation site.
#[derive(Debug, clap::Parser)]
#[command(name = "solana_kit_docs_site")]
pub enum Cli {
    /// Build the site.
    Build {
        /// Where to write the site. Defaults to `docs/dist`.
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Build the site, serve it, and rebuild and reload on every edit.
    Serve {
        /// Where to write the site. Defaults to `docs/dist`.
        #[arg(long)]
        out: Option<PathBuf>,
        /// The port to listen on, on 127.0.0.1.
        #[arg(long, default_value_t = DEFAULT_PORT)]
        port: u16,
    },
}

impl Cli {
    /// Runs the command. `build` returns `None` once the site is written;
    /// `serve` returns the running server, which the caller waits on.
    ///
    /// Both draw the ink into the book first, so the stylesheet's masks are
    /// always current.
    pub fn run(self) -> Result<Option<Server>, leptoskit_docs::Error> {
        let default_out = || docs().join("dist");
        let drawn = ink::write(&book().join(INK)).map_err(|error| leptoskit_docs::Error::Io {
            action: "draw the ink into",
            path: book().join(INK),
            source: error,
        })?;
        eprintln!("drew {drawn} ink files");

        match self {
            Self::Build { out } => {
                let out = out.unwrap_or_else(default_out);
                let build = site(book()).build(&out)?;
                eprintln!(
                    "built {} pages and copied {} files into {}",
                    build.pages,
                    build.files,
                    out.display()
                );
                Ok(None)
            }
            Self::Serve { out, port } => {
                let server =
                    site(book()).serve(out.unwrap_or_else(default_out), ("127.0.0.1", port))?;
                eprintln!("serving the docs at http://{}/", server.address());
                Ok(Some(server))
            }
        }
    }
}
