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
    let mark = ink::doodle(
        42.0,
        45.0,
        &ink::brand_mark(11),
        Palette::Page,
        Some("sk-hero-mark"),
        None,
    );
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

/// The looping arrow from the hero's note to its mark.
fn hero_arrow() -> String {
    let curve = ink::pen(31, 1.0, FillStyle::None, 5.0).curve(&[
        Point::new(6.0, 58.0),
        Point::new(18.0, 30.0),
        Point::new(40.0, 22.0),
        Point::new(62.0, 8.0),
    ]);
    let head = ink::pen(32, 0.8, FillStyle::None, 5.0).linear_path(
        &[
            Point::new(49.0, 7.0),
            Point::new(63.0, 7.0),
            Point::new(59.0, 20.0),
        ],
        false,
    );

    ink::doodle(
        72.0,
        66.0,
        &[
            Mark::new(curve, Tone::Coral).width(2.4),
            Mark::new(head, Tone::Coral).width(2.4),
        ],
        Palette::Page,
        None,
        None,
    )
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

fn highlight(props: &Props<'_>, children: Children) -> Result<AnyView, PropError> {
    let kind = props.parse("kind")?.unwrap_or(HighlightKind::Underline);
    let default_tone = match kind {
        HighlightKind::Marker => Tone::Sun,
        HighlightKind::Underline | HighlightKind::Loop => Tone::Coral,
    };
    let tone = props
        .parse::<ToneName>("tone")?
        .map_or(default_tone, |tone| tone.0);
    let seed = props.parse("seed")?.unwrap_or(7_i64);

    let (shape, fill, class) = match kind {
        HighlightKind::Underline => (Shape::Underline, FillStyle::None, "sk-highlight-underline"),
        HighlightKind::Loop => (Shape::Loop, FillStyle::None, "sk-highlight-loop"),
        HighlightKind::Marker => (Shape::Rectangle, FillStyle::Hachure, "sk-highlight-marker"),
    };
    let mut paths = ink::draw(shape, Canvas::WORD, seed, 1.2, fill);
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
        &ink::draw(Shape::Rectangle, Canvas::CARD, seed, 1.8, FillStyle::None),
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
        #[arg(long, default_value_t = 4410)]
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
